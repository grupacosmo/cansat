//! Binary crate targeting stm32f4 family microcontrollers.
#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod error;
mod tasks;

use defmt_rtt as _;
use error::{Report, WrapErr};
use heapless::String;
use lis3dh::Lis3dh;
use panic_probe as _;
use stm32f4xx_hal::{
    gpio, i2c, pac,
    prelude::*,
    serial, spi,
    timer::{monotonic::MonoTimerUs, DelayUs},
};
use tasks::*;

type Led = gpio::PC13<gpio::Output>;

type Scl1 = gpio::PB8<gpio::Alternate<4, gpio::OpenDrain>>;
type Sda1 = gpio::PB9<gpio::Alternate<4, gpio::OpenDrain>>;
type I2c1 = i2c::I2c1<(Scl1, Sda1)>;
type I2c1Proxy = shared_bus::I2cProxy<'static, shared_bus::AtomicCheckMutex<I2c1>>;
type Bme280 = bme280::i2c::BME280<I2c1Proxy>;
type Accelerometer = Lis3dh<lis3dh::Lis3dhI2C<I2c1Proxy>>;

type Rx1 = gpio::PB7<gpio::Alternate<7>>;
type Tx1 = gpio::PB6<gpio::Alternate<7>>;
type Serial1 = serial::Serial1<(Tx1, Rx1)>;
type Gps = cansat_gps::Gps<Serial1>;

type Sck2 = gpio::PB13<gpio::Alternate<5>>;
type Miso2 = gpio::PB14<gpio::Alternate<5>>;
type Mosi2 = gpio::PB15<gpio::Alternate<5>>;
type Spi2 = spi::Spi2<(Sck2, Miso2, Mosi2)>;
type Cs2 = gpio::PB12<gpio::Output>;
type SpiDevice2 = embedded_sdmmc::SdMmcSpi<Spi2, Cs2>;
type BlockSpi2 = embedded_sdmmc::BlockSpi<'static, Spi2, Cs2>;
const MAX_DIRS: usize = 4;
const MAX_FILES: usize = 4;
type SdmmcController = embedded_sdmmc::Controller<BlockSpi2, DummyClock, MAX_DIRS, MAX_FILES>;

/// Maximal length supported by embedded_sdmmc
const MAX_FILENAME_LEN: usize = 11;

pub struct DummyClock;

impl embedded_sdmmc::TimeSource for DummyClock {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        embedded_sdmmc::Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

pub struct I2c1Devices {
    pub bme280: Bme280,
    pub accelerometer: Accelerometer,
}

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [EXTI0])]
mod app {
    use super::*;

    #[shared]
    struct Shared {
        gps: Gps,
        #[lock_free]
        i2c1_devices: I2c1Devices,
    }

    #[local]
    struct Local {
        delay: DelayUs<pac::TIM3>,
        led: Led,
        controller: SdmmcController,
        filename: String<MAX_FILENAME_LEN>,
    }

    #[monotonic(binds = TIM2, default = true)]
    type MicrosecMono = MonoTimerUs<pac::TIM2>;

    extern "Rust" {
        #[task(shared = [gps], priority = 1)]
        fn log_nmea(ctx: log_nmea::Context);

        #[task(binds = USART1, shared = [gps], priority = 2)]
        fn gps_irq(ctx: gps_irq::Context);

        #[task(local = [led], priority = 1)]
        fn blink(ctx: blink::Context);

        #[task(local = [delay], shared = [i2c1_devices], priority = 1)]
        fn bme_measure(ctx: bme_measure::Context);

        #[task(local = [controller, filename], priority = 1)]
        fn sdmmc_log(ctx: sdmmc_log::Context);
    }

    #[init(local = [spi_device2: Option<SpiDevice2> = None])]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        try_init(ctx).unwrap_or_else(|e| {
            defmt::panic!("Initialization failed: {}", e);
        })
    }

    fn try_init(ctx: init::Context) -> Result<(Shared, Local, init::Monotonics), Report> {
        defmt::info!("Initializing");

        let device = ctx.device;
        let rcc = device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();
        let mono = device.TIM2.monotonic_us(&clocks);
        let mut delay = device.TIM3.delay_us(&clocks);

        let gpiob = device.GPIOB.split();
        let gpioc = device.GPIOC.split();

        let led = gpioc.pc13.into_push_pull_output();

        let i2c1_manager = {
            let i2c1 = {
                let scl1 = gpiob
                    .pb8
                    .into_alternate()
                    .internal_pull_up(false)
                    .set_open_drain();
                let sda1 = gpiob
                    .pb9
                    .into_alternate()
                    .internal_pull_up(false)
                    .set_open_drain();
                let mode = i2c::Mode::Fast {
                    frequency: 400000.Hz(),
                    duty_cycle: i2c::DutyCycle::Ratio2to1,
                };
                device.I2C1.i2c((scl1, sda1), mode, &clocks)
            };
            shared_bus::new_atomic_check!(I2c1 = i2c1).unwrap()
        };

        let bme280 = {
            let mut bme280 = Bme280::new_primary(i2c1_manager.acquire_i2c());
            bme280
                .init(&mut delay)
                .wrap_err("Failed to initialize BME280")?;
            bme280
        };

        let accelerometer =
            Lis3dh::new_i2c(i2c1_manager.acquire_i2c(), lis3dh::SlaveAddr::Default).unwrap();

        let gps = {
            let mut usart1 = {
                let tx1 = gpiob.pb6.into_alternate();
                let rx1 = gpiob.pb7.into_alternate();
                let config = serial::Config::default().baudrate(9600.bps());
                device
                    .USART1
                    .serial((tx1, rx1), config, &clocks)
                    .wrap_err("Failed to create USART3")?
            };
            usart1.listen(serial::Event::Rxne);
            Gps::new(usart1)
        };

        let mut controller = {
            let spi_device2 = ctx.local.spi_device2;
            *spi_device2 = Some({
                let spi2 = {
                    let sck2 = gpiob.pb13.into_alternate();
                    let miso2 = gpiob.pb14.into_alternate();
                    let mosi2 = gpiob.pb15.into_alternate();
                    let mode = spi::Mode {
                        polarity: spi::Polarity::IdleLow,
                        phase: spi::Phase::CaptureOnFirstTransition,
                    };
                    device
                        .SPI2
                        .spi((sck2, miso2, mosi2), mode, 400000.Hz(), &clocks)
                };
                let cs2 = gpiob.pb12.into_push_pull_output();
                embedded_sdmmc::SdMmcSpi::new(spi2, cs2)
            });
            let block_spi2 = spi_device2
                .as_mut()
                .unwrap()
                .acquire()
                .wrap_err("Failed to acquire block spi")?;
            SdmmcController::new(block_spi2, DummyClock)
        };

        let filename = {
            let volume = controller
                .get_volume(embedded_sdmmc::VolumeIdx(0))
                .wrap_err("Failed to get volume")?;
            let root_dir = controller.open_root_dir(&volume).unwrap();

            let mut log_count = 0;
            controller
                .iterate_dir(&volume, &root_dir, |_| {
                    log_count += 1;
                })
                .wrap_err("Failed to iterate directory")?;
            controller.close_dir(&volume, root_dir);

            let mut filename = String::from(log_count);
            filename
                .push_str("_log.txt")
                .expect("Filename buffer overflow");
            filename
        };

        defmt::info!("Filename: {}", filename.as_str());

        blink::spawn().unwrap();
        sdmmc_log::spawn().unwrap();
        bme_measure::spawn().unwrap();

        let i2c1_devices = I2c1Devices {
            bme280,
            accelerometer,
        };
        let shared = Shared { gps, i2c1_devices };
        let local = Local {
            delay,
            led,
            controller,
            filename,
        };
        let monotonics = init::Monotonics(mono);

        Ok((shared, local, monotonics))
    }
}

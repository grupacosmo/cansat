//! Binary crate targeting stm32f4 family microcontrollers.
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _;

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [EXTI0])]
mod app {
    use bme280::i2c::BME280;
    use cansat_gps::Gps;
    use core::fmt::Write;
    use cortex_m::asm::nop;
    use defmt::Debug2Format;
    use embedded_sdmmc::{BlockSpi, Controller, SdMmcSpi, TimeSource, Timestamp};
    use heapless::String;
    use stm32f4xx_hal::{
        gpio::{Alternate, OpenDrain, Output, Pin, PA5, PB6, PB7, PC10, PC11},
        i2c::{self, DutyCycle, I2c1},
        pac::{self, TIM3},
        prelude::*,
        serial::{self, Event, Serial3},
        spi::{Phase, Polarity, Spi1},
        timer::{monotonic::MonoTimerUs, DelayUs},
    };

    pub struct Clock;

    impl TimeSource for Clock {
        fn get_timestamp(&self) -> Timestamp {
            Timestamp {
                year_since_1970: 0,
                zero_indexed_month: 0,
                zero_indexed_day: 0,
                hours: 0,
                minutes: 0,
                seconds: 0,
            }
        }
    }

    type Scl = PB6<Alternate<4, OpenDrain>>;
    type Sda = PB7<Alternate<4, OpenDrain>>;

    type Rx3 = PC10<Alternate<7>>;
    type Tx3 = PC11<Alternate<7>>;

    type Sck1 = Pin<'B', 3, Alternate<5>>;
    type Miso1 = Pin<'A', 6, Alternate<5>>;
    type Mosi1 = Pin<'A', 7, Alternate<5>>;
    type Cs1 = Pin<'A', 15, Output>;
    type BlockSpi1<'a> = BlockSpi<'a, Spi1<(Sck1, Miso1, Mosi1)>, Cs1>;

    /// Maximal length supported by embedded_sdmmc
    const MAX_FILENAME_LEN: usize = 11;

    #[shared]
    struct Shared {
        gps: Gps<Serial3<(Rx3, Tx3)>>,
    }

    #[local]
    struct Local {
        delay: DelayUs<TIM3>,
        led: PA5<Output>,
        bme: BME280<I2c1<(Scl, Sda)>>,
        controller: Controller<BlockSpi1<'static>, Clock, 4, 4>,
        filename: String<MAX_FILENAME_LEN>,
    }

    #[monotonic(binds = TIM2, default = true)]
    type MicrosecMono = MonoTimerUs<pac::TIM2>;

    #[init(local = [spi_dev: Option<SdMmcSpi<Spi1<(Sck1, Miso1, Mosi1)>, Cs1>> = None])]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("Initializing");

        let device = ctx.device;
        let rcc = device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();
        let mono = device.TIM2.monotonic_us(&clocks);
        let mut delay = device.TIM3.delay_us(&clocks);

        let gpioa = device.GPIOA.split();
        let gpiob = device.GPIOB.split();
        let gpioc = device.GPIOC.split();

        let i2c_scl = gpiob
            .pb6
            .into_alternate()
            .internal_pull_up(false)
            .set_open_drain();
        let i2c_sda = gpiob
            .pb7
            .into_alternate()
            .internal_pull_up(false)
            .set_open_drain();

        let i2c = I2c1::new(
            device.I2C1,
            (i2c_scl, i2c_sda),
            i2c::Mode::Fast {
                frequency: 400000.Hz(),
                duty_cycle: DutyCycle::Ratio2to1,
            },
            &clocks,
        );

        let mut bme = BME280::new_primary(i2c);
        #[cfg(feature = "bme")]
        if let Err(e) = bme.init(&mut delay) {
            defmt::panic!("Failed to initalize bme280: {}", Debug2Format(&e));
        }

        let led = gpioa.pa5.into_push_pull_output();

        let gps = {
            let usart3_tx_pin = gpioc.pc10.into_alternate();
            let usart3_rx_pin = gpioc.pc11.into_alternate();
            let mut gps_serial = device
                .USART3
                .serial(
                    (usart3_tx_pin, usart3_rx_pin),
                    serial::Config::default().baudrate(9600.bps()),
                    &clocks,
                )
                .expect("Failed to setup usart3");
            gps_serial.listen(Event::Rxne);
            Gps::new(gps_serial)
        };

        let spi1_device = {
            let sck1 = gpiob.pb3.into_alternate();
            let miso1 = gpioa.pa6.into_alternate();
            let mosi1 = gpioa.pa7.into_alternate();
            let cs1 = gpioa.pa15.into_push_pull_output();
            let spi1 = device.SPI1.spi(
                (sck1, miso1, mosi1),
                stm32f4xx_hal::spi::Mode {
                    polarity: Polarity::IdleLow,
                    phase: Phase::CaptureOnFirstTransition,
                },
                400000.Hz(),
                &clocks,
            );
            embedded_sdmmc::SdMmcSpi::new(spi1, cs1)
        };

        *ctx.local.spi_dev = Some(spi1_device);
        let mut controller = match ctx.local.spi_dev.as_mut().unwrap().acquire() {
            Ok(sdmmc_spi) => Controller::new(sdmmc_spi, Clock),
            Err(e) => {
                defmt::panic!("Failed to create sdmmc controller: {}", e);
            }
        };

        let volume = match controller.get_volume(embedded_sdmmc::VolumeIdx(0)) {
            Ok(volume) => volume,
            Err(e) => defmt::panic!("Failed to get volume: {}", e),
        };
        let root_dir = controller.open_root_dir(&volume).unwrap();

        let mut log_count = 0;
        controller
            .iterate_dir(&volume, &root_dir, |_| {
                log_count += 1;
            })
            .unwrap();
        let mut filename = String::from(log_count);
        write!(filename, "_log.txt").unwrap();
        defmt::info!("Filename: {}", filename.as_str());
        controller.close_dir(&volume, root_dir);

        let shared = Shared { gps };
        let local = Local {
            delay,
            led,
            bme,
            controller,
            filename,
        };
        let monotonics = init::Monotonics(mono);
        blink::spawn().unwrap();
        sdmmc_log::spawn("Logs dump here ".into()).unwrap();

        (shared, local, monotonics)
    }

    #[idle]
    fn idle(_ctx: idle::Context) -> ! {
        defmt::info!("Started idle task");
        loop {
            nop();
        }
    }

    #[task(shared = [gps])]
    fn log_nmea(ctx: log_nmea::Context) {
        let mut gps = ctx.shared.gps;
        let msg = gps.lock(|gps| gps.last_nmea()).unwrap();
        defmt::info!("{=[u8]:a}", &msg);
    }

    /// USART3 interrupt handler that reads data into the gps working buffer
    #[task(binds = USART3, shared = [gps])]
    fn gps_irq(ctx: gps_irq::Context) {
        let mut gps = ctx.shared.gps;
        let (_, is_terminator) = gps.lock(|gps| gps.read_uart()).unwrap();
        if is_terminator {
            log_nmea::spawn().unwrap();
        }
    }

    /// Toggles led every second
    #[task(local = [led])]
    fn blink(ctx: blink::Context) {
        let led = ctx.local.led;
        led.toggle();
        defmt::debug!("Blink");
        blink::spawn_after(1.secs()).unwrap();
    }

    #[task(local = [delay, bme])]
    fn bme_measure(ctx: bme_measure::Context) {
        #[cfg(feature = "bme")]
        {
            let bme = ctx.local.bme;
            let delay = ctx.local.delay;
            let measurements = match bme.measure(delay) {
                Ok(m) => m,
                Err(e) => {
                    defmt::error!("Could not read bme280 measurements: {}", Debug2Format(&e));
                    return;
                }
            };
            let altitude = cansat::calculate_altitude(measurements.pressure);
            defmt::info!("Altitude = {} meters above sea level", altitude);
            defmt::info!("Relative Humidity = {}%", measurements.humidity);
            defmt::info!("Temperature = {} deg C", measurements.temperature);
            defmt::info!("Pressure = {} pascals", measurements.pressure);

            bme_measure::spawn_after(5.secs()).unwrap();
        }
    }
    #[task(local = [controller, filename])]
    fn sdmmc_log(ctx: sdmmc_log::Context, log_string: String<MAX_FILENAME_LEN>) {
        let controller = ctx.local.controller;
        let filename = ctx.local.filename;
        let mut volume = match controller.get_volume(embedded_sdmmc::VolumeIdx(0)) {
            Ok(volume) => volume,
            Err(e) => defmt::panic!("Failed to get volume: {}", e),
        };

        let root_dir = controller.open_root_dir(&volume).unwrap();

        let mut f = controller
            .open_file_in_dir(
                &mut volume,
                &root_dir,
                filename,
                embedded_sdmmc::Mode::ReadWriteCreateOrAppend,
            )
            .unwrap();
        let num_written = controller
            .write(&mut volume, &mut f, log_string.as_bytes())
            .unwrap();
        defmt::info!("Written: {} bytes\n", num_written);
        if let Err(e) = controller.close_file(&volume, f) {
            defmt::panic!("Failed to close file: {}", e);
        }
    }
}

//! Binary crate targeting stm32f4 family microcontrollers.
#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod error;
mod sd_logger;
mod startup;
mod tasks;

pub use sd_logger::SdLogger;

use defmt_rtt as _;
use panic_probe as _;
use stm32f4xx_hal::{
    gpio, i2c, pac, serial, spi,
    timer::{monotonic::MonoTimerUs, DelayUs},
};
use tasks::*;

type Monotonic = MonoTimerUs<pac::TIM2>;
type Delay = DelayUs<pac::TIM3>;
type Led = gpio::PC13<gpio::Output>;
type Bme280 = bme280::i2c::BME280<I2c1>;
type Gps = cansat_gps::Gps<Serial1>;
type SdmmcController =
    embedded_sdmmc::Controller<BlockSpi2, DummyClock, MAX_OPEN_DIRS, MAX_OPEN_FILES>;

type BlockSpi2 = embedded_sdmmc::BlockSpi<'static, Spi2, Cs2>;
const MAX_OPEN_DIRS: usize = 4;
const MAX_OPEN_FILES: usize = 4;

type I2c1 = i2c::I2c1<(Scl1, Sda1)>;
type Scl1 = gpio::PB8<gpio::Alternate<4, gpio::OpenDrain>>;
type Sda1 = gpio::PB9<gpio::Alternate<4, gpio::OpenDrain>>;

type Serial1 = serial::Serial1<(Tx1, Rx1)>;
type Tx1 = gpio::PB6<gpio::Alternate<7>>;
type Rx1 = gpio::PB7<gpio::Alternate<7>>;

type Spi2Device = embedded_sdmmc::SdMmcSpi<Spi2, Cs2>;
type Spi2 = spi::Spi2<(Sck2, Miso2, Mosi2)>;
type Cs2 = gpio::PB12<gpio::Output>;
type Sck2 = gpio::PB13<gpio::Alternate<5>>;
type Miso2 = gpio::PB14<gpio::Alternate<5>>;
type Mosi2 = gpio::PB15<gpio::Alternate<5>>;

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [EXTI0, EXTI1])]
mod app {
    use super::*;

    #[shared]
    struct Shared {
        gps: Gps,
    }

    #[local]
    struct Local {
        delay: DelayUs<pac::TIM3>,
        led: Led,
        bme280: Bme280,
        sd_logger: SdLogger,
    }

    #[monotonic(binds = TIM2, default = true)]
    type MicrosecMono = MonoTimerUs<pac::TIM2>;

    extern "Rust" {
        #[task(binds = USART1, shared = [gps], priority = 2)]
        fn gps_irq(ctx: gps_irq::Context);

        #[task(local = [led], priority = 1)]
        fn blink(ctx: blink::Context);
    }

    #[idle(local = [bme280, delay, sd_logger], shared = [gps])]
    fn idle(ctx: idle::Context) -> ! {
        tasks::idle(ctx)
    }

    #[init(local = [spi2_device: Option<Spi2Device> = None])]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let board = startup::init_board(ctx.device);
        let cansat = startup::init_drivers(board, ctx.local.spi2_device).unwrap_or_else(|e| {
            defmt::panic!("Drivers initialization failed: {}", e);
        });

        blink::spawn().unwrap();

        let shared = Shared { gps: cansat.gps };
        let local = Local {
            delay: cansat.delay,
            led: cansat.led,
            bme280: cansat.bme280,
            sd_logger: cansat.sd_logger,
        };
        let monotonics = init::Monotonics(cansat.monotonic);

        (shared, local, monotonics)
    }
}

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

//! Binary crate targeting stm32f4 family microcontrollers.
#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod sd_logger;
mod startup;
mod tasks;

pub use sd_logger::SdLogger;
pub use startup::{
    Bme280, Bme280Error, Delay, Gps, I2c1Devices, Led, Lis3dh, Lis3dhError, Monotonic,
    SdmmcController, SdmmcError, Lora
};

use defmt_rtt as _;
use panic_probe as _;
use tasks::*;

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [EXTI0, EXTI1])]
mod app {
    use super::*;

    #[shared]
    struct Shared {
        gps: Gps,
    }

    #[local]
    struct Local {
        delay: Delay,
        led: Led,
        sd_logger: SdLogger,
        tracker: accelerometer::Tracker,
        i2c1_devices: I2c1Devices,
        lora: Lora,
    }

    #[monotonic(binds = TIM2, default = true)]
    type MicrosecMono = Monotonic;

    #[init(local = [statik: startup::Statik = startup::Statik::new()])]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let board = startup::init_board(ctx.device);
        let cansat = startup::init_drivers(board, ctx.local.statik).unwrap_or_else(|e| {
            defmt::panic!("Failed to initialize drivers: {}", e);
        });

        blink::spawn().unwrap();

        let shared = Shared { gps: cansat.gps };
        let local = Local {
            delay: cansat.delay,
            led: cansat.led,
            sd_logger: cansat.sd_logger,
            tracker: cansat.tracker,
            i2c1_devices: cansat.i2c1_devices,
            lora: cansat.lora,
        };
        let monotonics = init::Monotonics(cansat.monotonic);

        (shared, local, monotonics)
    }

    #[idle(local = [delay, sd_logger, tracker, i2c1_devices], shared = [gps])]
    fn idle(ctx: idle::Context) -> ! {
        tasks::idle(ctx)
    }

    extern "Rust" {
        #[task(binds = USART1, shared = [gps], priority = 2)]
        fn gps_irq(ctx: gps_irq::Context);

        #[task(local = [led], priority = 1)]
        fn blink(ctx: blink::Context);
    }
}

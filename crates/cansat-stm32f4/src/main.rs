//! Binary crate targeting stm32f4 family microcontrollers.
#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod error;
mod sd_logger;
mod startup;
mod tasks;

use heapless::Vec;
pub use sd_logger::SdLogger;
pub use startup::{
    Bme280, Bme280Error, Delay, Gps, I2c1Devices, Led, Lis3dh, Lis3dhError, Lora, LoraError,
    Monotonic, SdmmcController, SdmmcError,
};

#[cfg(all(debug_assertions))]
use panic_probe as _;
#[cfg(all(not(debug_assertions), feature = "panic-reset"))]
use panic_reset as _;
#[cfg(all(not(debug_assertions), not(feature = "panic-reset")))]
compile_error!("Run `--release` builds with `--no-default-features --features=panic-reset` flags");

use defmt_rtt as _;
use tasks::*;

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [EXTI0, EXTI1])]
mod app {
    use super::*;

    #[shared]
    struct Shared {
        gps: Gps,
        csv_record: Vec<u8, 512>,
    }

    #[local]
    struct Local {
        delay: Delay,
        led: Led,
        sd_logger: Option<SdLogger>,
        tracker: accelerometer::Tracker,
        i2c1_devices: I2c1Devices,
        lora: Option<Lora>,
    }

    #[monotonic(binds = TIM2, default = true)]
    type MicrosecMono = Monotonic;

    #[init(local = [statik: startup::Statik = startup::Statik::new()])]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let board = startup::init_board(ctx.device);
        let cansat = startup::init_drivers(board, ctx.local.statik).unwrap_or_else(|e| {
            defmt::panic!("Initalization error: {}", e);
        });

        blink::spawn().unwrap();
        send_meas::spawn().unwrap();

        let shared = Shared {
            gps: cansat.gps,
            csv_record: Vec::new(),
        };
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

    #[idle(local = [delay, sd_logger, tracker, i2c1_devices], shared = [gps, csv_record])]
    fn idle(ctx: idle::Context) -> ! {
        tasks::idle(ctx)
    }

    extern "Rust" {
        #[task(local = [lora], shared = [csv_record], priority = 1)]
        fn send_meas(ctx: send_meas::Context);

        #[task(binds = USART1, shared = [gps], priority = 2)]
        fn gps_irq(ctx: gps_irq::Context);

        #[task(local = [led], priority = 1)]
        fn blink(ctx: blink::Context);
    }
}

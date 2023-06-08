//! Binary crate targeting stm32f4 family microcontrollers.
#![deny(unsafe_code)]
#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(result_option_inspect)]

mod error;
mod sd_logger;
mod startup;
mod tasks;

use heapless::Vec;
pub use sd_logger::SdLogger;
pub use startup::{
    Bme280, Bme280Error, Delay, Gps, I2c1Devices, Led, Lis3dh, Lis3dhError, Lora, LoraError,
    SdmmcController, SdmmcError,
};

#[cfg(debug_assertions)]
use panic_probe as _;
#[cfg(all(not(debug_assertions), feature = "panic-reset"))]
use panic_reset as _;
#[cfg(all(not(debug_assertions), not(feature = "panic-reset")))]
compile_error!("Run `--release` builds with `--no-default-features --features=panic-reset` flags");

use defmt_rtt as _;
use tasks::*;

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [EXTI0, EXTI1])]
mod app {
    use super::*;

    #[shared]
    pub struct Shared {
        pub gps: Gps,
        pub csv_record: Vec<u8, 512>,
    }

    #[local]
    pub struct Local {
        pub delay: Delay,
        pub led: Led,
        pub sd_logger: Option<SdLogger>,
        pub tracker: accelerometer::Tracker,
        pub i2c1_devices: I2c1Devices,
        pub lora: Option<Lora>,
    }

    #[init(local = [statik: startup::Statik = startup::Statik::new()])]
    fn init(ctx: init::Context) -> (Shared, Local) {
        startup::init(ctx)
    }

    #[idle(local = [delay, sd_logger, tracker, i2c1_devices], shared = [gps, csv_record])]
    fn idle(ctx: idle::Context) -> ! {
        tasks::idle(ctx)
    }

    extern "Rust" {
        #[task(local = [lora], shared = [csv_record], priority = 1)]
        async fn send_meas(ctx: send_meas::Context);

        #[task(binds = USART1, shared = [gps], priority = 2)]
        fn gps_irq(ctx: gps_irq::Context);

        #[task(local = [led], priority = 1)]
        async fn blink(ctx: blink::Context);
    }
}

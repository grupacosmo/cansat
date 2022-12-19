#![deny(unsafe_code)]
#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _;

#[rtic::app(device = stm32f4xx_hal::pac)]
mod app {
    use cansat::defmt;
    use stm32f4xx_hal::{pac, prelude::*, timer::monotonic::MonoTimerUs};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[monotonic(binds = TIM2, default = true)]
    type MicrosecMono = MonoTimerUs<pac::TIM2>;

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("Initializing");
        let rcc = ctx.device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();
        let mono = ctx.device.TIM2.monotonic_us(&clocks);
        (Shared {}, Local {}, init::Monotonics(mono))
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("Started idle task");
        #[allow(clippy::empty_loop)]
        loop {}
    }
}

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::pac)]
mod app {
    use cansat::rtt_target::{rprintln, rtt_init_print};
    use stm32f4xx_hal::{pac, prelude::*, timer::monotonic::MonoTimerUs};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[monotonic(binds = TIM2, default = true)]
    type MicrosecMono = MonoTimerUs<pac::TIM2>;

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("Initialization");
        let rcc = ctx.device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();
        let mono = ctx.device.TIM2.monotonic_us(&clocks);
        (Shared {}, Local {}, init::Monotonics(mono))
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        #[allow(clippy::empty_loop)]
        loop {}
    }
}

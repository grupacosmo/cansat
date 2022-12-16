//! Blinking LED using  blocking `Delay` in  TIM5.
#![deny(unsafe_code)]

use crate::hal::{pac, prelude::*};
use panic_halt as _; // panic handler

pub fn blink_loop() {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Setting up LED - its pin PA5 on Nucleo446RE
        let gpioa = dp.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();
        // Set up the system clock at48MHz.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        // Create a delay based on SysTick
        let mut delay = cp.SYST.delay(&clocks);

        loop {
            // toggle led every sec
            led.toggle();
            delay.delay_ms(1000_u32);
        }
    }
}

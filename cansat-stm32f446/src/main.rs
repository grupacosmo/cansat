#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal as hal;
use cansat::rtt_target::{rtt_init_print, rprintln};

mod blink;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world!");
    
    loop {
        blink::blink_loop();
        //will never exit function above
    }

}

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal as _;

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();
    rtt_target::rprintln!("Hello, world!");
    loop {}
}

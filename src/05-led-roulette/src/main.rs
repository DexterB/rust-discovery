#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();
    let half_period = 25_u16;
    let mut is_on = false;

    let mut iter = 0;
    let max_pos = 8;
    loop {
        // Infinite loop guaranteeing that we don't leave the stack frame.
        if !is_on {
            leds[iter].on();
        } else {
            leds[iter].off();
        }
        delay.delay_ms(half_period);

        iter = (iter + 1) % max_pos;
        if iter == 0 {
            is_on = !is_on;
        }
    }
}

//! Blinks an LED

#![no_main]
#![no_std]

use stm32g4_hal as hal;

use crate::hal::gpio::{gpioa, gpioc};
use crate::hal::pac;

use cortex_m_rt::entry;

pub use defmt::{debug, error, info, trace, warn};
use defmt_rtt as _;

#[entry]
fn main() -> ! {
    info!("start");
    let p = pac::Peripherals::take().unwrap();

    let gpioa = gpioa::Pins::new(p.gpioa);

    let a0 = gpioa.pa0.into_push_pull_output();
    let a1 = gpioa.pa1.into_push_pull_output();
    let a4 = gpioa.pa4.into_push_pull_output();

    let mut array = hal::gpio::outport::OutPort3(a0, a1, a4);

    array.write(0b111);

    let mut a3 = gpioa.pa3.into_push_pull_output().erase();
    a3.set_high();

    info!("Init Led");
    let gpioc = gpioc::Pins::new(p.gpioc);
    let mut led = gpioc.pc4.into_push_pull_output();

    loop {
        info!("Set Led High");
        for _ in 0..10_000_000 {
            led.set_high();
        }

        info!("Set Led low");
        for _ in 0..10_000_000 {
            led.set_low();
        }
    }
}

#[inline(never)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    hal::panic(info)
}

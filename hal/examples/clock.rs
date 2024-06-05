//! Blinks an LED

// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

use stm32g4_hal as hal;

use crate::hal::{
    gpio::GpioExt,
    pac,
    pwr::{self, PwrExt},
    rcc::{clock, RccExt},
};
use cortex_m_rt::entry;

use fugit::RateExtU32;

pub use defmt::{debug, error, info, trace, warn};
use defmt_rtt as _;

#[entry]
fn main() -> ! {
    info!("start");
    let p = pac::Peripherals::take().unwrap();

    let pwr = p
        .pwr
        .constrain()
        .vos(pwr::VoltageScale::Range1 { enable_boost: true })
        .freeze();
    let rcc = p.rcc.constrain();

    let (rcc, clock) = rcc
        .clock_src(clock::SysClockSrc::PLL)
        .pll_cfg(clock::PllConfig {
            mux: clock::PLLSrc::HSE(25.MHz()),
            m: clock::PllMDiv::DIV_5,
            n: clock::PllNMul::MUL_68,
            r: Some(clock::PllRDiv::DIV_2),
            q: Some(clock::PllQDiv::DIV_2),
            p: Some(clock::PllPDiv::DIV_2),
        })
        .pwr_cfg(pwr)
        .freeze();
    info!("clock: {:?}", clock);

    info!("Init Led");
    let gpioc = p.gpioc.split();
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

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

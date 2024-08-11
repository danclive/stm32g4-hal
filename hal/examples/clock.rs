//! Blinks an LED

#![no_main]
#![no_std]

use stm32g4_hal as hal;

use crate::hal::gpio::gpioc;
use crate::hal::prelude::*;
use crate::hal::{pac, pwr, rcc};

use cortex_m_rt::entry;

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

    let rcc_config = rcc::Config::new()
        .hse(25.MHz(), false)
        .clock_src(rcc::SysClockSrc::PLL)
        .pll_cfg(rcc::PllConfig {
            mux: rcc::PLLSrc::HSE,
            m: rcc::PllMDiv::DIV_5,
            n: rcc::PllNMul::MUL_68,
            r: Some(rcc::PllRDiv::DIV_2),
            q: Some(rcc::PllQDiv::DIV_2),
            p: Some(rcc::PllPDiv::DIV_2),
        })
        .pwr_cfg(pwr);

    let rcc = rcc.freeze(rcc_config);

    info!("clock: {:?}", rcc.clocks());

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

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

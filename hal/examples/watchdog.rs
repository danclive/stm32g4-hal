//! Blinks an LED

#![no_main]
#![no_std]

use stm32g4_hal as hal;

use crate::hal::gpio::gpioc;
use crate::hal::prelude::*;
use crate::hal::watchdog::watchdog;
use crate::hal::{pac, pwr, rcc};

use cortex_m_rt::entry;

pub use defmt::{debug, error, info, trace, warn};
use defmt_rtt as _;

#[entry]
fn main() -> ! {
    info!("start");
    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let pwr = p
        .pwr
        .constrain()
        .vos(pwr::VoltageScale::Range1 { enable_boost: true })
        .freeze();
    let rcc = p.rcc.constrain();

    let rcc = rcc
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

    let rcc = rcc.freeze();

    info!("clock: {:?}", rcc.clocks());

    let mut delay = cp.SYST.delay(rcc.clocks());

    info!("Init Led");
    let gpioc = gpioc::Pins::new(p.gpioc);
    let mut led = gpioc.pc4.into_push_pull_output();

    let mut wag = watchdog(p.wwdg, &rcc.clocks());

    wag.start(300.millis());

    loop {
        info!("Set Led High");
        led.set_high();

        delay.delay(100.millis());

        info!("Set Led low");
        led.set_low();

        delay.delay(100.millis());

        wag.feed();
    }
}

#[inline(never)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    hal::panic(info)
}

use cortex_m_rt::exception;

#[exception]
#[allow(non_snake_case)]
fn SysTick() {
    // ..
}

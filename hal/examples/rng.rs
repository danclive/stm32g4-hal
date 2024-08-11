//! Blinks an LED

#![no_main]
#![no_std]

use stm32g4_hal as hal;

use crate::hal::gpio::gpioc;
use crate::hal::prelude::*;
use crate::hal::rng::Rng;
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

    let mut delay = cp.SYST.delay(rcc.clocks());

    info!("Init Led");
    let gpioc = gpioc::Pins::new(p.gpioc);
    let mut led = gpioc.pc4.into_push_pull_output();

    let mut rng = Rng::new(p.rng);

    loop {
        info!("Set Led High");
        led.set_high();

        delay.delay(1.secs());

        info!("Set Led low");
        led.set_low();

        delay.delay(1.secs());

        let random_number = rng.value();
        info!("Random number: {}", random_number);
    }
}

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use defmt::error as println;

    println!("");
    println!("");

    if let Some(location) = info.location() {
        let (file, line, column) = (location.file(), location.line(), location.column());
        println!(
            "!! A panic occured in '{}', at line {}, column {}:",
            file, line, column
        );
    } else {
        println!("!! A panic occured at an unknown location:");
    }

    #[cfg(not(nightly))]
    {
        #[cfg(not(feature = "defmt"))]
        println!("{:#?}", info);

        #[cfg(feature = "defmt")]
        println!("{:#?}", defmt::Display2Format(info));
    }

    #[cfg(nightly)]
    {
        if let Some(message) = info.message() {
            #[cfg(not(feature = "defmt"))]
            println!("{}", message);

            #[cfg(feature = "defmt")]
            println!("{}", defmt::Display2Format(message));
        }
    }

    if let Some(s) = info.payload().downcast_ref::<&str>() {
        println!("panic occurred: {}", s);
    } else {
        println!("panic occurred");
    }

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

use cortex_m_rt::exception;

#[exception]
#[allow(non_snake_case)]
fn SysTick() {
    // ..
}

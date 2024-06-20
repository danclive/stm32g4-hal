//! Blinks an LED

// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

use stm32g4_hal as hal;

use crate::hal::gpio::{gpioa, gpioc};
use crate::hal::prelude::*;
use crate::hal::{pac, pwr, rcc::clock};

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

    let (_rcc, clocks) = rcc
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
    info!("clock: {:?}", clocks);

    let mut delay = cp.SYST.delay(&clocks);

    info!("Init Led");
    let gpioc = gpioc::Pins::new(p.gpioc);
    let mut led = gpioc.pc4.into_push_pull_output();

    let gpioa = gpioa::Pins::new(p.gpioa);

    let pin = gpioa.pa8.into_alt();
    let npin = gpioa.pa7.into_alt();

    let c0 = p.tim1.pwm(pin, 100.Hz(), &clocks);
    let mut c0 = c0.into_complementary(npin);

    let duty = c0.get_duty();
    info!("Duty: {:?}", duty);
    let max_duty = c0.get_max_duty();
    info!("Max Duty: {:?}", max_duty);

    c0.set_duty(max_duty / 2);
    c0.enable();

    loop {
        info!("Set Led High");
        led.set_high();

        delay.delay(1.secs());

        info!("Set Led low");
        led.set_low();

        delay.delay(1.secs());
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

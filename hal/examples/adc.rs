//! Blinks an LED

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use stm32g4_hal as hal;

use crate::hal::gpio::{gpioa, gpioc};
use crate::hal::prelude::*;
use crate::hal::{pac, pwr, rcc};

use crate::hal::adc;

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
        .clock_src(rcc::SysClockSrc::PLL)
        .pll_cfg(rcc::PllConfig {
            mux: rcc::PLLSrc::HSE(25.MHz()),
            m: rcc::PllMDiv::DIV_5,
            n: rcc::PllNMul::MUL_68,
            r: Some(rcc::PllRDiv::DIV_2),
            q: Some(rcc::PllQDiv::DIV_2),
            p: Some(rcc::PllPDiv::DIV_2),
        })
        .pwr_cfg(pwr)
        .freeze();
    info!("clock: {:?}", clocks);

    let mut delay = cp.SYST.delay(&clocks);

    info!("Init Led");
    let gpioc = gpioc::Pins::new(p.gpioc);
    let mut led = gpioc.pc4.into_push_pull_output();

    let mut adc = p.adc1.adc(
        &clocks,
        adc::ClockSource::SystemClock,
        25.MHz(),
        adc::Config::default(),
        &mut delay,
    );

    adc.set_clock_mode(adc::ClockMode::Asynchronous);

    let mut adc = adc.enable();

    let gpica = gpioa::Pins::new(p.gpioa);
    let pa0 = gpica.pa0.into_analog();

    loop {
        info!("Set Led High");
        led.set_high();

        delay.delay(1.secs());

        info!("Set Led low");
        led.set_low();

        delay.delay(1.secs());

        let sample = adc.convert(&pa0, adc::SampleTime::Cycles_640_5);
        info!("sample: {}", sample);

        let millivolts = adc.sample_to_millivolts(sample);
        info!("pa0: {}mV", millivolts);
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

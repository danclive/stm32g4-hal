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
use crate::hal::signature::{VrefCal, VDDA_CALIB};

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

    let mut vref = adc::pins::Vref::new();
    vref.enable(&adc);

    let mut temp = adc::pins::Temperature::new();
    temp.enable(&adc);

    adc.set_clock_mode(adc::ClockMode::Asynchronous);
    adc.set_auto_delay(true);
    adc.set_continuous(adc::Continuous::Continuous);
    adc.reset_sequence();

    let gpica = gpioa::Pins::new(p.gpioa);
    let pa0 = gpica.pa0.into_analog();
    let pa1 = gpica.pa1.into_analog();

    adc.configure_channel(&pa0, adc::Sequence::SEQ_1, adc::SampleTime::Cycles_640_5);
    adc.configure_channel(&pa1, adc::Sequence::SEQ_2, adc::SampleTime::Cycles_640_5);
    adc.configure_channel(&vref, adc::Sequence::SEQ_3, adc::SampleTime::Cycles_640_5);
    adc.configure_channel(&temp, adc::Sequence::SEQ_4, adc::SampleTime::Cycles_640_5);

    let mut adc = adc.enable().start_conversion();

    loop {
        info!("Set Led High");
        led.set_high();

        delay.delay(1.secs());

        info!("Set Led low");
        led.set_low();

        delay.delay(1.secs());

        adc.wait_for_conversion_sequence();
        let millivolts = adc.sample_to_millivolts(adc.current_sample());
        info!("pa0: {}mV", millivolts);

        adc.wait_for_conversion_sequence();
        let millivolts = adc.sample_to_millivolts(adc.current_sample());
        info!("pa1: {}mV", millivolts);

        adc.wait_for_conversion_sequence();
        let vref_sample = adc.current_sample();
        let millivolts = adc.sample_to_millivolts(vref_sample);
        info!("vref: {}mV", millivolts);

        let vdda = VDDA_CALIB * VrefCal::get().read() as u32 / vref_sample as u32;

        adc.wait_for_conversion_sequence();
        let sample = adc.current_sample();
        let temp = adc::pins::Temperature::temperature_to_degrees_centigrade(
            sample,
            vdda as f32 / 1000.,
            adc::Resolution::Twelve,
        );
        info!("temp: {}°C", temp);
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

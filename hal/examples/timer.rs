//! Blinks an LED

#![no_main]
#![no_std]

use stm32g4_hal as hal;

use crate::hal::gpio::gpioc;
use crate::hal::prelude::*;
use crate::hal::timer::counter::CounterUs;
use crate::hal::timer::{Event, Flag};
use crate::hal::{interrupt, pac, pwr, rcc};
use crate::pac::{Interrupt, Tim2};

use cortex_m_rt::{entry, exception, ExceptionFrame};

use core::cell::RefCell;

use critical_section::Mutex;

pub use defmt::{debug, error, info, trace, warn};
use defmt_rtt as _;

#[entry]
fn main() -> ! {
    info!("start");
    let p = pac::Peripherals::take().unwrap();
    // let cp = cortex_m::Peripherals::take().unwrap();

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

    let mut delay = p.tim1.delay_us(&rcc.clocks());

    // Set up a timer expiring after 1s
    let mut timer = p.tim2.counter_us(&rcc.clocks());
    timer.start(1.secs()).unwrap();

    timer.listen(Event::Update);

    // Move the timer into our global storage
    critical_section::with(|cs| G_TIM.borrow_ref_mut(cs).replace(timer));
    // critical_section::with(|cs| *G_TIM.borrow(cs).borrow_mut() = Some(timer));

    //enable TIM2 interrupt
    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2);
    }

    info!("Init Led");
    let gpioc = gpioc::Pins::new(p.gpioc);
    let mut led = gpioc.pc4.into_push_pull_output();

    loop {
        info!("Set Led High");
        led.set_high();

        delay.delay(1.secs());

        info!("Set Led low");
        led.set_low();

        delay.delay(1.secs());
    }
}

// Make timer interrupt registers globally available
static G_TIM: Mutex<RefCell<Option<CounterUs<Tim2>>>> = Mutex::new(RefCell::new(None));

#[interrupt]
#[allow(non_snake_case)]
fn TIM2() {
    critical_section::with(|cs| {
        // Move LED pin here, leaving a None in its place
        if let Some(tim) = G_TIM.borrow_ref_mut(cs).as_mut() {
            tim.clear_flags(Flag::Update);
        }

        // if let Some(tim) = G_TIM.borrow(cs).borrow_mut().as_deref_mut() {
        //     tim.clear_flags(Flag::Update);
        // }
    });

    info!("TIM2 !!!!!!!!!!!");
}

#[inline(never)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    hal::panic(info)
}

#[exception]
#[allow(non_snake_case)]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

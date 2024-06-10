//! Blinks an LED

#![no_main]
#![no_std]

use stm32g4_hal as hal;

use crate::hal::prelude::*;
use crate::hal::timer::counter::CounterUs;
use crate::hal::timer::{Event, Flag};
use crate::hal::{interrupt, pac, pwr, rcc::clock};
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

    let mut delay = p.tim1.delay_us(&clocks);

    // Set up a timer expiring after 1s
    let mut timer = p.tim2.counter_us(&clocks);
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
    let gpioc = p.gpioc.split();
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

#[exception]
#[allow(non_snake_case)]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

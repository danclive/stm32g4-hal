//! Timer

use cortex_m::peripheral::syst::SystClkSource;
use fugit::{HertzU32 as Hertz, MicrosDurationU32};

use crate::pac::{DCB, DWT, SYST};
use crate::rcc::clock::Clocks;

use counter::{SYSTCounter, SYSTCounterHz, SYSTCounterUs};
use delay::SYSTDelay;

pub mod counter;
pub mod delay;

/// Timer wrapper.
///
/// This wrapper can be used both for the system timer (SYST) or the
/// general-purpose timers (TIMx).
///
/// Note: If you want to use the timer to sleep a certain amount of time, use
/// [`Delay`](`crate::timer::delay::Delay`).
pub struct Timer<TIM> {
    tim: TIM,
    clk: Hertz,
}

impl Timer<SYST> {
    /// Initialize SysTick timer
    pub fn syst(mut tim: SYST, clocks: &Clocks) -> Self {
        tim.set_clock_source(SystClkSource::Core);

        let rcc = unsafe { &*crate::pac::Rcc::PTR };
        rcc.rcc_ccipr().read().lptim1sel();
        Self {
            tim,
            clk: clocks.ahb_clk(),
        }
    }

    /// Initialize SysTick timer and set it frequency to `AHB_CLK / 8`
    pub fn syst_external(mut tim: SYST, clocks: &Clocks) -> Self {
        tim.set_clock_source(SystClkSource::External);
        Self {
            tim,
            clk: clocks.ahb_clk() / 8,
        }
    }

    pub fn configure(&mut self, clocks: &Clocks) {
        self.tim.set_clock_source(SystClkSource::Core);
        self.clk = clocks.ahb_clk();
    }

    pub fn configure_external(&mut self, clocks: &Clocks) {
        self.tim.set_clock_source(SystClkSource::External);
        self.clk = clocks.ahb_clk() / 8;
    }

    pub fn release(self) -> SYST {
        self.tim
    }

    /// Starts listening for an `event`
    pub fn listen(&mut self, event: SYSTEvent) {
        match event {
            SYSTEvent::Update => self.tim.enable_interrupt(),
        }
    }

    /// Stops listening for an `event`
    pub fn unlisten(&mut self, event: SYSTEvent) {
        match event {
            SYSTEvent::Update => self.tim.disable_interrupt(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SYSTEvent {
    Update,
}

pub trait SYSTimerExt: Sized {
    /// Creates timer which takes [Hertz] as Duration
    fn counter_hz(self, clocks: &Clocks) -> SYSTCounterHz;

    /// Creates timer with custom precision (core frequency recommended is known)
    fn counter<const FREQ: u32>(self, clocks: &Clocks) -> SYSTCounter<FREQ>;

    /// Creates timer with precision of 1 μs (1 MHz sampling)
    fn counter_us(self, clocks: &Clocks) -> SYSTCounterUs {
        self.counter::<1_000_000>(clocks)
    }

    /// Blocking [Delay] with custom precision
    fn delay(self, clocks: &Clocks) -> SYSTDelay;
}

impl SYSTimerExt for SYST {
    fn counter_hz(self, clocks: &Clocks) -> SYSTCounterHz {
        Timer::syst(self, clocks).counter_hz()
    }
    fn counter<const FREQ: u32>(self, clocks: &Clocks) -> SYSTCounter<FREQ> {
        Timer::syst(self, clocks).counter()
    }
    fn delay(self, clocks: &Clocks) -> SYSTDelay {
        Timer::syst_external(self, clocks).delay()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Timer is disabled
    Disabled,
    WrongAutoReload,
}

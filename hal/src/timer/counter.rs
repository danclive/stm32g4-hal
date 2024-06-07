//! Counter
use core::ops::{Deref, DerefMut};

use fugit::{HertzU32 as Hertz, TimerDurationU32, TimerInstantU32};

use super::*;

/// Hardware timers
pub struct SYSTCounterHz(Timer<SYST>);

impl Deref for SYSTCounterHz {
    type Target = Timer<SYST>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SYSTCounterHz {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl SYSTCounterHz {
    pub fn start(&mut self, timeout: Hertz) -> Result<(), Error> {
        let rvr = self.clk.raw() / timeout.raw() - 1;

        if rvr >= (1 << 24) {
            return Err(Error::WrongAutoReload);
        }

        self.tim.set_reload(rvr);
        self.tim.clear_current();
        self.tim.enable_counter();

        Ok(())
    }

    pub fn wait(&mut self) -> bool {
        if self.tim.has_wrapped() {
            true
        } else {
            false
        }
    }

    pub fn cancel(&mut self) -> Result<(), Error> {
        if !self.tim.is_counter_enabled() {
            return Err(Error::Disabled);
        }

        self.tim.disable_counter();
        Ok(())
    }
}

pub type SYSTCounterUs = SYSTCounter<1_000_000>;

/// SysTick timer with precision of 1 μs (1 MHz sampling)
pub struct SYSTCounter<const FREQ: u32>(Timer<SYST>);

impl<const FREQ: u32> Deref for SYSTCounter<FREQ> {
    type Target = Timer<SYST>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const FREQ: u32> DerefMut for SYSTCounter<FREQ> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const FREQ: u32> SYSTCounter<FREQ> {
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

    pub fn now(&self) -> TimerInstantU32<FREQ> {
        TimerInstantU32::from_ticks(
            (SYST::get_reload() - SYST::get_current()) / (self.clk.raw() / FREQ),
        )
    }

    pub fn start(&mut self, timeout: TimerDurationU32<FREQ>) -> Result<(), Error> {
        let rvr = timeout.ticks() * (self.clk.raw() / FREQ) - 1;

        if rvr >= (1 << 24) {
            return Err(Error::WrongAutoReload);
        }

        self.tim.set_reload(rvr);
        self.tim.clear_current();
        self.tim.enable_counter();

        Ok(())
    }

    pub fn wait(&mut self) -> bool {
        if self.tim.has_wrapped() {
            true
        } else {
            false
        }
    }

    pub fn cancel(&mut self) -> Result<(), Error> {
        if !self.tim.is_counter_enabled() {
            return Err(Error::Disabled);
        }

        self.tim.disable_counter();
        Ok(())
    }
}

impl Timer<SYST> {
    /// Creates [SYSTCounterHz] which takes [Hertz] as Duration
    pub fn counter_hz(self) -> SYSTCounterHz {
        SYSTCounterHz(self)
    }

    /// Creates [SYSTCounter] with custom precision (core frequency recommended is known)
    pub fn counter<const FREQ: u32>(self) -> SYSTCounter<FREQ> {
        SYSTCounter(self)
    }

    /// Creates [SYSTCounter] 1 microsecond precision
    pub fn counter_us(self) -> SYSTCounterUs {
        SYSTCounter(self)
    }
}

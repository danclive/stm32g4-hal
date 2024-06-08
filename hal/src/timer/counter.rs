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

/// Hardware timers
pub struct CounterHz<TIM>(pub(super) Timer<TIM>);

impl<T> Deref for CounterHz<T> {
    type Target = Timer<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for CounterHz<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<TIM: Instance> CounterHz<TIM> {
    /// Releases the TIM peripheral
    pub fn release(mut self) -> Timer<TIM> {
        // stop timer
        self.tim.reset_control_register1();
        self.0
    }
}

impl<TIM: Instance> CounterHz<TIM> {
    pub fn start(&mut self, timeout: Hertz) -> Result<(), Error> {
        // pause
        self.tim.enable_counter(false);
        // reset counter
        self.tim.reset_counter();
        self.tim.clear_interrupt_flag(Flag::Update.into());

        let (psc, arr) = compute_arr_presc(timeout.raw(), self.clk.raw());
        self.tim.set_prescaler(psc);
        self.tim.set_auto_reload(arr)?;

        // Trigger update event to load the registers
        self.tim.trigger_update();

        // start counter
        self.tim.enable_counter(true);

        Ok(())
    }

    pub fn wait(&mut self) -> bool {
        if self.tim.get_interrupt_flag().contains(Flag::Update) {
            self.tim.clear_interrupt_flag(Flag::Update.into());
            true
        } else {
            false
        }
    }

    pub fn cancel(&mut self) -> Result<(), Error> {
        if !self.tim.is_counter_enabled() {
            return Err(Error::Disabled);
        }

        // disable counter
        self.tim.enable_counter(false);
        Ok(())
    }
}

pub struct Counter<TIM, const FREQ: u32>(pub(super) FreqTimer<TIM, FREQ>);

impl<T, const FREQ: u32> Deref for Counter<T, FREQ> {
    type Target = FreqTimer<T, FREQ>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const FREQ: u32> DerefMut for Counter<T, FREQ> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// `Counter` with precision of 1 μs (1 MHz sampling)
pub type CounterUs<TIM> = Counter<TIM, 1_000_000>;

/// `Counter` with precision of of 1 ms (1 kHz sampling)
///
/// NOTE: don't use this if your system frequency more than 65 MHz
pub type CounterMs<TIM> = Counter<TIM, 1_000>;

impl<TIM: Instance, const FREQ: u32> Counter<TIM, FREQ> {
    /// Releases the TIM peripheral
    pub fn release(mut self) -> FreqTimer<TIM, FREQ> {
        // stop counter
        self.tim.reset_control_register1();
        self.0
    }

    pub fn now(&self) -> TimerInstantU32<FREQ> {
        TimerInstantU32::from_ticks(self.tim.read_count().into())
    }

    pub fn start(&mut self, timeout: TimerDurationU32<FREQ>) -> Result<(), Error> {
        // pause
        self.tim.enable_counter(false);
        // reset counter
        self.tim.reset_counter();
        self.tim.clear_interrupt_flag(Flag::Update.into());

        self.tim.set_auto_reload(timeout.ticks() - 1)?;

        // Trigger update event to load the registers
        self.tim.trigger_update();

        // start counter
        self.tim.enable_counter(true);

        Ok(())
    }

    pub fn wait(&mut self) -> bool {
        if self.tim.get_interrupt_flag().contains(Flag::Update) {
            self.tim.clear_interrupt_flag(Flag::Update.into());
            true
        } else {
            false
        }
    }

    pub fn cancel(&mut self) -> Result<(), Error> {
        if !self.tim.is_counter_enabled() {
            return Err(Error::Disabled);
        }

        // disable counter
        self.tim.enable_counter(false);
        Ok(())
    }
}

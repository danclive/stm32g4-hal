//! Delays
use core::ops::{Deref, DerefMut};

use fugit::{MicrosDurationU32, TimerDurationU32};

use super::*;

/// Timer as a delay provider (SysTick by default)
pub struct SYSTDelay(Timer<SYST>);

impl Deref for SYSTDelay {
    type Target = Timer<SYST>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SYSTDelay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl SYSTDelay {
    /// Releases the timer resource
    pub fn release(self) -> Timer<SYST> {
        self.0
    }
}

impl Timer<SYST> {
    pub fn delay(self) -> SYSTDelay {
        SYSTDelay(self)
    }
}

impl SYSTDelay {
    pub fn delay(&mut self, us: MicrosDurationU32) {
        // The SysTick Reload Value register supports values between 1 and 0x00FFFFFF.
        const MAX_RVR: u32 = 0x00FF_FFFF;

        let mut total_rvr = us.ticks() * (self.clk.raw() / 1_000_000);

        while total_rvr != 0 {
            let current_rvr = total_rvr.min(MAX_RVR);

            self.tim.set_reload(current_rvr);
            self.tim.clear_current();
            self.tim.enable_counter();

            // Update the tracking variable while we are waiting...
            total_rvr -= current_rvr;

            while !self.tim.has_wrapped() {}

            self.tim.disable_counter();
        }
    }
}

/// Delay.
///
/// ### Example: Millisecond precision
///
/// To instantiate this with the TIM2 timer and millisecond precision:
///
/// ```rust
/// let mut delay = dp.TIM2.delay_ms(&clocks);
/// delay.delay_ms(320_u32);
/// ```
///
/// With millisecond precision, you can wait from 2 ms to 49 days.
///
/// ### Example: Microsecond precision
///
/// To instantiate this with the TIM5 timer and microsecond precision:
///
/// ```rust
/// let delay = dp.TIM5.delay_us(&clocks);
/// delay.delay_us(30_u32);
/// delay.delay_ms(5_u32);
/// delay.delay(5.millis());
/// delay.delay(3.secs());
/// ```
///
/// With microsecond precision, you can wait from 2 µs to 71 min.
pub struct Delay<TIM, const FREQ: u32>(pub(super) FixedTimer<TIM, FREQ>);

impl<T, const FREQ: u32> Deref for Delay<T, FREQ> {
    type Target = FixedTimer<T, FREQ>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const FREQ: u32> DerefMut for Delay<T, FREQ> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// `Delay` with precision of 1 μs (1 MHz sampling)
pub type DelayUs<TIM> = Delay<TIM, 1_000_000>;

/// `Delay` with precision of 1 ms (1 kHz sampling)
///
/// NOTE: don't use this if your system frequency more than 65 MHz
pub type DelayMs<TIM> = Delay<TIM, 1_000>;

impl<TIM: Instance, const FREQ: u32> Delay<TIM, FREQ> {
    /// Sleep for given time
    pub fn delay(&mut self, time: TimerDurationU32<FREQ>) {
        let mut ticks = time.ticks();
        if ticks > 1 {
            ticks -= 1;
        }
        while ticks != 0 {
            let reload = ticks.min(TIM::max_auto_reload());

            // Write Auto-Reload Register (ARR)
            unsafe {
                self.tim.set_auto_reload_unchecked(reload);
            }

            // Trigger update event (UEV) in the event generation register (EGR)
            // in order to immediately apply the config
            self.tim.trigger_update();

            // Configure the counter in one-pulse mode (counter stops counting at
            // the next updateevent, clearing the CEN bit) and enable the counter.
            self.tim.start_one_pulse();

            // Update the tracking variable while we are waiting...
            ticks -= reload;
            // Wait for CEN bit to clear
            while self.tim.is_counter_enabled() { /* wait */ }
        }
    }

    pub fn max_delay(&self) -> TimerDurationU32<FREQ> {
        TimerDurationU32::from_ticks(TIM::max_auto_reload())
    }

    /// Releases the TIM peripheral
    pub fn release(mut self) -> FixedTimer<TIM, FREQ> {
        // stop counter
        self.tim.reset_control_register1();
        self.0
    }
}

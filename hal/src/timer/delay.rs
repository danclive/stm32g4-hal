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

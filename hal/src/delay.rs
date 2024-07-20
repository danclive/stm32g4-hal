//! Delay

use fugit::{MicrosDurationU32, MillisDurationU32};

use crate::timer::delay::{DelayMs as _DelayMs, DelayUs as _DelayUs, SYSTDelay};
use crate::timer::Instance;

/// Millisecond delay
///
/// `UXX` denotes the range type of the delay time. `UXX` can be `u8`, `u16`, etc. A single type can
/// implement this trait for different types of `UXX`.
pub trait DelayMs {
    /// Pauses execution for `ms` milliseconds
    fn delay_ms(&mut self, ms: MillisDurationU32);
}

/// Microsecond delay
///
/// `UXX` denotes the range type of the delay time. `UXX` can be `u8`, `u16`, etc. A single type can
/// implement this trait for different types of `UXX`.
pub trait DelayUs {
    /// Pauses execution for `us` microseconds
    fn delay_us(&mut self, us: MicrosDurationU32);
}

impl DelayMs for SYSTDelay {
    fn delay_ms(&mut self, ms: MillisDurationU32) {
        self.delay(ms.convert());
    }
}

impl DelayUs for SYSTDelay {
    fn delay_us(&mut self, us: MicrosDurationU32) {
        self.delay(us);
    }
}

impl<TIM: Instance> DelayMs for _DelayMs<TIM> {
    fn delay_ms(&mut self, ms: MillisDurationU32) {
        self.delay(ms);
    }
}

impl<TIM: Instance> DelayUs for _DelayUs<TIM> {
    fn delay_us(&mut self, us: MicrosDurationU32) {
        self.delay(us);
    }
}

// pub use cortex_m::delay::Delay;
// use fugit::MicrosDurationU32;

// use crate::rcc::clock::Clocks;
// use crate::rcc::SYST;

// pub trait SYSTDelayExt {
//     fn delay(self, clocks: &Clocks) -> Delay;
// }

// impl SYSTDelayExt for SYST {
//     fn delay(self, clocks: &Clocks) -> Delay {
//         Delay::new(self, clocks.ahb_clk().raw())
//     }
// }

// pub trait DelayExt {
//     fn delay<T>(&mut self, delay: T)
//     where
//         T: Into<MicrosDurationU32>;
// }

// impl DelayExt for Delay {
//     fn delay<T>(&mut self, delay: T)
//     where
//         T: Into<MicrosDurationU32>,
//     {
//         self.delay_us(delay.into().ticks())
//     }
// }

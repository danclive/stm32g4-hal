//! Delay providers

pub use cortex_m::delay::Delay;
use fugit::MicrosDurationU32;

use crate::rcc::clock::Clocks;
use crate::rcc::SYST;

pub trait SYSTDelayExt {
    fn delay(self, clocks: &Clocks) -> Delay;
}

impl SYSTDelayExt for SYST {
    fn delay(self, clocks: &Clocks) -> Delay {
        Delay::new(self, clocks.ahb_clk().raw())
    }
}

pub trait DelayExt {
    fn delay<T>(&mut self, delay: T)
    where
        T: Into<MicrosDurationU32>;
}

impl DelayExt for Delay {
    fn delay<T>(&mut self, delay: T)
    where
        T: Into<MicrosDurationU32>,
    {
        self.delay_us(delay.into().ticks())
    }
}

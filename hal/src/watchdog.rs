//! Watchdog

use fugit::{HertzU32 as Hertz, MillisDurationU32};

use crate::pac;
use crate::rcc::Clocks;

pub struct Watchdog {
    wwdg: pac::Wwdg,
    down_counter: u8,
    pclk: Hertz,
}

pub fn watchdog(wwdg: pac::Wwdg, clocks: &Clocks) -> Watchdog {
    unsafe {
        let rcc = &(*pac::Rcc::PTR);
        rcc.rcc_apb1enr1().modify(|_, w| w.wwdgen().set_bit());
    }

    Watchdog {
        wwdg,
        down_counter: 0,
        pclk: clocks.pclk1(),
    }
}

impl Watchdog {
    /// Starts the watchdog with a given period
    pub fn start(&mut self, period: impl Into<MillisDurationU32>) {
        let period_ms = period.into().ticks();
        let maximum = (4096 * 2u32.pow(7) * 64) / (self.pclk.raw() / 1000);
        assert!(period_ms <= maximum);

        // timeout = pclk * 4096 * 2^WDGTB[2:0] * (t[5:0] +1)
        let ratio = period_ms * (self.pclk.raw() / 1000) / 4096;

        // Prescaler
        let (tb_div, wdgtb) = match ratio / 64 {
            0 => (1, 0),
            1 => (2, 1),
            2..=3 => (4, 2),
            4..=7 => (8, 3),
            8..=15 => (16, 4),
            16..=31 => (32, 5),
            32..=63 => (64, 6),
            64..=127 => (128, 7),
            _ => (128, 7),
        };

        let t = ratio / tb_div;
        assert!(t < 64);

        self.down_counter = (t as u8) | (1 << 6);

        // write the config values, matching the set timeout the most
        self.wwdg
            .cfr()
            .modify(|_, w| unsafe { w.wdgtb().bits(wdgtb) });
        self.wwdg
            .cfr()
            .modify(|_, w| unsafe { w.w().bits(self.down_counter) });
        self.wwdg
            .cr()
            .modify(|_, w| unsafe { w.t().bits(self.down_counter) });
        // For some reason, setting the t value makes the early wakeup pending.
        // That's bad behaviour, so lets turn it off again.
        self.unpend();
        // enable the watchdog
        self.wwdg.cr().modify(|_, w| w.wdga().set_bit());
    }

    /// Feeds the watchdog in order to avoid a reset, only executes properly if the watchdog
    /// has already been started aka. the down_counter is not 0 anymore
    pub fn feed(&mut self) {
        // if this value is 0 it is assumed that the watchdog has not yet been started
        assert!(self.down_counter != 0);
        self.wwdg
            .cr()
            .modify(|_, w| unsafe { w.t().bits(self.down_counter) });
    }

    /// Start listening for `event`
    pub fn listen(&mut self) {
        self.wwdg.cfr().modify(|_, w| w.ewi().set_bit());
    }

    /// Stop listening for `event`
    pub fn unlisten(&mut self) {
        panic!("Early wakeup of the Watchdog can only be cleared by hardware after a reset");
    }

    /// Returns `true` if `event` is pending
    pub fn is_pending(&self) -> bool {
        self.wwdg.sr().read().ewif().bit_is_set()
    }

    /// Clears the interrupt flag for `event`
    pub fn unpend(&mut self) {
        self.wwdg.sr().write(|w| w.ewif().clear_bit());
    }

    /// Returns a reference to the inner peripheral
    pub fn inner(&self) -> &pac::Wwdg {
        &self.wwdg
    }

    /// Returns a mutable reference to the inner peripheral
    pub fn inner_mut(&mut self) -> &mut pac::Wwdg {
        &mut self.wwdg
    }
}

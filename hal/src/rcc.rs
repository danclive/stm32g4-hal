//! Clock configuration.
use fugit::HertzU32 as Hertz;

use crate::pac::{
    self,
    rcc::{self, RegisterBlock as RccRB},
};

use clock::{Clocks, Rcc};

pub mod clock;
pub mod clockout;
mod enable;

/// Extension trait that constrains the `RCC` peripheral
pub trait RccExt {
    /// Constrains the `RCC` peripheral so it plays nicely with the other abstractions
    fn constrain(self) -> Rcc;
}

impl RccExt for pac::Rcc {
    fn constrain(self) -> Rcc {
        Rcc::new(self)
    }
}

/// Bus associated to peripheral
pub trait RccBus: crate::Sealed {
    /// Bus type;
    type Bus;
}

/// Enable/disable peripheral
#[allow(clippy::missing_safety_doc)]
pub trait Enable: RccBus {
    /// Enables peripheral
    fn enable(rcc: &RccRB);

    /// Disables peripheral
    fn disable(rcc: &RccRB);

    /// Check if peripheral enabled
    fn is_enabled() -> bool;

    fn enable_for_sleep_stop(rcc: &RccRB);

    /// Check if peripheral disabled
    #[inline]
    fn is_disabled() -> bool {
        !Self::is_enabled()
    }

    /// # Safety
    ///
    /// Enables peripheral. Takes access to RCC internally
    unsafe fn enable_unchecked() {
        let rcc = &*pac::Rcc::PTR;
        Self::enable(rcc);
    }

    /// # Safety
    ///
    /// Disables peripheral. Takes access to RCC internally
    unsafe fn disable_unchecked() {
        let rcc = pac::Rcc::PTR;
        Self::disable(&*rcc);
    }
}

/// Reset peripheral
#[allow(clippy::missing_safety_doc)]
pub trait Reset: RccBus {
    /// Resets peripheral
    fn reset(rcc: &RccRB);

    /// # Safety
    ///
    /// Resets peripheral. Takes access to RCC internally
    unsafe fn reset_unchecked() {
        let rcc = pac::Rcc::PTR;
        Self::reset(&*rcc);
    }
}

macro_rules! bus_struct {
    ($( $(#[$attr:meta])* $busX:ident => ($EN:ident, $en:ident, $RST:ident, $rst:ident, $SMEN:ident, $smen:ident, $doc:literal),)+) => {
        $(
            $(#[$attr])*
            #[doc = $doc]
            pub struct $busX {
                _0: (),
            }

            $(#[$attr])*
            impl $busX {
                #[allow(unused)]
                #[inline(always)]
                pub(crate) fn enr(rcc: &RccRB) -> &rcc::$EN {
                    &rcc.$en()
                }

                #[allow(unused)]
                #[inline(always)]
                pub(crate) fn rstr(rcc: &RccRB) -> &rcc::$RST {
                    &rcc.$rst()
                }

                #[allow(unused)]
                #[inline(always)]
                pub(crate) fn smenr(rcc: &RccRB) -> &rcc::$SMEN {
                    &rcc.$smen()
                }
            }
        )+
    };
}

bus_struct! {
    AHB1 => (RccAhb1enr, rcc_ahb1enr, RccAhb1rstr, rcc_ahb1rstr, RccAhb1smenr, rcc_ahb1smenr, "Advanced High-performance Bus 1 (AHB1) registers"),
    AHB2 => (RccAhb2enr, rcc_ahb2enr, RccAhb2rstr, rcc_ahb2rstr, RccAhb2smenr, rcc_ahb2smenr, "Advanced High-performance Bus 2 (AHB2) registers"),
    AHB3 => (RccAhb3enr, rcc_ahb3enr, RccAhb3rstr, rcc_ahb3rstr, RccAhb3smenr, rcc_ahb3smenr, "Advanced High-performance Bus 3 (AHB3) registers"),
    APB1_1 => (RccApb1enr1, rcc_apb1enr1, RccApb1rstr1, rcc_apb1rstr1, RccApb1smenr1, rcc_apb1smenr1, "Advanced Peripheral Bus 1_1 (APB1_1) registers"),
    APB1_2 => (RccApb1enr2, rcc_apb1enr2, RccApb1rstr2, rcc_apb1rstr2, RccApb1smenr2, rcc_apb1smenr2, "Advanced Peripheral Bus 1_2 (APB1_2) registers"),
    APB2 => (RccApb2enr, rcc_apb2enr, RccApb2rstr, rcc_apb2rstr, RccApb2smenr, rcc_apb2smenr, "Advanced Peripheral Bus 2 (APB2) registers"),
}

/// Frequency on bus that peripheral is connected in
pub trait BusClock {
    /// Calculates frequency depending on `Clock` state
    fn clock(clocks: &Clocks) -> Hertz;
}

impl<T> BusClock for T
where
    T: RccBus,
    T::Bus: BusClock,
{
    fn clock(clocks: &Clocks) -> Hertz {
        T::Bus::clock(clocks)
    }
}

impl BusClock for AHB1 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.ahb_clk()
    }
}

impl BusClock for AHB2 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.ahb_clk()
    }
}

impl BusClock for AHB3 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.ahb_clk()
    }
}

impl BusClock for APB1_1 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.apb1_clk()
    }
}

impl BusClock for APB1_2 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.apb1_clk()
    }
}

impl BusClock for APB2 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.apb2_clk()
    }
}

/// Frequency on bus that timer is connected in
pub trait BusTimerClock {
    /// Calculates base frequency of timer depending on `Clock` state
    fn timer_clock(clocks: &Clocks) -> Hertz;
}

impl<T> BusTimerClock for T
where
    T: RccBus,
    T::Bus: BusTimerClock,
{
    fn timer_clock(clocks: &Clocks) -> Hertz {
        T::Bus::timer_clock(clocks)
    }
}

impl BusTimerClock for APB1_1 {
    fn timer_clock(clocks: &Clocks) -> Hertz {
        clocks.apb1_tim_clk()
    }
}

impl BusTimerClock for APB1_2 {
    fn timer_clock(clocks: &Clocks) -> Hertz {
        clocks.apb1_tim_clk()
    }
}

impl BusTimerClock for APB2 {
    fn timer_clock(clocks: &Clocks) -> Hertz {
        clocks.apb2_tim_clk()
    }
}

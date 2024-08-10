//! RTC

use core::marker::PhantomData;

use cortex_m::interrupt;

use crate::pac;

/// Reset, Enable and Clock functionality for RTC
pub struct Rtc {
    pub(super) _marker: PhantomData<*const ()>,
}

unsafe impl Send for Rtc {}

/// RTC kernel clock source selection
pub type RtcClkSel = crate::pac::rcc::rcc_bdcr::Rtcsel;

impl Rtc {
    #[inline(always)]
    pub fn enable() {
        // unsafe: Owned exclusive access to this bitfield
        interrupt::free(|_| {
            let rcc = unsafe { &*pac::Rcc::PTR };
            rcc.rcc_bdcr().modify(|_, w| w.rtcen().set_bit());
        });
    }

    #[inline(always)]
    pub fn disable() {
        // unsafe: Owned exclusive access to this bitfield
        interrupt::free(|_| {
            let rcc = unsafe { &*pac::Rcc::PTR };
            rcc.rcc_bdcr().modify(|_, w| w.rtcen().clear_bit());
        });
    }

    #[inline(always)]
    pub fn is_enabled() -> bool {
        interrupt::free(|_| {
            let rcc = unsafe { &*pac::Rcc::PTR };
            rcc.rcc_bdcr().read().rtcen().bit_is_set()
        })
    }

    #[inline(always)]
    pub fn reset() {
        // unsafe: Owned exclusive access to this bitfield
        interrupt::free(|_| {
            let rcc = unsafe { &*pac::Rcc::PTR };
            rcc.rcc_bdcr().modify(|_, w| w.bdrst().set_bit());
        });
    }

    /// Modify a kernel clock for this
    /// peripheral. See RM0433 Section 8.5.8.
    ///
    /// # NOTE
    ///
    /// This may only be written one time per peripheral reset.
    /// Check `get_kernel_clk_mux()` to see if the write succeeded.
    #[inline(always)]
    pub fn kernel_clk_mux(sel: RtcClkSel) {
        // unsafe: Owned exclusive access to this bitfield
        interrupt::free(|_| {
            let rcc = unsafe { &*pac::Rcc::PTR };
            rcc.rcc_bdcr().modify(|_, w| w.rtcsel().variant(sel));
        });
    }

    /// Return the current kernel clock selection
    #[inline(always)]
    pub fn get_kernel_clk_mux() -> RtcClkSel {
        // unsafe: We only read from this bitfield
        let rcc = unsafe { &*pac::Rcc::PTR };
        rcc.rcc_bdcr().read().rtcsel().variant()
    }
}

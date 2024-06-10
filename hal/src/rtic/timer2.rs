//! [`Monotonic`](rtic_time::Monotonic) implementations for STM32 chips.
//!
//! Not all timers are available on all parts. Ensure that only available
//! timers are exposed by having the correct `stm32*` feature enabled for `rtic-monotonics`.
//!
//! # Example
//!
//! ```
//! use rtic_monotonics::stm32::prelude::*;
//!
//! // Define the monotonic and set it to 1MHz tick rate
//! stm32_tim2_monotonic!(Mono, 1_000_000);
//!
//! fn init() {
//!     // If using `embassy-stm32` HAL, timer clock can be read out like this:
//!     let timer_clock_hz = embassy_stm32::peripherals::TIM2::frequency();
//!     // Or define it manually if you are using other HAL or know correct frequency:
//!     let timer_clock_hz = 64_000_000;
//!
//!     // Start the monotonic
//!     Mono::start(timer_clock_hz);
//! }
//!
//! async fn usage() {
//!     loop {
//!          // Use the monotonic
//!          let timestamp = Mono::now();
//!          Mono::delay(100.millis()).await;
//!     }
//! }
//! ```

use portable_atomic::{AtomicU64, Ordering};
use rtic_time::{
    half_period_counter::calculate_now,
    timer_queue::{TimerQueue, TimerQueueBackend},
};

use crate::pac;

use super::*;

// mod _generated {
//     #![allow(dead_code)]
//     #![allow(unused_imports)]
//     #![allow(non_snake_case)]

//     include!(concat!(env!("OUT_DIR"), "/_generated.rs"));
// }

#[doc(hidden)]
#[macro_export]
macro_rules! __internal_create_stm32_timer_interrupt {
    ($mono_backend:ident, $interrupt_name:ident) => {
        #[no_mangle]
        #[allow(non_snake_case)]
        unsafe extern "C" fn $interrupt_name() {
            use $crate::rtic::TimerQueueBackend;
            $crate::rtic::timer2::$mono_backend::timer_queue().on_monotonic_interrupt();
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __internal_create_stm32_timer_struct {
    ($name:ident, $mono_backend:ident, $timer:ident, $tick_rate_hz:expr) => {
        /// A `Monotonic` based on an STM32 timer peripheral.
        pub struct $name;

        impl $name {
            /// Starts the `Monotonic`.
            ///
            /// - `tim_clock_hz`: `TIMx` peripheral clock frequency.
            ///
            /// Panics if it is impossible to achieve the desired monotonic tick rate based
            /// on the given `tim_clock_hz` parameter. If that happens, adjust the desired monotonic tick rate.
            ///
            /// This method must be called only once.
            pub fn start(tim_clock_hz: u32) {
                $crate::__internal_create_stm32_timer_interrupt!($mono_backend, $timer);

                $crate::rtic::timer2::$mono_backend::_start(tim_clock_hz, $tick_rate_hz);
            }
        }

        impl $crate::rtic::TimerQueueBasedMonotonic for $name {
            type Backend = $crate::rtic::timer2::$mono_backend;
            type Instant = $crate::fugit::Instant<
                <Self::Backend as $crate::rtic::TimerQueueBackend>::Ticks,
                1,
                { $tick_rate_hz },
            >;
            type Duration = $crate::fugit::Duration<
                <Self::Backend as $crate::rtic::TimerQueueBackend>::Ticks,
                1,
                { $tick_rate_hz },
            >;
        }

        $crate::rtic::rtic_time::impl_embedded_hal_delay_fugit!($name);
        $crate::rtic::rtic_time::impl_embedded_hal_async_delay_fugit!($name);
    };
}

/// Create a TIM4 based monotonic and register the TIM4 interrupt for it.
///
/// See [`crate::stm32`] for more details.
///
/// # Arguments
///
/// * `name` - The name that the monotonic type will have.
/// * `tick_rate_hz` - The tick rate of the timer peripheral.
///
#[cfg(feature = "rtic-tim2")]
#[macro_export]
macro_rules! stm32_tim2_monotonic {
    ($name:ident, $tick_rate_hz:expr) => {
        $crate::__internal_create_stm32_timer_struct!($name, Tim2Backend, TIM2, $tick_rate_hz);
    };
}

macro_rules! make_timer {
    ($tim: ident, $backend_name:ident, $timer:ident, $bits:ident, $overflow:ident, $tq:ident$(, doc: ($($doc:tt)*))?) => {
        /// Monotonic timer backend implementation.
        $(
            #[cfg_attr(docsrs, doc(cfg($($doc)*)))]
        )?

        pub struct $backend_name;

        // use pac::$timer;

        static $overflow: AtomicU64 = AtomicU64::new(0);
        static $tq: TimerQueue<$backend_name> = TimerQueue::new();

        #[inline(always)]
        fn tim() -> &'static pac::$tim::RegisterBlock {
            unsafe { &*<pac::$timer>::ptr() }
        }

        impl $backend_name {
            /// Starts the timer.
            ///
            /// **Do not use this function directly.**
            ///
            /// Use the prelude macros instead.
            pub fn _start(tim_clock_hz: u32, timer_hz: u32) {
                // _generated::$timer::enable();
                // _generated::$timer::reset();
                use $crate::rcc::{Enable, Reset};
                unsafe { $crate::pac::$timer::enable_unchecked(); }
                unsafe { $crate::pac::$timer::reset_unchecked(); }

                // $timer.cr1().modify(|r| r.set_cen(false));
                // 0x00 - control register 1 Bit 0 - Counter enable
                tim().cr1().modify(|_, w| w.cen().bit(false));

                assert!((tim_clock_hz % timer_hz) == 0, "Unable to find suitable timer prescaler value!");
                let psc = tim_clock_hz / timer_hz - 1;
                defmt::info!("psc: {}", psc);
                // $timer.psc().write(|r| r.set_psc(psc as u16));
                // 0x28 - prescaler Bits 0:15 - Prescaler value
                tim().psc().write(|w| unsafe { w.psc().bits(psc as u16) });

                // Enable full-period interrupt.
                // $timer.dier().modify(|r| r.set_uie(true));
                // 0x0c - DMA/Interrupt enable register Bit 0 - Update interrupt enable
                tim().dier().modify(|_, w| w.uie().set_bit());

                // Configure and enable half-period interrupt
                // $timer.ccr(2).write(|r| r.set_ccr(($bits::MAX - ($bits::MAX >> 1)).into()));
                // 0x38 - capture/compare register 2 Bits 0:15 - Capture/Compare 2 value
                tim()
                    .ccr2()
                    .write(|w| unsafe { w.ccr2().bits(($bits::MAX - ($bits::MAX >> 1)) as _) });
                // $timer.dier().modify(|r| r.set_ccie(2, true));
                // 0x0c - DMA/Interrupt enable register Bit 2 - Capture/Compare 2 interrupt enable
                tim().dier().modify(|_, w| w.cc2ie().set_bit());

                // Trigger an update event to load the prescaler value to the clock.
                // $timer.egr().write(|r| r.set_ug(true));
                // 0x14 - event generation register  Bit 0 - Update generation
                tim().egr().write(|w| w.ug().set_bit());

                // The above line raises an update event which will indicate that the timer is already finished.
                // Since this is not the case, it should be cleared.
                // $timer.sr().modify(|r| r.set_uif(false));
                // 0x10 - status register Bit 0 - Update interrupt flag
                tim().sr().modify(|_, w| w.uif().clear_bit());

                $tq.initialize(Self {});
                $overflow.store(0, Ordering::SeqCst);

                // Start the counter.
                // 0x00 - control register 1 Bit 0 - Counter enable
                tim().cr1().modify(|_, w| w.cen().bit(true));

                // SAFETY: We take full ownership of the peripheral and interrupt vector,
                // plus we are not using any external shared resources so we won't impact
                // basepri/source masking based critical sections.
                unsafe {
                    let mut nvic: cortex_m::peripheral::NVIC = core::mem::transmute(());
                    set_monotonic_prio(&mut nvic, pac::NVIC_PRIO_BITS, <pac::$timer>::IRQ);
                    cortex_m::peripheral::NVIC::unmask(<pac::$timer>::IRQ);
                }
            }
        }

        impl TimerQueueBackend for $backend_name {
            type Ticks = u64;

            fn now() -> Self::Ticks {
                // defmt::info!("cnt!: {}", tim().cnt().read().bits());
                // defmt::info!("cnt!222: {}", $overflow.load(Ordering::Relaxed));
                // 0x24 - counter Bits 0:15 - counter value
                calculate_now(
                    || $overflow.load(Ordering::Relaxed),
                    || tim().cnt().read().bits()
                )
            }

            fn set_compare(instant: Self::Ticks) {
                defmt::info!("---------");
                defmt::info!("instant: {}", instant);
                let now = Self::now();
                defmt::info!("now: {}", now);
                defmt::info!("instant - now: {}", instant - now);
                defmt::info!("=========");

                // Since the timer may or may not overflow based on the requested compare val, we check how many ticks are left.
                // `wrapping_sup` takes care of the u64 integer overflow special case.
                let val = if instant.wrapping_sub(now) <= ($bits::MAX as u64) {
                    instant as $bits
                } else {
                    // In the past or will overflow
                    0
                };

                defmt::info!("val: {}", val);

                // $timer.ccr(1).write(|r| r.set_ccr(val.into()));
                // 0x34 - capture/compare register 1 Bits 0:15 - Capture/Compare 1 value
                tim()
                    .ccr1()
                    .write(|w| unsafe { w.ccr1().bits(val as _) });
            }

            fn clear_compare_flag() {
                // $timer.sr().modify(|r| r.set_ccif(1, false));
                // 0x10 - status register Bit 1 - Capture/compare 1 interrupt flag
                tim().sr().modify(|_, w| w.cc1if().clear_bit());
            }

            fn pend_interrupt() {
                cortex_m::peripheral::NVIC::pend(<pac::$timer>::IRQ);
            }

            fn enable_timer() {
                // $timer.dier().modify(|r| r.set_ccie(1, true));
                // 0x0c - DMA/Interrupt enable register Bit 1 - Capture/Compare 1 interrupt enable
                tim().dier().modify(|_, w| w.cc1ie().set_bit());
            }

            fn disable_timer() {
                // $timer.dier().modify(|r| r.set_ccie(1, false));
                tim().dier().modify(|_, w| w.cc1ie().clear_bit());
            }

            fn on_interrupt() {
                // Full period
                // if $timer.sr().read().uif() {
                //     $timer.sr().modify(|r| r.set_uif(false));
                //     let prev = $overflow.fetch_add(1, Ordering::Relaxed);
                //     assert!(prev % 2 == 1, "Monotonic must have missed an interrupt!");
                // }
                // // Half period
                // if $timer.sr().read().ccif(2) {
                //     $timer.sr().modify(|r| r.set_ccif(2, false));
                //     let prev = $overflow.fetch_add(1, Ordering::Relaxed);
                //     assert!(prev % 2 == 0, "Monotonic must have missed an interrupt!");
                // }
                // 0x10 - status register Bit 0 - Update interrupt flag
                if tim().sr().read().uif().bit_is_set() {
                    tim().sr().modify(|_, w| w.uif().clear_bit());
                    let prev = $overflow.fetch_add(1, Ordering::Relaxed);
                    assert!(prev % 2 == 1, "Monotonic must have missed an interrupt!");
                }
                // Half period
                // 0x10 - status register Bit 2 - Capture/Compare 2 interrupt flag
                if tim().sr().read().cc2if().bit_is_set() {
                    tim().sr().modify(|_, w| w.cc2if().clear_bit());
                    let prev = $overflow.fetch_add(1, Ordering::Relaxed);
                    assert!(prev % 2 == 0, "Monotonic must have missed an interrupt!");
                }
            }

            fn timer_queue() -> &'static TimerQueue<$backend_name> {
                &$tq
            }
        }
    };
}

#[cfg(feature = "rtic-tim2")]
make_timer!(tim2, Tim2Backend, Tim2, u32, TIMER4_OVERFLOWS, TIMER4_TQ);

pub trait Irq {
    const IRQ: pac::Interrupt;
}

impl Irq for pac::Tim2 {
    const IRQ: pac::Interrupt = pac::Interrupt::TIM2;
}

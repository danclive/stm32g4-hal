//! rtic_time::Monotonic implementations for timer.

use portable_atomic::{AtomicU64, Ordering};
use rtic_time::{
    half_period_counter::calculate_now,
    timer_queue::{TimerQueue, TimerQueueBackend},
};

use super::*;

#[doc(hidden)]
#[macro_export]
macro_rules! __internal_create_timer_interrupt {
    ($interrupt_name:ident, $mono_backend:ident) => {
        #[unsafe(no_mangle)]
        #[allow(non_snake_case)]
        unsafe extern "C" fn $interrupt_name() {
            use $crate::rtic::TimerQueueBackend;
            $crate::rtic::timer::$mono_backend::timer_queue().on_monotonic_interrupt();
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __internal_create_timer_struct {
    ($name:ident, $timer:ident, $interrupt_name:ident, $mono_backend:ident, $tick_rate_hz:expr) => {
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
            pub fn start(
                nvic: &mut cortex_m::peripheral::NVIC,
                tim: $crate::pac::$timer,
                clocks: &$crate::rcc::Clocks,
            ) {
                $crate::__internal_create_timer_interrupt!($interrupt_name, $mono_backend);

                $crate::rtic::timer::$mono_backend::_start(
                    nvic,
                    tim,
                    clocks.sys_clk().raw(),
                    $tick_rate_hz,
                );
            }
        }

        trait __MonoTimerExt: Sized {
            fn monotonic(self, nvic: &mut cortex_m::peripheral::NVIC, clocks: &$crate::rcc::Clocks);
        }

        impl __MonoTimerExt for $crate::pac::$timer {
            fn monotonic(
                self,
                nvic: &mut cortex_m::peripheral::NVIC,
                clocks: &$crate::rcc::Clocks,
            ) {
                $name::start(nvic, self, clocks);
            }
        }

        impl $crate::rtic::TimerQueueBasedMonotonic for $name {
            type Backend = $crate::rtic::timer::$mono_backend;
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

/// Create a TIM2 based monotonic and register the TIM2 interrupt for it.
///
/// # Arguments
///
/// * `name` - The name that the monotonic type will have.
/// * `tick_rate_hz` - The tick rate of the timer peripheral.
///
#[cfg(feature = "rtic-tim2")]
#[macro_export]
macro_rules! tim2_monotonic {
    ($name:ident, $tick_rate_hz:expr) => {
        $crate::__internal_create_timer_struct!($name, Tim2, TIM2, Tim2Backend, $tick_rate_hz);
    };
}

/// Create a TIM5 based monotonic and register the TIM5 interrupt for it.
///
/// # Arguments
///
/// * `name` - The name that the monotonic type will have.
/// * `tick_rate_hz` - The tick rate of the timer peripheral.
///
#[cfg(all(feature = "tim5", feature = "rtic-tim5"))]
#[macro_export]
macro_rules! tim5_monotonic {
    ($name:ident, $tick_rate_hz:expr) => {
        $crate::__internal_create_timer_struct!($name, Tim5, TIM5, Tim5Backend, $tick_rate_hz);
    };
}

macro_rules! make_timer {
    ($tim: ident, $timer:ident, $bits:ident, $backend_name:ident, $overflow:ident, $tq:ident$(, doc: ($($doc:tt)*))?) => {
        /// Monotonic timer backend implementation.
        $(
            #[cfg_attr(docsrs, doc(cfg($($doc)*)))]
        )?

        pub struct $backend_name;

        static $overflow: AtomicU64 = AtomicU64::new(0);
        static $tq: TimerQueue<$backend_name> = TimerQueue::new();

        impl $backend_name {
            /// Starts the timer.
            ///
            /// **Do not use this function directly.**
            ///
            /// Use the prelude macros instead.
            pub fn _start(nvic: &mut cortex_m::peripheral::NVIC, tim: $crate::pac::$timer, tim_clock_hz: u32, timer_hz: u32) {
                use $crate::rcc::{Enable, Reset};
                unsafe { $crate::pac::$timer::enable_unchecked(); }
                unsafe { $crate::pac::$timer::reset_unchecked(); }

                tim.cr1().modify(|_, w| w.cen().bit(false));

                assert!((tim_clock_hz % timer_hz) == 0, "Unable to find suitable timer prescaler value!");
                let psc = tim_clock_hz / timer_hz - 1;

                tim.psc().write(|w| unsafe { w.psc().bits(psc as u16) });

                // Enable full-period interrupt.
                tim.dier().modify(|_, w| w.uie().set_bit());

                // Configure and enable half-period interrupt
                tim
                    .ccr2()
                    .write(|w| unsafe { w.ccr2().bits(($bits::MAX - ($bits::MAX >> 1)).into()) });
                tim.dier().modify(|_, w| w.cc2ie().set_bit());

                // Trigger an update event to load the prescaler value to the clock.
                tim.egr().write(|w| w.ug().set_bit());

                // The above line raises an update event which will indicate that the timer is already finished.
                // Since this is not the case, it should be cleared.
                tim.sr().modify(|_, w| w.uif().clear_bit());

                $tq.initialize(Self {});
                $overflow.store(0, Ordering::SeqCst);

                // Start the counter.
                tim.cr1().modify(|_, w| w.cen().bit(true));

                // SAFETY: We take full ownership of the peripheral and interrupt vector,
                // plus we are not using any external shared resources so we won't impact
                // basepri/source masking based critical sections.
                unsafe {
                    set_monotonic_prio(nvic, $crate::pac::NVIC_PRIO_BITS, <$crate::pac::$timer>::IRQ);
                    cortex_m::peripheral::NVIC::unmask(<$crate::pac::$timer>::IRQ);
                }
            }

            #[inline(always)]
            fn tim() -> &'static $crate::pac::$tim::RegisterBlock {
                unsafe { &*<$crate::pac::$timer>::ptr() }
            }
        }

        impl TimerQueueBackend for $backend_name {
            type Ticks = u64;

            fn now() -> Self::Ticks {
                calculate_now(
                    || $overflow.load(Ordering::Relaxed),
                    || Self::tim().cnt().read().bits()
                )
            }

            fn set_compare(instant: Self::Ticks) {
                let now = Self::now();

                // Since the timer may or may not overflow based on the requested compare val, we check how many ticks are left.
                // `wrapping_sup` takes care of the u64 integer overflow special case.
                let val = if instant.wrapping_sub(now) <= ($bits::MAX as u64) {
                    instant as $bits
                } else {
                    // In the past or will overflow
                    0
                };

                Self::tim()
                    .ccr1()
                    .write(|w| unsafe { w.ccr1().bits(val.into()) });
            }

            fn clear_compare_flag() {
                Self::tim().sr().modify(|_, w| w.cc1if().clear_bit());
            }

            fn pend_interrupt() {
                cortex_m::peripheral::NVIC::pend(<$crate::pac::$timer>::IRQ);
            }

            fn enable_timer() {
                Self::tim().dier().modify(|_, w| w.cc1ie().set_bit());
            }

            fn disable_timer() {
                Self::tim().dier().modify(|_, w| w.cc1ie().clear_bit());
            }

            fn on_interrupt() {
                // Full period
                if Self::tim().sr().read().uif().bit_is_set() {
                    Self::tim().sr().modify(|_, w| w.uif().clear_bit());
                    let prev = $overflow.fetch_add(1, Ordering::Relaxed);
                    assert!(prev % 2 == 1, "Monotonic must have missed an interrupt!");
                }
                // Half period
                if Self::tim().sr().read().cc2if().bit_is_set() {
                    Self::tim().sr().modify(|_, w| w.cc2if().clear_bit());
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
make_timer!(tim2, Tim2, u32, Tim2Backend, TIMER2_OVERFLOWS, TIMER2_TQ);

#[cfg(all(feature = "tim5", feature = "rtic-tim5"))]
make_timer!(tim5, Tim5, u32, Tim5Backend, TIMER5_OVERFLOWS, TIMER5_TQ);

pub trait Irq {
    const IRQ: crate::pac::Interrupt;
}

impl Irq for crate::pac::Tim2 {
    const IRQ: crate::pac::Interrupt = crate::pac::Interrupt::TIM2;
}

#[cfg(feature = "tim5")]
impl Irq for crate::pac::Tim5 {
    const IRQ: crate::pac::Interrupt = crate::pac::Interrupt::TIM5;
}

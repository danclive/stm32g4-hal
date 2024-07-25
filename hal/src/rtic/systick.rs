//! rtic_time::Monotonic implementations for systick.

use portable_atomic::Ordering;
use rtic_time::timer_queue::{TimerQueue, TimerQueueBackend};

cfg_if::cfg_if! {
    if #[cfg(feature = "rtic-systick-64bit")] {
        use portable_atomic::AtomicU64;
        static SYSTICK_CNT: AtomicU64 = AtomicU64::new(0);
    } else {
        use portable_atomic::AtomicU32;
        static SYSTICK_CNT: AtomicU32 = AtomicU32::new(0);
    }
}

static SYSTICK_TIMER_QUEUE: TimerQueue<SystickBackend> = TimerQueue::new();

/// Systick based [`TimerQueueBackend`].
pub struct SystickBackend;

impl SystickBackend {
    /// Starts the monotonic timer.
    ///
    /// **Do not use this function directly.**
    ///
    /// Use the prelude macros instead.
    pub fn _start(mut systick: crate::pac::SYST, sysclk: u32, timer_hz: u32) {
        assert!(
            (sysclk % timer_hz) == 0,
            "timer_hz cannot evenly divide sysclk! Please adjust the timer or sysclk frequency."
        );
        let reload = sysclk / timer_hz - 1;

        assert!(reload <= 0x00ff_ffff);
        assert!(reload > 0);

        systick.disable_counter();
        systick.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
        systick.set_reload(reload);
        systick.enable_interrupt();
        systick.enable_counter();

        SYSTICK_TIMER_QUEUE.initialize(SystickBackend {});
    }

    fn systick() -> crate::pac::SYST {
        unsafe { core::mem::transmute::<(), crate::pac::SYST>(()) }
    }
}

impl TimerQueueBackend for SystickBackend {
    cfg_if::cfg_if! {
        if #[cfg(feature = "rtic-systick-64bit")] {
            type Ticks = u64;
        } else {
            type Ticks = u32;
        }
    }

    fn now() -> Self::Ticks {
        if Self::systick().has_wrapped() {
            SYSTICK_CNT.fetch_add(1, Ordering::AcqRel);
        }

        SYSTICK_CNT.load(Ordering::Relaxed)
    }

    fn set_compare(_: Self::Ticks) {
        // No need to do something here, we get interrupts anyway.
    }

    fn clear_compare_flag() {
        // NOOP with SysTick interrupt
    }

    fn pend_interrupt() {
        cortex_m::peripheral::SCB::set_pendst();
    }

    fn on_interrupt() {
        if Self::systick().has_wrapped() {
            SYSTICK_CNT.fetch_add(1, Ordering::AcqRel);
        }
    }

    fn timer_queue() -> &'static TimerQueue<Self> {
        &SYSTICK_TIMER_QUEUE
    }
}

/// Create a Systick based monotonic and register the Systick interrupt for it.
///
/// See [`crate::rtic::systick`] for more details.
///
/// # Arguments
///
/// * `name` - The name that the monotonic type will have.
/// * `tick_rate_hz` - The tick rate of the timer peripheral.
///                    Can be omitted; defaults to 1kHz.
#[macro_export]
macro_rules! systick_monotonic {
    ($name:ident) => {
        $crate::systick_monotonic($name, 1_000);
    };
    ($name:ident, $tick_rate_hz:expr) => {
        /// A `Monotonic` based on SysTick.
        pub struct $name;

        impl $name {
            /// Starts the `Monotonic`.
            ///
            /// The `sysclk` parameter is the speed at which SysTick runs at. This value should come from
            /// the clock generation function of the used HAL.
            ///
            /// Panics if it is impossible to achieve the desired monotonic tick rate based
            /// on the given `sysclk` parameter. If that happens, adjust the desired monotonic tick rate.
            ///
            /// This method must be called only once.
            pub fn start(systick: $crate::pac::SYST, clocks: &$crate::rcc::Clocks) {
                #[no_mangle]
                #[allow(non_snake_case)]
                unsafe extern "C" fn SysTick() {
                    use $crate::rtic::TimerQueueBackend;
                    $crate::rtic::systick::SystickBackend::timer_queue().on_monotonic_interrupt();
                }

                $crate::rtic::systick::SystickBackend::_start(
                    systick,
                    clocks.sys_clk().raw(),
                    $tick_rate_hz,
                );
            }
        }

        trait __MonoSystickExt: Sized {
            fn monotonic(self, clocks: &$crate::rcc::Clocks);
        }

        impl __MonoSystickExt for $crate::pac::SYST {
            fn monotonic(self, clocks: &$crate::rcc::Clocks) {
                $name::start(self, clocks);
            }
        }

        impl $crate::rtic::TimerQueueBasedMonotonic for $name {
            type Backend = $crate::rtic::systick::SystickBackend;
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

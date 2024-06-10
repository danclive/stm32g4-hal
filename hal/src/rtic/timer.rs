use core::marker::PhantomData;

use portable_atomic::{AtomicU64, Ordering};
use rtic_time::{
    half_period_counter::calculate_now,
    timer_queue::{TimerQueue, TimerQueueBackend},
};

use crate::timer::sealed::General;
use crate::{pac, rcc::clock::Clocks, timer::FixedTimer};

use super::*;

pub struct MonoTimer<TIM, const FREQ: u32> {
    _tim: PhantomData<TIM>,
}
/// `MonoTimer` with precision of 1 μs (1 MHz sampling)
pub type MonoTimerUs<TIM> = MonoTimer<TIM, 1_000_000>;

pub struct MonoTimerBackend<TIM> {
    _tim: PhantomData<TIM>,
}

impl<TIM: 'static, const FREQ: u32> TimerQueueBasedMonotonic for MonoTimer<TIM, FREQ>
where
    MonoTimerBackend<TIM>: TimerQueueBackend<Ticks = u64>,
{
    type Backend = MonoTimerBackend<TIM>;
    type Instant = fugit::TimerInstantU64<FREQ>;
    type Duration = fugit::TimerDurationU64<FREQ>;
}

pub trait MonoTimerExt: Sized {
    fn monotonic<const FREQ: u32>(
        self,
        nvic: &mut cortex_m::peripheral::NVIC,
        clocks: &Clocks,
    ) -> MonoTimer<Self, FREQ>;
    fn monotonic_us(
        self,
        nvic: &mut cortex_m::peripheral::NVIC,
        clocks: &Clocks,
    ) -> MonoTimer<Self, 1_000_000> {
        self.monotonic::<1_000_000>(nvic, clocks)
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __internal_create_stm32_timer_interrupt {
    ($timer:ident, $interrupt_name:ident) => {
        #[no_mangle]
        #[allow(non_snake_case)]
        unsafe extern "C" fn $interrupt_name() {
            use $crate::rtic::TimerQueueBackend;
            crate::rtic::timer::MonoTimerBackend::<$crate::pac::$timer>::timer_queue()
                .on_monotonic_interrupt();
        }
    };
}

macro_rules! make_timer {
    ($tim: ident, $timer:ident, $interrupt_name:ident, $bits:ident, $overflow:ident, $tq:ident$(, doc: ($($doc:tt)*))?) => {
        static $overflow: AtomicU64 = AtomicU64::new(0);
        static $tq: TimerQueue<MonoTimerBackend<pac::$timer>> = TimerQueue::new();

        impl MonoTimerExt for pac::$timer {
            fn monotonic<const FREQ: u32>(
                self,
                nvic: &mut cortex_m::peripheral::NVIC,
                clocks: &Clocks,
            ) -> MonoTimer<Self, FREQ> {
                FixedTimer::new(self, clocks).monotonic(nvic)
            }
        }

        impl<const FREQ: u32> FixedTimer<pac::$timer, FREQ> {
            pub fn monotonic(
                mut self,
                nvic: &mut cortex_m::peripheral::NVIC,
            ) -> MonoTimer<pac::$timer, FREQ> {
                __internal_create_stm32_timer_interrupt!($timer, $interrupt_name);

                // Enable full-period interrupt.
                self.tim.dier().modify(|_, w| w.uie().set_bit());

                // Configure and enable half-period interrupt
                self.tim
                    .ccr2()
                    .write(|w| unsafe { w.ccr2().bits(($bits::MAX - ($bits::MAX >> 1)) as _) });
                self.tim.dier().modify(|_, w| w.cc2ie().set_bit());

                // Trigger an update event to load the prescaler value to the clock.
                self.tim.egr().write(|w| w.ug().set_bit());

                // The above line raises an update event which will indicate that the timer is already finished.
                // Since this is not the case, it should be cleared.
                self.tim.sr().modify(|_, w| w.uif().clear_bit());

                $tq.initialize(MonoTimerBackend::<pac::$timer> { _tim: PhantomData });
                $overflow.store(0, Ordering::SeqCst);

                // Start the counter.
                self.tim.enable_counter(true);

                // SAFETY: We take full ownership of the peripheral and interrupt vector,
                // plus we are not using any external shared resources so we won't impact
                // basepri/source masking based critical sections.
                unsafe {
                    set_monotonic_prio(nvic, pac::NVIC_PRIO_BITS, <pac::$timer>::IRQ);
                    cortex_m::peripheral::NVIC::unmask(<pac::$timer>::IRQ);
                }
                MonoTimer { _tim: PhantomData }
            }
        }

        impl MonoTimerBackend<pac::$timer> {
            #[inline(always)]
            fn tim() -> &'static pac::$tim::RegisterBlock {
                unsafe { &*<pac::$timer>::ptr() }
            }
        }

        impl TimerQueueBackend for MonoTimerBackend<pac::$timer> {
            type Ticks = u64;

            fn now() -> Self::Ticks {
                calculate_now(
                    || $overflow.load(Ordering::Relaxed),
                    || Self::tim().cnt().read().bits(),
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
                    .write(|w| unsafe { w.ccr1().bits(val as _) });
            }

            fn clear_compare_flag() {
                Self::tim().sr().modify(|_, w| w.cc1if().clear_bit());
            }

            fn pend_interrupt() {
                cortex_m::peripheral::NVIC::pend(<pac::$timer>::IRQ);
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

            fn timer_queue() -> &'static TimerQueue<Self> {
                &$tq
            }
        }
    };
}

#[cfg(all(feature = "tim2", feature = "rtic-tim2"))]
make_timer!(tim2, Tim2, TIM2, u32, TIMER2_OVERFLOWS, TIMER2_TQ);

#[cfg(all(feature = "tim3", feature = "rtic-tim3"))]
make_timer!(tim3, Tim3, TIM3, u16, TIMER3_OVERFLOWS, TIMER3_TQ);

#[cfg(all(feature = "tim4", feature = "rtic-tim4"))]
make_timer!(tim4, Tim4, TIM4, u16, TIMER4_OVERFLOWS, TIMER4_TQ);

#[cfg(all(feature = "tim5", feature = "rtic-tim5"))]
make_timer!(tim5, Tim5, TIM5, u16, TIMER5_OVERFLOWS, TIMER5_TQ);

pub trait Irq {
    const IRQ: pac::Interrupt;
}

#[cfg(feature = "tim2")]
impl Irq for pac::Tim2 {
    const IRQ: pac::Interrupt = pac::Interrupt::TIM2;
}

#[cfg(feature = "tim3")]
impl Irq for pac::Tim3 {
    const IRQ: pac::Interrupt = pac::Interrupt::TIM3;
}

#[cfg(feature = "tim4")]
impl Irq for pac::Tim4 {
    const IRQ: pac::Interrupt = pac::Interrupt::TIM4;
}

#[cfg(feature = "tim5")]
impl Irq for pac::Tim5 {
    const IRQ: pac::Interrupt = pac::Interrupt::TIM5;
}

impl<TIM: 'static, const FREQ: u32> embedded_hal::delay::DelayNs for MonoTimer<TIM, FREQ>
where
    Self:
        Monotonic<Instant = fugit::TimerInstantU64<FREQ>, Duration = fugit::TimerDurationU64<FREQ>>,
{
    fn delay_ns(&mut self, ns: u32) {
        let now = Self::now();
        let mut done = now + <Self as Monotonic>::Duration::nanos_at_least(ns.into());
        if now != done {
            // Compensate for sub-tick uncertainty
            done += <Self as Monotonic>::Duration::from_ticks(1);
        }

        while Self::now() < done {}
    }

    fn delay_us(&mut self, us: u32) {
        let now = Self::now();
        let mut done = now + <Self as Monotonic>::Duration::micros_at_least(us.into());
        if now != done {
            // Compensate for sub-tick uncertainty
            done += <Self as Monotonic>::Duration::from_ticks(1);
        }

        while Self::now() < done {}
    }

    fn delay_ms(&mut self, ms: u32) {
        let now = Self::now();
        let mut done = now + <Self as Monotonic>::Duration::millis_at_least(ms.into());
        if now != done {
            // Compensate for sub-tick uncertainty
            done += <Self as Monotonic>::Duration::from_ticks(1);
        }

        while Self::now() < done {}
    }
}

impl<TIM: 'static, const FREQ: u32> embedded_hal_async::delay::DelayNs for MonoTimer<TIM, FREQ>
where
    Self:
        Monotonic<Instant = fugit::TimerInstantU64<FREQ>, Duration = fugit::TimerDurationU64<FREQ>>,
{
    #[inline]
    async fn delay_ns(&mut self, ns: u32) {
        Self::delay(<Self as Monotonic>::Duration::nanos_at_least(ns.into())).await;
    }

    #[inline]
    async fn delay_us(&mut self, us: u32) {
        Self::delay(<Self as Monotonic>::Duration::micros_at_least(us.into())).await;
    }

    #[inline]
    async fn delay_ms(&mut self, ms: u32) {
        Self::delay(<Self as Monotonic>::Duration::millis_at_least(ms.into())).await;
    }
}

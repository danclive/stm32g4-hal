//! ritc

#[cfg(feature = "rtic-systick")]
pub mod systick;
#[cfg(any(feature = "rtic-tim2", feature = "rtic-tim5"))]
pub mod timer;

pub use rtic_time::{
    self, monotonic::TimerQueueBasedMonotonic, timer_queue::TimerQueueBackend, Monotonic,
    TimeoutError,
};

pub(crate) unsafe fn set_monotonic_prio(
    nvic: &mut cortex_m::peripheral::NVIC,
    prio_bits: u8,
    interrupt: impl cortex_m::interrupt::InterruptNumber,
) {
    extern "C" {
        static RTIC_ASYNC_MAX_LOGICAL_PRIO: u8;
    }

    let max_prio = RTIC_ASYNC_MAX_LOGICAL_PRIO.max(1).min(1 << prio_bits);

    let hw_prio = cortex_logical2hw(max_prio, prio_bits);

    nvic.set_priority(interrupt, hw_prio);
}

pub(crate) const fn cortex_logical2hw(logical: u8, nvic_prio_bits: u8) -> u8 {
    ((1 << nvic_prio_bits) - logical) << (8 - nvic_prio_bits)
}

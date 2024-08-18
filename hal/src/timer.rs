//! Timer
use core::convert::TryFrom;
use cortex_m::peripheral::syst::SystClkSource;
use enumflags2::BitFlags;
use fugit::HertzU32 as Hertz;

use crate::dma;
use crate::pac::{self, SYST};
use crate::rcc::{self, Clocks};

use counter::{
    Counter, CounterHz, CounterMs, CounterUs, SYSTCounter, SYSTCounterHz, SYSTCounterUs,
};
use delay::{Delay, DelayMs, DelayUs, SYSTDelay};
use sealed::General;

pub mod counter;
pub mod delay;
mod ehal;
pub(crate) mod sealed;

/// Timer wrapper.
///
/// This wrapper can be used both for the system timer (SYST) or the
/// general-purpose timers (TIMx).
///
/// Note: If you want to use the timer to sleep a certain amount of time, use
/// [`Delay`](`crate::timer::delay::Delay`).
pub struct Timer<TIM> {
    pub(crate) tim: TIM,
    pub(crate) clk: Hertz,
}

impl Timer<SYST> {
    /// Initialize SysTick timer
    pub fn syst(mut tim: SYST, clocks: &Clocks) -> Self {
        tim.set_clock_source(SystClkSource::Core);

        Self {
            tim,
            clk: clocks.ahb_clk(),
        }
    }

    /// Initialize SysTick timer and set it frequency to `AHB_CLK / 8`
    pub fn syst_external(mut tim: SYST, clocks: &Clocks) -> Self {
        tim.set_clock_source(SystClkSource::External);
        Self {
            tim,
            clk: clocks.ahb_clk() / 8,
        }
    }

    pub fn configure(&mut self, clocks: &Clocks) {
        self.tim.set_clock_source(SystClkSource::Core);
        self.clk = clocks.ahb_clk();
    }

    pub fn configure_external(&mut self, clocks: &Clocks) {
        self.tim.set_clock_source(SystClkSource::External);
        self.clk = clocks.ahb_clk() / 8;
    }

    pub fn release(self) -> SYST {
        self.tim
    }

    /// Starts listening for an `event`
    pub fn listen(&mut self, event: SYSTEvent) {
        match event {
            SYSTEvent::Update => self.tim.enable_interrupt(),
        }
    }

    /// Stops listening for an `event`
    pub fn unlisten(&mut self, event: SYSTEvent) {
        match event {
            SYSTEvent::Update => self.tim.disable_interrupt(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SYSTEvent {
    Update,
}

pub trait SYSTimerExt: Sized {
    /// Creates timer which takes [Hertz] as Duration
    fn counter_hz(self, clocks: &Clocks) -> SYSTCounterHz;

    /// Creates timer with custom precision (core frequency recommended is known)
    fn counter<const FREQ: u32>(self, clocks: &Clocks) -> SYSTCounter<FREQ>;

    /// Creates timer with precision of 1 μs (1 MHz sampling)
    fn counter_us(self, clocks: &Clocks) -> SYSTCounterUs {
        self.counter::<1_000_000>(clocks)
    }

    /// Blocking [Delay] with custom precision
    fn delay(self, clocks: &Clocks) -> SYSTDelay;
}

impl SYSTimerExt for SYST {
    fn counter_hz(self, clocks: &Clocks) -> SYSTCounterHz {
        Timer::syst(self, clocks).counter_hz()
    }
    fn counter<const FREQ: u32>(self, clocks: &Clocks) -> SYSTCounter<FREQ> {
        Timer::syst(self, clocks).counter()
    }
    fn delay(self, clocks: &Clocks) -> SYSTDelay {
        Timer::syst_external(self, clocks).delay()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Timer is disabled
    Disabled,
    WrongAutoReload,
}

pub trait Instance:
    crate::Sealed + rcc::Enable + rcc::Reset + rcc::BusTimerClock + General
{
}

impl<TIM> Timer<TIM>
where
    TIM: Instance,
{
    /// Initialize timer
    pub fn new(tim: TIM, clocks: &Clocks) -> Self {
        unsafe {
            //NOTE(unsafe) this reference will only be used for atomic writes with no side effects
            let rcc = &(*pac::Rcc::PTR);
            // Enable and reset the timer peripheral
            TIM::enable(rcc);
            TIM::reset(rcc);
        }

        Self {
            clk: TIM::timer_clock(clocks),
            tim,
        }
    }

    pub fn configure(&mut self, clocks: &Clocks) {
        self.clk = TIM::timer_clock(clocks);
    }

    pub fn counter_hz(self) -> CounterHz<TIM> {
        CounterHz(self)
    }

    pub fn release(self) -> TIM {
        self.tim
    }

    pub fn listen(&mut self, event: impl Into<BitFlags<Event>>) {
        self.tim.listen_event(None, Some(event.into()));
    }

    pub fn listen_only(&mut self, event: impl Into<BitFlags<Event>>) {
        self.tim
            .listen_event(Some(BitFlags::ALL), Some(event.into()));
    }

    pub fn unlisten(&mut self, event: impl Into<BitFlags<Event>>) {
        self.tim.listen_event(Some(event.into()), None);
    }

    pub fn clear_flags(&mut self, event: impl Into<BitFlags<Flag>>) {
        self.tim.clear_interrupt_flag(event.into());
    }

    pub fn flags(&self) -> BitFlags<Flag> {
        self.tim.get_interrupt_flag()
    }
}

pub trait TimerExt: Sized {
    /// Non-blocking [Counter] with custom fixed precision
    fn counter<const FREQ: u32>(self, clocks: &Clocks) -> Counter<Self, FREQ>;
    /// Non-blocking [Counter] with fixed precision of 1 ms (1 kHz sampling)
    ///
    /// Can wait from 2 ms to 65 sec for 16-bit timer and from 2 ms to 49 days for 32-bit timer.
    ///
    /// NOTE: don't use this if your system frequency more than 65 MHz
    fn counter_ms(self, clocks: &Clocks) -> CounterMs<Self> {
        self.counter::<1_000>(clocks)
    }
    /// Non-blocking [Counter] with fixed precision of 1 μs (1 MHz sampling)
    ///
    /// Can wait from 2 μs to 65 ms for 16-bit timer and from 2 μs to 71 min for 32-bit timer.
    fn counter_us(self, clocks: &Clocks) -> CounterUs<Self> {
        self.counter::<1_000_000>(clocks)
    }
    /// Non-blocking [Counter] with dynamic precision which uses `Hertz` as Duration units
    fn counter_hz(self, clocks: &Clocks) -> CounterHz<Self>;

    /// Blocking [Delay] with custom fixed precision
    fn delay<const FREQ: u32>(self, clocks: &Clocks) -> Delay<Self, FREQ>;
    /// Blocking [Delay] with fixed precision of 1 ms (1 kHz sampling)
    ///
    /// Can wait from 2 ms to 49 days.
    ///
    /// NOTE: don't use this if your system frequency more than 65 MHz
    fn delay_ms(self, clocks: &Clocks) -> DelayMs<Self> {
        self.delay::<1_000>(clocks)
    }
    /// Blocking [Delay] with fixed precision of 1 μs (1 MHz sampling)
    ///
    /// Can wait from 2 μs to 71 min.
    fn delay_us(self, clocks: &Clocks) -> DelayUs<Self> {
        self.delay::<1_000_000>(clocks)
    }
}

impl<TIM: Instance> TimerExt for TIM {
    fn counter<const FREQ: u32>(self, clocks: &Clocks) -> Counter<Self, FREQ> {
        FixedTimer::new(self, clocks).counter()
    }
    fn counter_hz(self, clocks: &Clocks) -> CounterHz<Self> {
        Timer::new(self, clocks).counter_hz()
    }
    fn delay<const FREQ: u32>(self, clocks: &Clocks) -> Delay<Self, FREQ> {
        FixedTimer::new(self, clocks).delay()
    }
}

/// Trigger output source
pub enum TriggerSource {
    /// Timer reset - UG as trigger output
    Reset,
    /// Timer enable - CNT_EN as trigger output
    Enable = 0b001,
    /// Update event - Update event as trigger output
    Update = 0b010,
    /// Compare Pulse - Positive pulse if CC1IF is setted
    ComparePulse = 0b011,
    /// Compare1 - OC1REFC as trigger output
    Compare1 = 0b100,
    /// Compare2 - OC2REFC as trigger output
    Compare2 = 0b101,
    /// Compare3 - OC3REFC as trigger output
    Compare3 = 0b110,
    /// Compare4 - OC4REFC as trigger output
    Compare4 = 0b111,
}

impl Timer<pac::Tim2> {
    pub fn set_trigger_source(&mut self, trigger_source: TriggerSource) {
        self.tim
            .cr2()
            .modify(|_, w| unsafe { w.mms2().bits(trigger_source as u8) });
    }
}

/// Timer wrapper for fixed precision timers.
///
/// Uses `fugit::TimerDurationU32` for most of operations
pub struct FixedTimer<TIM, const FREQ: u32> {
    pub(crate) tim: TIM,
}

/// `FixedTimer` with precision of 1 μs (1 MHz sampling)
pub type FixedTimerUs<TIM> = FixedTimer<TIM, 1_000_000>;

/// `FTimer` with precision of 1 ms (1 kHz sampling)
///
/// NOTE: don't use this if your system frequency more than 65 MHz
pub type FixedTimerMs<TIM> = FixedTimer<TIM, 1_000>;

impl<TIM: Instance, const FREQ: u32> FixedTimer<TIM, FREQ> {
    /// Initialize timer
    pub fn new(tim: TIM, clocks: &Clocks) -> Self {
        unsafe {
            // Enable and reset the timer peripheral
            TIM::enable_unchecked();
            TIM::reset_unchecked();
        }

        let mut t = Self { tim };
        t.configure(clocks);
        t
    }

    /// Calculate prescaler depending on `Clocks` state
    pub fn configure(&mut self, clocks: &Clocks) {
        let clk = TIM::timer_clock(clocks);
        assert!(clk.raw() % FREQ == 0);
        let psc = clk.raw() / FREQ;
        self.tim.set_prescaler(u16::try_from(psc - 1).unwrap());
    }

    /// Creates `Counter`
    pub fn counter(self) -> Counter<TIM, FREQ> {
        Counter(self)
    }

    /// Creates `Delay`
    pub fn delay(self) -> Delay<TIM, FREQ> {
        Delay(self)
    }

    /// Releases the TIM peripheral
    pub fn release(self) -> TIM {
        self.tim
    }

    pub fn listen(&mut self, event: impl Into<BitFlags<Event>>) {
        self.tim.listen_event(None, Some(event.into()));
    }

    pub fn listen_only(&mut self, event: impl Into<BitFlags<Event>>) {
        self.tim
            .listen_event(Some(BitFlags::ALL), Some(event.into()));
    }

    pub fn unlisten(&mut self, event: impl Into<BitFlags<Event>>) {
        self.tim.listen_event(Some(event.into()), None);
    }

    pub fn clear_flags(&mut self, event: impl Into<BitFlags<Flag>>) {
        self.tim.clear_interrupt_flag(event.into());
    }

    pub fn flags(&self) -> BitFlags<Flag> {
        self.tim.get_interrupt_flag()
    }
}

/// TIM interrupt events
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Event {
    /// Update interrupt enable
    Update = 1 << 0,
    /// Capture/Compare 1 interrupt enable
    C1 = 1 << 1,
    /// Capture/Compare 2 interrupt enable
    C2 = 1 << 2,
    /// Capture/Compare 3 interrupt enable
    C3 = 1 << 3,
    /// Capture/Compare 4 interrupt enable
    C4 = 1 << 4,
    /// COM interrupt enable
    COM = 1 << 5,
    /// Trigger interrupt enable
    Trigger = 1 << 6,
    /// Break interrupt enable
    Break = 1 << 7,
}

// TIM status flags
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flag {
    /// Update interrupt flag
    Update = 1 << 0,
    /// Capture/Compare 1 interrupt flag
    C1 = 1 << 1,
    /// Capture/Compare 2 interrupt flag
    C2 = 1 << 2,
    /// Capture/Compare 3 interrupt flag
    C3 = 1 << 3,
    /// Capture/Compare 4 interrupt flag
    C4 = 1 << 4,
    /// COM interrupt flag
    COM = 1 << 5,
    /// Trigger interrupt flag
    Trigger = 1 << 6,
    /// Break interrupt flag
    Break = 1 << 7,
    /// Capture/Compare 1 overcapture flag
    C1Overcapture = 1 << 9,
    /// Capture/Compare 2 overcapture flag
    C2Overcapture = 1 << 10,
    /// Capture/Compare 3 overcapture flag
    C3Overcapture = 1 << 11,
    /// Capture/Compare 4 overcapture flag
    C4Overcapture = 1 << 12,
}

macro_rules! tim {
    ($TIMX:ident: [$TimerX:ident, $bits:ty, $(dmar: $memsize:ty,)?]) => {
        impl Instance for pac::$TIMX {}

        impl General for pac::$TIMX {
            type Width = $bits;

            #[inline(always)]
            fn max_auto_reload() -> u32 {
                <Self::Width>::MAX as u32
            }

            #[inline(always)]
            unsafe fn set_auto_reload_unchecked(&mut self, arr: u32) {
                self.arr().write(|w| w.bits(arr))
            }

            #[inline(always)]
            fn set_auto_reload(&mut self, arr: u32) -> Result<(), Error> {
                // Note: Make it impossible to set the ARR value to 0, since this
                // would cause an infinite loop.
                if arr > 0 && arr <= Self::max_auto_reload() {
                    Ok(unsafe { self.set_auto_reload_unchecked(arr) })
                } else {
                    Err(Error::WrongAutoReload)
                }
            }

            #[inline(always)]
            fn read_auto_reload() -> u32 {
                let tim = unsafe { &*<pac::$TIMX>::ptr() };
                tim.arr().read().bits()
            }

            #[inline(always)]
            fn enable_preload(&mut self, b: bool) {
                self.cr1().modify(|_, w| w.arpe().bit(b));
            }

            #[inline(always)]
            fn enable_counter(&mut self, b: bool) {
                self.cr1().modify(|_, w| w.cen().bit(b));
            }

            #[inline(always)]
            fn is_counter_enabled(&self) -> bool {
                self.cr1().read().cen().bit()
            }

            #[inline(always)]
            fn reset_counter(&mut self) {
                self.cnt().reset();
            }

            #[inline(always)]
            fn set_prescaler(&mut self, psc: u16) {
                self.psc().write(|w| unsafe { w.psc().bits(psc) });
            }

            #[inline(always)]
            fn read_prescaler(&self) -> u16 {
                self.psc().read().psc().bits()
            }

            #[inline(always)]
            fn trigger_update(&mut self) {
                self.cr1().modify(|_, w| w.urs().set_bit());
                self.egr().write(|w| w.ug().set_bit());
                self.cr1().modify(|_, w| w.urs().clear_bit());
            }

            #[inline(always)]
            fn listen_event(
                &mut self,
                disable: Option<BitFlags<Event>>,
                enable: Option<BitFlags<Event>>,
            ) {
                self.dier().modify(|r, w| unsafe {
                    w.bits({
                        let mut bits = r.bits();
                        if let Some(d) = disable {
                            bits &= !(d.bits() as u32);
                        }
                        if let Some(e) = enable {
                            bits |= e.bits() as u32;
                        }
                        bits
                    })
                });
            }
            #[inline(always)]
            fn clear_interrupt_flag(&mut self, event: BitFlags<Flag>) {
                self.sr()
                    .write(|w| unsafe { w.bits(0xffff & !(event.bits() as u32)) });
            }

            #[inline(always)]
            fn get_interrupt_flag(&self) -> BitFlags<Flag> {
                BitFlags::from_bits_truncate(self.sr().read().bits())
            }

            #[inline(always)]
            fn read_count(&self) -> Self::Width {
                self.cnt().read().bits() as Self::Width
            }

            #[inline(always)]
            fn write_count(&mut self, value: Self::Width) {
                // TODO: TIM2 and TIM5 are 32 bit
                self.cnt().write(|w| unsafe { w.cnt().bits(value as _) });
            }

            #[inline(always)]
            fn start_one_pulse(&mut self) {
                self.cr1()
                    .modify(|_, w| unsafe { w.bits(1 << 3) }.cen().set_bit());
            }

            #[inline(always)]
            fn start_free(&mut self, update: bool) {
                self.cr1()
                    .modify(|_, w| w.cen().set_bit().udis().bit(!update));
            }

            #[inline(always)]
            fn reset_control_register1(&mut self) {
                self.cr1().reset();
            }
        }

        $(dmar!($TIMX, $memsize);)?
    };
}

/// Wrapper type that indicates which register of the contained timer to use for DMA.
pub struct DMAR<T>(T);

macro_rules! dmar {
    ($TIMX:ident, $memsize:ty) => {
        unsafe impl dma::PeriAddress for DMAR<pac::$TIMX> {
            type MemSize = $memsize;

            fn address(&self) -> u32 {
                self.0.dmar().as_ptr() as u32
            }
        }
    };
}

tim!(Tim1: [Timer1, u16, dmar: u32,]);

tim!(Tim2: [Timer2, u32, dmar: u16,]);

tim!(Tim3: [Timer3, u16, dmar: u16,]);

tim!(Tim4: [Timer4, u16, dmar: u16,]);

#[cfg(feature = "tim5")]
tim!(Tim5: [Timer5, u32, dmar: u16,]);

tim!(Tim6: [Timer6, u16,]);

tim!(Tim7: [Timer7, u16,]);

tim!(Tim8: [Timer8, u16, dmar: u32,]);

tim!(Tim15: [Timer15, u16, dmar: u16,]);

tim!(Tim16: [Timer16, u16, dmar: u16,]);

tim!(Tim17: [Timer17, u16, dmar: u16,]);

#[cfg(feature = "tim20")]
tim!(Tim20: [Timer20, u16, dmar: u32,]);

#[inline(always)]
pub(crate) const fn compute_arr_presc(freq: u32, clock: u32) -> (u16, u32) {
    let ticks = clock / freq;
    let psc = (ticks - 1) / (1 << 16);
    let arr = ticks / (psc + 1) - 1;
    (psc as u16, arr)
}

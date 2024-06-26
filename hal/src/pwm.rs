//! pwm
use core::marker::PhantomData;
use core::mem::MaybeUninit;

use fugit::{ExtU32, HertzU32 as Hertz, NanosDurationU32};

use crate::pac;

use crate::rcc::clock::Clocks;
use crate::rcc::{BusTimerClock, Enable, Reset};

mod pins;

pub trait Pins<TIM, CHANNEL, COMP> {
    type Channel;
}

pub trait NPins<TIM, CHANNEL> {}

pub trait FaultPins<TIM> {
    const INPUT: BreakInput;
}

pub struct C1;

pub struct C2;

pub struct C3;

pub struct C4;

pub struct ComplementaryImpossible;

pub struct ComplementaryDisabled;

pub struct ComplementaryEnabled;

pub enum Polarity {
    ActiveHigh,
    ActiveLow,
}

#[derive(PartialEq)]
pub enum BreakInput {
    BreakIn,
    BreakIn2,
}

enum CountSettings<WIDTH> {
    Frequency(Hertz),
    Explicit { period: WIDTH, prescaler: u16 },
}

pub struct ActiveHigh;

pub struct ActiveLow;

#[derive(Copy, Clone, Debug)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

/// Marker struct indicating that a PwmControl is in charge of fault monitoring
pub struct FaultEnabled;

/// Marker struct indicating that a PwmControl does not handle fault monitoring
pub struct FaultDisabled;

pub trait PwmExt: Sized {
    /// The requested frequency will be rounded to the nearest achievable frequency; the actual frequency may be higher or lower than requested.
    fn pwm<PINS, T, U, V>(self, _pins: PINS, frequency: T, clocks: &Clocks) -> PINS::Channel
    where
        PINS: Pins<Self, U, V>,
        T: Into<Hertz>;
}

macro_rules! tim_hal {
    ($TIMX:ty, $timX:ident $(, BDTR: $bdtr:ident)*) => {
        impl PwmExt for $TIMX {
            fn pwm<PINS, T, U, V>(self, _pins: PINS, freq: T, clocks: &Clocks) -> PINS::Channel
            where
                PINS: Pins<Self, U, V>,
                T: Into<Hertz>,
            {
                $timX(self, _pins, freq, clocks)
            }
        }

        pub fn $timX<PINS, T, U, V>(tim: $TIMX, _pins: PINS, freq: T, clocks: &Clocks) -> PINS::Channel
        where
            PINS: Pins<$TIMX, U, V>,
            T: Into<Hertz>,
        {
            unsafe {
                let rcc = &(*pac::Rcc::PTR);
                <$TIMX>::enable(rcc);
                <$TIMX>::reset(rcc);
            }

            let freq = freq.into();
            let clk = <$TIMX>::timer_clock(clocks);

            let (period, psc) = match 32 {
                16 => calculate_frequency_16bit(clk, freq, Alignment::Left),
                _ => calculate_frequency_32bit(clk, freq, Alignment::Left),
            };

            // Write prescale
            tim.psc()
                .write(|w| unsafe { w.psc().bits(psc as u16) });

            // Write period
            tim.arr().write(|w| unsafe { w.arr().bits(period.into()) });

            // BDTR: Advanced-control timers
            //tim.bdtr().write(|w| w.moe().set_bit());
            $(
                // Set CCxP = OCxREF / CCxNP = !OCxREF
                // Refer to RM0433 Rev 6 - Table 324.
                tim.$bdtr().write(|w| w.moe().set_bit()
                );
            )*

            tim.cr1().write(|w| w.cen().set_bit());

            unsafe { MaybeUninit::<PINS::Channel>::uninit().assume_init() }
        }
    };
}

#[cfg(feature = "tim1")]
tim_hal!(pac::Tim1, tim1, BDTR: bdtr);

#[cfg(feature = "tim2")]
tim_hal!(pac::Tim2, tim2);

#[cfg(feature = "tim3")]
tim_hal!(pac::Tim3, tim3);

#[cfg(feature = "tim4")]
tim_hal!(pac::Tim4, tim4);

#[cfg(feature = "tim5")]
tim_hal!(pac::Tim5, tim5);

#[cfg(feature = "tim8")]
tim_hal!(pac::Tim8, tim8);

#[cfg(feature = "tim15")]
tim_hal!(pac::Tim15, tim15);

#[cfg(feature = "tim16")]
tim_hal!(pac::Tim16, tim16);

#[cfg(feature = "tim17")]
tim_hal!(pac::Tim17, tim17);

#[cfg(feature = "tim20")]
tim_hal!(pac::Tim20, tim20);

pub trait PwmAdvExt<WIDTH>: Sized {
    fn pwm_advanced<PINS, CHANNEL, COMP>(
        self,
        _pins: PINS,
        clocks: &Clocks,
    ) -> PwmBuilder<Self, PINS, CHANNEL, FaultDisabled, COMP, WIDTH>
    where
        PINS: Pins<Self, CHANNEL, COMP>;
}

/// PwmBuilder is used to configure advanced PWM features
pub struct PwmBuilder<TIM, PINS, CHANNEL, FAULT, COMP, WIDTH> {
    _tim: PhantomData<TIM>,
    _pins: PhantomData<PINS>,
    _channel: PhantomData<CHANNEL>,
    _fault: PhantomData<FAULT>,
    _comp: PhantomData<COMP>,
    alignment: Alignment,
    base_freq: Hertz,
    count: CountSettings<WIDTH>,
    bkin_enabled: bool, // If the FAULT type parameter is FaultEnabled, either bkin or bkin2 must be enabled
    bkin2_enabled: bool,
    fault_polarity: Polarity,
    deadtime: NanosDurationU32,
}

/// Allows a PwmControl to monitor and control faults (break inputs) of a timer's PWM channels
pub trait FaultMonitor {
    /// Returns true if a fault is preventing PWM output
    fn is_fault_active(&self) -> bool;

    /// Enables PWM output, clearing fault state and immediately resuming PWM; if the break pin is still active, this can't clear the fault.
    fn clear_fault(&mut self);

    /// Disables PWM output, setting fault state; this can be used to stop all PWM from a timer in software detected faults
    fn set_fault(&mut self);
}

/// Exposes timer wide advanced features, such as [FaultMonitor](trait.FaultMonitor.html)
/// or future features like trigger outputs for synchronization with ADCs and other peripherals
pub struct PwmControl<TIM, FAULT> {
    _tim: PhantomData<TIM>,
    _fault: PhantomData<FAULT>,
}

macro_rules! tim_adv_hal {
    ($TIMX:ty, $timX:ident, $typ:ty, $bits:expr $(, DIR: $cms:ident)* $(, BDTR: $bdtr:ident, $bkinp_setting:ident $(, $bk2inp_setting:ident)*)*) => {
        impl PwmAdvExt<$typ> for $TIMX {
            fn pwm_advanced<PINS, CHANNEL, COMP>(
                self,
                _pins: PINS,
                clocks: &Clocks,
            ) -> PwmBuilder<Self, PINS, CHANNEL, FaultDisabled, COMP, $typ>
            where
                PINS: Pins<Self, CHANNEL, COMP>,
            {
                unsafe {
                    let rcc = &(*pac::Rcc::PTR);
                    <$TIMX>::enable(rcc);
                    <$TIMX>::reset(rcc);
                }

                let clk = <$TIMX>::timer_clock(clocks);

                PwmBuilder {
                    _tim: PhantomData,
                    _pins: PhantomData,
                    _channel: PhantomData,
                    _fault: PhantomData,
                    _comp: PhantomData,
                    alignment: Alignment::Left,
                    base_freq: clk,
                    count: CountSettings::Explicit {
                        period: 65535,
                        prescaler: 0,
                    },
                    bkin_enabled: false,
                    bkin2_enabled: false,
                    fault_polarity: Polarity::ActiveHigh,
                    deadtime: 0.nanos(),
                }
            }
        }

        impl<PINS, CHANNEL, FAULT, COMP>
                PwmBuilder<$TIMX, PINS, CHANNEL, FAULT, COMP, $typ>
            where
                PINS: Pins<$TIMX, CHANNEL, COMP>,
            {
                pub fn finalize(self) -> (PwmControl<$TIMX, FAULT>, PINS::Channel) {
                    let tim = unsafe { &*<$TIMX>::PTR };

                    let (period, prescaler) = match self.count {
                        CountSettings::Explicit { period, prescaler } => (period as u32, prescaler),
                        CountSettings::Frequency( freq ) => {
                            match $bits {
                                16 => calculate_frequency_16bit(self.base_freq, freq, self.alignment),
                                _ => calculate_frequency_32bit(self.base_freq, freq, self.alignment),
                            }
                        },
                    };

                    // Write prescaler
                    tim.psc().write(|w| unsafe { w.psc().bits(prescaler as u16) });

                    // Write period
                    tim.arr().write(|w| unsafe { w.arr().bits(period.into()) });

                    $(
                        let (dtg, ckd) = calculate_deadtime(self.base_freq, self.deadtime);

                        match ckd {
                            1 => tim.cr1().modify(|_, w| unsafe { w.ckd().bits(0) }),
                            2 => tim.cr1().modify(|_, w| unsafe { w.ckd().bits(1) }),
                            4 => tim.cr1().modify(|_, w| unsafe { w.ckd().bits(2) }),
                            _ => panic!("Should be unreachable, invalid deadtime prescaler"),
                        }

                        let bkp = match self.fault_polarity {
                            Polarity::ActiveLow => false,
                            Polarity::ActiveHigh => true,
                        };

                        if self.bkin_enabled {
                            // BDTR:
                            //  BKF = 1 -> break pin filtering of 2 cycles of CK_INT (peripheral source clock)
                            //  AOE = 0 -> after a fault, master output enable MOE can only be set by software, not automatically
                            //  BKE = 1 -> break is enabled
                            //  BKP = 0 for active low, 1 for active high
                            // Safety: bkf is set to a constant value (1) that is a valid value for the field per the reference manual
                            unsafe { tim.$bdtr().write(|w| w.dtg().bits(dtg).bkf().bits(1).aoe().clear_bit().bke().set_bit().bkp().bit(bkp).moe().set_bit()); }

                            // AF1:
                            //  BKINE = 1 -> break input enabled
                            //  BKINP should make input active high (BDTR BKP will set polarity), bit value varies timer to timer
                            tim.af1().write(|w| w.bkine().set_bit().bkinp().$bkinp_setting());
                        }
                        $(
                            // Not all timers that have break inputs have break2 inputs
                            else if self.bkin2_enabled {
                                // BDTR:
                                //  BK2F = 1 -> break pin filtering of 2 cycles of CK_INT (peripheral source clock)
                                //  AOE = 0 -> after a fault, master output enable MOE can only be set by software, not automatically
                                //  BK2E = 1 -> break is enabled
                                //  BK2P = 0 for active low, 1 for active high
                                // Safety: bkf is set to a constant value (1) that is a valid value for the field per the reference manual
                                unsafe { tim.$bdtr().write(|w| w.dtg().bits(dtg).bk2f().bits(1).aoe().clear_bit().bk2e().set_bit().bk2p().bit(bkp).moe().set_bit()); }

                                // AF2:
                                //  BKINE = 1 -> break input enabled
                                //  BKINP should make input active high (BDTR BKP will set polarity), bit value varies timer to timer
                                tim.af2().write(|w| w.bkine().set_bit().bk2inp().$bk2inp_setting());
                            }
                        )*
                        else {
                            // Safety: the DTG field of BDTR allows any 8-bit deadtime value and the dtg variable is u8
                            unsafe {
                                tim.$bdtr().write(|w| w.dtg().bits(dtg).aoe().clear_bit().moe().set_bit());
                            }
                        }

                        // BDTR: Advanced-control timers
                        // Set CCxP = OCxREF / CCxNP = !OCxREF
                        // Refer to RM0433 Rev 6 - Table 324.
                        tim.$bdtr().modify(|_, w| w.moe().set_bit());
                    )*


                    $(
                        match self.alignment {
                            Alignment::Left => { },
                            Alignment::Right => { tim.cr1().modify(|_, w| w.dir().set_bit()); }, // Downcounter
                            Alignment::Center => { tim.cr1().modify(|_, w| unsafe { w.$cms().bits(3) }); } // Center-aligned mode 3
                        }
                    )*

                    tim.cr1().modify(|_, w| w.cen().set_bit());

                    unsafe {
                        MaybeUninit::<(PwmControl<$TIMX, FAULT>, PINS::Channel)>::uninit()
                            .assume_init()
                    }
                }

                /// Set the PWM frequency; will overwrite the previous prescaler and period
                /// The requested frequency will be rounded to the nearest achievable frequency; the actual frequency may be higher or lower than requested.
                pub fn frequency<T: Into<Hertz>>(mut self, freq: T) -> Self {
                    self.count = CountSettings::Frequency( freq.into() );

                    self
                }

                /// Set the prescaler; PWM count runs at base_frequency/(prescaler+1)
                pub fn prescaler(mut self, prescaler: u16) -> Self {
                    let period = match self.count {
                        CountSettings::Frequency(_) => 65535,
                        CountSettings::Explicit { period, prescaler: _ } => period,
                    };

                    self.count = CountSettings::Explicit { period, prescaler };

                    self
                }

                /// Set the period; PWM count runs from 0 to period, repeating every (period+1) counts
                pub fn period(mut self, period: $typ) -> Self {
                    let prescaler = match self.count {
                        CountSettings::Frequency(_) => 0,
                        CountSettings::Explicit { period: _, prescaler } => prescaler,
                    };

                    self.count = CountSettings::Explicit { period, prescaler };

                    self
                }


                // Timers with complementary and deadtime and faults
                $(
                    /// Set the deadtime for complementary PWM channels of this timer
                    pub fn with_deadtime<T: Into<NanosDurationU32>>(mut self, deadtime: T) -> Self {
                        // $bdtr is an Ident that only exists for timers with deadtime, so we can use it as a variable name to
                        // only implement this method for timers that support deadtime.
                        let $bdtr = deadtime.into();

                        self.deadtime = $bdtr;

                        self
                    }
                )*

                pub fn left_aligned( mut self ) -> Self {
                    self.alignment = Alignment::Left;

                    self
                }

                // Timers with advanced counting options, including center aligned and right aligned PWM
                $(
                    pub fn center_aligned(mut self) -> Self {
                        // $cms is an Ident that only exists for timers with center/right aligned PWM, so we can use it as a variable name to
                        // only implement this method for timers that support center/right aligned PWM.
                        let $cms = Alignment::Center;

                        self.alignment = $cms;

                        self
                    }

                    pub fn right_aligned(mut self) -> Self {
                        self.alignment = Alignment::Right;

                        self
                    }
                )*
            }

            // Timers with break/fault, dead time, and complimentary capabilities
            $(
                impl<PINS, CHANNEL, COMP> PwmBuilder<$TIMX, PINS, CHANNEL, FaultDisabled, COMP, $typ> {
                    /// Configure a break pin that will disable PWM when activated (active level based on polarity argument)
                    /// Note: not all timers have fault inputs; `FaultPins<TIM>` is only implemented for valid pins/timers.
                    pub fn with_break_pin<P: FaultPins<$TIMX>>(self, _pin: P, polarity: Polarity) -> PwmBuilder<$TIMX, PINS, CHANNEL, FaultEnabled, COMP, $typ> {
                        PwmBuilder {
                            _tim: PhantomData,
                            _pins: PhantomData,
                            _channel: PhantomData,
                            _fault: PhantomData,
                            _comp: PhantomData,
                            alignment: self.alignment,
                            base_freq: self.base_freq,
                            count: self.count,
                            bkin_enabled: self.bkin_enabled || P::INPUT == BreakInput::BreakIn,
                            bkin2_enabled: self.bkin2_enabled || P::INPUT == BreakInput::BreakIn2,
                            fault_polarity: polarity,
                            deadtime: self.deadtime,
                        }
                    }
                }

                impl FaultMonitor for PwmControl<$TIMX, FaultEnabled> {
                    fn is_fault_active(&self) -> bool {
                        let tim = unsafe { &*<$TIMX>::PTR };

                        !tim.$bdtr().read().moe().bit()
                    }

                    fn clear_fault(&mut self) {
                        let tim = unsafe { &*<$TIMX>::PTR };

                        tim.$bdtr().modify(|_, w| w.moe().set_bit());
                    }

                    fn set_fault(&mut self) {
                        let tim = unsafe { &*<$TIMX>::PTR };

                        tim.$bdtr().modify(|_, w| w.moe().clear_bit());
                    }
                }
            )*
    };
}

#[cfg(feature = "tim1")]
tim_adv_hal!(pac::Tim1, tim1, u16, 16, DIR: cms, BDTR: bdtr, clear_bit, clear_bit);

#[cfg(feature = "tim2")]
tim_adv_hal!(pac::Tim2, tim2, u32, 32, DIR: cms);

#[cfg(feature = "tim3")]
tim_adv_hal!(pac::Tim3, tim3, u16, 16, DIR: cms);

#[cfg(feature = "tim4")]
tim_adv_hal!(pac::Tim4, tim4, u16, 16, DIR: cms);

#[cfg(feature = "tim5")]
tim_adv_hal!(pac::Tim5, tim5, u32, 32, DIR: cms);

#[cfg(feature = "tim8")]
tim_adv_hal!(pac::Tim8, tim1, u16, 16, DIR: cms, BDTR: bdtr, clear_bit, clear_bit);

#[cfg(feature = "tim15")]
tim_adv_hal!(pac::Tim15, tim15, u16, 16, BDTR: bdtr, set_bit);

#[cfg(feature = "tim16")]
tim_adv_hal!(pac::Tim16, tim16, u16, 16, BDTR: bdtr, set_bit);

#[cfg(feature = "tim17")]
tim_adv_hal!(pac::Tim17, tim17, u16, 16, BDTR: bdtr, set_bit);

#[cfg(feature = "tim20")]
tim_adv_hal!(pac::Tim20, tim20, u16, 16, BDTR: bdtr, set_bit);

/// Pwm represents one PWM channel; it is created by calling TIM?.pwm(...) and lets you control the channel through the PwmPin trait
pub struct Pwm<TIM, CHANNEL, COMP, POL, NPOL> {
    _tim: PhantomData<TIM>,
    _channel: PhantomData<CHANNEL>,
    _complementary: PhantomData<COMP>,
    _polarity: PhantomData<POL>,
    _npolarity: PhantomData<NPOL>,
}

pub trait PwmPinEnable {
    fn ccer_enable(&mut self);
    fn ccer_disable(&mut self);
}

// Implement PwmPin for timer channels
macro_rules! tim_pin_hal {
    // Standard pins (no complementary functionality)
    ($($TIMX:ty:
       ($CH:ty, $ccxe:ident, $ccxp:ident, $ccmrx_output:ident, $ocxpe:ident, $ocxm:ident, $ocxm_3:ident,
        $ccrx:ident, $typ:ident $(,$ccxne:ident, $ccxnp:ident)*),)+
    ) => {
        $(
            impl<COMP, POL, NPOL> Pwm<$TIMX, $CH, COMP, POL, NPOL>
                where Pwm<$TIMX, $CH, COMP, POL, NPOL>: PwmPinEnable {

                // You may not access self in the following methods!
                // See unsafe above

                pub fn disable(&mut self) {
                    self.ccer_disable();
                }

                pub fn enable(&mut self) {
                    let tim = unsafe { &*<$TIMX>::PTR };

                    tim.$ccmrx_output().modify(|_, w|
                        unsafe {
                            w.$ocxpe()
                            .set_bit() // Enable preload
                            .$ocxm()
                            .bits(0b110) // PWM mode 1
                            .$ocxm_3()
                            .clear_bit()
                        }

                    );

                    self.ccer_enable();
                }

                pub fn get_duty(&self) -> $typ {
                    let tim = unsafe { &*<$TIMX>::PTR };

                    // Even though the field is 20 bits long for 16-bit counters, only 16 bits are
                    // valid, so we convert to the appropriate type.
                    tim.$ccrx().read().$ccrx().bits() as $typ
                }

                pub fn get_max_duty(&self) -> $typ {
                    let tim = unsafe { &*<$TIMX>::PTR };

                    // Even though the field is 20 bits long for 16-bit counters, only 16 bits are
                    // valid, so we convert to the appropriate type.
                    let arr = tim.arr().read().arr().bits() as $typ;

                    // One PWM cycle is ARR+1 counts long
                    // Valid PWM duty cycles are 0 to ARR+1
                    // However, if ARR is 65535 on a 16-bit timer, we can't add 1
                    // In that case, 100% duty cycle is not possible, only 65535/65536
                    if arr == $typ::MAX {
                        arr
                    }
                    else {
                        arr + 1
                    }
                }

                pub fn set_duty(&mut self, duty: $typ) {
                    let tim = unsafe { &*<$TIMX>::PTR };

                    tim.$ccrx().write(|w| unsafe { w.$ccrx().bits(duty.into()) });
                }
            }

            // Enable implementation for ComplementaryImpossible
            impl<POL, NPOL> PwmPinEnable for Pwm<$TIMX, $CH, ComplementaryImpossible, POL, NPOL> {
                fn ccer_enable(&mut self) {
                    let tim = unsafe { &*<$TIMX>::PTR };

                    tim.ccer().modify(|_, w| w.$ccxe().set_bit());
                }
                fn ccer_disable(&mut self) {
                    let tim = unsafe { &*<$TIMX>::PTR };

                    tim.ccer().modify(|_, w| w.$ccxe().clear_bit());
                }
            }

            impl<COMP, NPOL> Pwm<$TIMX, $CH, COMP, ActiveHigh, NPOL> {
                pub fn into_active_low(self) -> Pwm<$TIMX, $CH, COMP, ActiveLow, NPOL> {
                    let tim = unsafe { &*<$TIMX>::PTR };

                    tim.ccer().modify(|_, w| w.$ccxp().set_bit());

                    Pwm {
                        _channel: PhantomData,
                        _tim: PhantomData,
                        _complementary: PhantomData,
                        _polarity: PhantomData,
                        _npolarity: PhantomData,
                    }
                }
            }

            impl<COMP, NPOL> Pwm<$TIMX, $CH, COMP, ActiveLow, NPOL> {
                pub fn into_active_high(self) -> Pwm<$TIMX, $CH, COMP, ActiveHigh, NPOL> {
                    let tim = unsafe { &*<$TIMX>::PTR };

                    tim.ccer().modify(|_, w| w.$ccxp().clear_bit());

                    Pwm {
                        _channel: PhantomData,
                        _tim: PhantomData,
                        _complementary: PhantomData,
                        _polarity: PhantomData,
                        _npolarity: PhantomData,
                    }
                }
            }

            // Complementary channels
            $(
                // Enable implementation for ComplementaryDisabled
                impl<POL, NPOL> PwmPinEnable for Pwm<$TIMX, $CH, ComplementaryDisabled, POL, NPOL> {
                    fn ccer_enable(&mut self) {
                        let tim = unsafe { &*<$TIMX>::PTR };

                        tim.ccer().modify(|_, w| w.$ccxe().set_bit());
                    }
                    fn ccer_disable(&mut self) {
                        let tim = unsafe { &*<$TIMX>::PTR };

                        tim.ccer().modify(|_, w| w.$ccxe().clear_bit());
                    }
                }

                // Enable implementation for ComplementaryEnabled
                impl<POL, NPOL> PwmPinEnable for Pwm<$TIMX, $CH, ComplementaryEnabled, POL, NPOL> {
                    fn ccer_enable(&mut self) {
                        let tim = unsafe { &*<$TIMX>::PTR };

                        tim.ccer().modify(|_, w| w.$ccxe().set_bit().$ccxne().set_bit());
                    }
                    fn ccer_disable(&mut self) {
                        let tim = unsafe { &*<$TIMX>::PTR };

                        tim.ccer().modify(|_, w| w.$ccxe().clear_bit().$ccxne().clear_bit());
                    }
                }

                impl<POL, NPOL> Pwm<$TIMX, $CH, ComplementaryDisabled, POL, NPOL> {
                    pub fn into_complementary<NPIN>(self, _npin: NPIN) -> Pwm<$TIMX, $CH, ComplementaryEnabled, POL, NPOL>
                        where NPIN: NPins<$TIMX, $CH> {
                        // Make sure we aren't switching to complementary after we enable the channel
                        let tim = unsafe { &*<$TIMX>::PTR };

                        let enabled = tim.ccer().read().$ccxe().bit();

                        assert!(!enabled);

                        Pwm {
                            _channel: PhantomData,
                            _tim: PhantomData,
                            _complementary: PhantomData,
                            _polarity: PhantomData,
                            _npolarity: PhantomData,
                        }
                    }
                }

                impl<POL> Pwm<$TIMX, $CH, ComplementaryEnabled, POL, ActiveHigh> {
                    pub fn into_comp_active_low(self) -> Pwm<$TIMX, $CH, ComplementaryEnabled, POL, ActiveLow> {
                        let tim = unsafe { &*<$TIMX>::PTR };

                        tim.ccer().modify(|_, w| w.$ccxnp().set_bit());

                        Pwm {
                            _channel: PhantomData,
                            _tim: PhantomData,
                            _complementary: PhantomData,
                            _polarity: PhantomData,
                            _npolarity: PhantomData,
                        }
                    }
                }

                impl<POL> Pwm<$TIMX, $CH, ComplementaryEnabled, POL, ActiveLow> {
                    pub fn into_comp_active_high(self) -> Pwm<$TIMX, $CH, ComplementaryEnabled, POL, ActiveHigh> {
                        let tim = unsafe { &*<$TIMX>::PTR };

                        tim.ccer().modify(|_, w| w.$ccxnp().clear_bit());

                        Pwm {
                            _channel: PhantomData,
                            _tim: PhantomData,
                            _complementary: PhantomData,
                            _polarity: PhantomData,
                            _npolarity: PhantomData,
                        }
                    }
                }
            )*
        )+
    };
}

#[cfg(feature = "tim1")]
tim_pin_hal! {
    pac::Tim1: (C1, cc1e, cc1p, ccmr1_output, oc1pe, oc1m, oc1m_3, ccr1, u16, cc1ne, cc1np),
    pac::Tim1: (C2, cc2e, cc2p, ccmr1_output, oc2pe, oc2m, oc2m_3, ccr2, u16, cc2ne, cc2np),
    pac::Tim1: (C3, cc3e, cc3p, ccmr2_output, oc3pe, oc3m, oc3m_3, ccr3, u16, cc3ne, cc3np),
    pac::Tim1: (C4, cc4e, cc4p, ccmr2_output, oc4pe, oc4m, oc4m_3, ccr4, u16, cc4ne, cc4np),
}

#[cfg(feature = "tim2")]
tim_pin_hal! {
    pac::Tim2: (C1, cc1e, cc1p, ccmr1_output, oc1pe, oc1m, oc1m_3, ccr1, u32),
    pac::Tim2: (C2, cc2e, cc2p, ccmr1_output, oc2pe, oc2m, oc2m_3, ccr2, u32),
    pac::Tim2: (C3, cc3e, cc3p, ccmr2_output, oc3pe, oc3m, oc3m_3, ccr3, u32),
    pac::Tim2: (C4, cc4e, cc4p, ccmr2_output, oc4pe, oc4m, oc4m_3, ccr4, u32),
}

#[cfg(feature = "tim3")]
tim_pin_hal! {
    pac::Tim3: (C1, cc1e, cc1p, ccmr1_output, oc1pe, oc1m, oc1m_3, ccr1, u16),
    pac::Tim3: (C2, cc2e, cc2p, ccmr1_output, oc2pe, oc2m, oc2m_3, ccr2, u16),
    pac::Tim3: (C3, cc3e, cc3p, ccmr2_output, oc3pe, oc3m, oc3m_3, ccr3, u16),
    pac::Tim3: (C4, cc4e, cc4p, ccmr2_output, oc4pe, oc4m, oc4m_3, ccr4, u16),
}

#[cfg(feature = "tim4")]
tim_pin_hal! {
    pac::Tim4: (C1, cc1e, cc1p, ccmr1_output, oc1pe, oc1m, oc1m_3, ccr1, u16),
    pac::Tim4: (C2, cc2e, cc2p, ccmr1_output, oc2pe, oc2m, oc2m_3, ccr2, u16),
    pac::Tim4: (C3, cc3e, cc3p, ccmr2_output, oc3pe, oc3m, oc3m_3, ccr3, u16),
    pac::Tim4: (C4, cc4e, cc4p, ccmr2_output, oc4pe, oc4m, oc4m_3, ccr4, u16),
}

#[cfg(feature = "tim5")]
tim_pin_hal! {
    pac::Tim5: (C1, cc1e, cc1p, ccmr1_output, oc1pe, oc1m, oc1m_3, ccr1, u32),
    pac::Tim5: (C2, cc2e, cc2p, ccmr1_output, oc2pe, oc2m, oc1m_3, ccr2, u32),
    pac::Tim5: (C3, cc3e, cc3p, ccmr2_output, oc3pe, oc3m, oc1m_3, ccr3, u32),
    pac::Tim5: (C4, cc4e, cc4p, ccmr2_output, oc4pe, oc4m, oc1m_3, ccr4, u32),
}

#[cfg(feature = "tim8")]
tim_pin_hal! {
    pac::Tim8: (C1, cc1e, cc1p, ccmr1_output, oc1pe, oc1m, oc1m_3, ccr1, u16, cc1ne, cc1np),
    pac::Tim8: (C2, cc2e, cc2p, ccmr1_output, oc2pe, oc2m, oc2m_3, ccr2, u16, cc2ne, cc2np),
    pac::Tim8: (C3, cc3e, cc3p, ccmr2_output, oc3pe, oc3m, oc3m_3, ccr3, u16, cc3ne, cc3np),
    pac::Tim8: (C4, cc4e, cc4p, ccmr2_output, oc4pe, oc4m, oc4m_3, ccr4, u16, cc4ne, cc4np),
}

#[cfg(feature = "tim20")]
tim_pin_hal! {
    pac::Tim20: (C1, cc1e, cc1p, ccmr1_output, oc1pe, oc1m, oc1m_3, ccr1, u16, cc1ne, cc1np),
    pac::Tim20: (C2, cc2e, cc2p, ccmr1_output, oc2pe, oc2m, oc2m_3, ccr2, u16, cc2ne, cc2np),
    pac::Tim20: (C3, cc3e, cc3p, ccmr2_output, oc3pe, oc3m, oc3m_3, ccr3, u16, cc3ne, cc3np),
    pac::Tim20: (C4, cc4e, cc4p, ccmr2_output, oc4pe, oc4m, oc4m_3, ccr4, u16, cc4ne, cc4np),
}

// Period and prescaler calculator for 32-bit timers
// Returns (arr, psc)
fn calculate_frequency_32bit(base_freq: Hertz, freq: Hertz, alignment: Alignment) -> (u32, u16) {
    let divisor = if let Alignment::Center = alignment {
        freq * 2
    } else {
        freq
    };

    // Round to the nearest period
    let arr = (base_freq + (divisor / 2)) / divisor - 1;

    (arr, 0)
}

// Period and prescaler calculator for 16-bit timers
// Returns (arr, psc)
// Returns as (u32, u16) to be compatible but arr will always be a valid u16
fn calculate_frequency_16bit(base_freq: Hertz, freq: Hertz, alignment: Alignment) -> (u32, u16) {
    let ideal_period = calculate_frequency_32bit(base_freq, freq, alignment).0 + 1;

    // Division factor is (PSC + 1)
    let prescale = (ideal_period - 1) / (1 << 16);

    // This will always fit in a 16-bit value because u32::MAX / (1 << 16) fits in a 16 bit

    // Round to the nearest period
    let period = (ideal_period + (prescale >> 1)) / (prescale + 1) - 1;

    // It should be impossible to fail these asserts
    assert!(period <= 0xFFFF);
    assert!(prescale <= 0xFFFF);

    (period, prescale as u16)
}

// Deadtime calculator helper function
// Returns (BDTR.DTG, CR1.CKD)
fn calculate_deadtime(base_freq: Hertz, deadtime: NanosDurationU32) -> (u8, u8) {
    // tDTS is based on tCK_INT which is before the prescaler
    // It uses its own separate prescaler CR1.CKD

    // ticks = ns * GHz = ns * Hz / 1e9
    // Cortex-M7 has 32x32->64 multiply but no 64-bit divide
    // Divide by 100000 then 10000 by multiplying and shifting
    // This can't overflow because both values being multiplied are u32
    let deadtime_ticks = deadtime.ticks() as u64 * base_freq.raw() as u64;
    // Make sure we won't overflow when multiplying; DTG is max 1008 ticks and CKD is max prescaler of 4
    // so deadtimes over 4032 ticks are impossible (4032*10^9 before dividing)
    assert!(deadtime_ticks <= 4_032_000_000_000u64);
    let deadtime_ticks = deadtime_ticks * 42950;
    let deadtime_ticks = (deadtime_ticks >> 32) as u32;
    let deadtime_ticks = deadtime_ticks as u64 * 429497;
    let deadtime_ticks = (deadtime_ticks >> 32) as u32;

    // Choose CR1 CKD divider of 1, 2, or 4 to determine tDTS
    let (deadtime_ticks, ckd) = match deadtime_ticks {
        t if t <= 1008 => (deadtime_ticks, 1),
        t if t <= 2016 => (deadtime_ticks / 2, 2),
        t if t <= 4032 => (deadtime_ticks / 4, 4),
        _ => {
            panic!("Deadtime must be less than 4032 ticks of timer base clock.")
        }
    };

    // Choose BDTR DTG bits to match deadtime_ticks
    // We want the smallest value of DTG that gives a deadtime >= the requested deadtime
    for dtg in 0..=255 {
        let actual_deadtime: u32 = match dtg {
            d if d < 128 => d,
            d if d < 192 => 2 * (64 + (d & 0x3F)),
            d if d < 224 => 8 * (32 + (d & 0x1F)),
            _ => 16 * (32 + (dtg & 0x1F)),
        };

        if actual_deadtime >= deadtime_ticks {
            return (dtg as u8, ckd);
        }
    }

    panic!("This should be unreachable.");
}

// Low-power timers
macro_rules! lptim_hal {
    ($($TIMX:ty: $timX:ident,)+) => {
        $(
            impl PwmExt for $TIMX {
                fn pwm<PINS, T, U, V>(self, _pins: PINS, frequency: T, clocks: &Clocks) -> PINS::Channel
                where
                    PINS: Pins<Self, U, V>,
                    T: Into<Hertz>,
                {
                    $timX(self, _pins, frequency.into(), clocks)
                }
            }

            /// Configures PWM signal on the LPTIM OUT pin.
            fn $timX<PINS, T, U>(tim: $TIMX, _pins: PINS, freq: Hertz, clocks: &Clocks) -> PINS::Channel
            where
                PINS: Pins<$TIMX, T, U>,
            {
                unsafe {
                    let rcc = &(*pac::Rcc::PTR);
                    <$TIMX>::enable(rcc);
                    <$TIMX>::reset(rcc);
                }

                let clk = <$TIMX>::timer_clock(clocks);
                let reload = clk / freq;
                assert!(reload < 128 * (1 << 16));

                // Calculate prescaler
                let (prescale, prescale_div) = match reload / (1 << 16) {
                    0 => (0b000, 1),
                    1 => (0b001, 2),
                    2..=3 => (0b010, 4),
                    4..=7 => (0b011, 8),
                    8..=15 => (0b100, 16),
                    16..=31 => (0b101, 32),
                    32..=63 => (0b110, 64),
                    _ => (0b111, 128),
                };

                // Calcuate reload
                let arr = reload / prescale_div;
                assert!(arr <= 0xFFFF);
                assert!(arr > 0);

                // CFGR
                tim.cfgr().modify(|_, w| unsafe { w.presc().bits(prescale) });

                // Enable
                tim.cr().modify(|_, w| w.enable().set_bit());

                // Write ARR: LPTIM must be enabled
                tim.arr().write(|w| unsafe { w.arr().bits(arr as u16) });
                while !tim.isr().read().arrok().bit_is_set() {}
                tim.icr().write(|w| w.arrokcf().set_bit());

                // PWM output is disabled by default, disable the
                // entire timer
                tim.cr().modify(|_, w| w.enable().clear_bit());

                unsafe { MaybeUninit::<PINS::Channel>::uninit().assume_init() }
            }

            impl Pwm<$TIMX, C1, ComplementaryImpossible, ActiveHigh, ActiveHigh> {
                fn disable(&mut self) {
                    let tim = unsafe { &*<$TIMX>::PTR };

                    // LPTIM only has one output, so we disable the
                    // entire timer
                    tim.cr().modify(|_, w| w.enable().clear_bit());
                }

                fn enable(&mut self) {
                    let tim = unsafe { &*<$TIMX>::PTR };

                    tim.cr()
                        .modify(|_, w| w.cntstrt().set_bit().enable().set_bit());
                }

                fn get_duty(&self) -> u16 {
                    let tim = unsafe { &*<$TIMX>::PTR };

                    tim.cmp().read().cmp().bits()
                }

                fn get_max_duty(&self) -> u16 {
                    let tim = unsafe { &*<$TIMX>::PTR };

                    tim.arr().read().arr().bits()
                }

                fn set_duty(&mut self, duty: u16) {
                    let tim = unsafe { &*<$TIMX>::PTR };

                    tim.cmp().write(|w| unsafe { w.cmp().bits(duty) });
                    while !tim.isr().read().cmpok().bit_is_set() {}
                    tim.icr().write(|w| w.cmpokcf().set_bit());
                }
            }
        )+
    }
}

lptim_hal! {
    pac::Lptimer1: lptimer1,
}

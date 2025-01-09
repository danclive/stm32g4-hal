//! pwm input
use core::marker::PhantomData;

use fugit::HertzU32 as Hertz;

use crate::pac;
use crate::rcc::Clocks;
use crate::rcc::{BusTimerClock, Enable, Reset};

pub trait Pin<TIM, CHANNEL> {}

pub struct C1;

pub struct C2;

impl C1 {
    const CH: u8 = 1;
}

impl C2 {
    const CH: u8 = 2;
}

pub trait Channel {
    const CH: u8;
}

impl Channel for C1 {
    const CH: u8 = C1::CH;
}

impl Channel for C2 {
    const CH: u8 = C2::CH;
}

pub struct PwmInput<TIM, CHANNEL> {
    _tim: PhantomData<TIM>,
    _channel: PhantomData<CHANNEL>,
}

pub trait PwmInputExt: Sized {
    fn pwm_input<PIN, CHANNEL, T>(
        self,
        clocks: &Clocks,
        freq: T,
        _pin: PIN,
    ) -> PwmInput<Self, CHANNEL>
    where
        PIN: Pin<Self, CHANNEL>,
        CHANNEL: Channel,
        T: Into<Hertz>;
}

macro_rules! tim_hal {
    ($TIMX:ident, $timX:ident) => {
        impl PwmInputExt for pac::$TIMX {
            fn pwm_input<PIN, CHANNEL, T>(
                self,
                clocks: &Clocks,
                freq: T,
                pin: PIN,
            ) -> PwmInput<Self, CHANNEL>
            where
                PIN: Pin<Self, CHANNEL>,
                CHANNEL: Channel,
                T: Into<Hertz>,
            {
                $timX(self, clocks, freq, pin)
            }
        }

        pub fn $timX<PIN, CHANNEL, T>(
            tim: pac::$TIMX,
            clocks: &Clocks,
            freq: T,
            _pin: PIN,
        ) -> PwmInput<pac::$TIMX, CHANNEL>
        where
            PIN: Pin<pac::$TIMX, CHANNEL>,
            CHANNEL: Channel,
            T: Into<Hertz>,
        {
            unsafe {
                let rcc = &(*pac::Rcc::PTR);
                <pac::$TIMX>::enable(rcc);
                <pac::$TIMX>::reset(rcc);
            }

            let clk = <pac::$TIMX>::timer_clock(clocks);

            /*
            Borrowed from PWM implementation.
            Sets the TIMer's prescaler such that the TIMer that it ticks at about the best-guess
             frequency.
            */
            let ticks = clk.raw() / freq.into().raw();
            assert!(ticks != 0);
            let psc = (ticks - 1) / (1 << 16);
            // Write prescale
            tim.psc().write(|w| unsafe { w.psc().bits(psc as u16) });

            /*
            For example, one can measure the period (in TIMx_CCR1 register) and the duty cycle (in
            TIMx_CCR2 register) of the PWM applied on TI1 using the following procedure (depending
            on CK_INT frequency and prescaler value):

            from RM0390 16.3.7
             */
            let ch = CHANNEL::CH;

            // 1. Select the proper tim_tix_in[15:0] source (internal or external) with the TI1SEL[3:0] bits
            // in the TIMx_TISEL register.

            // 2. Select the active input for TIMx_CCR1: write the CC1S bits to 01 in the TIMx_CCMR1
            // register (TI1 selected).
            match ch {
                C1::CH => {
                    tim.ccmr1_input()
                        .modify(|_, w| unsafe { w.cc1s().bits(0b01) });
                }
                C2::CH => {
                    tim.ccmr1_input()
                        .modify(|_, w| unsafe { w.cc2s().bits(0b01) });
                }
                _ => unreachable!(),
            }

            // 3. Select the active polarity for TI1FP1 (used both for capture in TIMx_CCR1 and counter
            // clear): write the CC1P and CC1NP bits to ‘0’ (active on rising edge).
            match ch {
                C1::CH => {
                    tim.ccer()
                        .modify(|_, w| w.cc1p().clear_bit().cc1np().clear_bit());
                }
                C2::CH => {
                    tim.ccer()
                        .modify(|_, w| w.cc2p().clear_bit().cc2np().clear_bit());
                }
                _ => unreachable!(),
            }

            // disable filters and disable the input capture prescalers.
            tim.ccmr1_input().modify(|_, w| unsafe {
                w.ic1f().bits(0).ic2f().bits(0);
                w.ic1psc().bits(0).ic2psc().bits(0)
            });

            // 4. Select the active input for TIMx_CCR2: write the CC2S bits to 10 in the TIMx_CCMR1
            // register (TI1 selected)
            match ch {
                C1::CH => {
                    tim.ccmr1_input()
                        .modify(|_, w| unsafe { w.cc2s().bits(0b10) });
                }
                C2::CH => {
                    tim.ccmr1_input()
                        .modify(|_, w| unsafe { w.cc1s().bits(0b10) });
                }
                _ => unreachable!(),
            }

            // 5. Select the active polarity for TI1FP2 (used for capture in TIMx_CCR2): write the CC2P
            // and CC2NP bits to ‘1’ (active on falling edge).
            match ch {
                C1::CH => {
                    tim.ccer()
                        .modify(|_, w| w.cc2p().set_bit().cc2np().set_bit());
                }
                C2::CH => {
                    tim.ccer()
                        .modify(|_, w| w.cc1p().set_bit().cc1np().set_bit());
                }
                _ => unreachable!(),
            }

            // 6. Select the valid trigger input: write the TS bits to 101 in the TIMx_SMCR register
            // (TI1FP1 selected).
            match ch {
                C1::CH => {
                    tim.smcr().modify(|_, w| unsafe { w.ts().bits(0b101) });
                }
                C2::CH => {
                    tim.smcr().modify(|_, w| unsafe { w.ts().bits(0b110) });
                }
                _ => unreachable!(),
            }

            // 7. Configure the slave mode controller in reset mode: write the SMS bits to 100 in the
            // TIMx_SMCR register.
            tim.smcr().modify(|_, w| unsafe { w.sms().bits(0b100) });

            // 8. Enable the captures: write the CC1E and CC2E bits to ‘1’ in the TIMx_CCER register.
            // tim.ccer()
            //     .modify(|_, w| w.cc1e().set_bit().cc2e().set_bit());

            // enable interrupts.
            match ch {
                C1::CH => {
                    tim.dier().modify(|_, w| w.cc2ie().set_bit());
                }
                C2::CH => {
                    tim.dier().modify(|_, w| w.cc1ie().set_bit());
                }
                _ => unreachable!(),
            }

            // enable the counter.
            tim.cr1().modify(|_, w| w.cen().bit(true));

            PwmInput {
                _tim: PhantomData,
                _channel: PhantomData,
            }
        }

        impl<CHANNEL: Channel> PwmInput<pac::$TIMX, CHANNEL> {
            pub fn enable(&mut self) {
                let tim = unsafe { &*<pac::$TIMX>::PTR };

                tim.ccer()
                    .modify(|_, w| w.cc1e().set_bit().cc2e().set_bit());
            }

            pub fn disable(&mut self) {
                let tim = unsafe { &*<pac::$TIMX>::PTR };

                tim.ccer()
                    .modify(|_, w| w.cc1e().clear_bit().cc2e().clear_bit());
            }

            pub fn is_enable(&self) -> bool {
                let tim = unsafe { &*<pac::$TIMX>::PTR };

                tim.ccer().read().cc1e().bit() && tim.ccer().read().cc2e().bit()
            }

            /// Period of PWM signal in terms of clock cycles
            pub fn get_period_clocks(&self) -> u32 {
                let tim = unsafe { &*<pac::$TIMX>::PTR };
                let ch = CHANNEL::CH;

                match ch {
                    C1::CH => tim.ccr1().read().ccr1().bits(),
                    C2::CH => tim.ccr2().read().ccr2().bits(),
                    _ => unreachable!(),
                }
            }

            /// Duty cycle in terms of clock cycles
            pub fn get_duty_cycle_clocks(&self) -> u32 {
                let tim = unsafe { &*<pac::$TIMX>::PTR };
                let ch = CHANNEL::CH;

                match ch {
                    C1::CH => tim.ccr2().read().ccr2().bits(),
                    C2::CH => tim.ccr1().read().ccr1().bits(),
                    _ => unreachable!(),
                }
            }

            /// Observed duty cycle as a float in range [0.00, 1.00]
            pub fn get_duty_cycle(&self) -> f32 {
                let period_clocks = self.get_period_clocks();
                if period_clocks == 0 {
                    return 0.;
                };
                (self.get_duty_cycle_clocks() as f32 / period_clocks as f32) * 100f32
            }

            /// Returns whether the timer's duty cycle is a valid observation
            /// (Limitation of how the captures work is extra CC2 interrupts are generated when the
            /// PWM cycle enters a new period).
            pub fn is_valid_capture(&self) -> bool {
                self.get_duty_cycle_clocks() != self.get_period_clocks()
            }
        }
    };
}

tim_hal!(Tim1, tim1);

tim_hal!(Tim2, tim2);

tim_hal!(Tim3, tim3);

tim_hal!(Tim4, tim4);

#[cfg(feature = "tim5")]
tim_hal!(Tim5, tim5);

tim_hal!(Tim8, tim8);

tim_hal!(Tim15, tim15);

#[cfg(feature = "tim20")]
tim_hal!(Tim20, tim20);

macro_rules! pin {
    ($TIMX:ident {
        CH1: [$($( #[ $pmeta1:meta ] )* $CH1:ty),*]
        CH2: [$($( #[ $pmeta2:meta ] )* $CH2:ty),*] }) => {

        $(
            $( #[ $pmeta1 ] )*
            impl Pin<pac::$TIMX, C1> for $CH1 {}
        )*
        $(
            $( #[ $pmeta2 ] )*
            impl Pin<pac::$TIMX, C2> for $CH2 {}
        )*
    };
}

use crate::gpio::*;

pin! {
    Tim1 {
        CH1: [
            PA8<Alt<6>>,
            PC0<Alt<2>>,
            PE9<Alt<2>>
        ]
        CH2: [
            PA9<Alt<6>>,
            PC1<Alt<2>>,
            PE11<Alt<2>>
        ]
    }
}

pin! {
    Tim2 {
        CH1: [
            PA0<Alt<1>>,
            PA5<Alt<1>>,
            PA15<Alt<1>>,
            PD3<Alt<2>>
        ]
        CH2: [
            PA1<Alt<1>>,
            PB3<Alt<1>>,
            PD4<Alt<2>>
        ]
    }
}

pin! {
    Tim3 {
        CH1: [
            PA6<Alt<2>>,
            PB4<Alt<2>>,
            PC6<Alt<2>>,
            PE2<Alt<2>>
        ]
        CH2: [
            PA4<Alt<2>>,
            PA7<Alt<2>>,
            PB5<Alt<2>>,
            PC7<Alt<2>>,
            PE3<Alt<2>>
        ]
    }
}

pin! {
    Tim4 {
        CH1: [
            PA11<Alt<10>>,
            PB6<Alt<2>>,
            PD12<Alt<2>>
        ]
        CH2: [
            PA12<Alt<10>>,
            PB7<Alt<2>>,
            PD13<Alt<2>>
        ]
    }
}

#[cfg(feature = "tim5")]
pin! {
    Tim5 {
        CH1: [
            PA0<Alt<2>>,
            PB2<Alt<2>>,
            PF6<Alt<6>>
        ]
        CH2: [
            PA1<Alt<2>>,
            PC12<Alt<1>>,
            PF7<Alt<6>>
        ]
    }
}

pin! {
    Tim8 {
        CH1: [
            PA15<Alt<2>>,
            PB6<Alt<5>>,
            PC6<Alt<4>>
        ]
        CH2: [
            PA14<Alt<5>>,
            PB8<Alt<10>>,
            PC7<Alt<4>>
        ]
    }
}

pin! {
    Tim15 {
        CH1: [
            PA2<Alt<9>>,
            PB14<Alt<1>>,
            PF9<Alt<3>>
        ]
        CH2: [
            PA3<Alt<9>>,
            PB15<Alt<1>>,
            PF10<Alt<3>>
        ]
    }
}

#[cfg(feature = "tim20")]
pin! {
    Tim20 {
        CH1: [
            PB2<Alt<3>>,
            PE2<Alt<6>>,
            #[cfg(any(
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484",
            ))]
            PF12<Alt<2>>
        ]
        CH2: [
            PC2<Alt<6>>,
            PE3<Alt<6>>,
            #[cfg(any(
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484",
            ))]
            PF13<Alt<2>>
        ]
    }
}

//! Quadrature Encoder Input

use crate::pac;

use crate::rcc::{Enable, Reset};

pub trait CH1Pin<TIM> {}

pub trait CH2Pin<TIM> {}

/// Hardware quadrature encoder interface peripheral
pub struct Qei<TIM> {
    tim: TIM,
}

pub trait QeiExt<TIM> {
    fn qei<CH1, CH2>(self, _pins: (CH1, CH2)) -> Qei<TIM>
    where
        CH1: CH1Pin<TIM>,
        CH2: CH2Pin<TIM>;

    fn qei_unchecked(self) -> Qei<TIM>;
}

/// Count direction
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    /// 3, 2, 1
    Downcounting,
    /// 1, 2, 3
    Upcounting,
}

macro_rules! tim_hal {
    ($TIMX:ident: ($timX:ident, $bits:ident)) => {
        impl QeiExt<pac::$TIMX> for pac::$TIMX {
            fn qei<CH1, CH2>(self, _pins: (CH1, CH2)) -> Qei<pac::$TIMX> {
                $timX(self)
            }

            fn qei_unchecked(self) -> Qei<pac::$TIMX> {
                $timX(self)
            }
        }

        /// Configures a TIM peripheral as a quadrature
        /// encoder interface input
        pub fn $timX(tim: pac::$TIMX) -> Qei<pac::$TIMX> {
            // enable and reset peripheral to a clean slate
            unsafe {
                let rcc = &(*pac::Rcc::PTR);
                <pac::$TIMX>::enable(rcc);
                <pac::$TIMX>::reset(rcc);
            }

            // Configure TxC1 and TxC2 as captures
            tim.ccmr1_output()
                .write(|w| unsafe { w.cc1s().bits(0b01).cc2s().bits(0b01) });

            // enable and configure to capture on rising edge
            tim.ccer().write(|w| {
                w.cc1e()
                    .set_bit()
                    .cc1p()
                    .clear_bit()
                    .cc2e()
                    .set_bit()
                    .cc2p()
                    .clear_bit()
            });

            // configure as quadrature encoder
            tim.smcr().write(|w| unsafe { w.sms().bits(0x011) });

            tim.arr().write(|w| unsafe { w.bits(u32::MAX) });
            tim.cr1().write(|w| w.cen().set_bit());

            Qei { tim }
        }

        impl Qei<pac::$TIMX> {
            /// Releases the TIM peripheral
            pub fn release(self) -> pac::$TIMX {
                self.tim
            }

            pub fn count(&self) -> $bits {
                self.tim.cnt().read().bits() as $bits
            }

            pub fn direction(&self) -> Direction {
                if self.tim.cr1().read().dir().bit_is_clear() {
                    Direction::Upcounting
                } else {
                    Direction::Downcounting
                }
            }
        }
    };
}

tim_hal!(Tim1: (tim1, u16));

tim_hal!(Tim2: (tim2, u32));

tim_hal!(Tim3: (tim3, u16));

tim_hal!(Tim4: (tim4, u16));

#[cfg(feature = "tim5")]
tim_hal!(Tim5: (tim5, u16));

tim_hal!(Tim8: (tim8, u16));

#[cfg(feature = "tim20")]
tim_hal!(Tim20: (tim20, u16));

macro_rules! pins {
    ($TIMX:ident {
       CH1: [$($( #[ $pmeta1:meta ] )* $CH1:ty),*]
       CH2: [$($( #[ $pmeta2:meta ] )* $CH2:ty),*] }) => {
        $(
            $( #[ $pmeta1 ] )*
            impl CH1Pin<pac::$TIMX> for $CH1 {}
        )*
        $(
            $( #[ $pmeta2 ] )*
            impl CH2Pin<pac::$TIMX> for $CH2 {}
        )*
    }
}

use crate::gpio::*;

pins! {
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

pins! {
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

pins! {
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

pins! {
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
pins! {
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

pins! {
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

#[cfg(feature = "tim20")]
pins! {
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

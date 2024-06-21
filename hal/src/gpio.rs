//! General Purpose Input / Output

use core::fmt;
use core::marker::PhantomData;
use core::{convert::From, ops::Not};

pub use convert::PinMode;
pub use dynamic::{Dynamic, DynamicPin};
pub use erased::ErasedPin;
pub use exti::ExtiPin;
pub use partially_erased::PartiallyErasedPin;

mod convert;
mod dynamic;
mod erased;
mod exti;
pub mod marker;
pub mod outport;
mod partially_erased;

/// Input mode (type state)
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Input;

/// Pull setting for an input.
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pull {
    /// Floating
    None = 0,
    /// Pulled up
    Up = 1,
    /// Pulled down
    Down = 2,
}

/// Output mode (type state)
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Output<MODE = PushPull> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PushPull;

/// Open drain input or output (type state)
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OpenDrain;

/// Analog mode (type state)
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Analog;

/// Some alternate mode (type state)
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Alt<const A: u8, OutType = PushPull>(PhantomData<OutType>);

/// JTAG/SWD mote (type state)
pub type Debugger = Alt<0, PushPull>;

/// A filler pin type
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct NoPin<OutType = PushPull>(PhantomData<OutType>);
impl<OutType> NoPin<OutType> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

/// GPIO Pin speed selection
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Speed {
    /// Low speed
    Low = 0,
    /// Medium speed
    Medium = 1,
    /// High speed
    High = 2,
    /// Very high speed
    VeryHigh = 3,
}

/// GPIO interrupt trigger edge selection
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Edge {
    /// Rising edge of voltage
    Rising,
    /// Falling edge of voltage
    Falling,
    /// Rising and falling edge of voltage
    RisingFalling,
}

/// Digital output pin state
///
/// Conversion from `bool` and logical negation are also implemented
/// for this type.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PinState {
    /// Low pin state
    Low,
    /// High pin state
    High,
}

impl From<bool> for PinState {
    fn from(value: bool) -> Self {
        match value {
            false => PinState::Low,
            true => PinState::High,
        }
    }
}

impl Not for PinState {
    type Output = PinState;

    fn not(self) -> Self::Output {
        match self {
            PinState::High => PinState::Low,
            PinState::Low => PinState::High,
        }
    }
}

macro_rules! af {
    ($($i:literal: $AFi:ident),+) => {
        $(
            #[doc = concat!("Alternate function ", $i, " (type state)" )]
            pub type $AFi<OutType = PushPull> = Alt<$i, OutType>;
        )+
    };
}

af!(
    0: AF0,
    1: AF1,
    2: AF2,
    3: AF3,
    4: AF4,
    5: AF5,
    6: AF6,
    7: AF7,
    8: AF8,
    9: AF9,
    10: AF10,
    11: AF11,
    12: AF12,
    13: AF13,
    14: AF14,
    15: AF15
);

/// Generic pin type
///
/// - `MODE` is one of the pin modes (see [Modes](crate::gpio#modes) section).
/// - `P` is port name: `A` for GPIOA, `B` for GPIOB, etc.
/// - `N` is pin number: from `0` to `15`.
pub struct Pin<const P: char, const N: u8, MODE = Input> {
    _mode: PhantomData<MODE>,
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE> {
    const fn new() -> Self {
        Self { _mode: PhantomData }
    }
}

impl<const P: char, const N: u8, MODE> fmt::Debug for Pin<P, N, MODE> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "P{}{}<{}>",
            P,
            N,
            crate::stripped_type_name::<MODE>()
        ))
    }
}

#[cfg(feature = "defmt")]
impl<const P: char, const N: u8, MODE> defmt::Format for Pin<P, N, MODE> {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "P{}{}<{}>", P, N, crate::stripped_type_name::<MODE>());
    }
}

/// Id, port and mode for any pin
pub trait PinExt {
    /// Current pin mode
    type Mode;
    /// Pin number
    fn pin_id(&self) -> u8;
    /// Port number starting from 0
    fn port_id(&self) -> u8;
}

impl<const P: char, const N: u8, MODE> PinExt for Pin<P, N, MODE> {
    type Mode = MODE;

    #[inline(always)]
    fn pin_id(&self) -> u8 {
        N
    }

    #[inline(always)]
    fn port_id(&self) -> u8 {
        P as u8 - b'A'
    }
}

pub trait PinSpeed: Sized {
    /// Set pin speed
    fn set_speed(&mut self, speed: Speed);

    #[inline(always)]
    fn speed(mut self, speed: Speed) -> Self {
        self.set_speed(speed);
        self
    }
}

impl<const P: char, const N: u8, MODE> PinSpeed for Pin<P, N, MODE>
where
    MODE: marker::OutputSpeed,
{
    #[inline(always)]
    fn set_speed(&mut self, speed: Speed) {
        self.set_speed(speed)
    }
}

pub trait PinPull: Sized {
    /// Set the internal pull-up and pull-down resistor
    fn set_internal_resistor(&mut self, resistor: Pull);

    #[inline(always)]
    fn internal_resistor(mut self, resistor: Pull) -> Self {
        self.set_internal_resistor(resistor);
        self
    }
}

impl<const P: char, const N: u8, MODE> PinPull for Pin<P, N, MODE>
where
    MODE: marker::Active,
{
    #[inline(always)]
    fn set_internal_resistor(&mut self, resistor: Pull) {
        self.set_internal_resistor(resistor)
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE> {
    /// Erases the pin number from the type
    ///
    /// This is useful when you want to collect the pins into an array where you
    /// need all the elements to have the same type
    pub fn erase_number(self) -> PartiallyErasedPin<P, MODE> {
        PartiallyErasedPin::new(N)
    }

    /// Erases the pin number and the port from the type
    ///
    /// This is useful when you want to collect the pins into an array where you
    /// need all the elements to have the same type
    pub fn erase(self) -> ErasedPin<MODE> {
        ErasedPin::new(P as u8 - b'A', N)
    }
}

impl<const P: char, const N: u8, MODE> From<Pin<P, N, MODE>> for PartiallyErasedPin<P, MODE> {
    /// Pin-to-partially erased pin conversion using the [`From`] trait.
    ///
    /// Note that [`From`] is the reciprocal of [`Into`].
    fn from(p: Pin<P, N, MODE>) -> Self {
        p.erase_number()
    }
}

impl<const P: char, const N: u8, MODE> From<Pin<P, N, MODE>> for ErasedPin<MODE> {
    /// Pin-to-erased pin conversion using the [`From`] trait.
    ///
    /// Note that [`From`] is the reciprocal of [`Into`].
    fn from(p: Pin<P, N, MODE>) -> Self {
        p.erase()
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, Output<MODE>> {
    /// Drives the pin high
    #[inline(always)]
    pub fn set_high(&mut self) {
        self._set_high()
    }

    /// Drives the pin low
    #[inline(always)]
    pub fn set_low(&mut self) {
        self._set_low()
    }

    /// Is the pin in drive high or low mode?
    #[inline(always)]
    pub fn get_state(&self) -> PinState {
        if self.is_set_low() {
            PinState::Low
        } else {
            PinState::High
        }
    }

    /// Drives the pin high or low depending on the provided value
    #[inline(always)]
    pub fn set_state(&mut self, state: PinState) {
        match state {
            PinState::Low => self.set_low(),
            PinState::High => self.set_high(),
        }
    }

    /// Is the pin in drive high mode?
    #[inline(always)]
    pub fn is_set_high(&self) -> bool {
        !self.is_set_low()
    }

    /// Is the pin in drive low mode?
    #[inline(always)]
    pub fn is_set_low(&self) -> bool {
        self._is_set_low()
    }

    /// Toggle pin output
    #[inline(always)]
    pub fn toggle(&mut self) {
        if self.is_set_low() {
            self.set_high()
        } else {
            self.set_low()
        }
    }
}

pub trait ReadPin {
    #[inline(always)]
    fn is_high(&self) -> bool {
        !self.is_low()
    }
    fn is_low(&self) -> bool;
}

impl<const P: char, const N: u8, MODE> ReadPin for Pin<P, N, MODE>
where
    MODE: marker::Readable,
{
    #[inline(always)]
    fn is_low(&self) -> bool {
        self.is_low()
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE>
where
    MODE: marker::Readable,
{
    /// Is the input pin high?
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        !self.is_low()
    }

    /// Is the input pin low?
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        self._is_low()
    }
}

const fn gpiox<const P: char>() -> *const crate::pac::gpioa::RegisterBlock {
    match P {
        'A' => crate::pac::Gpioa::PTR,
        'B' => crate::pac::Gpiob::PTR as _,
        'C' => crate::pac::Gpioc::PTR as _,
        #[cfg(feature = "gpiod")]
        'D' => crate::pac::Gpiod::PTR as _,
        #[cfg(feature = "gpioe")]
        'E' => crate::pac::Gpioe::PTR as _,
        #[cfg(feature = "gpiof")]
        'F' => crate::pac::Gpiof::PTR as _,
        #[cfg(feature = "gpiog")]
        'G' => crate::pac::Gpiog::PTR as _,
        #[cfg(feature = "gpioh")]
        'H' => crate::pac::Gpioh::PTR as _,
        #[cfg(feature = "gpioi")]
        'I' => crate::pac::Gpioi::PTR as _,
        #[cfg(feature = "gpioj")]
        'J' => crate::pac::Gpioj::PTR as _,
        #[cfg(feature = "gpiok")]
        'K' => crate::pac::Gpiok::PTR as _,
        _ => panic!("Unknown GPIO port"),
    }
}

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $PEPin:ident, $port_id:expr, $PXn:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, [$($A:literal),*] $(, $MODE:ty)?),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use crate::pac::$GPIOX;
            use crate::rcc::{Enable, Reset};

            /// GPIO pins
            pub struct Pins {
                $(
                    /// Pin
                    pub $pxi: $PXi $(<$MODE>)?,
                )+
            }

            impl Pins {
                pub const P: char = $port_id;

                pub fn new(_: $GPIOX) -> Pins {
                    unsafe {
                        // Enable clock.
                        $GPIOX::enable_unchecked();
                        $GPIOX::reset_unchecked();
                    }
                    Pins {
                        $(
                            $pxi: $PXi::new(),
                        )+
                    }
                }
            }

            #[doc="Common type for "]
            #[doc=stringify!($GPIOX)]
            #[doc=" related pins"]
            pub type $PXn<MODE> = crate::gpio::PartiallyErasedPin<$port_id, MODE>;

            $(
                #[doc=stringify!($PXi)]
                #[doc=" pin"]
                pub type $PXi<MODE = crate::gpio::Input> = crate::gpio::Pin<$port_id, $i, MODE>;

                $(
                    impl<MODE> crate::gpio::marker::IntoAf<$A> for $PXi<MODE> { }
                )*
            )+

        }

        #[allow(unused_imports)]
        pub(crate) use $gpiox::{ $($PXi,)+ };
    }
}

gpio!(Gpioa, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 7, 8, 9, 10, 14, 15]),
    PA1: (pa1, 1, [0, 1, 7, 9, 15]),
    PA2: (pa2, 2, [1, 7, 8, 9, 12, 14, 15]),
    PA3: (pa3, 3, [1, 3, 7, 9, 12, 13, 15]),
    PA4: (pa4, 4, [2, 5, 6, 7, 13, 15]),
    PA5: (pa5, 5, [1, 2, 5, 14, 15]),
    PA6: (pa6, 6, [1, 2, 4, 5, 6, 8, 12, 15]),
    PA7: (pa7, 7, [1, 2, 4, 5, 6, 8, 14, 15]),
    PA8: (pa8, 8, [0, 2, 4, 5, 6, 7, 10, 12, 14, 15]),
    PA9: (pa9, 9, [2, 4, 5, 6, 7, 9, 10, 14, 15]),
    PA10: (pa10, 10, [1, 3, 4, 5, 6, 7, 10, 11, 12, 14, 15]),
    PA11: (pa11, 11, [5, 6, 7, 8, 9, 10, 11, 12, 15]),
    PA12: (pa12, 12, [1, 5, 6, 7, 8, 9, 10, 11, 15]),
    PA13: (pa13, 13, [0, 1, 4, 5, 7, 10, 13, 15], super::Debugger),
    PA14: (pa14, 14, [0, 1, 4, 5, 6, 7, 13, 15], super::Debugger),
    PA15: (pa15, 15, [0, 1, 2, 4, 5, 6, 7, 8, 9, 14, 15], super::Debugger),
]);

gpio!(Gpiob, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [2, 4, 6, 14, 15]),
    PB1: (pb1, 1, [2, 4, 6, 8, 12, 15]),
    PB2: (pb2, 2, [0, 1, 4, 15]),
    PB3: (pb3, 3, [0, 1, 2, 3, 4, 5, 6, 7, 10, 14, 15]),
    PB4: (pb4, 4, [0, 1, 2, 4, 5, 6, 7, 10, 14, 15]),
    PB5: (pb5, 5, [1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 15]),
    PB6: (pb6, 6, [1, 2, 5, 6, 7, 8, 10, 11, 14, 15]),
    PB7: (pb7, 7, [1, 2, 4, 5, 7, 8, 10, 11, 14, 15]),
    PB8: (pb8, 8, [1, 2, 3, 4, 7, 8, 9, 10, 12, 14, 15]),
    PB9: (pb9, 9, [1, 2, 3, 4, 6, 7, 8, 9, 10, 12, 14, 15]),
    PB10: (pb10, 10, [1, 7, 8, 12, 14, 15]),
    PB11: (pb11, 11, [1, 7, 8, 15]),
    PB12: (pb12, 12, [4, 5, 6, 7, 8, 15]),
    PB13: (pb13, 13, [5, 6, 7, 8, 15]),
    PB14: (pb14, 14, [1, 5, 6, 7, 8, 15]),
    PB15: (pb15, 15, [0, 1, 2, 3, 4, 5, 15]),
]);

gpio!(Gpioc, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [1, 2, 8, 15]),
    PC1: (pc1, 1, [1, 2, 8, 13, 15]),
    PC2: (pc2, 2, [1, 2, 3, 15]),
    PC3: (pc3, 3, [1, 2, 3, 6, 13, 15]),
    PC4: (pc4, 4, [2, 4, 7, 15]),
    PC5: (pc5, 5, [2, 3, 6, 7, 15]),
    PC6: (pc6, 6, [2, 4, 6, 15]),
    PC7: (pc7, 7, [2, 4, 6, 15]),
    PC8: (pc8, 8, [2, 4, 8, 15]),
    PC9: (pc9, 9, [2, 4, 5, 6, 8, 15]),
    PC10: (pc10, 10, [4, 5, 6, 7, 15]),
    PC11: (pc11, 11, [4, 5, 6, 7, 8, 15]),
    PC12: (pc12, 12, [4, 6, 7, 14, 15]),
    PC13: (pc13, 13, [2, 4, 6, 15]),
    PC14: (pc14, 14, [15]),
    PC15: (pc15, 15, [15]),
]);

gpio!(Gpiod, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, [6, 9, 15]),
    PD1: (pd1, 1, [4, 6, 9, 15]),
    PD2: (pd2, 2, [2, 4, 15]),
    PD3: (pd3, 3, [2, 7, 15]),
    PD4: (pd4, 4, [2, 7, 15]),
    PD5: (pd5, 5, [7, 15]),
    PD6: (pd6, 6, [2, 3, 7, 15]),
    PD7: (pd7, 7, [2, 7, 15]),
    PD8: (pd8, 8, [7, 15]),
    PD9: (pd9, 9, [7, 15]),
    PD10: (pd10, 10, [7, 15]),
    PD11: (pd11, 11, [7, 15]),
    PD12: (pd12, 12, [2, 7, 15]),
    PD13: (pd13, 13, [2, 15]),
    PD14: (pd14, 14, [2, 15]),
    PD15: (pd15, 15, [2, 6, 15]),
]);

gpio!(Gpioe, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [2, 3, 4, 15]),
    PE1: (pe1, 1, [4, 7, 15]),
    PE2: (pe2, 2, [0, 2, 3, 13, 15]),
    PE3: (pe3, 3, [0, 2, 13, 15]),
    PE4: (pe4, 4, [0, 2, 3, 13, 15]),
    PE5: (pe5, 5, [0, 2, 3, 13, 15]),
    PE6: (pe6, 6, [0, 3, 13, 15]),
    PE7: (pe7, 7, [2, 13, 15]),
    PE8: (pe8, 8, [2, 13, 15]),
    PE9: (pe9, 9, [2, 13, 15]),
    PE10: (pe10, 10, [2, 13, 15]),
    PE11: (pe11, 11, [2, 15]),
    PE12: (pe12, 12, [2, 15]),
    PE13: (pe13, 13, [2, 15]),
    PE14: (pe14, 14, [2, 6, 15]),
    PE15: (pe15, 15, [2, 6, 7, 15]),
]);

gpio!(Gpiof, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [4, 5, 6, 15]),
    PF1: (pf1, 1, [5, 15]),
    PF2: (pf2, 2, [4, 15]),
    PF9: (pf9, 9, [3, 5, 13, 15]),
    PF10: (pf10, 10, [3, 5, 13, 15]),
]);

gpio!(Gpiog, gpiog, PG, 'G', PGn, [
    PG10: (pg10, 10, [0, 15]),
]);

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE> {
    /// Set the output of the pin regardless of its mode.
    /// Primarily used to set the output value of the pin
    /// before changing its mode to an output to avoid
    /// a short spike of an incorrect value
    #[inline(always)]
    fn _set_state(&mut self, state: PinState) {
        match state {
            PinState::High => self._set_high(),
            PinState::Low => self._set_low(),
        }
    }
    #[inline(always)]
    fn _set_high(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { (*gpiox::<P>()).bsrr().write(|w| w.bits(1 << N)) }
    }
    #[inline(always)]
    fn _set_low(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { (*gpiox::<P>()).bsrr().write(|w| w.bits(1 << (16 + N))) }
    }
    #[inline(always)]
    fn _is_set_low(&self) -> bool {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*gpiox::<P>()).odr().read().bits() & (1 << N) == 0 }
    }
    #[inline(always)]
    fn _is_low(&self) -> bool {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*gpiox::<P>()).idr().read().bits() & (1 << N) == 0 }
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE>
where
    MODE: marker::OutputSpeed,
{
    /// Set pin speed
    pub fn set_speed(&mut self, speed: Speed) {
        let offset = 2 * { N };

        unsafe {
            (*gpiox::<P>())
                .ospeedr()
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | ((speed as u32) << offset)));
        }
    }

    /// Set pin speed
    pub fn speed(mut self, speed: Speed) -> Self {
        self.set_speed(speed);
        self
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE>
where
    MODE: marker::Active,
{
    /// Set the internal pull-up and pull-down resistor
    pub fn set_internal_resistor(&mut self, resistor: Pull) {
        let offset = 2 * { N };
        let value = resistor as u32;
        unsafe {
            (*gpiox::<P>())
                .pupdr()
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (value << offset)));
        }
    }

    /// Set the internal pull-up and pull-down resistor
    pub fn internal_resistor(mut self, resistor: Pull) -> Self {
        self.set_internal_resistor(resistor);
        self
    }

    /// Enables / disables the internal pull up
    pub fn internal_pull_up(self, on: bool) -> Self {
        if on {
            self.internal_resistor(Pull::Up)
        } else {
            self.internal_resistor(Pull::None)
        }
    }

    /// Enables / disables the internal pull down
    pub fn internal_pull_down(self, on: bool) -> Self {
        if on {
            self.internal_resistor(Pull::Down)
        } else {
            self.internal_resistor(Pull::None)
        }
    }
}

//! # Controller Area Network (CAN) Interface

use crate::pac;
use crate::rcc;

use fdcan;

/// A TX pin configured for CAN communication
pub trait Tx<CAN> {}
/// An RX pin configured for CAN communication
pub trait Rx<CAN> {}

/// Select an FDCAN Clock Source
#[allow(clippy::upper_case_acronyms)]
#[allow(dead_code)]
enum FdCanClockSource {
    /// Select HSE as the FDCAN clock source
    HSE = 0b00,
    /// Select PLL "Q" clock as the FDCAN clock source
    PLLQ = 0b01,
    /// Select "P" clock as the FDCAN clock source
    PCLK = 0b10,
    //Reserved = 0b10,
}

/// Storage type for the CAN controller
#[derive(Debug)]
pub struct Can<FDCAN> {
    rb: FDCAN,
}
#[allow(dead_code)]
impl<FDCAN> Can<FDCAN> {
    /// Returns a reference to the inner peripheral
    fn inner(&self) -> &FDCAN {
        &self.rb
    }
}

/// Extension trait for CAN controller
pub trait CanExt: Sized
where
    Self: rcc::Enable + rcc::Reset,
    Can<Self>: fdcan::Instance,
{
    fn fdcan<TX, RX>(self, _tx: TX, _rx: RX) -> fdcan::FdCan<Can<Self>, fdcan::ConfigMode>
    where
        TX: Tx<Self>,
        RX: Rx<Self>,
    {
        let rcc = unsafe { &(*pac::Rcc::PTR) };
        Self::enable(rcc);

        if rcc.rcc_ccipr().read().fdcansel().bits() == FdCanClockSource::HSE as u8 {
            // Select P clock as FDCAN clock source
            rcc.rcc_ccipr().modify(|_, w| {
                // This is sound, as `FdCanClockSource` only contains valid values for this field.
                unsafe {
                    w.fdcansel().bits(FdCanClockSource::PCLK as u8);
                }

                w
            });
        }

        self.fdcan_unchecked()
    }

    fn fdcan_unchecked(self) -> fdcan::FdCan<Can<Self>, fdcan::ConfigMode>;
}
/// Implements Tx,Rx for pins associated with a CAN peripheral
macro_rules! pins {
    ($PER:ident =>
        (tx: [ $($( #[ $pmetatx:meta ] )* $tx:ty),+ $(,)? ],
         rx: [ $($( #[ $pmetarx:meta ] )* $rx:ty),+ $(,)? ])) => {
        $(
            $( #[ $pmetatx ] )*
            impl Tx<$PER> for $tx {}
        )+
        $(
            $( #[ $pmetarx ] )*
            impl Rx<$PER> for $rx {}
        )+
    };
}

mod fdcan1 {
    use super::{Can, CanExt, Rx, Tx};
    use crate::gpio::*;
    use crate::pac::Fdcan1;
    use fdcan;

    // All STM32G4 models with CAN support these pins
    pins! {
        Fdcan1 => (
            tx: [
                PA12<AF9>,
                PB9<AF9>,
                PD1<AF9>,
            ],
            rx: [
                PA11<AF9>,
                PB8<AF9>,
                PD0<AF9>,
            ]
        )
    }

    impl Can<Fdcan1> {
        pub fn fdcan1(rb: Fdcan1) -> fdcan::FdCan<Self, fdcan::ConfigMode> {
            fdcan::FdCan::new(Self { rb }).into_config_mode()
        }
    }
    impl CanExt for Fdcan1 {
        fn fdcan_unchecked(self) -> fdcan::FdCan<Can<Self>, fdcan::ConfigMode> {
            Can::fdcan1(self)
        }
    }
    unsafe impl fdcan::Instance for Can<Fdcan1> {
        const REGISTERS: *mut fdcan::RegisterBlock = Fdcan1::PTR as *mut _;
    }
    unsafe impl fdcan::message_ram::Instance for Can<Fdcan1> {
        const MSG_RAM: *mut fdcan::message_ram::RegisterBlock = (0x4000_a400 as *mut _);
    }
}

#[cfg(feature = "can2")]
mod fdcan2 {
    use super::{Can, CanExt, Rx, Tx};
    use crate::gpio::*;
    use crate::pac::Fdcan2;
    use fdcan;

    pins! {
        Fdcan2 => (
            tx: [
                PB6<AF9>,
                PB13<AF9>,
            ],
            rx: [
                PB5<AF9>,
                PB12<AF9>,
        ])
    }

    impl Can<Fdcan2> {
        pub fn fdcan2(rb: Fdcan2) -> fdcan::FdCan<Self, fdcan::ConfigMode> {
            fdcan::FdCan::new(Self { rb }).into_config_mode()
        }
    }
    impl CanExt for Fdcan2 {
        fn fdcan_unchecked(self) -> fdcan::FdCan<Can<Self>, fdcan::ConfigMode> {
            Can::fdcan2(self)
        }
    }
    unsafe impl fdcan::Instance for Can<Fdcan2> {
        const REGISTERS: *mut fdcan::RegisterBlock = Fdcan2::PTR as *mut _;
    }
    unsafe impl fdcan::message_ram::Instance for Can<Fdcan2> {
        const MSG_RAM: *mut fdcan::message_ram::RegisterBlock = (0x4000_a750 as *mut _);
    }
}

#[cfg(feature = "can3")]
mod fdcan3 {
    use super::{Can, CanExt, Rx, Tx};
    use crate::gpio::*;
    use crate::pac::Fdcan3;
    use fdcan;

    pins! {
        Fdcan3 => (
            tx: [
                PA15<AF11>,
                PB4<AF11>,
            ],
            rx: [
                PA8<AF11>,
                PB3<AF11>,
        ])
    }

    impl Can<Fdcan3> {
        pub fn fdcan3(rb: Fdcan3) -> fdcan::FdCan<Self, fdcan::ConfigMode> {
            fdcan::FdCan::new(Self { rb }).into_config_mode()
        }
    }
    impl CanExt for Fdcan3 {
        fn fdcan_unchecked(self) -> fdcan::FdCan<Can<Self>, fdcan::ConfigMode> {
            Can::fdcan3(self)
        }
    }
    unsafe impl fdcan::Instance for Can<Fdcan3> {
        const REGISTERS: *mut fdcan::RegisterBlock = Fdcan3::PTR as *mut _;
    }
    unsafe impl fdcan::message_ram::Instance for Can<Fdcan3> {
        const MSG_RAM: *mut fdcan::message_ram::RegisterBlock = (0x4000_aaa0 as *mut _);
    }
}

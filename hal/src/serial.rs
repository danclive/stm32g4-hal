//! Serial
use core::marker::PhantomData;

use crate::pac;

use crate::gpio::*;
use crate::nb;
use crate::rcc::Clocks;
use crate::rcc::{BusTimerClock, Enable, Reset};

pub mod config;

/// Serial error
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum Error {
    /// Framing error
    Framing,
    /// Noise error
    Noise,
    /// RX buffer overrun
    Overrun,
    /// Parity check error
    Parity,
}

/// UART interrupt events
#[enumflags2::bitflags]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum Event {
    /// New data has been received
    Rxne,
    /// New data can be sent
    Txe,
    /// Idle line state detected
    Idle,

    ///Tx threshlold interrupt enable
    Txftie,

    ///Rx threshlold interrupt enable
    Rxftie,
}

pub trait Pins<USART> {
    const SYNC: bool = false;
}

pub trait TXPin<USART> {}

pub trait RXPin<USART> {}

pub trait CKPin<USART> {}

impl<USART, TX, RX> Pins<USART> for (TX, RX)
where
    TX: TXPin<USART>,
    RX: RXPin<USART>,
{
}

impl<USART, TX, RX, CK> Pins<USART> for (TX, RX, CK)
where
    TX: TXPin<USART>,
    RX: RXPin<USART>,
    CK: CKPin<USART>,
{
    const SYNC: bool = true;
}

/// A filler type for when the Tx pin is unnecessary
pub type NoTx = NoPin;
/// A filler type for when the Rx pin is unnecessary
pub type NoRx = NoPin;
/// A filler type for when the Ck pin is unnecessary
pub type NoCk = NoPin;

impl<USART> TXPin<USART> for NoTx {}

impl<USART> RXPin<USART> for NoRx {}

impl<USART> CKPin<USART> for NoCk {}

pub struct Rx<USART> {
    _usart: PhantomData<USART>,
}

pub struct Tx<USART> {
    _usart: PhantomData<USART>,
}

pub struct Serial<USART> {
    pub(crate) usart: USART,
    tx: Tx<USART>,
    rx: Rx<USART>,
}

macro_rules! uart_hal_shared {
    ($USARTX:ty) => {
        impl Rx<$USARTX> {
            /// Starts listening for an interrupt event
            pub fn listen(&mut self) {
                let usart = unsafe { &(*<$USARTX>::PTR) };
                usart.cr1().modify(|_, w| w.rxneie().set_bit());
            }

            /// Stop listening for an interrupt event
            pub fn unlisten(&mut self) {
                let usart = unsafe { &(*<$USARTX>::PTR) };
                usart.cr1().modify(|_, w| w.rxneie().clear_bit());
                let _ = usart.cr1().read();
                let _ = usart.cr1().read(); // Delay 2 peripheral clocks
            }

            /// Return true if the line idle status is set
            ///
            /// The line idle status bit is set when the peripheral detects the receive line is idle.
            /// The bit is cleared by software, by calling `clear_idle()`.
            pub fn is_idle(&self) -> bool {
                let usart = unsafe { &(*<$USARTX>::PTR) };
                usart.isr().read().idle().bit_is_set()
            }

            /// Clear the line idle status bit
            pub fn clear_idle(&mut self) {
                let usart = unsafe { &(*<$USARTX>::PTR) };
                usart.icr().write(|w| w.idlecf().set_bit());
                let _ = usart.isr().read();
                let _ = usart.isr().read(); // Delay 2 peripheral clocks
            }

            /// Return true if the line busy status is set
            ///
            /// The busy status bit is set when there is communication active on the receive line,
            /// and reset at the end of reception.
            pub fn is_busy(&self) -> bool {
                let usart = unsafe { &(*<$USARTX>::PTR) };
                usart.isr().read().busy().bit_is_set()
            }

            /// Return true if the rx register is not empty (and can be read)
            pub fn is_rxne(&self) -> bool {
                let usart = unsafe { &(*<$USARTX>::PTR) };
                usart.isr().read().rxne().bit_is_set()
            }

            /// Returns true if the rx fifo threshold has been reached.
            pub fn fifo_threshold_reached(&self) -> bool {
                let usart = unsafe { &(*<$USARTX>::PTR) };
                usart.isr().read().rxft().bit_is_set()
            }

            pub fn read(&mut self) -> nb::Result<u8, Error> {
                let usart = unsafe { &(*<$USARTX>::PTR) };
                let isr = usart.isr().read();
                Err(if isr.pe().bit_is_set() {
                    usart.icr().write(|w| w.pecf().set_bit());
                    nb::Error::Other(Error::Parity)
                } else if isr.fe().bit_is_set() {
                    usart.icr().write(|w| w.fecf().set_bit());
                    nb::Error::Other(Error::Framing)
                } else if isr.nf().bit_is_set() {
                    usart.icr().write(|w| w.ncf().set_bit());
                    nb::Error::Other(Error::Noise)
                } else if isr.ore().bit_is_set() {
                    usart.icr().write(|w| w.orecf().set_bit());
                    nb::Error::Other(Error::Overrun)
                } else if isr.rxne().bit_is_set() {
                    return Ok(usart.rdr().read().bits() as u8);
                } else {
                    nb::Error::WouldBlock
                })
            }
        }

        impl Tx<$USARTX> {
            /// Starts listening for an interrupt event
            pub fn listen(&mut self) {
                let usart = unsafe { &(*<$USARTX>::PTR) };
                usart.cr1().modify(|_, w| w.txeie().set_bit());
            }

            /// Stop listening for an interrupt event
            pub fn unlisten(&mut self) {
                let usart = unsafe { &(*<$USARTX>::PTR) };
                usart.cr1().modify(|_, w| w.txeie().clear_bit());
                let _ = usart.cr1().read();
                let _ = usart.cr1().read(); // Delay 2 peripheral clocks
            }

            /// Return true if the tx register is empty (and can accept data)
            pub fn is_txe(&self) -> bool {
                let usart = unsafe { &(*<$USARTX>::PTR) };
                usart.isr().read().txe().bit_is_set()
            }

            /// Returns true if the tx fifo threshold has been reached.
            pub fn fifo_threshold_reached(&self) -> bool {
                let usart = unsafe { &(*<$USARTX>::PTR) };
                usart.isr().read().txft().bit_is_set()
            }

            pub fn flush(&mut self) -> nb::Result<(), Error> {
                let usart = unsafe { &(*<$USARTX>::PTR) };
                if usart.isr().read().tc().bit_is_set() {
                    Ok(())
                } else {
                    Err(nb::Error::WouldBlock)
                }
            }

            pub fn write(&mut self, byte: u8) -> nb::Result<(), Error> {
                let usart = unsafe { &(*<$USARTX>::PTR) };
                if usart.isr().read().txe().bit_is_set() {
                    usart.tdr().write(|w| unsafe { w.bits(byte as u32) });
                    Ok(())
                } else {
                    Err(nb::Error::WouldBlock)
                }
            }
        }

        impl Serial<$USARTX> {
            pub fn read(&mut self) -> nb::Result<u8, Error> {
                self.rx.read()
            }

            pub fn flush(&mut self) -> nb::Result<(), Error> {
                self.tx.flush()
            }

            pub fn write(&mut self, byte: u8) -> nb::Result<(), Error> {
                self.tx.write(byte)
            }

            /// Separates the serial struct into separate channel objects for sending (Tx) and
            /// receiving (Rx)
            pub fn split(self) -> (Tx<$USARTX>, Rx<$USARTX>) {
                (self.tx, self.rx)
            }

            /// Joins the objects created by `split()` back into one Serial object.
            ///
            /// This function can be used in combination with `release()` to deinitialize the
            /// peripheral after it has been split.
            pub fn join(tx: Tx<$USARTX>, rx: Rx<$USARTX>) -> Self {
                assert_eq!(core::mem::size_of::<$USARTX>(), 0);
                let usart = unsafe { core::mem::zeroed::<$USARTX>() };
                Serial { usart, tx, rx }
            }

            /// Disables the USART and returns the peripheral as well the pins.
            ///
            /// This function makes the components available for further use. For example, the
            /// USART can later be reinitialized with a different baud rate or other configuration
            /// changes.
            pub fn release(self) -> $USARTX {
                // Disable the UART as well as its clock.
                self.usart.cr1().modify(|_, w| w.ue().clear_bit());
                unsafe {
                    let rcc_ptr = &(*pac::Rcc::PTR);
                    <$USARTX>::disable(rcc_ptr);
                }

                self.usart
            }

            /// Returns a reference to the inner peripheral
            pub fn inner(&self) -> &$USARTX {
                &self.usart
            }

            /// Returns a mutable reference to the inner peripheral
            pub fn inner_mut(&mut self) -> &mut $USARTX {
                &mut self.usart
            }

            /// Starts listening for an interrupt event
            pub fn listen(&mut self, event: Event) {
                match event {
                    Event::Rxne => self.usart.cr1().modify(|_, w| w.rxneie().set_bit()),
                    Event::Txe => self.usart.cr1().modify(|_, w| w.txeie().set_bit()),
                    Event::Idle => self.usart.cr1().modify(|_, w| w.idleie().set_bit()),
                    Event::Txftie => self.usart.cr3().modify(|_, w| w.txftie().set_bit()),
                    Event::Rxftie => self.usart.cr3().modify(|_, w| w.rxftie().set_bit()),
                }
            }

            /// Stop listening for an interrupt event
            pub fn unlisten(&mut self, event: Event) {
                match event {
                    Event::Rxne => self.usart.cr1().modify(|_, w| w.rxneie().clear_bit()),
                    Event::Txe => self.usart.cr1().modify(|_, w| w.txeie().clear_bit()),
                    Event::Idle => self.usart.cr1().modify(|_, w| w.idleie().clear_bit()),
                    Event::Txftie => self.usart.cr3().modify(|_, w| w.txftie().clear_bit()),
                    Event::Rxftie => self.usart.cr3().modify(|_, w| w.rxftie().clear_bit()),
                }
                let _ = self.usart.cr1().read();
                let _ = self.usart.cr1().read(); // Delay 2 peripheral clocks
            }

            /// Return true if the line idle status is set
            ///
            /// The line idle status bit is set when the peripheral detects the receive line is idle.
            /// The bit is cleared by software, by calling `clear_idle()`.
            pub fn is_idle(&self) -> bool {
                unsafe { (*<$USARTX>::PTR).isr().read().idle().bit_is_set() }
            }

            /// Clear the line idle status bit
            pub fn clear_idle(&mut self) {
                unsafe { (*<$USARTX>::PTR).icr().write(|w| w.idlecf().set_bit()) }
                let _ = self.usart.isr().read();
                let _ = self.usart.isr().read(); // Delay 2 peripheral clocks
            }

            /// Return true if the line busy status is set
            ///
            /// The busy status bit is set when there is communication active on the receive line,
            /// and reset at the end of reception.
            pub fn is_busy(&self) -> bool {
                unsafe { (*<$USARTX>::PTR).isr().read().busy().bit_is_set() }
            }

            /// Return true if the tx register is empty (and can accept data)
            pub fn is_txe(&self) -> bool {
                unsafe { (*<$USARTX>::PTR).isr().read().txe().bit_is_set() }
            }

            /// Return true if the rx register is not empty (and can be read)
            pub fn is_rxne(&self) -> bool {
                unsafe { (*<$USARTX>::PTR).isr().read().rxne().bit_is_set() }
            }
        }
    };
}

pub trait SerialExt: Sized {
    fn usart<PINS>(
        self,
        pins: PINS,
        config: config::Config,
        clocks: &Clocks,
    ) -> Result<Serial<Self>, config::InvalidConfig>
    where
        PINS: Pins<Self>;
}

macro_rules! usart_hal {
    ($USARTX:ty, $usartX:ident) => {
        impl SerialExt for $USARTX {
            fn usart<PINS>(
                self,
                pins: PINS,
                config: config::Config,
                clocks: &Clocks,
            ) -> Result<Serial<Self>, config::InvalidConfig>
            where
                PINS: Pins<Self>,
            {
                $usartX(self, pins, config, clocks)
            }
        }

        pub fn $usartX<PINS>(
            usart: $USARTX,
            _pins: PINS,
            config: config::Config,
            clocks: &Clocks,
        ) -> Result<Serial<$USARTX>, config::InvalidConfig>
        where
            PINS: Pins<$USARTX>,
        {
            unsafe {
                let rcc_ptr = &(*pac::Rcc::PTR);
                <$USARTX>::enable(rcc_ptr);
                <$USARTX>::reset(rcc_ptr);
            }

            // TODO: By default, all UARTs are clocked from PCLK. We could modify RCC_CCIPR to
            // try SYSCLK if PCLK is not high enough. We could also select 8x oversampling
            // instead of 16x.

            // let sync = PINS::SYNC;

            let clk = <$USARTX>::timer_clock(clocks);
            usart.presc().reset();

            // Calculate baudrate divisor
            let div = clk / config.baudrate;
            assert!(div <= 65_536);

            if div < 16 {
                // We need 16x oversampling.
                return Err(config::InvalidConfig);
            }

            usart.brr().write(|w| unsafe { w.bits(div) });

            // Reset the UART and disable it (UE=0)
            usart.cr1().reset();
            usart.cr2().reset();
            usart.cr3().reset();

            usart.cr2().write(|w| unsafe {
                w.stop()
                    .bits(config.stopbits.bits())
                    .swap()
                    .bit(config.swaptxrx)
                    .rxinv()
                    .bit(config.invertrx)
                    .txinv()
                    .bit(config.inverttx)
            });

            if let Some(timeout) = config.receiver_timeout {
                usart.cr1().write(|w| w.rtoie().set_bit());
                usart.cr2().modify(|_, w| w.rtoen().set_bit());
                usart.rtor().write(|w| unsafe { w.rto().bits(timeout) });
            }

            usart.cr3().write(|w| unsafe {
                w.txftcfg()
                    .bits(config.tx_fifo_threshold.bits())
                    .rxftcfg()
                    .bits(config.rx_fifo_threshold.bits())
                    .txftie()
                    .bit(config.tx_fifo_interrupt)
                    .rxftie()
                    .bit(config.rx_fifo_interrupt)
            });

            // Enable the UART and perform remaining configuration.
            usart.cr1().modify(|_, w| {
                w.ue()
                    .set_bit()
                    .te()
                    .set_bit()
                    .re()
                    .set_bit()
                    .m0()
                    .bit(config.wordlength == config::WordLength::DataBits7)
                    .m1()
                    .bit(config.wordlength == config::WordLength::DataBits9)
                    .pce()
                    .bit(config.parity != config::Parity::ParityNone)
                    .ps()
                    .bit(config.parity == config::Parity::ParityOdd)
                    .fifoen()
                    .bit(config.fifo_enable)
            });

            Ok(Serial {
                usart,
                tx: Tx {
                    _usart: PhantomData,
                },
                rx: Rx {
                    _usart: PhantomData,
                },
            })
        }

        uart_hal_shared!($USARTX);
    };
}

usart_hal!(pac::Usart1, usart1);

usart_hal!(pac::Usart2, usart2);

usart_hal!(pac::Usart3, usart3);

usart_hal!(pac::Uart4, uart4);

#[cfg(feature = "uart5")]
usart_hal!(pac::Uart5, uart5);

pub trait LPSerialExt: Sized {
    fn usart<PINS>(
        self,
        pins: PINS,
        config: config::LPConfig,
        clocks: &Clocks,
    ) -> Result<Serial<Self>, config::InvalidConfig>
    where
        PINS: Pins<Self>;
}

macro_rules! lpusart_hal {
    ($USARTX:ty, $usartX:ident) => {
        impl LPSerialExt for $USARTX {
            fn usart<PINS>(
                self,
                pins: PINS,
                config: config::LPConfig,
                clocks: &Clocks,
            ) -> Result<Serial<Self>, config::InvalidConfig>
            where
                PINS: Pins<Self>,
            {
                $usartX(self, pins, config, clocks)
            }
        }

        pub fn $usartX<PINS>(
            usart: $USARTX,
            _pins: PINS,
            config: config::LPConfig,
            clocks: &Clocks,
        ) -> Result<Serial<$USARTX>, config::InvalidConfig>
        where
            PINS: Pins<$USARTX>,
        {
            unsafe {
                let rcc_ptr = &(*pac::Rcc::PTR);
                <$USARTX>::enable(rcc_ptr);
                <$USARTX>::reset(rcc_ptr);
            }

            // TODO: By default, all UARTs are clocked from PCLK. We could modify RCC_CCIPR to
            // try SYSCLK if PCLK is not high enough. We could also select 8x oversampling
            // instead of 16x.

            // let sync = PINS::SYNC;

            let clk = <$USARTX>::timer_clock(clocks);
            usart.presc().reset();

            // Calculate baudrate divisor
            let div = clk / config.baudrate;
            assert!(div <= 65_536);

            if div < 16 {
                // We need 16x oversampling.
                return Err(config::InvalidConfig);
            }

            usart.brr().write(|w| unsafe { w.bits(div) });

            // Reset the UART and disable it (UE=0)
            usart.cr1().reset();
            usart.cr2().reset();
            usart.cr3().reset();

            usart.cr2().write(|w| unsafe {
                w.stop()
                    .bits(config.stopbits.bits())
                    .swap()
                    .bit(config.swaptxrx)
                    .rxinv()
                    .bit(config.invertrx)
                    .txinv()
                    .bit(config.inverttx)
            });

            usart.cr3().write(|w| unsafe {
                w.txftcfg()
                    .bits(config.tx_fifo_threshold.bits())
                    .rxftcfg()
                    .bits(config.rx_fifo_threshold.bits())
                    .txftie()
                    .bit(config.tx_fifo_interrupt)
                    .rxftie()
                    .bit(config.rx_fifo_interrupt)
            });

            // Enable the UART and perform remaining configuration.
            usart.cr1().modify(|_, w| {
                w.ue()
                    .set_bit()
                    .te()
                    .set_bit()
                    .re()
                    .set_bit()
                    .m0()
                    .bit(config.wordlength == config::WordLength::DataBits7)
                    .m1()
                    .bit(config.wordlength == config::WordLength::DataBits9)
                    .pce()
                    .bit(config.parity != config::Parity::ParityNone)
                    .ps()
                    .bit(config.parity == config::Parity::ParityOdd)
                    .fifoen()
                    .bit(config.fifo_enable)
            });

            Ok(Serial {
                usart,
                tx: Tx {
                    _usart: PhantomData,
                },
                rx: Rx {
                    _usart: PhantomData,
                },
            })
        }

        uart_hal_shared!($USARTX);
    };
}

lpusart_hal!(pac::Lpuart1, lpuart1);

macro_rules! usart_pins {
    ($USARTX:ty: {
        TX: [$($( #[ $pmeta1:meta ] )* $TX:ty),+ $(,)*]
        RX: [$($( #[ $pmeta2:meta ] )* $RX:ty),+ $(,)*]
        CK: [$($( #[ $pmeta3:meta ] )* $CK:ty),+ $(,)*]
    }) => {
        $(
            $( #[ $pmeta1 ] )*
            impl TXPin<$USARTX> for $TX {}
        )*

        $(
            $( #[ $pmeta2 ] )*
            impl RXPin<$USARTX> for $RX {}
        )*

        $(
            $( #[ $pmeta3 ] )*
            impl CKPin<$USARTX> for $CK {}
        )*
    }
}

macro_rules! uart_pins {
    ($UARTX:ty: {
       TX: [$($( #[ $pmeta1:meta ] )* $TX:ty),+ $(,)*]
       RX: [$($( #[ $pmeta2:meta ] )* $RX:ty),+ $(,)*]
    }) => {
        $(
            $( #[ $pmeta1 ] )*
            impl TXPin<$UARTX> for $TX {}
        )*
        $(
            $( #[ $pmeta2 ] )*
            impl RXPin<$UARTX> for $RX {}
        )*
    }
}

usart_pins!(
    pac::Usart1: {
        TX: [
            PA9<AF7>,
            PB6<AF7>,
            PC4<AF7>,
            PE0<AF7>,
            #[cfg(any(
                feature = "stm32g471",
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484"
            ))]
            PG9<AF7>
        ]
        RX: [
            PA10<AF7>,
            PB7<AF7>,
            PC5<AF7>,
            PE1<AF7>,
        ]
        CK: [
            PA8<AF7>
        ]
    }
);

usart_pins!(
    pac::Usart2: {
        TX: [
            PA2<AF7>,
            PA14<AF7>,
            PB3<AF7>,
            PD5<AF7>
        ]
        RX: [
            PA3<AF7>,
            PA15<AF7>,
            PB3<AF7>,
            PD6<AF7>
        ]
        CK: [
            PA4<AF7>,
            PB5<AF7>,
            PD7<AF7>
        ]
    }
);

usart_pins!(
    pac::Usart3: {
        TX: [
            PB9<AF7>,
            PB10<AF7>,
            PC10<AF7>,
            PD8<AF7>
        ]
        RX: [
            PB8<AF7>,
            PB11<AF7>,
            PC11<AF7>,
            PD9<AF7>,
            PE15<AF7>
        ]
        CK: [
            PB12<AF7>,
            PC12<AF7>,
            PD10<AF7>,
        ]
    }
);

uart_pins!(
    pac::Uart4: {
        TX: [PC10<AF5>]
        RX: [PC11<AF5>]
    }
);

#[cfg(feature = "uart5")]
uart_pins!(
    pac::Uart5: {
        TX: [PC12<AF5>]
        RX: [PD2<AF5>]
    }
);

uart_pins!(
    pac::Lpuart1: {
        TX: [
            PA2<AF12>,
            PB11<AF8>,
            PC1<AF8>,
            #[cfg(any(
                feature = "stm32g471",
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484"
            ))]
            PG7<AF8>,
        ]
        RX: [
            PA3<AF12>,
            PB10<AF8>,
            PC0<AF8>,
            #[cfg(any(
                feature = "stm32g471",
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484"
            ))]
            PG8<AF8>,
        ]
    }
);

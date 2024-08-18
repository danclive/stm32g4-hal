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

pub trait TXPin<USART> {}

pub trait RXPin<USART> {}

pub trait CKPin<USART> {}

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
    ($UARTX:ident) => {
        impl Rx<pac::$UARTX> {
            /// Starts listening for an interrupt event
            pub fn listen(&mut self) {
                let usart = unsafe { &(*<pac::$UARTX>::PTR) };
                usart.cr1().modify(|_, w| w.rxneie().set_bit());
            }

            /// Stop listening for an interrupt event
            pub fn unlisten(&mut self) {
                let usart = unsafe { &(*<pac::$UARTX>::PTR) };
                usart.cr1().modify(|_, w| w.rxneie().clear_bit());
                let _ = usart.cr1().read();
                let _ = usart.cr1().read(); // Delay 2 peripheral clocks
            }

            /// Return true if the line idle status is set
            ///
            /// The line idle status bit is set when the peripheral detects the receive line is idle.
            /// The bit is cleared by software, by calling `clear_idle()`.
            pub fn is_idle(&self) -> bool {
                let usart = unsafe { &(*<pac::$UARTX>::PTR) };
                usart.isr().read().idle().bit_is_set()
            }

            /// Clear the line idle status bit
            pub fn clear_idle(&mut self) {
                let usart = unsafe { &(*<pac::$UARTX>::PTR) };
                usart.icr().write(|w| w.idlecf().set_bit());
                let _ = usart.isr().read();
                let _ = usart.isr().read(); // Delay 2 peripheral clocks
            }

            /// Return true if the line busy status is set
            ///
            /// The busy status bit is set when there is communication active on the receive line,
            /// and reset at the end of reception.
            pub fn is_busy(&self) -> bool {
                let usart = unsafe { &(*<pac::$UARTX>::PTR) };
                usart.isr().read().busy().bit_is_set()
            }

            /// Return true if the rx register is not empty (and can be read)
            pub fn is_rxne(&self) -> bool {
                let usart = unsafe { &(*<pac::$UARTX>::PTR) };
                usart.isr().read().rxne().bit_is_set()
            }

            /// Returns true if the rx fifo threshold has been reached.
            pub fn fifo_threshold_reached(&self) -> bool {
                let usart = unsafe { &(*<pac::$UARTX>::PTR) };
                usart.isr().read().rxft().bit_is_set()
            }

            pub fn read(&mut self) -> nb::Result<u8, Error> {
                let usart = unsafe { &(*<pac::$UARTX>::PTR) };
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

        impl Tx<pac::$UARTX> {
            /// Starts listening for an interrupt event
            pub fn listen(&mut self) {
                let usart = unsafe { &(*<pac::$UARTX>::PTR) };
                usart.cr1().modify(|_, w| w.txeie().set_bit());
            }

            /// Stop listening for an interrupt event
            pub fn unlisten(&mut self) {
                let usart = unsafe { &(*<pac::$UARTX>::PTR) };
                usart.cr1().modify(|_, w| w.txeie().clear_bit());
                let _ = usart.cr1().read();
                let _ = usart.cr1().read(); // Delay 2 peripheral clocks
            }

            /// Return true if the tx register is empty (and can accept data)
            pub fn is_txe(&self) -> bool {
                let usart = unsafe { &(*<pac::$UARTX>::PTR) };
                usart.isr().read().txe().bit_is_set()
            }

            /// Returns true if the tx fifo threshold has been reached.
            pub fn fifo_threshold_reached(&self) -> bool {
                let usart = unsafe { &(*<pac::$UARTX>::PTR) };
                usart.isr().read().txft().bit_is_set()
            }

            pub fn flush(&mut self) -> nb::Result<(), Error> {
                let usart = unsafe { &(*<pac::$UARTX>::PTR) };
                if usart.isr().read().tc().bit_is_set() {
                    Ok(())
                } else {
                    Err(nb::Error::WouldBlock)
                }
            }

            pub fn write(&mut self, byte: u8) -> nb::Result<(), Error> {
                let usart = unsafe { &(*<pac::$UARTX>::PTR) };
                if usart.isr().read().txe().bit_is_set() {
                    usart.tdr().write(|w| unsafe { w.bits(byte as u32) });
                    Ok(())
                } else {
                    Err(nb::Error::WouldBlock)
                }
            }
        }

        impl Serial<pac::$UARTX> {
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
            pub fn split(self) -> (Tx<pac::$UARTX>, Rx<pac::$UARTX>) {
                (self.tx, self.rx)
            }

            /// Joins the objects created by `split()` back into one Serial object.
            ///
            /// This function can be used in combination with `release()` to deinitialize the
            /// peripheral after it has been split.
            pub fn join(tx: Tx<pac::$UARTX>, rx: Rx<pac::$UARTX>) -> Self {
                assert_eq!(core::mem::size_of::<pac::$UARTX>(), 0);
                let usart = unsafe { core::mem::zeroed::<pac::$UARTX>() };
                Serial { usart, tx, rx }
            }

            /// Disables the USART and returns the peripheral as well the pins.
            ///
            /// This function makes the components available for further use. For example, the
            /// USART can later be reinitialized with a different baud rate or other configuration
            /// changes.
            pub fn release(self) -> pac::$UARTX {
                // Disable the UART as well as its clock.
                self.usart.cr1().modify(|_, w| w.ue().clear_bit());
                unsafe {
                    let rcc_ptr = &(*pac::Rcc::PTR);
                    <pac::$UARTX>::disable(rcc_ptr);
                }

                self.usart
            }

            /// Returns a reference to the inner peripheral
            pub fn inner(&self) -> &pac::$UARTX {
                &self.usart
            }

            /// Returns a mutable reference to the inner peripheral
            pub fn inner_mut(&mut self) -> &mut pac::$UARTX {
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
                unsafe { (*<pac::$UARTX>::PTR).isr().read().idle().bit_is_set() }
            }

            /// Clear the line idle status bit
            pub fn clear_idle(&mut self) {
                unsafe { (*<pac::$UARTX>::PTR).icr().write(|w| w.idlecf().set_bit()) }
                let _ = self.usart.isr().read();
                let _ = self.usart.isr().read(); // Delay 2 peripheral clocks
            }

            /// Return true if the line busy status is set
            ///
            /// The busy status bit is set when there is communication active on the receive line,
            /// and reset at the end of reception.
            pub fn is_busy(&self) -> bool {
                unsafe { (*<pac::$UARTX>::PTR).isr().read().busy().bit_is_set() }
            }

            /// Return true if the tx register is empty (and can accept data)
            pub fn is_txe(&self) -> bool {
                unsafe { (*<pac::$UARTX>::PTR).isr().read().txe().bit_is_set() }
            }

            /// Return true if the rx register is not empty (and can be read)
            pub fn is_rxne(&self) -> bool {
                unsafe { (*<pac::$UARTX>::PTR).isr().read().rxne().bit_is_set() }
            }
        }
    };
}

pub trait UartExt<USART>: Sized {
    fn usart<TX, RX>(
        self,
        pins: (TX, RX),
        config: config::Config,
        clocks: &Clocks,
    ) -> Serial<USART>
    where
        TX: TXPin<USART>,
        RX: RXPin<USART>;
}

macro_rules! uart_hal {
    ($UARTX:ident, $uartX:ident) => {
        impl UartExt<pac::$UARTX> for pac::$UARTX {
            fn usart<TX, RX>(
                self,
                pins: (TX, RX),
                config: config::Config,
                clocks: &Clocks,
            ) -> Serial<Self>
            where
                TX: TXPin<pac::$UARTX>,
                RX: RXPin<pac::$UARTX>,
            {
                $uartX(self, pins, config, clocks)
            }
        }

        pub fn $uartX<TX, RX>(
            usart: pac::$UARTX,
            _pins: (TX, RX),
            config: config::Config,
            clocks: &Clocks,
        ) -> Serial<pac::$UARTX>
        where
            TX: TXPin<pac::$UARTX>,
            RX: RXPin<pac::$UARTX>,
        {
            unsafe {
                let rcc_ptr = &(*pac::Rcc::PTR);
                <pac::$UARTX>::enable(rcc_ptr);
                <pac::$UARTX>::reset(rcc_ptr);
            }

            // TODO: By default, all UARTs are clocked from PCLK. We could modify RCC_CCIPR to
            // try SYSCLK if PCLK is not high enough. We could also select 8x oversampling
            // instead of 16x.

            // let sync = PINS::SYNC;

            let clk = <pac::$UARTX>::timer_clock(clocks);
            usart.presc().reset();

            // Calculate baudrate divisor
            let div = clk / config.baudrate;
            assert!(div <= 65_536);

            if div < 16 {
                // We need 16x oversampling.
                panic!("16x oversampling required");
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

            Serial {
                usart,
                tx: Tx {
                    _usart: PhantomData,
                },
                rx: Rx {
                    _usart: PhantomData,
                },
            }
        }

        uart_hal_shared!($UARTX);
    };
}

uart_hal!(Usart1, usart1);

uart_hal!(Usart2, usart2);

uart_hal!(Usart3, usart3);

uart_hal!(Uart4, uart4);

#[cfg(feature = "uart5")]
uart_hal!(Uart5, uart5);

pub trait LPUartExt<UART>: Sized {
    fn usart<TX, RX>(
        self,
        pins: (TX, RX),
        config: config::LPConfig,
        clocks: &Clocks,
    ) -> Serial<Self>
    where
        TX: TXPin<UART>,
        RX: RXPin<UART>;
}

macro_rules! lpusart_hal {
    ($UARTX:ident, $uartX:ident) => {
        impl LPUartExt<pac::$UARTX> for pac::$UARTX {
            fn usart<TX, RX>(
                self,
                pins: (TX, RX),
                config: config::LPConfig,
                clocks: &Clocks,
            ) -> Serial<Self>
            where
                TX: TXPin<pac::$UARTX>,
                RX: RXPin<pac::$UARTX>,
            {
                $uartX(self, pins, config, clocks)
            }
        }

        pub fn $uartX<TX, RX>(
            usart: pac::$UARTX,
            _pins: (TX, RX),
            config: config::LPConfig,
            clocks: &Clocks,
        ) -> Serial<pac::$UARTX>
        where
            TX: TXPin<pac::$UARTX>,
            RX: RXPin<pac::$UARTX>,
        {
            unsafe {
                let rcc_ptr = &(*pac::Rcc::PTR);
                <pac::$UARTX>::enable(rcc_ptr);
                <pac::$UARTX>::reset(rcc_ptr);
            }

            // TODO: By default, all UARTs are clocked from PCLK. We could modify RCC_CCIPR to
            // try SYSCLK if PCLK is not high enough. We could also select 8x oversampling
            // instead of 16x.

            let clk = <pac::$UARTX>::timer_clock(clocks);
            usart.presc().reset();

            // Calculate baudrate divisor
            let div = clk / config.baudrate;
            assert!(div <= 65_536);

            if div < 16 {
                // We need 16x oversampling.
                panic!("16x oversampling required");
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

            Serial {
                usart,
                tx: Tx {
                    _usart: PhantomData,
                },
                rx: Rx {
                    _usart: PhantomData,
                },
            }
        }

        uart_hal_shared!($UARTX);
    };
}

lpusart_hal!(Lpuart1, lpuart1);

macro_rules! usart_pins {
    ($USARTX:ident: {
        TX: [$($( #[ $pmeta1:meta ] )* $TX:ty),+ $(,)*]
        RX: [$($( #[ $pmeta2:meta ] )* $RX:ty),+ $(,)*]
        CK: [$($( #[ $pmeta3:meta ] )* $CK:ty),+ $(,)*]
    }) => {
        $(
            $( #[ $pmeta1 ] )*
            impl TXPin<pac::$USARTX> for $TX {}
        )*

        $(
            $( #[ $pmeta2 ] )*
            impl RXPin<pac::$USARTX> for $RX {}
        )*

        $(
            $( #[ $pmeta3 ] )*
            impl CKPin<pac::$USARTX> for $CK {}
        )*
    }
}

macro_rules! uart_pins {
    ($UARTX:ident: {
       TX: [$($( #[ $pmeta1:meta ] )* $TX:ty),+ $(,)*]
       RX: [$($( #[ $pmeta2:meta ] )* $RX:ty),+ $(,)*]
    }) => {
        $(
            $( #[ $pmeta1 ] )*
            impl TXPin<pac::$UARTX> for $TX {}
        )*
        $(
            $( #[ $pmeta2 ] )*
            impl RXPin<pac::$UARTX> for $RX {}
        )*
    }
}

usart_pins!(
    Usart1: {
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
    Usart2: {
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
    Usart3: {
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
    Uart4: {
        TX: [PC10<AF5>]
        RX: [PC11<AF5>]
    }
);

#[cfg(feature = "uart5")]
uart_pins!(
    Uart5: {
        TX: [PC12<AF5>]
        RX: [PD2<AF5>]
    }
);

uart_pins!(
    Lpuart1: {
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

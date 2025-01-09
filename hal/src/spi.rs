//! SPI

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::ptr;

use enumflags2::BitFlags;
use fugit::HertzU32 as Hertz;

use crate::pac;

use crate::gpio::*;
use crate::rcc::Clocks;
use crate::rcc::{BusTimerClock, Enable, Reset};
use crate::{block, nb};

pub trait SCKPin<SPI> {}

pub trait MISOPin<SPI> {}

pub trait MOSIPin<SPI> {}

pub trait NSSPin<SPI> {}

/// A filler type for when the SCK pin is unnecessary
pub type NoSck = NoPin;
/// A filler type for when the Miso pin is unnecessary
pub type NoMiso = NoPin;
/// A filler type for when the Mosi pin is unnecessary
pub type NoMosi = NoPin;

impl<SPI> SCKPin<SPI> for NoSck {}

impl<SPI> MISOPin<SPI> for NoMiso {}

impl<SPI> MOSIPin<SPI> for NoMosi {}

impl<SPI> NSSPin<SPI> for NoPin {}

/// SPI interrupt events
#[enumflags2::bitflags]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum Event {
    /// An error occurred.
    ///
    /// This bit controls the generation of an interrupt
    /// when an error condition occurs
    /// (OVR, CRCERR, MODF, FRE in SPI mode,
    /// and UDR, OVR, FRE in I2S mode)
    Error = 1 << 5,
    /// New data has been received
    ///
    /// RX buffer not empty interrupt enable
    RxNotEmpty = 1 << 6,
    /// Data can be sent
    ///
    /// Tx buffer empty interrupt enable
    TxEmpty = 1 << 7,
}

/// SPI status flags
#[enumflags2::bitflags]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum Flag {
    /// Receive buffer not empty
    RxNotEmpty = 1 << 0,
    /// Transmit buffer empty
    TxEmpty = 1 << 1,
    /// CRC error flag
    CrcError = 1 << 4,
    /// Mode fault
    ModeFault = 1 << 5,
    /// Overrun flag
    Overrun = 1 << 6,
    /// Busy flag
    Busy = 1 << 7,
    /// Frame Error
    FrameError = 1 << 8,
}

/// SPI error
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum Error {
    /// Overrun occurred
    Overrun,
    /// Mode fault occurred
    ModeFault,
    /// CRC error
    Crc,
}

/// Clock polarity
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    /// Clock signal low when idle
    IdleLow,
    /// Clock signal high when idle
    IdleHigh,
}

/// Clock phase
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// Data in "captured" on the first clock transition
    CaptureOnFirstTransition,
    /// Data in "captured" on the second clock transition
    CaptureOnSecondTransition,
}

/// SPI mode
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mode {
    /// Clock polarity
    pub polarity: Polarity,
    /// Clock phase
    pub phase: Phase,
}

/// Helper for CPOL = 0, CPHA = 0
pub const MODE_0: Mode = Mode {
    polarity: Polarity::IdleLow,
    phase: Phase::CaptureOnFirstTransition,
};

/// Helper for CPOL = 0, CPHA = 1
pub const MODE_1: Mode = Mode {
    polarity: Polarity::IdleLow,
    phase: Phase::CaptureOnSecondTransition,
};

/// Helper for CPOL = 1, CPHA = 0
pub const MODE_2: Mode = Mode {
    polarity: Polarity::IdleHigh,
    phase: Phase::CaptureOnFirstTransition,
};

/// Helper for CPOL = 1, CPHA = 1
pub const MODE_3: Mode = Mode {
    polarity: Polarity::IdleHigh,
    phase: Phase::CaptureOnSecondTransition,
};

/// The bit format to send the data in
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitFormat {
    /// Least significant bit first
    LsbFirst,
    /// Most significant bit first
    MsbFirst,
}

#[derive(Debug)]
pub struct Spi<SPI, SCK, MISO, MOSI, const BIDI: bool = false> {
    inner: Inner<SPI>,
    pins: (SCK, MISO, MOSI),
}

#[derive(Debug)]
pub struct SpiSlave<SPI, SCK, MISO, MOSI, NSS, const BIDI: bool = false> {
    inner: Inner<SPI>,
    pins: (SCK, MISO, MOSI, Option<NSS>),
}

#[derive(Debug)]
pub struct Inner<SPI> {
    spi: SPI,
}

pub trait SpiExt<SPI>: Sized {
    fn spi<SCK, MISO, MOSI>(
        self,
        pins: (SCK, MISO, MOSI),
        mode: Mode,
        clocks: &Clocks,
        freq: impl Into<Hertz>,
    ) -> Spi<SPI, SCK, MISO, MOSI>
    where
        SCK: SCKPin<SPI>,
        MISO: MISOPin<SPI>,
        MOSI: MOSIPin<SPI>;

    fn spi_bidi<SCK, MOSI>(
        self,
        pins: (SCK, MOSI),
        mode: Mode,
        clocks: &Clocks,
        freq: impl Into<Hertz>,
    ) -> Spi<SPI, SCK, NoMiso, MOSI, true>
    where
        SCK: SCKPin<SPI>,
        MOSI: MOSIPin<SPI>;

    fn spi_slave<SCK, MISO, MOSI, NSS>(
        self,
        pins: (SCK, MISO, MOSI, Option<NSS>),
        mode: Mode,
    ) -> SpiSlave<SPI, SCK, MISO, MOSI, NSS>
    where
        SCK: SCKPin<SPI>,
        MISO: MISOPin<SPI>,
        MOSI: MOSIPin<SPI>,
        NSS: NSSPin<SPI>;

    fn spi_slave_bidi<SCK, MISO, NSS>(
        self,
        pins: (SCK, MISO, Option<NSS>),
        mode: Mode,
    ) -> SpiSlave<SPI, SCK, MISO, NoMosi, NSS, true>
    where
        SCK: SCKPin<SPI>,
        MISO: MISOPin<SPI>,
        NSS: NSSPin<SPI>;
}

macro_rules! spi_hal {
    ($SPIX:ident, $spiX:ident, $spiX_slave:ident) => {
        impl SpiExt<pac::$SPIX> for pac::$SPIX {
            fn spi<SCK, MISO, MOSI>(
                self,
                pins: (SCK, MISO, MOSI),
                mode: Mode,
                clocks: &Clocks,
                freq: impl Into<Hertz>,
            ) -> Spi<pac::$SPIX, SCK, MISO, MOSI>
            where
                SCK: SCKPin<pac::$SPIX>,
                MISO: MISOPin<pac::$SPIX>,
                MOSI: MOSIPin<pac::$SPIX>,
            {
                $spiX(self, pins, mode, clocks, freq)
            }

            fn spi_bidi<SCK, MOSI>(
                self,
                pins: (SCK, MOSI),
                mode: Mode,
                clocks: &Clocks,
                freq: impl Into<Hertz>,
            ) -> Spi<pac::$SPIX, SCK, NoMiso, MOSI, true>
            where
                SCK: SCKPin<pac::$SPIX>,
                MOSI: MOSIPin<pac::$SPIX>,
            {
                $spiX(self, (pins.0, NoPin::new(), pins.1), mode, clocks, freq)
            }

            fn spi_slave<SCK, MISO, MOSI, NSS>(
                self,
                pins: (SCK, MISO, MOSI, Option<NSS>),
                mode: Mode,
            ) -> SpiSlave<pac::$SPIX, SCK, MISO, MOSI, NSS>
            where
                SCK: SCKPin<pac::$SPIX>,
                MISO: MISOPin<pac::$SPIX>,
                MOSI: MOSIPin<pac::$SPIX>,
                NSS: NSSPin<pac::$SPIX>,
            {
                $spiX_slave(self, pins, mode)
            }

            fn spi_slave_bidi<SCK, MISO, NSS>(
                self,
                pins: (SCK, MISO, Option<NSS>),
                mode: Mode,
            ) -> SpiSlave<pac::$SPIX, SCK, MISO, NoMosi, NSS, true>
            where
                SCK: SCKPin<pac::$SPIX>,
                MISO: MISOPin<pac::$SPIX>,
                NSS: NSSPin<pac::$SPIX>,
            {
                $spiX_slave(self, (pins.0, pins.1, NoPin::new(), pins.2), mode)
            }
        }

        pub fn $spiX<SCK, MISO, MOSI, const BIDI: bool>(
            spi: pac::$SPIX,
            pins: (SCK, MISO, MOSI),
            mode: Mode,
            clocks: &Clocks,
            freq: impl Into<Hertz>,
        ) -> Spi<pac::$SPIX, SCK, MISO, MOSI, BIDI>
        where
            SCK: SCKPin<pac::$SPIX>,
            MISO: MISOPin<pac::$SPIX>,
            MOSI: MOSIPin<pac::$SPIX>,
        {
            unsafe {
                let rcc_ptr = &(*pac::Rcc::PTR);
                <pac::$SPIX>::enable(rcc_ptr);
                <pac::$SPIX>::reset(rcc_ptr);
            }

            // disable SS output
            spi.cr2().write(|w| w.ssoe().clear_bit());

            let freq = freq.into().raw();
            let clk = <pac::$SPIX>::timer_clock(clocks).raw();

            let br = match clk / freq {
                0 => unreachable!(),
                1..=2 => 0b000,
                3..=5 => 0b001,
                6..=11 => 0b010,
                12..=23 => 0b011,
                24..=47 => 0b100,
                48..=95 => 0b101,
                96..=191 => 0b110,
                _ => 0b111,
            };

            spi.cr2()
                .write(|w| unsafe { w.frxth().set_bit().ds().bits(0b111).ssoe().clear_bit() });

            spi.cr1().write(|w| unsafe {
                w.cpha()
                    .bit(mode.phase == Phase::CaptureOnSecondTransition)
                    .cpol()
                    .bit(mode.polarity == Polarity::IdleHigh)
                    .mstr()
                    .set_bit()
                    .br()
                    .bits(br)
                    .lsbfirst()
                    .clear_bit()
                    .ssi()
                    .set_bit()
                    .ssm()
                    .set_bit()
                    .rxonly()
                    .clear_bit()
                    .dff()
                    .clear_bit()
                    .bidimode()
                    .bit(BIDI)
                    .bidioe()
                    .bit(BIDI)
                    .spe()
                    .set_bit()
            });

            Spi {
                inner: Inner::<pac::$SPIX>::new(spi),
                pins,
            }
        }

        impl<SCK, MISO, MOSI, const BIDI: bool> Spi<pac::$SPIX, SCK, MISO, MOSI, BIDI>
        where
            SCK: SCKPin<pac::$SPIX>,
            MISO: MISOPin<pac::$SPIX>,
            MOSI: MOSIPin<pac::$SPIX>,
        {
            pub fn read_nonblocking(&mut self) -> nb::Result<u8, Error> {
                if BIDI {
                    self.bidi_input();
                }
                self.check_read()
            }

            pub fn write_nonblocking(&mut self, byte: u8) -> nb::Result<(), Error> {
                if BIDI {
                    self.bidi_output();
                }
                self.check_send(byte)
            }

            pub fn transfer_in_place(&mut self, bytes: &mut [u8]) -> Result<(), Error> {
                for byte in bytes {
                    block!(self.write_nonblocking(*byte))?;
                    *byte = block!(self.read_nonblocking())?;
                }

                Ok(())
            }

            pub fn transfer(&mut self, buff: &mut [u8], data: &[u8]) -> Result<(), Error> {
                assert_eq!(data.len(), buff.len());

                for (d, b) in data.iter().cloned().zip(buff.iter_mut()) {
                    block!(self.write_nonblocking(d))?;
                    *b = block!(self.read_nonblocking())?;
                }

                Ok(())
            }

            pub fn write(&mut self, bytes: &[u8]) -> Result<(), Error> {
                if BIDI {
                    self.bidi_output();
                    for byte in bytes {
                        block!(self.check_send(*byte))?;
                    }
                } else {
                    for byte in bytes {
                        block!(self.check_send(*byte))?;
                        block!(self.check_read())?;
                    }
                }

                Ok(())
            }

            pub fn write_iter(&mut self, bytes: impl IntoIterator<Item = u8>) -> Result<(), Error> {
                if BIDI {
                    self.bidi_output();
                    for byte in bytes.into_iter() {
                        block!(self.check_send(byte))?;
                    }
                } else {
                    for byte in bytes.into_iter() {
                        block!(self.check_send(byte))?;
                        block!(self.check_read())?;
                    }
                }

                Ok(())
            }

            pub fn read(&mut self, bytes: &mut [u8]) -> Result<(), Error> {
                if BIDI {
                    self.bidi_input();
                    for byte in bytes {
                        *byte = block!(self.check_read())?;
                    }
                } else {
                    for byte in bytes {
                        block!(self.check_send(u8::default()))?;
                        *byte = block!(self.check_read())?;
                    }
                }

                Ok(())
            }

            pub fn release(self) -> (pac::$SPIX, (SCK, MISO, MOSI)) {
                unsafe {
                    let rcc = &(*pac::Rcc::PTR);
                    <pac::$SPIX>::reset(rcc);
                    <pac::$SPIX>::disable(rcc);
                }

                (self.inner.spi, self.pins)
            }
        }

        pub fn $spiX_slave<SCK, MISO, MOSI, NSS, const BIDI: bool>(
            spi: pac::$SPIX,
            pins: (SCK, MISO, MOSI, Option<NSS>),
            mode: Mode,
        ) -> SpiSlave<pac::$SPIX, SCK, MISO, MOSI, NSS, BIDI>
        where
            SCK: SCKPin<pac::$SPIX>,
            MISO: MISOPin<pac::$SPIX>,
            MOSI: MOSIPin<pac::$SPIX>,
            NSS: NSSPin<pac::$SPIX>,
        {
            unsafe {
                let rcc_ptr = &(*pac::Rcc::PTR);
                <pac::$SPIX>::enable(rcc_ptr);
                <pac::$SPIX>::reset(rcc_ptr);
            }

            // disable SS output
            spi.cr2().write(|w| w.ssoe().clear_bit());

            spi.cr2()
                .write(|w| unsafe { w.frxth().set_bit().ds().bits(0b111).ssoe().clear_bit() });

            spi.cr1().write(|w| unsafe {
                w.cpha()
                    .bit(mode.phase == Phase::CaptureOnSecondTransition)
                    .cpol()
                    .bit(mode.polarity == Polarity::IdleHigh)
                    .mstr()
                    .clear_bit()
                    .br()
                    .bits(0)
                    .lsbfirst()
                    .clear_bit()
                    .ssi()
                    .set_bit()
                    .ssm()
                    .bit(pins.3.is_none())
                    .rxonly()
                    .clear_bit()
                    .dff()
                    .clear_bit()
                    .bidimode()
                    .bit(BIDI)
                    .bidioe()
                    .bit(BIDI)
                    .spe()
                    .set_bit()
            });

            SpiSlave {
                inner: Inner::<pac::$SPIX>::new(spi),
                pins,
            }
        }

        impl<SCK, MISO, MOSI, NSS, const BIDI: bool>
            SpiSlave<pac::$SPIX, SCK, MISO, MOSI, NSS, BIDI>
        where
            SCK: SCKPin<pac::$SPIX>,
            MISO: MISOPin<pac::$SPIX>,
            MOSI: MOSIPin<pac::$SPIX>,
            NSS: NSSPin<pac::$SPIX>,
        {
            pub fn read_nonblocking(&mut self) -> nb::Result<u8, Error> {
                if BIDI {
                    self.bidi_input();
                }
                self.check_read()
            }

            pub fn write_nonblocking(&mut self, byte: u8) -> nb::Result<(), Error> {
                if BIDI {
                    self.bidi_output();
                }
                self.check_send(byte)
            }

            pub fn transfer_in_place(&mut self, bytes: &mut [u8]) -> Result<(), Error> {
                for byte in bytes {
                    block!(self.write_nonblocking(*byte))?;
                    *byte = block!(self.read_nonblocking())?;
                }

                Ok(())
            }

            pub fn transfer(&mut self, buff: &mut [u8], data: &[u8]) -> Result<(), Error> {
                assert_eq!(data.len(), buff.len());

                for (d, b) in data.iter().cloned().zip(buff.iter_mut()) {
                    block!(self.write_nonblocking(d))?;
                    *b = block!(self.read_nonblocking())?;
                }

                Ok(())
            }

            pub fn write(&mut self, bytes: &[u8]) -> Result<(), Error> {
                if BIDI {
                    self.bidi_output();
                    for byte in bytes {
                        block!(self.check_send(*byte))?;
                    }
                } else {
                    for byte in bytes {
                        block!(self.check_send(*byte))?;
                        block!(self.check_read())?;
                    }
                }

                Ok(())
            }

            pub fn write_iter(&mut self, bytes: impl IntoIterator<Item = u8>) -> Result<(), Error> {
                if BIDI {
                    self.bidi_output();
                    for byte in bytes.into_iter() {
                        block!(self.check_send(byte))?;
                    }
                } else {
                    for byte in bytes.into_iter() {
                        block!(self.check_send(byte))?;
                        block!(self.check_read())?;
                    }
                }

                Ok(())
            }

            pub fn read(&mut self, bytes: &mut [u8]) -> Result<(), Error> {
                if BIDI {
                    self.bidi_input();
                    for byte in bytes {
                        *byte = block!(self.check_read())?;
                    }
                } else {
                    for byte in bytes {
                        block!(self.check_send(u8::default()))?;
                        *byte = block!(self.check_read())?;
                    }
                }

                Ok(())
            }

            pub fn release(self) -> (pac::$SPIX, (SCK, MISO, MOSI, Option<NSS>)) {
                unsafe {
                    let rcc = &(*pac::Rcc::PTR);
                    <pac::$SPIX>::reset(rcc);
                    <pac::$SPIX>::disable(rcc);
                }

                (self.inner.spi, self.pins)
            }
        }

        impl<SCK, MISO, MOSI, const BIDI: bool> Deref for Spi<pac::$SPIX, SCK, MISO, MOSI, BIDI> {
            type Target = Inner<pac::$SPIX>;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl<SCK, MISO, MOSI, const BIDI: bool> DerefMut
            for Spi<pac::$SPIX, SCK, MISO, MOSI, BIDI>
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        impl<SCK, MISO, MOSI, NSS, const BIDI: bool> Deref
            for SpiSlave<pac::$SPIX, SCK, MISO, MOSI, NSS, BIDI>
        {
            type Target = Inner<pac::$SPIX>;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl<SCK, MISO, MOSI, NSS, const BIDI: bool> DerefMut
            for SpiSlave<pac::$SPIX, SCK, MISO, MOSI, NSS, BIDI>
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        impl Inner<pac::$SPIX> {
            fn new(spi: pac::$SPIX) -> Self {
                Self { spi }
            }

            /// Enable/disable spi
            #[inline(always)]
            pub fn enable(&mut self, enable: bool) {
                self.spi.cr1().modify(|_, w| {
                    // spe: enable the SPI bus
                    w.spe().bit(enable)
                });
            }

            /// Select which frame format is used for data transfers
            #[inline(always)]
            pub fn bit_format(&mut self, format: BitFormat) {
                self.spi
                    .cr1()
                    .modify(|_, w| w.lsbfirst().bit(format == BitFormat::LsbFirst));
            }

            /// Return `true` if the TXE flag is set, i.e. new data to transmit
            /// can be written to the SPI.
            #[inline(always)]
            pub fn is_tx_empty(&self) -> bool {
                self.spi.sr().read().txe().bit_is_set()
            }

            /// Return `true` if the RXNE flag is set, i.e. new data has been received
            /// and can be read from the SPI.
            #[inline(always)]
            pub fn is_rx_not_empty(&self) -> bool {
                self.spi.sr().read().rxne().bit_is_set()
            }

            /// Return `true` if the MODF flag is set, i.e. the SPI has experienced a
            /// Master Mode Fault. (see chapter 28.3.10 of the STM32F4 Reference Manual)
            #[inline(always)]
            pub fn is_modf(&self) -> bool {
                self.spi.sr().read().modf().bit_is_set()
            }

            /// Returns true if the transfer is in progress
            #[inline(always)]
            pub fn is_busy(&self) -> bool {
                self.spi.sr().read().bsy().bit_is_set()
            }

            /// Return `true` if the OVR flag is set, i.e. new data has been received
            /// while the receive data register was already filled.
            #[inline(always)]
            pub fn is_overrun(&self) -> bool {
                self.spi.sr().read().ovr().bit_is_set()
            }

            #[inline(always)]
            fn bidi_output(&mut self) {
                self.spi.cr1().modify(|_, w| w.bidioe().set_bit());
            }

            #[inline(always)]
            fn bidi_input(&mut self) {
                self.spi.cr1().modify(|_, w| w.bidioe().set_bit());
            }

            #[inline(always)]
            fn read_data_reg(&mut self) -> u8 {
                // NOTE(read_volatile) read only 1 byte (the svd2rust API only allows
                // reading a half-word)
                unsafe { ptr::read_volatile(self.spi.dr() as *const _ as *const u8) }
            }

            #[inline(always)]
            fn write_data_reg(&mut self, byte: u8) {
                let dr = self.spi.dr() as *const _ as *const UnsafeCell<u8>;
                // NOTE(write_volatile) see note above
                unsafe { ptr::write_volatile(UnsafeCell::raw_get(dr), byte) };
            }

            pub fn read(&mut self) -> u8 {
                while self.is_rx_not_empty() {
                    let _ = self.read_data_reg();
                }

                while !self.is_rx_not_empty() {}

                self.read_data_reg()
            }

            pub fn write(&mut self, byte: u8) {
                while !self.is_tx_empty() {}

                self.write_data_reg(byte);
            }

            #[inline(always)]
            fn check_read(&mut self) -> nb::Result<u8, Error> {
                let sr = self.spi.sr().read();

                Err(if sr.ovr().bit_is_set() {
                    Error::Overrun.into()
                } else if sr.modf().bit_is_set() {
                    Error::ModeFault.into()
                } else if sr.crcerr().bit_is_set() {
                    Error::Crc.into()
                } else if sr.rxne().bit_is_set() {
                    return Ok(self.read_data_reg());
                } else {
                    nb::Error::WouldBlock
                })
            }

            #[inline(always)]
            fn check_send(&mut self, byte: u8) -> nb::Result<(), Error> {
                let sr = self.spi.sr().read();

                Err(if sr.ovr().bit_is_set() {
                    // Read from the DR to clear the OVR bit
                    let _ = self.spi.dr().read();
                    Error::Overrun.into()
                } else if sr.modf().bit_is_set() {
                    // Write to CR1 to clear MODF
                    self.spi.cr1().modify(|_r, w| w);
                    Error::ModeFault.into()
                } else if sr.crcerr().bit_is_set() {
                    // Clear the CRCERR bit
                    self.spi.sr().modify(|_r, w| w.crcerr().clear_bit());
                    Error::Crc.into()
                } else if sr.txe().bit_is_set() {
                    self.write_data_reg(byte);
                    return Ok(());
                } else {
                    nb::Error::WouldBlock
                })
            }

            #[inline(always)]
            fn listen_event(
                &mut self,
                disable: Option<BitFlags<Event>>,
                enable: Option<BitFlags<Event>>,
            ) {
                self.spi.cr2().modify(|r, w| unsafe {
                    w.bits({
                        let mut bits = r.bits();
                        if let Some(d) = disable {
                            bits &= !d.bits();
                        }
                        if let Some(e) = enable {
                            bits |= e.bits();
                        }
                        bits
                    })
                });
            }

            #[inline(always)]
            pub fn listen(&mut self, event: impl Into<BitFlags<Event>>) {
                self.listen_event(None, Some(event.into()));
            }

            #[inline(always)]
            pub fn listen_only(&mut self, event: impl Into<BitFlags<Event>>) {
                self.listen_event(Some(BitFlags::ALL), Some(event.into()));
            }

            #[inline(always)]
            pub fn unlisten(&mut self, event: impl Into<BitFlags<Event>>) {
                self.listen_event(Some(event.into()), None);
            }

            #[inline(always)]
            pub fn listen_all(&mut self) {
                self.listen(BitFlags::ALL);
            }

            #[inline(always)]
            pub fn unlisten_all(&mut self) {
                self.unlisten(BitFlags::ALL);
            }

            #[inline(always)]
            pub fn flags(&self) -> BitFlags<Flag> {
                BitFlags::from_bits_truncate(self.spi.sr().read().bits())
            }

            #[inline(always)]
            pub fn clear_flags(&mut self, flags: impl Into<BitFlags<Flag>>) {
                if flags.into().contains(Flag::CrcError) {
                    self.spi
                        .sr()
                        .write(|w| unsafe { w.bits(0xffff).crcerr().clear_bit() });
                }
            }

            /// Clears all interrupts flags
            #[inline(always)]
            pub fn clear_all_flags(&mut self) {
                self.clear_flags(BitFlags::ALL)
            }
        }
    };
}

spi_hal!(Spi1, spi1, spi1_slave);

spi_hal!(Spi2, spi2, spi2_slave);

spi_hal!(Spi3, spi3, spi3_slave);

#[cfg(feature = "spi4")]
spi_hal!(Spi4, spi4, spi4_slave);

macro_rules! pin {
    ($SPIX:ident: {
        sck: [$($( #[ $pmetasda:meta ] )* $PSCK:ty),+ $(,)*]
        miso: [$($( #[ $pmetasdi:meta ] )* $PMISO:ty),+ $(,)*]
        mosi: [$($( #[ $pmetasdo:meta ] )* $PMOSI:ty),+ $(,)*]
        nss: [$($( #[ $pmetasdn:meta ] )* $PNSS:ty),+ $(,)*]
    }) => {
        $(
            $( #[ $pmetasda ] )*
            impl SCKPin<pac::$SPIX> for $PSCK {}
        )+

        $(
            $( #[ $pmetasdi ] )*
            impl MISOPin<pac::$SPIX> for $PMISO {}
        )+

        $(
            $( #[ $pmetasdo ] )*
            impl MOSIPin<pac::$SPIX> for $PMOSI {}
        )+

        $(
            $( #[ $pmetasdn ] )*
            impl NSSPin<pac::$SPIX> for $PNSS {}
        )+
    };
}

pin!(
    Spi1: {
        sck: [
            PA5<AF5>,
            PB3<AF5>,
            #[cfg(any(
                feature = "stm32g471",
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484"
            ))]
            PG2<AF5>,
        ]
        miso: [
            PA6<AF5>,
            PB4<AF5>,
            #[cfg(any(
                feature = "stm32g471",
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484"
            ))]
            PG3<AF5>,
        ]
        mosi: [
            PA7<AF5>,
            PB5<AF5>,
            #[cfg(any(
                feature = "stm32g471",
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484"
            ))]
            PG4<AF5>,
        ]
        nss: [
            PA4<AF5>,
            PA15<AF5>,
        ]
    }
);

pin!(
    Spi2: {
        sck: [
            PB13<AF5>,
            PF1<AF5>,
            PF9<AF5>,
            PF10<AF5>,
        ]
        miso: [
            PA10<AF5>,
            PB14<AF5>,
        ]
        mosi: [
            PA11<AF5>,
            PB15<AF5>,
        ]
        nss: [
            PB12<AF5>,
            PD15<AF6>
        ]
    }
);

pin!(
    Spi3: {
        sck: [
            PB3<AF6>,
            PC10<AF6>,
            #[cfg(any(
                feature = "stm32g471",
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484"
            ))]
            PG9<AF6>,
        ]
        miso: [
            PB4<AF6>,
            PC11<AF6>,
        ]
        mosi: [
            PB5<AF6>,
            PC12<AF6>,
        ]
        nss: [
            PA4<AF6>,
            PA15<AF6>,
        ]
    }
);

#[cfg(feature = "spi4")]
pin!(
    Spi4: {
        sck: [
            PE2<AF5>,
            PE12<AF5>,
        ]
        miso: [
            PE5<AF5>,
            PE13<AF5>,
        ]
        mosi: [
            PE6<AF5>,
            PE14<AF5>,
        ]
        nss: [
            PE4<AF5>,
        ]
    }
);

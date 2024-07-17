//! i2c
use core::cmp;

use fugit::HertzU32 as Hertz;

use crate::pac;
use crate::rcc::Clocks;
use crate::rcc::{BusTimerClock, Enable, Reset};

/// I2C
pub struct I2c<I2C, SDA, SCL> {
    i2c: I2C,
    pins: (SDA, SCL),
}

pub trait SDAPin<I2C> {}

pub trait SCLPin<I2C> {}

pub trait I2cExt<I2C> {
    fn i2c<SDA, SCL, T>(self, clocks: &Clocks, pins: (SDA, SCL), freq: T) -> I2c<I2C, SDA, SCL>
    where
        SDA: SDAPin<I2C>,
        SCL: SCLPin<I2C>,
        T: Into<Hertz>;
}

/// I2C Events
///
/// Each event is a possible interrupt source, if enabled
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Event {
    /// (TXIE)
    Transmit,
    /// (RXIE)
    Receive,
    /// (TCIE)
    TransferComplete,
    /// Stop detection (STOPIE)
    Stop,
    /// (ERRIE)
    Errors,
    /// Not Acknowledge received (NACKIE)
    NotAcknowledge,
}

/// I2C Stop Configuration
///
/// Peripheral options for generating the STOP condition
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Stop {
    /// Software end mode: Must write register to generate STOP condition
    Software,
    /// Automatic end mode: A STOP condition is automatically generated once the
    /// configured number of bytes have been transferred
    Automatic,
}

/// I2C error
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Error {
    /// Bus error
    Bus,
    /// Arbitration loss
    Arbitration,
    /// No ack received
    NotAcknowledge,
    // Overrun, // slave mode only
    // Pec, // SMBUS mode only
    // Timeout, // SMBUS mode only
    // Alert, // SMBUS mode only
}

// Calculate I2C timing for Analog Filter ON, Digital Filter OFF
macro_rules! i2c_timing {
    ($i2cclk:ident, $freq:ident) => {{
        // Refer to RM0433 Rev 7 Figure 539 for setup and hold timing:
        //
        // t_I2CCLK = 1 / PCLK1
        // t_PRESC  = (PRESC + 1) * t_I2CCLK
        // t_SCLL   = (SCLL + 1) * t_PRESC
        // t_SCLH   = (SCLH + 1) * t_PRESC
        //
        // t_SYNC1 + t_SYNC2 > 4 * t_I2CCLK
        // t_SCL ~= t_SYNC1 + t_SYNC2 + t_SCLL + t_SCLH
        let ratio = $i2cclk / $freq;

        // For the standard-mode configuration method, we must have a ratio of 4
        // or higher
        assert!(
            ratio >= 4,
            "The I2C PCLK must be at least 4 times the bus frequency!"
        );

        let (presc_reg, scll, sclh, sdadel, scldel) = if $freq > 100_000 {
            // Fast-mode (Fm) or Fast-mode Plus (Fm+)
            // here we pick SCLL + 1 = 2 * (SCLH + 1)

            // Prescaler, 96 ticks for sclh/scll. Round up then subtract 1
            let presc_reg = ((ratio - 1) / 96) as u8;
            // ratio < 1200 by pclk 120MHz max., therefore presc < 16

            // Actual precale value selected
            let presc = (presc_reg + 1) as u32;

            let sclh = ((ratio / presc) - 3) / 3;
            let scll = (2 * (sclh + 1)) - 1;

            let (sdadel, scldel) = if $freq > 400_000 {
                // Fast-mode Plus (Fm+)
                assert!($i2cclk >= 17_000_000); // See table in datsheet

                let sdadel = $i2cclk / 8_000_000 / presc;
                let scldel = $i2cclk / 4_000_000 / presc - 1;

                (sdadel, scldel)
            } else {
                // Fast-mode (Fm)
                assert!($i2cclk >= 8_000_000); // See table in datsheet

                let sdadel = $i2cclk / 3_000_000 / presc;
                let scldel = $i2cclk / 1_000_000 / presc - 1;

                (sdadel, scldel)
            };

            (
                presc_reg,
                scll as u8,
                sclh as u8,
                sdadel as u8,
                scldel as u8,
            )
        } else {
            // Standard-mode (Sm)
            // here we pick SCLL = SCLH
            assert!($i2cclk >= 2_000_000); // See table in datsheet

            // Prescaler, 128 or 256 ticks for sclh/scll. Round up then
            // subtract 1
            let presc_reg = (ratio - 1)
                / if $freq < 8000 {
                    256
                } else if $freq < 80_000 {
                    128
                } else {
                    64
                };
            let presc_reg = cmp::min(presc_reg, 15) as u8;

            // Actual prescale value selected
            let presc = (presc_reg + 1) as u32;

            let sclh = ((ratio / presc) - 2) / 2;
            let scll = sclh;

            // Speed check
            assert!(
                sclh < 256,
                "The I2C PCLK is too fast for this bus frequency!"
            );

            let sdadel = $i2cclk / 2_000_000 / presc;
            let scldel = $i2cclk / 500_000 / presc - 1;

            (
                presc_reg,
                scll as u8,
                sclh as u8,
                sdadel as u8,
                scldel as u8,
            )
        };

        // Sanity check
        assert!(presc_reg < 16);

        // Keep values within reasonable limits for fast per_ck
        let sdadel = cmp::max(sdadel, 1);
        let scldel = cmp::max(scldel, 4);

        let sdadel = cmp::min(sdadel, 15);
        let scldel = cmp::min(scldel, 15);

        (presc_reg, scll, sclh, sdadel, scldel)
    }};
}

// Sequence to flush the TXDR register. This resets the TXIS and TXE
// flags
macro_rules! flush_txdr {
    ($i2c:expr) => {
        // If a pending TXIS flag is set, write dummy data to TXDR
        if $i2c.isr().read().txis().bit_is_set() {
            $i2c.txdr().write(|w| unsafe { w.txdata().bits(0) });
        }

        // If TXDR is not flagged as empty, write 1 to flush it
        if $i2c.isr().read().txe().bit_is_set() {
            $i2c.isr().write(|w| w.txe().set_bit());
        }
    };
}

macro_rules! busy_wait {
    ($i2c:expr, $flag:ident, $variant:ident) => {
        loop {
            let isr = $i2c.isr().read();

            if isr.$flag().$variant() {
                break;
            } else if isr.berr().bit_is_set() {
                $i2c.icr().write(|w| w.berrcf().set_bit());
                return Err(Error::Bus);
            } else if isr.arlo().bit_is_set() {
                $i2c.icr().write(|w| w.arlocf().set_bit());
                return Err(Error::Arbitration);
            } else if isr.nackf().bit_is_set() {
                $i2c.icr()
                    .write(|w| w.stopcf().set_bit().nackcf().set_bit());
                flush_txdr!($i2c);
                return Err(Error::NotAcknowledge);
            } else {
                // try again
            }
        }
    };
}

macro_rules! i2c_hal {
    ($I2CX:ty, $i2cX:ident) => {
        impl I2cExt<$I2CX> for $I2CX {
            fn i2c<SDA, SCL, T>(
                self,
                clocks: &Clocks,
                pins: (SDA, SCL),
                freq: T,
            ) -> I2c<$I2CX, SDA, SCL>
            where
                SDA: SDAPin<$I2CX>,
                SCL: SCLPin<$I2CX>,
                T: Into<Hertz>,
            {
                $i2cX(self, clocks, pins, freq)
            }
        }

        pub fn $i2cX<SDA, SCL, T>(
            i2c: $I2CX,
            clocks: &Clocks,
            pins: (SDA, SCL),
            freq: T,
        ) -> I2c<$I2CX, SDA, SCL>
        where
            SDA: SDAPin<$I2CX>,
            SCL: SCLPin<$I2CX>,
            T: Into<Hertz>,
        {
            // Enable and reset I2C
            unsafe {
                let rcc = &(*pac::Rcc::PTR);
                <$I2CX>::enable(rcc);
                <$I2CX>::reset(rcc);
            }

            let freq = freq.into().raw();

            let clk = <$I2CX>::timer_clock(clocks).raw();

            // Make sure the I2C unit is disabled so we can configure it
            i2c.cr1().modify(|_, w| w.pe().clear_bit());

            // Enable the Analog Noise Filter by setting
            // ANFOFF (Analog Noise Filter OFF) to 0.  This is
            // usually enabled by default
            i2c.cr1().modify(|_, w| w.anfoff().clear_bit());

            // Configure timing
            let (presc_reg, scll, sclh, sdadel, scldel) = i2c_timing!(clk, freq);
            i2c.timingr().write(|w| unsafe {
                w.presc()
                    .bits(presc_reg)
                    .scll()
                    .bits(scll)
                    .sclh()
                    .bits(sclh)
                    .sdadel()
                    .bits(sdadel)
                    .scldel()
                    .bits(scldel)
            });

            // Enable the peripheral
            i2c.cr1().write(|w| w.pe().set_bit());

            I2c { i2c, pins }
        }

        impl<SDA, SCL> I2c<$I2CX, SDA, SCL>
        where
            SDA: SDAPin<$I2CX>,
            SCL: SCLPin<$I2CX>,
        {
            /// Returns a reference to the inner peripheral
            pub fn inner(&self) -> &$I2CX {
                &self.i2c
            }

            /// Returns a mutable reference to the inner peripheral
            pub fn inner_mut(&mut self) -> &mut $I2CX {
                &mut self.i2c
            }

            /// Enable or disable the DMA mode for reception
            pub fn rx_dma(&mut self, enable: bool) {
                self.i2c.cr1().modify(|_, w| w.rxdmaen().bit(enable));
            }

            /// Enable or disable the DMA mode for transmission
            pub fn tx_dma(&mut self, enable: bool) {
                self.i2c.cr1().modify(|_, w| w.txdmaen().bit(enable));
            }

            /// Start listening for `event`
            pub fn listen(&mut self, event: Event) {
                self.i2c.cr1().modify(|_, w| match event {
                    Event::Transmit => w.txie().set_bit(),
                    Event::Receive => w.rxie().set_bit(),
                    Event::TransferComplete => w.tcie().set_bit(),
                    Event::Stop => w.stopie().set_bit(),
                    Event::Errors => w.errie().set_bit(),
                    Event::NotAcknowledge => w.nackie().set_bit(),
                });
            }

            /// Stop listening for `event`
            pub fn unlisten(&mut self, event: Event) {
                self.i2c.cr1().modify(|_, w| match event {
                    Event::Transmit => w.txie().clear_bit(),
                    Event::Receive => w.rxie().clear_bit(),
                    Event::TransferComplete => w.tcie().clear_bit(),
                    Event::Stop => w.stopie().clear_bit(),
                    Event::Errors => w.errie().clear_bit(),
                    Event::NotAcknowledge => w.nackie().clear_bit(),
                });
                let _ = self.i2c.cr1().read();
                let _ = self.i2c.cr1().read(); // Delay 2 peripheral clocks
            }

            /// Clears interrupt flag for `event`
            pub fn clear_irq(&mut self, event: Event) {
                self.i2c.icr().write(|w| match event {
                    Event::Stop => w.stopcf().set_bit(),
                    Event::Errors => w.berrcf().set_bit().arlocf().set_bit().ovrcf().set_bit(),
                    Event::NotAcknowledge => w.nackcf().set_bit(),
                    _ => w,
                });
                let _ = self.i2c.isr().read();
                let _ = self.i2c.isr().read(); // Delay 2 peripheral clocks
            }

            /// Disables I2C and releases the peripheral as well as the pins.
            pub fn release(self) -> ($I2CX, (SDA, SCL)) {
                // Disable I2C.
                unsafe {
                    let rcc = &(*pac::Rcc::PTR);
                    <$I2CX>::reset(rcc);
                    <$I2CX>::disable(rcc);
                }

                (self.i2c, self.pins)
            }

            /// Master read
            ///
            /// Perform an I2C start and prepare to receive `length` bytes.
            ///
            /// ```
            /// Master: ST SAD+R  ...  (SP)
            /// Slave:            ...
            /// ```
            pub fn master_read(&mut self, addr: u8, length: usize, stop: Stop) {
                assert!(length < 256 && length > 0);

                // Wait for any previous address sequence to end
                // automatically. This could be up to 50% of a bus
                // cycle (ie. up to 0.5/freq)
                while self.i2c.cr2().read().start().bit_is_set() {}

                // Set START and prepare to receive bytes into
                // `buffer`. The START bit can be set even if the bus
                // is BUSY or I2C is in slave mode.
                self.i2c.cr2().write(|w| unsafe {
                    w
                        // Start transfer
                        .start()
                        .set_bit()
                        // Set number of bytes to transfer
                        .nbytes()
                        .bits(length as u8)
                        // Set address to transfer to/from
                        .sadd()
                        .bits((addr << 1 | 0) as u16)
                        // 7-bit addressing mode
                        .add10()
                        .clear_bit()
                        // Set transfer direction to write
                        .rd_wrn()
                        .clear_bit()
                        // Software end mode
                        .autoend()
                        .bit(stop == Stop::Automatic)
                });
            }
            /// Master write
            ///
            /// Perform an I2C start and prepare to send `length` bytes.
            ///
            /// ```
            /// Master: ST SAD+W  ...  (SP)
            /// Slave:            ...
            /// ```
            pub fn master_write(&mut self, addr: u8, length: usize, stop: Stop) {
                assert!(length < 256 && length > 0);

                // Wait for any previous address sequence to end
                // automatically. This could be up to 50% of a bus
                // cycle (ie. up to 0.5/freq)
                while self.i2c.cr2().read().start().bit_is_set() {}

                // Set START and prepare to send `bytes`. The
                // START bit can be set even if the bus is BUSY or
                // I2C is in slave mode.
                self.i2c.cr2().write(|w| unsafe {
                    w
                        // Start transfer
                        .start()
                        .set_bit()
                        // Set number of bytes to transfer
                        .nbytes()
                        .bits(length as u8)
                        // Set address to transfer to/from
                        .sadd()
                        .bits((addr << 1 | 0) as u16)
                        // 7-bit addressing mode
                        .add10()
                        .clear_bit()
                        // Set transfer direction to read
                        .rd_wrn()
                        .set_bit()
                        // Automatic end mode
                        .autoend()
                        .bit(stop == Stop::Automatic)
                });
            }

            /// Master restart
            ///
            /// Performs an I2C restart following a write phase and prepare
            /// to receive `length` bytes. The I2C peripheral is configured
            /// to provide an automatic stop.
            ///
            /// ```
            /// Master: ...  SR  SAD+R  ...  (SP)
            /// Slave:  ...             ...
            /// ```
            pub fn master_re_start(&mut self, addr: u8, length: usize, stop: Stop) {
                assert!(length < 256 && length > 0);

                self.i2c.cr2().write(|w| {
                    unsafe {
                        w
                            // Start transfer
                            .start()
                            .set_bit()
                            // Set number of bytes to transfer
                            .nbytes()
                            .bits(length as u8)
                            // Set address to transfer to/from
                            .sadd()
                            .bits((addr << 1 | 1) as u16)
                            // 7-bit addressing mode
                            .add10()
                            .clear_bit()
                            // Set transfer direction to read
                            .rd_wrn()
                            .set_bit()
                            // Automatic end mode
                            .autoend()
                            .bit(stop == Stop::Automatic)
                    }
                });
            }

            /// Master stop
            ///
            /// Generate a stop condition.
            ///
            /// ```
            /// Master: ...  SP
            /// Slave:  ...
            /// ```
            pub fn master_stop(&mut self) {
                self.i2c.cr2().write(|w| w.stop().set_bit());
            }

            pub fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
                // TODO support transfers of more than 255 bytes
                assert!(bytes.len() < 256 && bytes.len() > 0);

                // I2C start
                //
                // ST SAD+W
                self.master_write(addr, bytes.len(), Stop::Software);

                for byte in bytes {
                    // Wait until we are allowed to send data
                    // (START has been ACKed or last byte when
                    // through)
                    busy_wait!(self.i2c, txis, bit_is_set);

                    // Put byte on the wire
                    self.i2c.txdr().write(|w| unsafe { w.txdata().bits(*byte) });
                }

                // Wait until the write finishes
                busy_wait!(self.i2c, tc, bit_is_set);

                // Stop
                self.master_stop();

                // Wait for stop
                busy_wait!(self.i2c, busy, bit_is_set);

                Ok(())
            }

            pub fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Error> {
                // TODO support transfers of more than 255 bytes
                assert!(buffer.len() < 256 && buffer.len() > 0);

                self.master_read(addr, buffer.len(), Stop::Automatic);

                for byte in buffer {
                    // Wait until we have received something
                    busy_wait!(self.i2c, rxne, bit_is_set);

                    *byte = self.i2c.rxdr().read().rxdata().bits();
                }

                // Wait for automatic stop
                busy_wait!(self.i2c, busy, bit_is_set);

                Ok(())
            }

            pub fn write_read(
                &mut self,
                addr: u8,
                bytes: &[u8],
                buffer: &mut [u8],
            ) -> Result<(), Error> {
                // TODO support transfers of more than 255 bytes
                assert!(bytes.len() < 256 && bytes.len() > 0);
                assert!(buffer.len() < 256 && buffer.len() > 0);

                // I2C start
                //
                // ST SAD+W
                self.master_write(addr, bytes.len(), Stop::Software);

                for byte in bytes {
                    // Wait until we are allowed to send data
                    // (START has been ACKed or last byte went through)
                    busy_wait!(self.i2c, txis, bit_is_set);

                    // Put byte on the wire
                    self.i2c.txdr().write(|w| unsafe { w.txdata().bits(*byte) });
                }

                // Wait until the write finishes before beginning to read.
                busy_wait!(self.i2c, tc, bit_is_set);

                // I2C re-start
                //
                // SR  SAD+R
                self.master_re_start(addr, buffer.len(), Stop::Automatic);

                for byte in buffer {
                    // Wait until we have received something
                    busy_wait!(self.i2c, rxne, bit_is_set);

                    *byte = self.i2c.rxdr().read().rxdata().bits();
                }

                // Wait for automatic stop
                busy_wait!(self.i2c, busy, bit_is_set);

                Ok(())
            }
        }
    };
}

i2c_hal!(pac::I2c1, i2c1);

i2c_hal!(pac::I2c2, i2c2);

i2c_hal!(pac::I2c3, i2c3);

#[cfg(feature = "i2c4")]
i2c_hal!(pac::I2c4, i2c4);

macro_rules! pin {
    ($I2CX:ty {
        sda: [$($( #[ $pmetasda:meta ] )* $PSDA:ty),+ $(,)*]
        scl: [$($( #[ $pmetascl:meta ] )* $PSCL:ty),+ $(,)*]
    }) => {
        $(
            $( #[ $pmetasda ] )*
            impl SDAPin<$I2CX> for $PSDA {}
        )+

        $(
            $( #[ $pmetascl ] )*
            impl SCLPin<$I2CX> for $PSCL {}
        )+
    }
}

use crate::gpio::*;

pin!(
    pac::I2c1 {
        sda: [
            PA14<Alt<4, OpenDrain>>,
            PB7<Alt<4, OpenDrain>>,
            PB9<Alt<4, OpenDrain>>,
        ]
        scl: [
            PA13<Alt<4, OpenDrain>>,
            PA15<Alt<4, OpenDrain>>,
            PB8<Alt<4, OpenDrain>>,
        ]
    }
);

pin!(
    pac::I2c2 {
        sda: [
            PA8<Alt<4, OpenDrain>>,
            PF0<Alt<4, OpenDrain>>,
        ]
        scl: [
            PA9<Alt<4, OpenDrain>>,
            PC4<Alt<4, OpenDrain>>,
            #[cfg(any(
                feature = "stm32g471",
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484"
            ))]
            PF6<Alt<4, OpenDrain>>,
        ]
    }
);

pin!(
    pac::I2c3 {
        sda: [
            PB5<Alt<8, OpenDrain>>,
            PC11<Alt<8, OpenDrain>>,
            PC9<Alt<8, OpenDrain>>,
            #[cfg(any(
                feature = "stm32g471",
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484"
            ))]
            PF4<Alt<4, OpenDrain>>,
            #[cfg(any(
                feature = "stm32g471",
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484"
            ))]
            PG8<Alt<4, OpenDrain>>,
        ]
        scl: [
            PA8<Alt<2, OpenDrain>>,
            PC8<Alt<8, OpenDrain>>,
            #[cfg(any(
                feature = "stm32g471",
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484"
            ))]
            PF3<Alt<4, OpenDrain>>,
            #[cfg(any(
                feature = "stm32g471",
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484"
            ))]
            PG7<Alt<4, OpenDrain>>,
        ]
    }
);

#[cfg(any(
    feature = "stm32g471",
    feature = "stm32g473",
    feature = "stm32g474",
    feature = "stm32g483",
    feature = "stm32g484"
))]
pin!(
    pac::I2c4 {
        sda: [
            PB7<Alt<AF3>>,
            PC7<Alt<8, OpenDrain>>,
            PF15<Alt<4, OpenDrain>>,
            PG4<Alt<4, OpenDrain>>,
        ]
        scl: [
            PA13<Alt<AF3>>,
            PC6<Alt<8, OpenDrain>>,
            PF14<Alt<4, OpenDrain>>,
            PG3<Alt<4, OpenDrain>>,
        ]
    }
);

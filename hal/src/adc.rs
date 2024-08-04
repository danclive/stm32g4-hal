//! ADC

use core::marker::PhantomData;

use fugit::{ExtU32, HertzU32 as Hertz};

use crate::delay::DelayUs;
use crate::pac;
use crate::rcc::{Clocks, Enable, Reset};
use crate::signature::VDDA_CALIB;

pub use config::*;

mod config;
pub mod pins;

pub trait Channel<ADC> {
    /// Channel ID type
    ///
    /// A type used to identify this ADC channel. For example, if the ADC has eight channels, this
    /// might be a `u8`. If the ADC has multiple banks of channels, it could be a tuple, like
    /// `(u8: bank_id, u8: channel_id)`.
    type ID;

    /// Get the specific ID that identifies this channel, for example `0_u8` for the first ADC
    /// channel, if Self::ID is u8.
    fn channel() -> Self::ID;

    // `channel` is a function due to [this reported issue](https://github.com/rust-lang/rust/issues/54973).
    // Something about blanket impls combined with `type ID; const CHANNEL: Self::ID;` causes problems.
    // const CHANNEL: Self::ID;
}

pub struct Adc<ADC, STATUS> {
    inner: Inner<ADC>,
    _enabled: PhantomData<STATUS>,
}

pub struct Disabled;

pub struct Enabled;

pub struct Active;

pub struct Dma;

struct Inner<ADC> {
    rb: ADC,
    clock: Hertz,
    config: config::Config,
    /// VDDA in millivolts calculated from the factory calibration and vrefint
    calibrated_vdda: u32,
}

pub trait AdcExt<ADC>: Sized {
    fn adc(
        self,
        clocks: &Clocks,
        cs: ClockSource,
        freq: impl Into<Hertz>,
        config: config::Config,
        delay: &mut impl DelayUs,
    ) -> Adc<ADC, Disabled>;
}

macro_rules! adc_hal {
    ($ADCX:ident, $ADC_COMMON:ident, $adcX: ident) => {
        impl AdcExt<pac::$ADCX> for pac::$ADCX {
            fn adc(
                self,
                clocks: &Clocks,
                cs: ClockSource,
                freq: impl Into<Hertz>,
                config: config::Config,
                delay: &mut impl DelayUs,
            ) -> Adc<pac::$ADCX, Disabled> {
                $adcX(self, clocks, cs, freq, config, delay)
            }
        }

        pub fn $adcX(
            adc: pac::$ADCX,
            clocks: &Clocks,
            cs: ClockSource,
            freq: impl Into<Hertz>,
            config: config::Config,
            delay: &mut impl DelayUs,
        ) -> Adc<pac::$ADCX, Disabled> {
            let mut adc = Adc {
                inner: Inner {
                    rb: adc,
                    clock: Hertz::Hz(0),
                    config,
                    calibrated_vdda: VDDA_CALIB,
                },
                _enabled: PhantomData,
            };

            // Consume ADC register block, produce Self with default
            // settings
            let rcc = unsafe { &*pac::Rcc::PTR };

            // Enable AHB clock
            <pac::$ADCX>::enable(rcc);

            // Power Down
            adc.power_down();

            // Reset peripheral
            <pac::$ADCX>::reset(rcc);

            adc.configure_clock(clocks, cs, freq.into());
            adc.power_up(delay);
            adc.inner.calibrate_all();

            adc
        }

        impl Adc<pac::$ADCX, Disabled> {
            /// Puts a Disabled Adc into Powered Mode
            #[inline(always)]
            pub fn power_down(&mut self) {
                self.inner
                    .rb
                    .cr()
                    .modify(|_, w| w.deeppwd().set_bit().advregen().clear_bit());
            }

            pub fn power_up(&mut self, delay: &mut impl DelayUs) {
                self.inner
                    .rb
                    .cr()
                    .modify(|_, w| w.deeppwd().clear_bit().advregen().set_bit());

                while !self.inner.rb.cr().read().advregen().bit_is_set() {}

                // According to the STM32G4xx Reference Manual, section 21.4.6, we need
                // to wait for T_ADCVREG_STUP after enabling the internal voltage
                // regulator. For the STM32G431, this is 20 us. We choose 25 us to
                // account for bad clocks.
                delay.delay_us(25.micros());

                if self.is_enabled() {
                    self.disable();
                }
            }

            /// Returns if the ADC is enabled (ADEN)
            #[inline(always)]
            pub fn is_enabled(&self) -> bool {
                self.inner.rb.cr().read().aden().bit_is_set()
            }

            /// Disables the adc, since we don't know in what state we get it.
            #[inline(always)]
            pub fn disable(&mut self) {
                // Disable any ongoing conversions
                self.inner.cancel_conversion();

                // Turn off ADC
                self.inner.rb.cr().modify(|_, w| w.addis().set_bit());
                while self.inner.rb.cr().read().addis().bit_is_set() {}

                // Wait until the ADC has turned off
                while self.inner.rb.cr().read().aden().bit_is_set() {}
            }

            /// Sets the clock configuration for this ADC. This is common
            /// between ADC1 and ADC2, so the prec block is used to ensure
            /// this method can only be called on one of the ADCs (or both,
            /// using the [adc12](#method.adc12) method).
            ///
            /// Only `CKMODE[1:0]` = 0 is supported
            fn configure_clock(&mut self, clocks: &Clocks, cs: ClockSource, freq: Hertz) {
                let rcc = unsafe { &(*pac::Rcc::PTR) };

                // Select system clock as ADC clock source
                rcc.rcc_ccipr().modify(|_, w| {
                    // This is sound, as `0b10` is a valid value for this field.
                    unsafe {
                        w.adc12sel().bits(cs.into());
                    }

                    w
                });

                let clk = match cs {
                    ClockSource::SystemClock => clocks.sys_clk(),
                    ClockSource::PLL_P => clocks.pll_clk().p.expect("PLL P not configured"),
                };

                let freq = freq.raw();

                let (divider, presc) = match (clk.raw() + freq - 1) / freq {
                    1 => (1, Clock::DIV_1),
                    2 => (2, Clock::DIV_2),
                    3..=4 => (4, Clock::DIV_4),
                    5..=6 => (6, Clock::DIV_6),
                    7..=8 => (8, Clock::DIV_8),
                    9..=10 => (10, Clock::DIV_10),
                    11..=12 => (12, Clock::DIV_12),
                    13..=16 => (16, Clock::DIV_16),
                    17..=32 => (32, Clock::DIV_32),
                    33..=64 => (64, Clock::DIV_64),
                    65..=128 => (128, Clock::DIV_128),
                    129..=256 => (256, Clock::DIV_256),
                    _ => panic!(
                        "Selecting the ADC clock required a prescaler > 256, \
                                 which is not possible in hardware. Either increase the ADC \
                                 clock frequency or decrease the kernel clock frequency"
                    ),
                };

                unsafe {
                    let common = &(*pac::$ADC_COMMON::PTR);
                    common.ccr().modify(|_, w| w.presc().bits(presc.into()));
                }

                self.inner.clock = clk / divider;
            }

            /// Enable the ADC
            pub fn enable(mut self) -> Adc<pac::$ADCX, Enabled> {
                self.inner.configure(self.inner.config);

                self.inner.rb.isr().modify(|_, w| w.adrdy().set_bit());
                self.inner.rb.cr().modify(|_, w| w.aden().set_bit());

                // Wait for adc to get ready
                while !self.inner.rb.isr().read().adrdy().bit_is_set() {}

                // Clear ready flag
                self.inner.rb.isr().modify(|_, w| w.adrdy().set_bit());

                // Resets the end-of-conversion flag
                self.inner.rb.isr().modify(|_, w| w.eoc().set_bit());

                Adc {
                    inner: self.inner,
                    _enabled: PhantomData,
                }
            }

            pub fn enable_dma(mut self, dma: config::Dma) -> Adc<pac::$ADCX, Enabled> {
                self.inner.set_dma(dma);
                self.enable()
            }

            /// Get ADC samping time
            pub fn get_sample_time(&self) -> SampleTime {
                self.inner.config.sample_time
            }

            /// Set ADC sampling time
            ///
            /// Options can be found in [SampleTime].
            pub fn set_sample_time(&mut self, t: SampleTime) {
                self.inner.config.sample_time = t;
            }

            /// Sets the clock_mode for the adc
            #[inline(always)]
            pub fn set_clock_mode(&mut self, clock_mode: config::ClockMode) {
                self.inner.set_clock_mode(clock_mode);
            }

            /// Sets the oversampling
            #[inline(always)]
            pub fn set_oversampling(
                &mut self,
                oversampling: config::OverSampling,
                shift: config::OverSamplingShift,
            ) {
                self.inner.set_oversampling(oversampling, shift)
            }

            /// Sets the sampling resolution
            #[inline(always)]
            pub fn set_resolution(&mut self, resolution: config::Resolution) {
                self.inner.set_resolution(resolution);
            }

            /// Sets the DR register alignment to left or right
            #[inline(always)]
            pub fn set_align(&mut self, align: config::Align) {
                self.inner.set_align(align);
            }

            /// Sets which external trigger to use and if it is disabled, rising, falling or both
            #[inline(always)]
            pub fn set_external_trigger(&mut self, (edge, extsel): (config::TriggerMode, ExternalTrigger)) {
                self.inner.set_external_trigger((edge, extsel));
            }

            /// Enables and disables dis-/continuous mode
            #[inline(always)]
            pub fn set_continuous(&mut self, continuous: config::Continuous) {
                self.inner.set_continuous(continuous);
            }

            #[inline(always)]
            // NOTE: The software is allowed to write these bits only when ADSTART = 0
            pub fn set_subgroup_len(&mut self, subgroup_len: config::SubGroupLength) {
                self.inner.set_subgroup_len(subgroup_len);
            }

            /// Sets if the end-of-conversion behaviour.
            /// The end-of-conversion interrupt occur either per conversion or for the whole sequence.
            #[inline(always)]
            pub fn set_end_of_conversion_interrupt(&mut self, eoc: config::Eoc) {
                self.inner.set_end_of_conversion_interrupt(eoc);
            }

            /// Enable/disable overrun interrupt
            ///
            /// This is triggered when the AD finishes a conversion before the last value was read by CPU/DMA
            pub fn set_overrun_interrupt(&mut self, enable: bool) {
                self.inner.set_overrun_interrupt(enable);
            }

            /// Sets the differential selection per channel.
            #[inline(always)]
            pub fn set_channel_input_type(&mut self, df: config::DifferentialSelection) {
                self.inner.set_channel_input_type(df);
            }

            /// Sets auto delay to true or false
            #[inline(always)]
            pub fn set_auto_delay(&mut self, delay: bool) {
                self.inner.set_auto_delay(delay);
            }

            /// Sets DMA to disabled, single or continuous
            #[inline(always)]
            pub fn set_dma(&mut self, dma: config::Dma) {
                self.inner.set_dma(dma);
            }

            /// Returns the address of the ADC data register. Primarily useful for configuring DMA.
            #[inline(always)]
            pub fn data_register_address(&self) -> u32 {
                &self.inner.rb.dr() as *const _ as u32
            }

            /// Reset the sequence
            #[inline(always)]
            pub fn reset_sequence(&mut self) {
                self.inner.reset_sequence()
            }

            /// Returns the current sequence length. Primarily useful for configuring DMA.
            #[inline(always)]
            pub fn sequence_length(&mut self) -> u8 {
                self.inner.sequence_length()
            }

            /// Calibrate the adc for <Input Type>
            #[inline(always)]
            pub fn calibrate(&mut self, it: config::InputType) {
                self.inner.calibrate(it)
            }

            /// Calibrate the Adc for all Input Types
            #[inline(always)]
            pub fn calibrate_all(&mut self) {
                self.inner.calibrate_all();
            }

            /// Configure a channel for sampling.
            /// It will make sure the sequence is at least as long as the `sequence` provided.
            /// # Arguments
            /// * `channel` - channel to configure
            /// * `sequence` - where in the sequence to sample the channel. Also called rank in some STM docs/code
            /// * `sample_time` - how long to sample for. See datasheet and ref manual to work out how long you need\
            ///   to sample for at a given ADC clock frequency
            #[inline(always)]
            pub fn configure_channel<CHANNEL>(
                &mut self,
                channel: &CHANNEL,
                sequence: config::Sequence,
                sample_time: config::SampleTime,
            ) where
                CHANNEL: Channel<pac::$ADCX, ID = u8>,
            {
                self.inner.configure_channel(channel, sequence, sample_time)
            }

            /// Disables the Voltage Regulator and release the ADC
            #[inline(always)]
            pub fn release(mut self) -> pac::$ADCX {
                self.inner.enable_deeppwd_down();

                self.inner.rb
            }
        }

        impl Adc<pac::$ADCX, Enabled> {
            pub fn disable(mut self) -> Adc<pac::$ADCX, Disabled> {
                let cr = self.inner.rb.cr().read();

                if cr.adstart().bit_is_set() {
                    self.stop_regular_conversion();
                }

                if cr.jadstart().bit_is_set() {
                    self.stop_injected_conversion();
                }

                // Turn off ADC
                self.inner.rb.cr().modify(|_, w| w.addis().set_bit());
                while self.inner.rb.cr().read().addis().bit_is_set() {}

                // Wait until the ADC has turned off
                while self.inner.rb.cr().read().aden().bit_is_set() {}

                Adc {
                    inner: self.inner,
                    _enabled: PhantomData,
                }
            }

            fn stop_regular_conversion(&mut self) {
                self.inner.rb.cr().modify(|_, w| w.adstp().set_bit());
                while self.inner.rb.cr().read().adstp().bit_is_set() {}
            }

            fn stop_injected_conversion(&mut self) {
                self.inner.rb.cr().modify(|_, w| w.jadstp().set_bit());
                while self.inner.rb.cr().read().jadstp().bit_is_set() {}
            }

            /// get current sample
            #[inline(always)]
            pub fn current_sample(&self) -> u16 {
                self.inner.current_sample()
            }

            /// clear end conversion flag
            #[inline(always)]
            pub fn clear_end_conversion_flag(&mut self) {
                self.inner.clear_end_of_conversion_flag();
            }

            /// Synchronously convert a single sample
            /// Note that it reconfigures the adc sequence and doesn't restore it
            #[inline(always)]
            pub fn convert<PIN>(&mut self, pin: &PIN, sample_time: config::SampleTime) -> u16
            where
                PIN: Channel<pac::$ADCX, ID = u8>,
            {
                self.inner.reset_sequence();
                self.inner
                    .configure_channel(pin, config::Sequence::SEQ_1, sample_time);
                self.inner.start_conversion();

                //Wait for the sequence to complete
                self.inner.wait_for_conversion_sequence();

                self.inner.current_sample()
            }

            /// Starts conversion sequence. Waits for the hardware to indicate it's actually started.
            #[inline(always)]
            pub fn start_conversion(mut self) -> Adc<pac::$ADCX, Active> {
                self.inner.clear_end_of_conversion_flag();
                self.inner.start_conversion();

                Adc {
                    inner: self.inner,
                    _enabled: PhantomData,
                }
            }

            /// Converts a sample value to millivolts using calibrated VDDA and configured resolution
            #[inline(always)]
            pub fn sample_to_millivolts(&self, sample: u16) -> u16 {
                self.inner.sample_to_millivolts(sample)
            }
        }

        impl Adc<pac::$ADCX, Active> {
            /// Block until the conversion is completed and return to configured
            pub fn wait_for_conversion_sequence(&mut self) {
                self.inner.wait_for_conversion_sequence()
            }

            /// Returns if a conversion has been completed
            /// Calling this before `wait_for_conversion_sequence`
            /// should make that function return immediatly
            pub fn is_conversion_done(&self) -> bool {
                !self.inner.is_conversion_active()
            }

            /// Cancels an ongoing conversion
            #[inline(always)]
            pub fn cancel_conversion(mut self) -> Adc<pac::$ADCX, Enabled> {
                self.inner.cancel_conversion();

                Adc {
                    inner: self.inner,
                    _enabled: PhantomData,
                }
            }

            /// get current sample
            #[inline(always)]
            pub fn current_sample(&self) -> u16 {
                self.inner.current_sample()
            }

            /// clear end conversion flag
            #[inline(always)]
            pub fn clear_end_conversion_flag(&mut self) {
                self.inner.clear_end_of_conversion_flag();
            }

            /// Converts a sample value to millivolts using calibrated VDDA and configured resolution
            #[inline(always)]
            pub fn sample_to_millivolts(&self, sample: u16) -> u16 {
                self.inner.sample_to_millivolts(sample)
            }
        }

        impl Inner<pac::$ADCX> {
            /// Disables the Voltage Regulator and release the ADC
            #[inline(always)]
            fn release(mut self) -> pac::$ADCX {
                self.enable_deeppwd_down();

                self.rb
            }

            /// Enables the Deep Power Down Modus
            #[inline(always)]
            fn enable_deeppwd_down(&mut self) {
                self.rb.cr().modify(|_, w| w.deeppwd().set_bit());
            }

            /// Disables the Deep Power Down Modus
            #[inline(always)]
            fn disable_deeppwd_down(&mut self) {
                self.rb.cr().modify(|_, w| w.deeppwd().clear_bit());
            }

            /// Get ADC samping time
            fn get_sample_time(&self) -> SampleTime {
                self.config.sample_time
            }

            /// Set ADC sampling time
            ///
            /// Options can be found in [SampleTime].
            fn set_sample_time(&mut self, t: SampleTime) {
                self.config.sample_time = t;
            }

            /// Enable oversampling
            #[inline(always)]
            fn set_oversampling(&mut self, oversampling: OverSampling, shift: OverSamplingShift) {
                self.rb.cfgr2().modify(|_, w| unsafe {
                    w.ovsr()
                        .bits(oversampling.into())
                        .ovss()
                        .bits(shift.into())
                        .rovse()
                        .set_bit()
                });
            }

            /// Configure a channel for sampling.
            /// It will make sure the sequence is at least as long as the `sequence` provided.
            /// # Arguments
            /// * `channel` - channel to configure
            /// * `sequence` - where in the sequence to sample the channel. Also called rank in some STM docs/code
            /// * `sample_time` - how long to sample for. See datasheet and ref manual to work out how long you need\
            ///   to sample for at a given ADC clock frequency
            fn configure_channel<CHANNEL>(
                &mut self,
                _channel: &CHANNEL,
                sequence: config::Sequence,
                sample_time: config::SampleTime,
            ) where
                CHANNEL: Channel<pac::$ADCX, ID = u8>,
            {
                // Check the sequence is long enough
                self.rb.sqr1().modify(|r, w| {
                    let prev: config::Sequence = r.l().bits().into();
                    if prev < sequence {
                        unsafe { w.l().bits(sequence.into()) }
                    } else {
                        w
                    }
                });

                let chan = CHANNEL::channel();

                // Set the channel in the right sequence field
                let seq = sequence as u8;
                match seq {
                    0..=3 => self.rb.sqr1().modify(|_, w| match seq {
                        0 => unsafe { w.sq1().bits(chan) },
                        1 => unsafe { w.sq2().bits(chan) },
                        2 => unsafe { w.sq3().bits(chan) },
                        3 => unsafe { w.sq4().bits(chan) },
                        _ => unreachable!(),
                    }),
                    4..=8 => self.rb.sqr2().modify(|_, w| match seq {
                        4 => unsafe { w.sq5().bits(chan) },
                        5 => unsafe { w.sq6().bits(chan) },
                        6 => unsafe { w.sq7().bits(chan) },
                        7 => unsafe { w.sq8().bits(chan) },
                        8 => unsafe { w.sq9().bits(chan) },
                        _ => unreachable!(),
                    }),
                    9..=13 => self.rb.sqr3().modify(|_, w| match seq {
                        9 => unsafe { w.sq10().bits(chan) },
                        10 => unsafe { w.sq11().bits(chan) },
                        11 => unsafe { w.sq12().bits(chan) },
                        12 => unsafe { w.sq13().bits(chan) },
                        13 => unsafe { w.sq14().bits(chan) },
                        _ => unreachable!(),
                    }),
                    14..=16 => self.rb.sqr4().modify(|_, w| match seq {
                        14 => unsafe { w.sq15().bits(chan) },
                        15 => unsafe { w.sq16().bits(chan) },
                        _ => unreachable!(),
                    }),
                    _ => unreachable!(),
                }

                // Set the sample time for the channel
                let st = u8::from(sample_time);

                if chan <= 9 {
                    self.rb.smpr1().modify(|_, w| match chan {
                        0 => unsafe { w.smp0().bits(st) },
                        1 => unsafe { w.smp1().bits(st) },
                        2 => unsafe { w.smp2().bits(st) },
                        3 => unsafe { w.smp3().bits(st) },
                        4 => unsafe { w.smp4().bits(st) },
                        5 => unsafe { w.smp5().bits(st) },
                        6 => unsafe { w.smp6().bits(st) },
                        7 => unsafe { w.smp7().bits(st) },
                        8 => unsafe { w.smp8().bits(st) },
                        9 => unsafe { w.smp9().bits(st) },
                        _ => unreachable!(),
                    })
                } else {
                    self.rb.smpr2().modify(|_, w| match chan {
                        10 => unsafe { w.smp10().bits(st) },
                        11 => unsafe { w.smp11().bits(st) },
                        12 => unsafe { w.smp12().bits(st) },
                        13 => unsafe { w.smp13().bits(st) },
                        14 => unsafe { w.smp14().bits(st) },
                        15 => unsafe { w.smp15().bits(st) },
                        16 => unsafe { w.smp16().bits(st) },
                        17 => unsafe { w.smp17().bits(st) },
                        18 => unsafe { w.smp18().bits(st) },
                        _ => unreachable!(),
                    })
                }
            }

            /// Calibrate the adc for Input Type
            ///
            /// Note: The ADC must be disabled
            #[inline(always)]
            fn calibrate(&mut self, it: config::InputType) {
                self.check_calibration_conditions();

                match it {
                    config::InputType::SingleEnded => {
                        self.rb.cr().modify(|_, w| w.adcaldif().clear_bit());
                    }
                    config::InputType::Differential => {
                        self.rb.cr().modify(|_, w| w.adcaldif().set_bit());
                    }
                }

                self.rb.cr().modify(|_, w| w.adcal().set_bit());
                while self.rb.cr().read().adcal().bit_is_set() {}
            }

            /// Calibrate the Adc for all Input Types
            ///
            /// Note: The ADC must be disabled
            #[inline(always)]
            fn calibrate_all(&mut self) {
                self.calibrate(config::InputType::Differential);
                self.calibrate(config::InputType::SingleEnded);
            }

            fn check_calibration_conditions(&self) {
                let cr = self.rb.cr().read();
                if cr.aden().bit_is_set() {
                    panic!("Cannot start calibration when the ADC is enabled");
                }
                if cr.deeppwd().bit_is_set() {
                    panic!("Cannot start calibration when the ADC is in deeppowerdown-mode");
                }
                if cr.advregen().bit_is_clear() {
                    panic!("Cannot start calibration when the ADC voltage regulator is disabled");
                }
            }

            /// Reset the sequence
            #[inline(always)]
            fn reset_sequence(&mut self) {
                //The reset state is One conversion selected
                self.rb
                    .sqr1()
                    .modify(|_, w| unsafe { w.l().bits(config::Sequence::SEQ_1.into()) });
            }

            /// Returns the current sequence length. Primarily useful for configuring DMA.
            #[inline(always)]
            fn sequence_length(&mut self) -> u8 {
                self.rb.sqr1().read().l().bits() + 1
            }

            /// Resets the end-of-conversion flag
            #[inline(always)]
            fn clear_end_of_conversion_flag(&mut self) {
                self.rb.isr().modify(|_, w| w.eoc().set_bit());
            }

            /// Block until the conversion is completed and return to configured
            fn wait_for_conversion_sequence(&mut self) {
                while !self.rb.isr().read().eoc().bit_is_set() {}
            }

            /// get current sample
            #[inline(always)]
            fn current_sample(&self) -> u16 {
                self.rb.dr().read().rdata().bits()
            }

            /// Starts conversion sequence. Waits for the hardware to indicate it's actually started.
            #[inline(always)]
            fn start_conversion(&mut self) {
                //Start conversion
                self.rb.cr().modify(|_, w| w.adstart().set_bit());
            }

            /// Cancels an ongoing conversion
            #[inline(always)]
            fn cancel_conversion(&mut self) {
                self.rb.cr().modify(|_, w| w.adstp().set_bit());
                while self.rb.cr().read().adstart().bit_is_set() {}
            }

            /// Returns if a conversion is active
            #[inline(always)]
            fn is_conversion_active(&self) -> bool {
                self.rb.cr().read().adstart().bit_is_set()
            }

            /// Returns if the Voltage Regulator is enabled
            #[inline(always)]
            fn is_vreg_enabled(&self) -> bool {
                self.rb.cr().read().advregen().bit_is_set()
            }

            /// Returns if Deep Power Down is enabled
            #[inline(always)]
            fn is_deeppwd_enabled(&self) -> bool {
                self.rb.cr().read().deeppwd().bit_is_set()
            }

            /// Read overrun flag
            #[inline(always)]
            fn get_overrun_flag(&self) -> bool {
                self.rb.isr().read().ovr().bit()
            }

            /// Converts a sample value to millivolts using calibrated VDDA and configured resolution
            #[inline(always)]
            fn sample_to_millivolts(&self, sample: u16) -> u16 {
                pins::Vref::sample_to_millivolts_ext(sample, self.calibrated_vdda, self.config.resolution)
            }

            /// Resets the overrun flag
            #[inline(always)]
            fn clear_overrun_flag(&mut self) {
                self.rb.isr().modify(|_, w| w.ovr().set_bit());
            }

            /// Applies all fields in Config
            #[inline(always)]
            fn configure(&mut self, config: config::Config) {
                self.set_clock_mode(config.clock_mode);
                self.set_resolution(config.resolution);
                self.set_align(config.align);
                self.set_external_trigger(config.external_trigger);
                self.set_continuous(config.continuous);
                self.set_subgroup_len(config.subgroup_len);
                self.set_end_of_conversion_interrupt(config.end_of_conversion_interrupt);
                self.set_overrun_interrupt(config.overrun_interrupt);
                self.set_sample_time(config.sample_time);
                self.set_channel_input_type(config.difsel);
                self.set_auto_delay(config.auto_delay);
                self.set_dma(config.dma);

                if let Some(vdda) = config.vdda {
                    self.calibrated_vdda = vdda;
                }
            }

            /// Sets the clock_mode for the adc
            #[inline(always)]
            fn set_clock_mode(&mut self, clock_mode: config::ClockMode) {
                self.config.clock_mode = clock_mode;

                unsafe {
                    let common = &(*pac::$ADC_COMMON::PTR);
                    common
                        .ccr()
                        .modify(|_, w| w.ckmode().bits(clock_mode.into()));
                }
            }

            /// Sets the sampling resolution
            #[inline(always)]
            fn set_resolution(&mut self, resolution: config::Resolution) {
                self.config.resolution = resolution;

                self.rb
                    .cfgr()
                    .modify(|_, w| unsafe { w.res().bits(resolution.into()) });
            }

            /// Sets the DR register alignment to left or right
            #[inline(always)]
            fn set_align(&mut self, align: config::Align) {
                self.config.align = align;

                self.rb.cfgr().modify(|_, w| w.align().bit(align.into()));
            }

            /// Sets which external trigger to use and if it is disabled, rising, falling or both
            #[inline(always)]
            fn set_external_trigger(&mut self, (edge, extsel): (config::TriggerMode, ExternalTrigger)) {
                self.config.external_trigger = (edge, extsel);

                self.rb
                    .cfgr()
                    .modify(|_, w| unsafe { w.extsel().bits(extsel.into()).exten().bits(edge.into()) });
            }

            /// Enables and disables dis-/continuous mode
            #[inline(always)]
            fn set_continuous(&mut self, continuous: config::Continuous) {
                self.config.continuous = continuous;

                self.rb.cfgr().modify(|_, w| {
                    w.cont()
                        .bit(continuous == config::Continuous::Continuous)
                        .discen()
                        .bit(continuous == config::Continuous::Discontinuous)
                });
            }

            #[inline(always)]
            // NOTE: The software is allowed to write these bits only when ADSTART = 0
            fn set_subgroup_len(&mut self, subgroup_len: config::SubGroupLength) {
                self.config.subgroup_len = subgroup_len;

                self.rb
                    .cfgr()
                    .modify(|_, w| unsafe { w.discnum().bits(subgroup_len as u8) })
            }

            /// Sets if the end-of-conversion behaviour.
            /// The end-of-conversion interrupt occur either per conversion or for the whole sequence.
            #[inline(always)]
            fn set_end_of_conversion_interrupt(&mut self, eoc: config::Eoc) {
                self.config.end_of_conversion_interrupt = eoc;

                let (en, eocs) = match eoc {
                    config::Eoc::Disabled => (false, false),
                    config::Eoc::Conversion => (true, true),
                    config::Eoc::Sequence => (true, false),
                };
                self.rb
                    .ier()
                    .modify(|_, w| w.eosie().bit(eocs).eocie().bit(en));
            }

            /// Enable/disable overrun interrupt
            ///
            /// This is triggered when the AD finishes a conversion before the last value was read by CPU/DMA
            fn set_overrun_interrupt(&mut self, enable: bool) {
                self.rb.ier().modify(|_, w| w.ovrie().bit(enable));
            }

            /// Sets the differential selection per channel.
            #[inline(always)]
            fn set_channel_input_type(&mut self, df: config::DifferentialSelection) {
                self.config.difsel = df;

                self.rb
                    .difsel()
                    .modify(|_, w| unsafe { w.difsel_1_18().bits(df.0) });
            }

            /// Sets auto delay to true or false
            #[inline(always)]
            fn set_auto_delay(&mut self, delay: bool) {
                self.config.auto_delay = delay;

                self.rb.cfgr().modify(|_, w| w.autdly().bit(delay));
            }

            /// Sets DMA to disabled, single or continuous
            #[inline(always)]
            fn set_dma(&mut self, dma: config::Dma) {
                self.config.dma = dma;

                let (dds, en) = match dma {
                    config::Dma::Disabled => (false, false),
                    config::Dma::Single => (false, true),
                    config::Dma::Continuous => (true, true),
                };
                self.rb.cfgr().modify(|_, w| {
                    w
                        //DDS stands for "DMA disable selection"
                        //0 means do one DMA then stop
                        //1 means keep sending DMA requests as long as DMA=1
                        .dmacfg()
                        .bit(dds)
                        .dmaen()
                        .bit(en)
                });
            }

            /// Returns the address of the ADC data register. Primarily useful for configuring DMA.
            #[inline(always)]
            fn data_register_address(&self) -> u32 {
                &self.rb.dr() as *const _ as u32
            }
        }
    };
}

adc_hal!(Adc1, Adc12Common, adc1);
adc_hal!(Adc2, Adc12Common, adc2);

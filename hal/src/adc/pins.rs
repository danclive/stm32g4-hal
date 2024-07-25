use crate::gpio::*;
use crate::signature::{VtempCal130, VtempCal30, VDDA_CALIB};

use super::*;

/// Core temperature internal signal
#[derive(Debug, Default)]
pub struct Temperature;

impl Temperature {
    /// Precompute the inverse of `VTEMP_CAL_VREFANALOG`, in volts,
    /// for floating point calculations
    const INV_VREFANALOG_VOLTS: f32 = 1000. / VDDA_CALIB as f32;
    /// Temperature at which temperature sensor has been calibrated in production
    /// for data into [`VtempCal30`] (tolerance: +-5 DegC) (unit: DegC).
    const VTEMP_CAL_T30: u16 = 30;
    /// Temperature at which temperature sensor has been calibrated in production
    /// for data into [`VtempCal130`] (tolerance: +-5 DegC) (unit: DegC).
    const VTEMP_CAL_T130: u16 = 130;

    /// Convert a sample to 12 bits. Reference voltages were captured at 12 bits.
    const fn to_12b(sample: u16, resolution: config::Resolution) -> u16 {
        match resolution {
            config::Resolution::Six => sample << 6,
            config::Resolution::Eight => sample << 4,
            config::Resolution::Ten => sample << 2,
            config::Resolution::Twelve => sample,
        }
    }

    /// Convert a raw sample from `Temperature` to deg C.
    ///
    /// ## Arguments
    /// * `sample`: ADC sample taken on the [`Temperature`] channel.
    /// * `vdda`: Analog reference voltage (vref+) when the temperature
    ///   sample was taken, in volts.
    /// * `resolution`: Configured ADC resolution.
    #[inline(always)]
    pub fn temperature_to_degrees_centigrade(
        sample: u16,
        vdda: f32,
        resolution: config::Resolution,
    ) -> f32 {
        // Reference measurements were taken at 12 bits
        let sample_12b = Self::to_12b(sample, resolution);

        // Normalize for the difference in VDDA
        let sample_normalized = sample_12b as f32 * (vdda * Self::INV_VREFANALOG_VOLTS);

        ((sample_normalized - VtempCal30::get().read() as f32)
            * ((Self::VTEMP_CAL_T130 - Self::VTEMP_CAL_T30) as f32))
            / ((VtempCal130::get().read() - VtempCal30::get().read()) as f32)
            + Self::VTEMP_CAL_T30 as f32
    }

    /// Convert a raw sample from `Temperature` to deg C
    ///
    /// ## Arguments
    /// * `sample`: ADC sample taken on the [`Temperature`] channel.
    /// * `vdda`: Analog reference voltage (vref+) when the temperature
    ///   sample was taken, in millivolts.
    /// * `resolution`: Configured ADC resolution.
    #[inline(always)]
    pub fn temperature_to_degrees_centigrade_coarse(
        sample: u16,
        vdda: u32,
        resolution: config::Resolution,
    ) -> i16 {
        // Reference measurements were taken at 12 bits
        let sample_12b = Self::to_12b(sample, resolution);

        // Normalize for the difference in VDDA
        let sample_normalized = ((sample_12b as u32 * vdda) / VDDA_CALIB) as u16;

        let t = ((sample_normalized as i32 - VtempCal30::get().read() as i32)
            * ((Self::VTEMP_CAL_T130 - Self::VTEMP_CAL_T30) as i32))
            / ((VtempCal130::get().read() - VtempCal30::get().read()) as i32)
            + Self::VTEMP_CAL_T30 as i32;

        t as i16
    }
}

/// Vbat internal signal, used for monitoring the battery (if used)
#[derive(Debug, Default)]
pub struct Vbat;

/// Vref internal signal, used for calibration
#[derive(Debug, Default)]
pub struct Vref;

impl Vref {
    /// Converts a sample value to millivolts using calibrated VDDA and configured resolution
    #[inline(always)]
    pub fn sample_to_millivolts_ext(sample: u16, vdda: u32, resolution: config::Resolution) -> u16 {
        let mx_s = resolution.to_max_sample();
        ((u32::from(sample) * vdda) / mx_s) as u16
    }
    /// Converts a sample value to millivolts using calibrated VDDA and configured resolution
    #[inline(always)]
    pub fn sample_to_millivolts(sample: u16) -> u16 {
        Self::sample_to_millivolts_ext(sample, VDDA_CALIB, config::Resolution::Twelve)
    }
}

macro_rules! pins {
    ($ADC:ident { $($pin:ty => $chan:expr),+ $(,)* }) => {
        $(
            impl Channel<crate::pac::$ADC> for $pin {
                type ID = u8;

                fn channel() -> u8 { $chan }
            }
        )+
    };
}

macro_rules! adc_internal {
    ([$INT_ADC:ident, $INT_ADC_COMMON:ident]; $($input:ty => ($chan:expr, $en:ident)),+ $(,)*) => {
        $(
            impl $input {
                pub fn new() -> Self {
                    Self {}
                }

                /// Enables the internal voltage/sensor
                /// ADC must be disabled.
                pub fn enable(&mut self, _adc: &Adc<crate::pac::$INT_ADC, Disabled>) {

                    let common = unsafe { &(*crate::pac::$INT_ADC_COMMON::PTR) };

                    common.ccr().modify(|_, w| w.$en().set_bit());
                }

                /// Disables the internal voltage/sdissor
                /// ADC must be disabled.
                pub fn disable(&mut self, _adc: &Adc<crate::pac::$INT_ADC, Disabled>) {

                    let common = unsafe { &*crate::pac::$INT_ADC_COMMON::PTR };

                    common.ccr().modify(|_, w| w.$en().clear_bit());
                }

                /// Returns if the internal channel is enabled
                /// ADC must be disabled.
                pub fn is_enabled(&self, _adc: &Adc<crate::pac::$INT_ADC, Disabled>) -> bool {

                    let common = unsafe { &*crate::pac::$INT_ADC_COMMON::PTR };

                    common.ccr().read().$en().bit_is_set()
                }
            }

            pins!($INT_ADC { $input => $chan });
        )+
    };
}

pins!(Adc1 {
    gpioa::PA0<Analog> => 1,
    gpioa::PA1<Analog> => 2,
    gpioa::PA2<Analog> => 3,
    gpioa::PA3<Analog> => 4,
    gpiob::PB14<Analog> => 5,
    gpioc::PC0<Analog> => 6,
    gpioc::PC1<Analog> => 7,
    gpioc::PC2<Analog> => 8,
    gpioc::PC3<Analog> => 9,
    gpiof::PF0<Analog> => 10,
    gpiob::PB12<Analog> => 11,
    gpiob::PB1<Analog> => 12,
    gpiob::PB11<Analog> => 14,
    gpiob::PB0<Analog> => 15,
});

adc_internal!(
    [Adc1, Adc12Common];

    Temperature => (16, vsensesel),
    Vbat => (17, vbatsel),
    Vref => (18, vrefen)
);

pins!(Adc2 {
    gpioa::PA0<Analog> => 1,
    gpioa::PA1<Analog> => 2,
    gpioa::PA6<Analog> => 3,
    gpioa::PA7<Analog> => 4,
    gpioc::PC4<Analog> => 5,
    gpioc::PC0<Analog> => 6,
    gpioc::PC1<Analog> => 7,
    gpioc::PC2<Analog> => 8,
    gpioc::PC3<Analog> => 9,
    gpiof::PF1<Analog> => 10,
    gpioc::PC5<Analog> => 11,
    gpiob::PB2<Analog> => 12,
    gpioa::PA5<Analog> => 13,
    gpiob::PB11<Analog> => 14,
    gpiob::PB15<Analog> => 15,
    gpioa::PA4<Analog> => 17,
});

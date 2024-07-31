//! DAC

use core::marker::PhantomData;
use core::mem::MaybeUninit;

use fugit::ExtU32;

use crate::pac;

use crate::delay::DelayUs;
use crate::gpio::{
    gpioa::{PA4, PA5, PA6},
    Analog,
};
use crate::rcc;

pub trait DacOut<V> {
    fn set_value(&mut self, val: V);
    fn get_value(&mut self) -> V;
}

pub struct GeneratorConfig {
    mode: u8,
    amp: u8,
}

impl GeneratorConfig {
    pub fn triangle(amplitude: u8) -> Self {
        Self {
            mode: 0b10,
            amp: amplitude,
        }
    }

    pub fn sawtooth(amplitude: u8) -> Self {
        Self {
            mode: 0b11,
            amp: amplitude,
        }
    }

    pub fn noise(seed: u8) -> Self {
        Self {
            mode: 0b01,
            amp: seed,
        }
    }
}

/// Enabled DAC (type state)
pub struct Enabled;
/// Enabled DAC wave generator (type state)
pub struct WaveGenerator;
/// Disabled DAC (type state)
pub struct Disabled;
pub trait ED {}
impl ED for Enabled {}
impl ED for WaveGenerator {}
impl ED for Disabled {}

pub struct C1<DAC, const MODE_BITS: u8, ED> {
    _dac: PhantomData<DAC>,
    _enabled: PhantomData<ED>,
}
pub struct C2<DAC, const MODE_BITS: u8, ED> {
    _dac: PhantomData<DAC>,
    _enabled: PhantomData<ED>,
}

/// Trait for GPIO pins that can be converted to DAC output pins
pub trait Pins<DAC> {
    type Output;
}

/// Dac output mode: external pin only
pub const M_EXT_PIN: u8 = 0b000;

/// Dac output mode: internal signal and external pin
pub const M_MIX_SIG: u8 = 0b001;

/// Dac output mode: internal signal only
pub const M_INT_SIG: u8 = 0b011;

pub struct Dac1IntSig1;
pub struct Dac1IntSig2;
pub struct Dac2IntSig1;
pub struct Dac3IntSig1;
pub struct Dac3IntSig2;
pub struct Dac4IntSig1;
pub struct Dac4IntSig2;

macro_rules! impl_pins {
    ($DAC:ty: $pin:ty, $output:ty) => {
        #[allow(unused_parens)]
        impl Pins<$DAC> for $pin {
            #[allow(unused_parens)]
            type Output = $output;
        }
    };
}
// DAC1
impl_pins!(pac::Dac1: PA4<Analog>, C1<pac::Dac1, M_EXT_PIN, Disabled>);
impl_pins!(pac::Dac1: PA5<Analog>, C2<pac::Dac1, M_EXT_PIN, Disabled>);
impl_pins!(
    pac::Dac1: (PA4<Analog>, PA5<Analog>),
    (C1<pac::Dac1, M_EXT_PIN, Disabled>, C2<pac::Dac1, M_EXT_PIN, Disabled>)
);

impl_pins!(pac::Dac1: Dac1IntSig1, C1<pac::Dac1, M_INT_SIG, Disabled>);
impl_pins!(pac::Dac1: Dac1IntSig2, C2<pac::Dac1, M_INT_SIG, Disabled>);
impl_pins!(
    pac::Dac1: (Dac1IntSig1, Dac1IntSig2),
    (C1<pac::Dac1, M_INT_SIG, Disabled>, C2<pac::Dac1, M_INT_SIG, Disabled>)
);

impl_pins!(pac::Dac1: (PA4<Analog>, Dac1IntSig1), C1<pac::Dac1, M_MIX_SIG, Disabled>);
impl_pins!(pac::Dac1: (PA5<Analog>, Dac1IntSig2), C2<pac::Dac1, M_MIX_SIG, Disabled>);

impl_pins!(pac::Dac1: (PA4<Analog>, Dac1IntSig2), (C1<pac::Dac1, M_EXT_PIN, Disabled>, C2<pac::Dac1, M_INT_SIG, Disabled>));
impl_pins!(pac::Dac1: (Dac1IntSig1, PA5<Analog>), (C1<pac::Dac1, M_INT_SIG, Disabled>, C2<pac::Dac1, M_EXT_PIN, Disabled>));

impl_pins!(
    pac::Dac1: (PA4<Analog>, (PA5<Analog>, Dac1IntSig2)),
    (C1<pac::Dac1, M_EXT_PIN, Disabled>, C2<pac::Dac1, M_MIX_SIG, Disabled>)
);

impl_pins!(
    pac::Dac1: ((PA4<Analog>, Dac1IntSig1), PA5<Analog>),
    (C1<pac::Dac1, M_MIX_SIG, Disabled>, C2<pac::Dac1, M_EXT_PIN, Disabled>)
);

impl_pins!(
    pac::Dac1: ((PA4<Analog>, Dac1IntSig1), (PA5<Analog>, Dac1IntSig2)),
    (C1<pac::Dac1, M_MIX_SIG, Disabled>, C2<pac::Dac1, M_MIX_SIG, Disabled>)
);

impl_pins!(
    pac::Dac1: ((PA4<Analog>, Dac1IntSig1), Dac1IntSig2),
    (C1<pac::Dac1, M_MIX_SIG, Disabled>, C2<pac::Dac1, M_INT_SIG, Disabled>)
);

impl_pins!(
    pac::Dac1: (Dac1IntSig1, (PA5<Analog>, Dac1IntSig2)),
    (C1<pac::Dac1, M_INT_SIG, Disabled>, C2<pac::Dac1, M_MIX_SIG, Disabled>)
);

// DAC2
impl_pins!(pac::Dac2: PA6<Analog>, C1<pac::Dac2, M_EXT_PIN, Disabled>);
impl_pins!(pac::Dac2: Dac2IntSig1, C1<pac::Dac2, M_INT_SIG, Disabled>);
impl_pins!(pac::Dac2: (PA6<Analog>, Dac2IntSig1), C1<pac::Dac2, M_MIX_SIG, Disabled>);

// DAC3
impl_pins!(pac::Dac3: Dac3IntSig1, C1<pac::Dac3, M_INT_SIG, Disabled>);
impl_pins!(pac::Dac3: Dac3IntSig2, C2<pac::Dac3, M_INT_SIG, Disabled>);
impl_pins!(
    pac::Dac3: (Dac3IntSig1, Dac3IntSig2),
    (C1<pac::Dac3, M_INT_SIG, Disabled>, C2<pac::Dac3, M_INT_SIG, Disabled>)
);

// DAC4
impl_pins!(pac::Dac4: Dac4IntSig1, C1<pac::Dac4, M_INT_SIG, Disabled>);
impl_pins!(pac::Dac4: Dac4IntSig2, C2<pac::Dac4, M_INT_SIG, Disabled>);
impl_pins!(
    pac::Dac4: (Dac4IntSig1, Dac4IntSig2),
    (C1<pac::Dac4, M_INT_SIG, Disabled>, C2<pac::Dac4, M_INT_SIG, Disabled>)
);

macro_rules! dac_hal_ch {
    ($DAC:ty, $CX:ident, $CN:tt) => {
        paste::paste! {
            impl<const MODE_BITS: u8> $CX<$DAC, MODE_BITS, Disabled> {
                /// TODO: The DAC does not seem to work unless `calibrate_buffer` has been callen
                /// even when only using dac output internally
                pub fn enable(self) -> $CX<$DAC, MODE_BITS, Enabled> {
                    let dac = unsafe { &(*<$DAC>::ptr()) };

                    dac.dac_mcr()
                        .modify(|_, w| unsafe { w.[<mode $CN>]().bits(MODE_BITS) });
                    dac.dac_cr().modify(|_, w| w.[<en $CN>]().set_bit());

                    $CX {
                        _dac: PhantomData,
                        _enabled: PhantomData,
                    }
                }

                pub fn enable_generator(
                    self,
                    config: GeneratorConfig,
                ) -> $CX<$DAC, MODE_BITS, WaveGenerator> {
                    let dac = unsafe { &(*<$DAC>::ptr()) };

                    dac.dac_mcr()
                        .modify(|_, w| unsafe { w.[<mode $CN>]().bits(MODE_BITS) });
                    dac.dac_cr().modify(|_, w| unsafe {
                        w.[<wave $CN>]().bits(config.mode);
                        w.[<ten $CN>]().set_bit();
                        w.[<mamp $CN>]().bits(config.amp);
                        w.[<en $CN>]().set_bit()
                    });

                    $CX {
                        _dac: PhantomData,
                        _enabled: PhantomData,
                    }
                }
            }

            impl<const MODE_BITS: u8, ED> $CX<$DAC, MODE_BITS, ED> {
                /// Calibrate the DAC output buffer by performing a "User
                /// trimming" operation. It is useful when the VDDA/VREF+
                /// voltage or temperature differ from the factory trimming
                /// conditions.
                ///
                /// The calibration is only valid when the DAC channel is
                /// operating with the buffer enabled. If applied in other
                /// modes it has no effect.
                ///
                /// After the calibration operation, the DAC channel is
                /// disabled.
                pub fn calibrate_buffer<T>(
                    self,
                    delay: &mut impl DelayUs,
                ) -> $CX<$DAC, MODE_BITS, Disabled> {
                    let dac = unsafe { &(*<$DAC>::ptr()) };
                    dac.dac_cr().modify(|_, w| w.[<en $CN>]().clear_bit());
                    dac.dac_mcr().modify(|_, w| unsafe { w.[<mode $CN>]().bits(0) });
                    dac.dac_cr().modify(|_, w| w.[<cen $CN>]().set_bit());
                    let mut trim = 0;
                    while true {
                        dac.dac_ccr().modify(|_, w| unsafe { w.[<otrim $CN>]().bits(trim) });
                        delay.delay_us(64.micros());
                        if dac.dac_sr().read().[<cal_flag $CN>]().bit() {
                            break;
                        }
                        trim += 1;
                    }
                    dac.dac_cr().modify(|_, w| w.[<cen $CN>]().clear_bit());

                    $CX {
                        _dac: PhantomData,
                        _enabled: PhantomData,
                    }
                }

                /// Disable the DAC channel
                pub fn disable(self) -> $CX<$DAC, MODE_BITS, Disabled> {
                    let dac = unsafe { &(*<$DAC>::ptr()) };
                    dac.dac_cr().modify(|_, w| unsafe {
                        w.[<en $CN>]().clear_bit().[<wave $CN>]().bits(0).[<ten $CN>]().clear_bit()
                    });

                    $CX {
                        _dac: PhantomData,
                        _enabled: PhantomData,
                    }
                }
            }

            /// DacOut implementation available in any Enabled/Disabled
            /// state
            impl<const MODE_BITS: u8, ED> DacOut<u16> for $CX<$DAC, MODE_BITS, ED> {
                fn set_value(&mut self, val: u16) {
                    let dac = unsafe { &(*<$DAC>::ptr()) };
                    dac.[<dac_dhr12r $CN>]().write(|w| unsafe { w.bits(val as u32) });
                }

                fn get_value(&mut self) -> u16 {
                    let dac = unsafe { &(*<$DAC>::ptr()) };
                    dac.[<dac_dor $CN>]().read().bits() as u16
                }
            }

            /// Wave generator state implementation
            impl<const MODE_BITS: u8> $CX<$DAC, MODE_BITS, WaveGenerator> {
                pub fn trigger(&mut self) {
                    let dac = unsafe { &(*<$DAC>::ptr()) };
                    dac.dac_swtrgr().write(|w| w.[<swtrig $CN>]().set_bit());
                }
            }

        }
    };
}

dac_hal_ch!(pac::Dac1, C1, 1);

dac_hal_ch!(pac::Dac1, C2, 2);

dac_hal_ch!(pac::Dac2, C1, 1);

dac_hal_ch!(pac::Dac3, C1, 1);

dac_hal_ch!(pac::Dac3, C2, 2);

dac_hal_ch!(pac::Dac4, C1, 1);

dac_hal_ch!(pac::Dac4, C2, 2);

pub trait DacExt: Sized {
    fn dac<PINS>(self, pins: PINS) -> PINS::Output
    where
        PINS: Pins<Self>;
}

macro_rules! dac_hal {
    ($DAC:ty) => {
        impl DacExt for $DAC {
            fn dac<PINS>(self, pins: PINS) -> PINS::Output
            where
                PINS: Pins<$DAC>,
            {
                dac(self, pins)
            }
        }
    };
}

pub fn dac<DAC, PINS>(_dac: DAC, _pins: PINS) -> PINS::Output
where
    DAC: rcc::Enable + rcc::Reset,
    PINS: Pins<DAC>,
{
    unsafe {
        let rcc = &(*pac::Rcc::PTR);
        DAC::enable(rcc);
        DAC::reset(rcc);
    }

    #[allow(clippy::uninit_assumed_init)]
    unsafe {
        MaybeUninit::uninit().assume_init()
    }
}

dac_hal!(pac::Dac1);
dac_hal!(pac::Dac2);
dac_hal!(pac::Dac3);
dac_hal!(pac::Dac4);

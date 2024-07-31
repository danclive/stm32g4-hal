//! Integrated opamps.
use core::marker::PhantomData;

use crate::pac;

pub struct Disabled;

pub struct Enabled;

pub trait INP<Opamp> {}

pub trait INM<Opamp> {}

pub trait INM0<Opamp> {}

pub trait INM1<Opamp> {}

pub trait OUT<Opamp> {}

macro_rules! opamp {
    ($($OPAMPi:ident: ($opampi:ident, $csr:ident, $tcmr:ident)),+ $(,)*) => {
        pub struct Opamp {
            $(
                pub $opampi: $OPAMPi<Disabled>,
            )+
        }

        impl Opamp {
            pub fn new(_opamp: pac::Opamp) -> Self {
                unsafe {
                    let rcc = &(*pac::Rcc::PTR);
                    rcc.rcc_apb2enr().modify(|_, w| w.syscfgen().set_bit());
                }

                Opamp {
                    $(
                        $opampi: $opampi(),
                    )+
                }
            }
        }

        $(
            pub struct $OPAMPi<STATUS> {
                _enabled: PhantomData<STATUS>,
            }

            fn $opampi() -> $OPAMPi<Disabled> {
                $OPAMPi {
                    _enabled: PhantomData,
                }
            }

            impl $OPAMPi<Disabled> {
                #[inline(always)]
                pub fn enable(self) -> $OPAMPi<Enabled> {
                    unsafe {
                        let opamp = &(*pac::Opamp::PTR);
                        opamp.$csr().modify(|_, w| w.opaen().set_bit());
                    }

                    $OPAMPi {
                        _enabled: PhantomData,
                    }
                }

                #[inline(always)]
                pub fn vp_sel(&mut self, sel: u8) {
                    unsafe {
                        let opamp = &(*pac::Opamp::PTR);
                        opamp.$csr().modify(|_, w| w.vp_sel().bits(sel));
                    }
                }

                #[inline(always)]
                pub fn user_trim(&mut self, enable: bool) {
                    unsafe {
                        let opamp = &(*pac::Opamp::PTR);
                        opamp.$csr().modify(|_, w| w.usertrim().bit(enable));
                    }
                }

                #[inline(always)]
                pub fn vm_sel(&mut self, sel: u8) {
                    unsafe {
                        let opamp = &(*pac::Opamp::PTR);
                        opamp.$csr().modify(|_, w| w.vm_sel().bits(sel));
                    }
                }

                #[inline(always)]
                pub fn high_speed_mode(&mut self, enable: bool) {
                    unsafe {
                        let opamp = &(*pac::Opamp::PTR);
                        opamp.$csr().modify(|_, w| w.opahsm().bit(enable));
                    }
                }

                #[inline(always)]
                pub fn internal_output(&mut self, enable: bool) {
                    unsafe {
                        let opamp = &(*pac::Opamp::PTR);
                        opamp.$csr().modify(|_, w| w.opaintoen().bit(enable));
                    }
                }

                #[inline(always)]
                pub fn calibrate_mode(&mut self, enable: bool) {
                    unsafe {
                        let opamp = &(*pac::Opamp::PTR);
                        opamp.$csr().modify(|_, w| w.calon().bit(enable));
                    }
                }

                #[inline(always)]
                pub fn calibration_select(&mut self, sel: u8) {
                    unsafe {
                        let opamp = &(*pac::Opamp::PTR);
                        opamp.$csr().modify(|_, w| w.calsel().bits(sel));
                    }
                }

                #[inline(always)]
                pub fn pga_gain(&mut self, gain: u8) {
                    unsafe {
                        let opamp = &(*pac::Opamp::PTR);
                        opamp.$csr().modify(|_, w| w.pga_gain().bits(gain));
                    }
                }

                #[inline(always)]
                pub fn trim_for_pmos(&mut self, trim: u8) {
                    unsafe {
                        let opamp = &(*pac::Opamp::PTR);
                        opamp.$csr().modify(|_, w| w.trimoffsetp().bits(trim));
                    }
                }

                #[inline(always)]
                pub fn trim_for_nmos(&mut self, trim: u8) {
                    unsafe {
                        let opamp = &(*pac::Opamp::PTR);
                        opamp.$csr().modify(|_, w| w.trimoffsetn().bits(trim));
                    }
                }

                #[inline(always)]
                pub fn calibrate_output(&mut self, enable: bool) {
                    unsafe {
                        let opamp = &(*pac::Opamp::PTR);
                        opamp.$csr().modify(|_, w| w.calout().bit(enable));
                    }
                }

                #[inline(always)]
                pub fn csr_lock(&mut self, enable: bool) {
                    unsafe {
                        let opamp = &(*pac::Opamp::PTR);
                        opamp.$csr().modify(|_, w| w.lock().bit(enable));
                    }
                }

                #[inline(always)]
                pub fn vms_sel(&mut self, sel: bool) {
                    unsafe {
                        let opamp = &(*pac::Opamp::PTR);
                        opamp.$tcmr().modify(|_, w| w.vms_sel().bit(sel));
                    }
                }

                #[inline(always)]
                pub fn vps_sel(&mut self, sel: u8) {
                    unsafe {
                        let opamp = &(*pac::Opamp::PTR);
                        opamp.$tcmr().modify(|_, w| w.vps_sel().bits(sel));
                    }
                }

                #[inline(always)]
                pub fn t1cm(&mut self, enable: bool) {
                    unsafe {
                        let opamp = &(*pac::Opamp::PTR);
                        opamp.$tcmr().modify(|_, w| w.t1cm_en().bit(enable));
                    }
                }

                #[inline(always)]
                pub fn t8cm(&mut self, enable: bool) {
                    unsafe {
                        let opamp = &(*pac::Opamp::PTR);
                        opamp.$tcmr().modify(|_, w| w.t8cm_en().bit(enable));
                    }
                }

                #[inline(always)]
                pub fn t20cm(&mut self, enable: bool) {
                    unsafe {
                        let opamp = &(*pac::Opamp::PTR);
                        opamp.$tcmr().modify(|_, w| w.t20cm_en().bit(enable));
                    }
                }

                #[inline(always)]
                pub fn tcmr_lock(&mut self, enable: bool) {
                    unsafe {
                        let opamp = &(*pac::Opamp::PTR);
                        opamp.$tcmr().modify(|_, w| w.lock().bit(enable));
                    }
                }
            }

            impl $OPAMPi<Enabled> {
                pub fn disable(self) -> $OPAMPi<Disabled> {
                    unsafe {
                        let opamp = &(*pac::Opamp::PTR);
                        opamp.$csr().modify(|_, w| w.opaen().clear_bit());
                    }

                    $OPAMPi {
                        _enabled: PhantomData,
                    }
                }
            }
        )+
    };
}

opamp!(
    Opamp1: (opamp1, opamp1_csr, opamp1_tcmr),
    Opamp2: (opamp2, opamp2_csr, opamp2_tcmr),
    Opamp3: (opamp3, opamp3_csr, opamp3_tcmr),
);

macro_rules! pin {
    ($OPAMPi:ident: {
        inp: [$($( #[ $pmetainp:meta ] )* $INP:ty),+ $(,)*]
        inm: [$($( #[ $pmetainm:meta ] )* $INM:ty),+ $(,)*]
        out: [$($( #[ $pmetaout:meta ] )* $OUT:ty),+ $(,)*]
        inm0: [$($( #[ $pmetainm0:meta ] )* $INM0:ty),+ $(,)*]
        inm1: [$($( #[ $pmetainm1:meta ] )* $INM1:ty),+ $(,)*]
    }) => {
        $(
            $( #[$pmetainp] )*
            impl<STATUS> INP<$OPAMPi<STATUS>> for $INP {}
        )+

        $(
            $( #[$pmetainm] )*
            impl<STATUS> INM<$OPAMPi<STATUS>> for $INM {}
        )+

        $(
            $( #[$pmetaout] )*
            impl<STATUS> OUT<$OPAMPi<STATUS>> for $OUT {}
        )+

        $(
            $( #[$pmetainm0] )*
            impl<STATUS> INM0<$OPAMPi<STATUS>> for $INM0 {}
        )+

        $(
            $( #[$pmetainm1] )*
            impl<STATUS> INM1<$OPAMPi<STATUS>> for $INM1 {}
        )+
    };
}

use crate::gpio::*;

pin!(
    Opamp1: {
        inp: [
            PA1<Analog>,
            PA3<Analog>,
            PA7<Analog>,
        ]
        inm: [
            PA3<Analog>,
            PC5<Analog>,
        ]
        out: [
            PA2<Analog>,
        ]
        inm0: [
            PA3<Analog>,
        ]
        inm1: [
            PC5<Analog>,
        ]
    }
);

pin!(
    Opamp2: {
        inp: [
            PA7<Analog>,
            PB14<Analog>,
            PB0<Analog>,
            PD14<Analog>,
        ]
        inm: [
            PA5<Analog>,
            PC5<Analog>,
        ]
        out: [
            PA6<Analog>,
        ]
        inm0: [
            PA5<Analog>,
        ]
        inm1: [
            PC5<Analog>,
        ]
    }
);

pin!(
    Opamp3: {
        inp: [
            PB0<Analog>,
            PB13<Analog>,
            PA1<Analog>,
        ]
        inm: [
            PB2<Analog>,
            PB10<Analog>,
        ]
        out: [
            PB1<Analog>,
        ]
        inm0: [
            PB2<Analog>,
        ]
        inm1: [
            PB10<Analog>,
        ]
    }
);

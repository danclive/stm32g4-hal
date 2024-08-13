//! Microcontroller Clock Output (MCO) configuration
//!
//! Output pin is PA8

use super::*;

/// MCO Configuration
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct MCOConfig {
    pub src: MCOSrc,
    pub psc: MCOPrescaler,
}

/// Microcontroller clock output source
/// System clock mux source
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[repr(u8)]
pub enum MCOSrc {
    #[default]
    DISABLE = 0b0000,
    SYSCLK = 0b0001,
    HSI = 0b0010,
    HSE = 0b0100,
    PLLCLK = 0b0101,
    LSI = 0b0110,
    LSE = 0b0111,
    HSI48 = 0b1000,
}

/// Prescaler
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[repr(u8)]
pub enum MCOPrescaler {
    #[default]
    Dvi1 = 0b000,
    Div2 = 0b001,
    Div4 = 0b010,
    Div8 = 0b011,
    Div16 = 0b100,
}

impl Rcc {
    pub fn mco(mut self, mco: MCOConfig) -> Self {
        self.config.mco = mco;
        self
    }

    pub(super) fn mco_setup(&mut self) {
        // SYSCLK always runs
        // HSI always runs

        // HSE must be explicitly stated
        if self.config.mco.src == MCOSrc::HSE {
            assert!(
                self.config.hse.is_some(),
                "HSE is required for MCO. Explicitly state its frequency with `use_hse`"
            );
        }

        // PLLCLK always runs
        // LSI always runs

        // LSE must be explicitly stated
        if self.config.mco.src == MCOSrc::LSE {
            assert!(
                self.config.lse.is_some(),
                "LSE is required for MCO. Explicitly state its frequency with `use_lse`"
            );
        }

        // HSI48 always runs

        let psc_bits = self.config.mco.psc as u8;

        self.rb
            .rcc_cfgr()
            .modify(|_, w| unsafe { w.mcosel().bits(0).mcopre().bits(psc_bits) });
    }

    pub fn enable_mco(&mut self) {
        let src_bits = self.config.mco.src as u8;
        self.rb
            .rcc_cfgr()
            .modify(|_, w| unsafe { w.mcosel().bits(src_bits) });
    }

    pub fn disable_mco(&mut self) {
        self.rb
            .rcc_cfgr()
            .modify(|_, w| unsafe { w.mcosel().bits(0) });
    }
}

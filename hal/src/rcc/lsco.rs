//! Low Speed Clock Output (LSCO) configuration
//!
//! Output pin is PA2

use super::*;

/// LSCO Configuration
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct LSCOConfig {
    pub src: LSCOSrc,
}

/// LSCO Source
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[repr(u8)]
pub enum LSCOSrc {
    #[default]
    LSI = 0b0,
    LSE = 0b1,
}

impl Rcc {
    pub fn lsco(mut self, lsco: LSCOConfig) -> Self {
        self.config.lsco = lsco;
        self
    }

    pub(super) fn lsco_setup(&mut self) {
        // LSI always runs

        // LSE must be explicitly stated
        if self.config.lsco.src == LSCOSrc::LSE {
            assert!(
                self.config.lse.is_some(),
                "LSE is required for LSCO. Explicitly state its frequency with `use_lse`"
            );
        }

        let src_bits = self.config.lsco.src as u8 != 0;

        self.rb.rcc_bdcr().modify(|_, w| w.lscosel().bit(src_bits));
    }

    pub fn enable_lsco(&mut self) {
        self.rb.rcc_bdcr().modify(|_, w| w.lscoen().set_bit());
    }

    pub fn disable_lsco(&mut self) {
        self.rb.rcc_bdcr().modify(|_, w| w.lscoen().clear_bit());
    }
}

#[doc = "Register `RCC_CIFR` reader"]
pub type R = crate::R<RccCifrSpec>;
#[doc = "LSI ready interrupt flag Set by hardware when the LSI clock becomes stable and LSIRDYDIE is set. Cleared by software setting the LSIRDYC bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsirdyf {
    #[doc = "0: No clock ready interrupt caused by the LSI oscillator"]
    B0x0 = 0,
    #[doc = "1: Clock ready interrupt caused by the LSI oscillator"]
    B0x1 = 1,
}
impl From<Lsirdyf> for bool {
    #[inline(always)]
    fn from(variant: Lsirdyf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYF` reader - LSI ready interrupt flag Set by hardware when the LSI clock becomes stable and LSIRDYDIE is set. Cleared by software setting the LSIRDYC bit."]
pub type LsirdyfR = crate::BitReader<Lsirdyf>;
impl LsirdyfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsirdyf {
        match self.bits {
            false => Lsirdyf::B0x0,
            true => Lsirdyf::B0x1,
        }
    }
    #[doc = "No clock ready interrupt caused by the LSI oscillator"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lsirdyf::B0x0
    }
    #[doc = "Clock ready interrupt caused by the LSI oscillator"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lsirdyf::B0x1
    }
}
#[doc = "LSE ready interrupt flag Set by hardware when the LSE clock becomes stable and LSERDYDIE is set. Cleared by software setting the LSERDYC bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lserdyf {
    #[doc = "0: No clock ready interrupt caused by the LSE oscillator"]
    B0x0 = 0,
    #[doc = "1: Clock ready interrupt caused by the LSE oscillator"]
    B0x1 = 1,
}
impl From<Lserdyf> for bool {
    #[inline(always)]
    fn from(variant: Lserdyf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSERDYF` reader - LSE ready interrupt flag Set by hardware when the LSE clock becomes stable and LSERDYDIE is set. Cleared by software setting the LSERDYC bit."]
pub type LserdyfR = crate::BitReader<Lserdyf>;
impl LserdyfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lserdyf {
        match self.bits {
            false => Lserdyf::B0x0,
            true => Lserdyf::B0x1,
        }
    }
    #[doc = "No clock ready interrupt caused by the LSE oscillator"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lserdyf::B0x0
    }
    #[doc = "Clock ready interrupt caused by the LSE oscillator"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lserdyf::B0x1
    }
}
#[doc = "HSI16 ready interrupt flag Set by hardware when the HSI16 clock becomes stable and HSIRDYDIE is set in a response to setting the HSION (refer to Clock control register (RCC_CR)). When HSION is not set but the HSI16 oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsirdyf {
    #[doc = "0: No clock ready interrupt caused by the HSI16 oscillator"]
    B0x0 = 0,
    #[doc = "1: Clock ready interrupt caused by the HSI16 oscillator"]
    B0x1 = 1,
}
impl From<Hsirdyf> for bool {
    #[inline(always)]
    fn from(variant: Hsirdyf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDYF` reader - HSI16 ready interrupt flag Set by hardware when the HSI16 clock becomes stable and HSIRDYDIE is set in a response to setting the HSION (refer to Clock control register (RCC_CR)). When HSION is not set but the HSI16 oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit."]
pub type HsirdyfR = crate::BitReader<Hsirdyf>;
impl HsirdyfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsirdyf {
        match self.bits {
            false => Hsirdyf::B0x0,
            true => Hsirdyf::B0x1,
        }
    }
    #[doc = "No clock ready interrupt caused by the HSI16 oscillator"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hsirdyf::B0x0
    }
    #[doc = "Clock ready interrupt caused by the HSI16 oscillator"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hsirdyf::B0x1
    }
}
#[doc = "HSE ready interrupt flag Set by hardware when the HSE clock becomes stable and HSERDYDIE is set. Cleared by software setting the HSERDYC bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hserdyf {
    #[doc = "0: No clock ready interrupt caused by the HSE oscillator"]
    B0x0 = 0,
    #[doc = "1: Clock ready interrupt caused by the HSE oscillator"]
    B0x1 = 1,
}
impl From<Hserdyf> for bool {
    #[inline(always)]
    fn from(variant: Hserdyf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag Set by hardware when the HSE clock becomes stable and HSERDYDIE is set. Cleared by software setting the HSERDYC bit."]
pub type HserdyfR = crate::BitReader<Hserdyf>;
impl HserdyfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hserdyf {
        match self.bits {
            false => Hserdyf::B0x0,
            true => Hserdyf::B0x1,
        }
    }
    #[doc = "No clock ready interrupt caused by the HSE oscillator"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hserdyf::B0x0
    }
    #[doc = "Clock ready interrupt caused by the HSE oscillator"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hserdyf::B0x1
    }
}
#[doc = "PLL ready interrupt flag Set by hardware when the PLL locks and PLLRDYDIE is set. Cleared by software setting the PLLRDYC bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllrdyf {
    #[doc = "0: No clock ready interrupt caused by PLL lock"]
    B0x0 = 0,
    #[doc = "1: Clock ready interrupt caused by PLL lock"]
    B0x1 = 1,
}
impl From<Pllrdyf> for bool {
    #[inline(always)]
    fn from(variant: Pllrdyf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLRDYF` reader - PLL ready interrupt flag Set by hardware when the PLL locks and PLLRDYDIE is set. Cleared by software setting the PLLRDYC bit."]
pub type PllrdyfR = crate::BitReader<Pllrdyf>;
impl PllrdyfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllrdyf {
        match self.bits {
            false => Pllrdyf::B0x0,
            true => Pllrdyf::B0x1,
        }
    }
    #[doc = "No clock ready interrupt caused by PLL lock"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pllrdyf::B0x0
    }
    #[doc = "Clock ready interrupt caused by PLL lock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllrdyf::B0x1
    }
}
#[doc = "Clock security system interrupt flag Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cssf {
    #[doc = "0: No clock security interrupt caused by HSE clock failure"]
    B0x0 = 0,
    #[doc = "1: Clock security interrupt caused by HSE clock failure"]
    B0x1 = 1,
}
impl From<Cssf> for bool {
    #[inline(always)]
    fn from(variant: Cssf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSF` reader - Clock security system interrupt flag Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit."]
pub type CssfR = crate::BitReader<Cssf>;
impl CssfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cssf {
        match self.bits {
            false => Cssf::B0x0,
            true => Cssf::B0x1,
        }
    }
    #[doc = "No clock security interrupt caused by HSE clock failure"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cssf::B0x0
    }
    #[doc = "Clock security interrupt caused by HSE clock failure"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cssf::B0x1
    }
}
#[doc = "LSE Clock security system interrupt flag Set by hardware when a failure is detected in the LSE oscillator. Cleared by software setting the LSECSSC bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsecssf {
    #[doc = "0: No clock security interrupt caused by LSE clock failure"]
    B0x0 = 0,
    #[doc = "1: Clock security interrupt caused by LSE clock failure"]
    B0x1 = 1,
}
impl From<Lsecssf> for bool {
    #[inline(always)]
    fn from(variant: Lsecssf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSF` reader - LSE Clock security system interrupt flag Set by hardware when a failure is detected in the LSE oscillator. Cleared by software setting the LSECSSC bit."]
pub type LsecssfR = crate::BitReader<Lsecssf>;
impl LsecssfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsecssf {
        match self.bits {
            false => Lsecssf::B0x0,
            true => Lsecssf::B0x1,
        }
    }
    #[doc = "No clock security interrupt caused by LSE clock failure"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lsecssf::B0x0
    }
    #[doc = "Clock security interrupt caused by LSE clock failure"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lsecssf::B0x1
    }
}
#[doc = "HSI48 ready interrupt flag Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set in a response to setting the HSI48ON (refer to Clock recovery RC register (RCC_CRRCR)). Cleared by software setting the HSI48RDYC bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsi48rdyf {
    #[doc = "0: No clock ready interrupt caused by the HSI48 oscillator"]
    B0x0 = 0,
    #[doc = "1: Clock ready interrupt caused by the HSI48 oscillator"]
    B0x1 = 1,
}
impl From<Hsi48rdyf> for bool {
    #[inline(always)]
    fn from(variant: Hsi48rdyf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48RDYF` reader - HSI48 ready interrupt flag Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set in a response to setting the HSI48ON (refer to Clock recovery RC register (RCC_CRRCR)). Cleared by software setting the HSI48RDYC bit."]
pub type Hsi48rdyfR = crate::BitReader<Hsi48rdyf>;
impl Hsi48rdyfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsi48rdyf {
        match self.bits {
            false => Hsi48rdyf::B0x0,
            true => Hsi48rdyf::B0x1,
        }
    }
    #[doc = "No clock ready interrupt caused by the HSI48 oscillator"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hsi48rdyf::B0x0
    }
    #[doc = "Clock ready interrupt caused by the HSI48 oscillator"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hsi48rdyf::B0x1
    }
}
impl R {
    #[doc = "Bit 0 - LSI ready interrupt flag Set by hardware when the LSI clock becomes stable and LSIRDYDIE is set. Cleared by software setting the LSIRDYC bit."]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LsirdyfR {
        LsirdyfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag Set by hardware when the LSE clock becomes stable and LSERDYDIE is set. Cleared by software setting the LSERDYC bit."]
    #[inline(always)]
    pub fn lserdyf(&self) -> LserdyfR {
        LserdyfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI16 ready interrupt flag Set by hardware when the HSI16 clock becomes stable and HSIRDYDIE is set in a response to setting the HSION (refer to Clock control register (RCC_CR)). When HSION is not set but the HSI16 oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit."]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HsirdyfR {
        HsirdyfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt flag Set by hardware when the HSE clock becomes stable and HSERDYDIE is set. Cleared by software setting the HSERDYC bit."]
    #[inline(always)]
    pub fn hserdyf(&self) -> HserdyfR {
        HserdyfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL ready interrupt flag Set by hardware when the PLL locks and PLLRDYDIE is set. Cleared by software setting the PLLRDYC bit."]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PllrdyfR {
        PllrdyfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Clock security system interrupt flag Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit."]
    #[inline(always)]
    pub fn cssf(&self) -> CssfR {
        CssfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE Clock security system interrupt flag Set by hardware when a failure is detected in the LSE oscillator. Cleared by software setting the LSECSSC bit."]
    #[inline(always)]
    pub fn lsecssf(&self) -> LsecssfR {
        LsecssfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt flag Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set in a response to setting the HSI48ON (refer to Clock recovery RC register (RCC_CRRCR)). Cleared by software setting the HSI48RDYC bit."]
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> Hsi48rdyfR {
        Hsi48rdyfR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CIFR")
            .field("lsirdyf", &self.lsirdyf())
            .field("lserdyf", &self.lserdyf())
            .field("hsirdyf", &self.hsirdyf())
            .field("hserdyf", &self.hserdyf())
            .field("pllrdyf", &self.pllrdyf())
            .field("cssf", &self.cssf())
            .field("lsecssf", &self.lsecssf())
            .field("hsi48rdyf", &self.hsi48rdyf())
            .finish()
    }
}
#[doc = "Clock interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_cifr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccCifrSpec;
impl crate::RegisterSpec for RccCifrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_cifr::R`](R) reader structure"]
impl crate::Readable for RccCifrSpec {}
#[doc = "`reset()` method sets RCC_CIFR to value 0"]
impl crate::Resettable for RccCifrSpec {
    const RESET_VALUE: u32 = 0;
}

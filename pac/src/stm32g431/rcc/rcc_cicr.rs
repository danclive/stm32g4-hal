#[doc = "Register `RCC_CICR` writer"]
pub type W = crate::W<RccCicrSpec>;
#[doc = "LSI ready interrupt clear This bit is set by software to clear the LSIRDYF flag.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsirdyc {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: LSIRDYF cleared"]
    B0x1 = 1,
}
impl From<Lsirdyc> for bool {
    #[inline(always)]
    fn from(variant: Lsirdyc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYC` writer - LSI ready interrupt clear This bit is set by software to clear the LSIRDYF flag."]
pub type LsirdycW<'a, REG> = crate::BitWriter<'a, REG, Lsirdyc>;
impl<'a, REG> LsirdycW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lsirdyc::B0x0)
    }
    #[doc = "LSIRDYF cleared"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lsirdyc::B0x1)
    }
}
#[doc = "LSE ready interrupt clear This bit is set by software to clear the LSERDYF flag.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lserdyc {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: LSERDYF cleared"]
    B0x1 = 1,
}
impl From<Lserdyc> for bool {
    #[inline(always)]
    fn from(variant: Lserdyc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSERDYC` writer - LSE ready interrupt clear This bit is set by software to clear the LSERDYF flag."]
pub type LserdycW<'a, REG> = crate::BitWriter<'a, REG, Lserdyc>;
impl<'a, REG> LserdycW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lserdyc::B0x0)
    }
    #[doc = "LSERDYF cleared"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lserdyc::B0x1)
    }
}
#[doc = "HSI16 ready interrupt clear This bit is set software to clear the HSIRDYF flag.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsirdyc {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Clear HSIRDYF flag"]
    B0x1 = 1,
}
impl From<Hsirdyc> for bool {
    #[inline(always)]
    fn from(variant: Hsirdyc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDYC` writer - HSI16 ready interrupt clear This bit is set software to clear the HSIRDYF flag."]
pub type HsirdycW<'a, REG> = crate::BitWriter<'a, REG, Hsirdyc>;
impl<'a, REG> HsirdycW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hsirdyc::B0x0)
    }
    #[doc = "Clear HSIRDYF flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hsirdyc::B0x1)
    }
}
#[doc = "HSE ready interrupt clear This bit is set by software to clear the HSERDYF flag.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hserdyc {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Clear HSERDYF flag"]
    B0x1 = 1,
}
impl From<Hserdyc> for bool {
    #[inline(always)]
    fn from(variant: Hserdyc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSERDYC` writer - HSE ready interrupt clear This bit is set by software to clear the HSERDYF flag."]
pub type HserdycW<'a, REG> = crate::BitWriter<'a, REG, Hserdyc>;
impl<'a, REG> HserdycW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hserdyc::B0x0)
    }
    #[doc = "Clear HSERDYF flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hserdyc::B0x1)
    }
}
#[doc = "PLL ready interrupt clear This bit is set by software to clear the PLLRDYF flag.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllrdyc {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Clear PLLRDYF flag"]
    B0x1 = 1,
}
impl From<Pllrdyc> for bool {
    #[inline(always)]
    fn from(variant: Pllrdyc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLRDYC` writer - PLL ready interrupt clear This bit is set by software to clear the PLLRDYF flag."]
pub type PllrdycW<'a, REG> = crate::BitWriter<'a, REG, Pllrdyc>;
impl<'a, REG> PllrdycW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllrdyc::B0x0)
    }
    #[doc = "Clear PLLRDYF flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllrdyc::B0x1)
    }
}
#[doc = "Clock security system interrupt clear This bit is set by software to clear the CSSF flag.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cssc {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Clear CSSF flag"]
    B0x1 = 1,
}
impl From<Cssc> for bool {
    #[inline(always)]
    fn from(variant: Cssc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSC` writer - Clock security system interrupt clear This bit is set by software to clear the CSSF flag."]
pub type CsscW<'a, REG> = crate::BitWriter<'a, REG, Cssc>;
impl<'a, REG> CsscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cssc::B0x0)
    }
    #[doc = "Clear CSSF flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cssc::B0x1)
    }
}
#[doc = "LSE Clock security system interrupt clear This bit is set by software to clear the LSECSSF flag.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsecssc {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Clear LSECSSF flag"]
    B0x1 = 1,
}
impl From<Lsecssc> for bool {
    #[inline(always)]
    fn from(variant: Lsecssc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSC` writer - LSE Clock security system interrupt clear This bit is set by software to clear the LSECSSF flag."]
pub type LsecsscW<'a, REG> = crate::BitWriter<'a, REG, Lsecssc>;
impl<'a, REG> LsecsscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lsecssc::B0x0)
    }
    #[doc = "Clear LSECSSF flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lsecssc::B0x1)
    }
}
#[doc = "HSI48 oscillator ready interrupt clear This bit is set by software to clear the HSI48RDYF flag.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsi48rdyc {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Clear the HSI48RDYC flag"]
    B0x1 = 1,
}
impl From<Hsi48rdyc> for bool {
    #[inline(always)]
    fn from(variant: Hsi48rdyc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48RDYC` writer - HSI48 oscillator ready interrupt clear This bit is set by software to clear the HSI48RDYF flag."]
pub type Hsi48rdycW<'a, REG> = crate::BitWriter<'a, REG, Hsi48rdyc>;
impl<'a, REG> Hsi48rdycW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hsi48rdyc::B0x0)
    }
    #[doc = "Clear the HSI48RDYC flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hsi48rdyc::B0x1)
    }
}
impl core::fmt::Debug for crate::generic::Reg<RccCicrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt clear This bit is set by software to clear the LSIRDYF flag."]
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LsirdycW<RccCicrSpec> {
        LsirdycW::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt clear This bit is set by software to clear the LSERDYF flag."]
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LserdycW<RccCicrSpec> {
        LserdycW::new(self, 1)
    }
    #[doc = "Bit 3 - HSI16 ready interrupt clear This bit is set software to clear the HSIRDYF flag."]
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HsirdycW<RccCicrSpec> {
        HsirdycW::new(self, 3)
    }
    #[doc = "Bit 4 - HSE ready interrupt clear This bit is set by software to clear the HSERDYF flag."]
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HserdycW<RccCicrSpec> {
        HserdycW::new(self, 4)
    }
    #[doc = "Bit 5 - PLL ready interrupt clear This bit is set by software to clear the PLLRDYF flag."]
    #[inline(always)]
    pub fn pllrdyc(&mut self) -> PllrdycW<RccCicrSpec> {
        PllrdycW::new(self, 5)
    }
    #[doc = "Bit 8 - Clock security system interrupt clear This bit is set by software to clear the CSSF flag."]
    #[inline(always)]
    pub fn cssc(&mut self) -> CsscW<RccCicrSpec> {
        CsscW::new(self, 8)
    }
    #[doc = "Bit 9 - LSE Clock security system interrupt clear This bit is set by software to clear the LSECSSF flag."]
    #[inline(always)]
    pub fn lsecssc(&mut self) -> LsecsscW<RccCicrSpec> {
        LsecsscW::new(self, 9)
    }
    #[doc = "Bit 10 - HSI48 oscillator ready interrupt clear This bit is set by software to clear the HSI48RDYF flag."]
    #[inline(always)]
    pub fn hsi48rdyc(&mut self) -> Hsi48rdycW<RccCicrSpec> {
        Hsi48rdycW::new(self, 10)
    }
}
#[doc = "Clock interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccCicrSpec;
impl crate::RegisterSpec for RccCicrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rcc_cicr::W`](W) writer structure"]
impl crate::Writable for RccCicrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_CICR to value 0"]
impl crate::Resettable for RccCicrSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CcrSpec>;
#[doc = "Field `DUAL` reader - Dual ADC mode selection"]
pub type DualR = crate::FieldReader;
#[doc = "Field `DUAL` writer - Dual ADC mode selection"]
pub type DualW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DELAY` reader - Delay between 2 sampling phases"]
pub type DelayR = crate::FieldReader;
#[doc = "Field `DELAY` writer - Delay between 2 sampling phases"]
pub type DelayW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMACFG` reader - DMA configuration (for multi-ADC mode)"]
pub type DmacfgR = crate::BitReader;
#[doc = "Field `DMACFG` writer - DMA configuration (for multi-ADC mode)"]
pub type DmacfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA` reader - Direct memory access mode for multi ADC mode"]
pub type MdmaR = crate::FieldReader;
#[doc = "Field `MDMA` writer - Direct memory access mode for multi ADC mode"]
pub type MdmaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKMODE` reader - ADC clock mode"]
pub type CkmodeR = crate::FieldReader;
#[doc = "Field `CKMODE` writer - ADC clock mode"]
pub type CkmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRESC` reader - ADC prescaler"]
pub type PrescR = crate::FieldReader;
#[doc = "Field `PRESC` writer - ADC prescaler"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VREFEN` reader - VREFINT enable"]
pub type VrefenR = crate::BitReader;
#[doc = "Field `VREFEN` writer - VREFINT enable"]
pub type VrefenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSENSESEL` reader - VTS selection"]
pub type VsenseselR = crate::BitReader;
#[doc = "Field `VSENSESEL` writer - VTS selection"]
pub type VsenseselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBATSEL` reader - VBAT selection"]
pub type VbatselR = crate::BitReader;
#[doc = "Field `VBATSEL` writer - VBAT selection"]
pub type VbatselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Dual ADC mode selection"]
    #[inline(always)]
    pub fn dual(&self) -> DualR {
        DualR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Delay between 2 sampling phases"]
    #[inline(always)]
    pub fn delay(&self) -> DelayR {
        DelayR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - DMA configuration (for multi-ADC mode)"]
    #[inline(always)]
    pub fn dmacfg(&self) -> DmacfgR {
        DmacfgR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Direct memory access mode for multi ADC mode"]
    #[inline(always)]
    pub fn mdma(&self) -> MdmaR {
        MdmaR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - ADC clock mode"]
    #[inline(always)]
    pub fn ckmode(&self) -> CkmodeR {
        CkmodeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - ADC prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - VREFINT enable"]
    #[inline(always)]
    pub fn vrefen(&self) -> VrefenR {
        VrefenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - VTS selection"]
    #[inline(always)]
    pub fn vsensesel(&self) -> VsenseselR {
        VsenseselR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - VBAT selection"]
    #[inline(always)]
    pub fn vbatsel(&self) -> VbatselR {
        VbatselR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("dual", &self.dual())
            .field("delay", &self.delay())
            .field("dmacfg", &self.dmacfg())
            .field("mdma", &self.mdma())
            .field("ckmode", &self.ckmode())
            .field("vrefen", &self.vrefen())
            .field("vsensesel", &self.vsensesel())
            .field("vbatsel", &self.vbatsel())
            .field("presc", &self.presc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Dual ADC mode selection"]
    #[inline(always)]
    pub fn dual(&mut self) -> DualW<CcrSpec> {
        DualW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Delay between 2 sampling phases"]
    #[inline(always)]
    pub fn delay(&mut self) -> DelayW<CcrSpec> {
        DelayW::new(self, 8)
    }
    #[doc = "Bit 13 - DMA configuration (for multi-ADC mode)"]
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DmacfgW<CcrSpec> {
        DmacfgW::new(self, 13)
    }
    #[doc = "Bits 14:15 - Direct memory access mode for multi ADC mode"]
    #[inline(always)]
    pub fn mdma(&mut self) -> MdmaW<CcrSpec> {
        MdmaW::new(self, 14)
    }
    #[doc = "Bits 16:17 - ADC clock mode"]
    #[inline(always)]
    pub fn ckmode(&mut self) -> CkmodeW<CcrSpec> {
        CkmodeW::new(self, 16)
    }
    #[doc = "Bits 18:21 - ADC prescaler"]
    #[inline(always)]
    pub fn presc(&mut self) -> PrescW<CcrSpec> {
        PrescW::new(self, 18)
    }
    #[doc = "Bit 22 - VREFINT enable"]
    #[inline(always)]
    pub fn vrefen(&mut self) -> VrefenW<CcrSpec> {
        VrefenW::new(self, 22)
    }
    #[doc = "Bit 23 - VTS selection"]
    #[inline(always)]
    pub fn vsensesel(&mut self) -> VsenseselW<CcrSpec> {
        VsenseselW::new(self, 23)
    }
    #[doc = "Bit 24 - VBAT selection"]
    #[inline(always)]
    pub fn vbatsel(&mut self) -> VbatselW<CcrSpec> {
        VbatselW::new(self, 24)
    }
}
#[doc = "ADC common control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CcrSpec {
    const RESET_VALUE: u32 = 0;
}

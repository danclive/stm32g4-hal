#[doc = "Register `RG3CR` reader"]
pub type R = crate::R<Rg3crSpec>;
#[doc = "Register `RG3CR` writer"]
pub type W = crate::W<Rg3crSpec>;
#[doc = "Field `SIG_ID` reader - DMA request trigger input selected"]
pub type SigIdR = crate::FieldReader;
#[doc = "Field `SIG_ID` writer - DMA request trigger input selected"]
pub type SigIdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OIE` reader - Interrupt enable at trigger event overrun"]
pub type OieR = crate::BitReader;
#[doc = "Field `OIE` writer - Interrupt enable at trigger event overrun"]
pub type OieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GE` reader - DMA request generator channel enable/disable"]
pub type GeR = crate::BitReader;
#[doc = "Field `GE` writer - DMA request generator channel enable/disable"]
pub type GeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPOL` reader - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
pub type GpolR = crate::FieldReader;
#[doc = "Field `GPOL` writer - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
pub type GpolW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GNBREQ` reader - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
pub type GnbreqR = crate::FieldReader;
#[doc = "Field `GNBREQ` writer - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
pub type GnbreqW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - DMA request trigger input selected"]
    #[inline(always)]
    pub fn sig_id(&self) -> SigIdR {
        SigIdR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Interrupt enable at trigger event overrun"]
    #[inline(always)]
    pub fn oie(&self) -> OieR {
        OieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA request generator channel enable/disable"]
    #[inline(always)]
    pub fn ge(&self) -> GeR {
        GeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
    #[inline(always)]
    pub fn gpol(&self) -> GpolR {
        GpolR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
    #[inline(always)]
    pub fn gnbreq(&self) -> GnbreqR {
        GnbreqR::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RG3CR")
            .field("sig_id", &self.sig_id())
            .field("oie", &self.oie())
            .field("ge", &self.ge())
            .field("gpol", &self.gpol())
            .field("gnbreq", &self.gnbreq())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA request trigger input selected"]
    #[inline(always)]
    pub fn sig_id(&mut self) -> SigIdW<Rg3crSpec> {
        SigIdW::new(self, 0)
    }
    #[doc = "Bit 8 - Interrupt enable at trigger event overrun"]
    #[inline(always)]
    pub fn oie(&mut self) -> OieW<Rg3crSpec> {
        OieW::new(self, 8)
    }
    #[doc = "Bit 16 - DMA request generator channel enable/disable"]
    #[inline(always)]
    pub fn ge(&mut self) -> GeW<Rg3crSpec> {
        GeW::new(self, 16)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
    #[inline(always)]
    pub fn gpol(&mut self) -> GpolW<Rg3crSpec> {
        GpolW::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
    #[inline(always)]
    pub fn gnbreq(&mut self) -> GnbreqW<Rg3crSpec> {
        GnbreqW::new(self, 19)
    }
}
#[doc = "DMAMux - DMA request generator channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg3cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg3cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rg3crSpec;
impl crate::RegisterSpec for Rg3crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rg3cr::R`](R) reader structure"]
impl crate::Readable for Rg3crSpec {}
#[doc = "`write(|w| ..)` method takes [`rg3cr::W`](W) writer structure"]
impl crate::Writable for Rg3crSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RG3CR to value 0"]
impl crate::Resettable for Rg3crSpec {
    const RESET_VALUE: u32 = 0;
}

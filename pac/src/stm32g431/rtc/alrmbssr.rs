#[doc = "Register `ALRMBSSR` reader"]
pub type R = crate::R<AlrmbssrSpec>;
#[doc = "Register `ALRMBSSR` writer"]
pub type W = crate::W<AlrmbssrSpec>;
#[doc = "Field `SS` reader - Sub seconds value"]
pub type SsR = crate::FieldReader<u16>;
#[doc = "Field `SS` writer - Sub seconds value"]
pub type SsW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `MASKSS` reader - Mask the most-significant bits starting at this bit"]
pub type MaskssR = crate::FieldReader;
#[doc = "Field `MASKSS` writer - Mask the most-significant bits starting at this bit"]
pub type MaskssW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:14 - Sub seconds value"]
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 24:27 - Mask the most-significant bits starting at this bit"]
    #[inline(always)]
    pub fn maskss(&self) -> MaskssR {
        MaskssR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALRMBSSR")
            .field("maskss", &self.maskss())
            .field("ss", &self.ss())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:14 - Sub seconds value"]
    #[inline(always)]
    pub fn ss(&mut self) -> SsW<AlrmbssrSpec> {
        SsW::new(self, 0)
    }
    #[doc = "Bits 24:27 - Mask the most-significant bits starting at this bit"]
    #[inline(always)]
    pub fn maskss(&mut self) -> MaskssW<AlrmbssrSpec> {
        MaskssW::new(self, 24)
    }
}
#[doc = "alarm B sub second register\n\nYou can [`read`](crate::Reg::read) this register and get [`alrmbssr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmbssr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlrmbssrSpec;
impl crate::RegisterSpec for AlrmbssrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alrmbssr::R`](R) reader structure"]
impl crate::Readable for AlrmbssrSpec {}
#[doc = "`write(|w| ..)` method takes [`alrmbssr::W`](W) writer structure"]
impl crate::Writable for AlrmbssrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALRMBSSR to value 0"]
impl crate::Resettable for AlrmbssrSpec {}

#[doc = "Register `ECR` reader"]
pub type R = crate::R<EcrSpec>;
#[doc = "Register `ECR` writer"]
pub type W = crate::W<EcrSpec>;
#[doc = "Field `IE` reader - Index Enable"]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - Index Enable"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDIR` reader - Index Direction"]
pub type IdirR = crate::FieldReader;
#[doc = "Field `IDIR` writer - Index Direction"]
pub type IdirW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IBLK` reader - Index Blanking"]
pub type IblkR = crate::FieldReader;
#[doc = "Field `IBLK` writer - Index Blanking"]
pub type IblkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIDX` reader - First Index"]
pub type FidxR = crate::BitReader;
#[doc = "Field `FIDX` writer - First Index"]
pub type FidxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPOS` reader - Index Positioning"]
pub type IposR = crate::FieldReader;
#[doc = "Field `IPOS` writer - Index Positioning"]
pub type IposW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PW` reader - Pulse width"]
pub type PwR = crate::FieldReader;
#[doc = "Field `PW` writer - Pulse width"]
pub type PwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PWPRSC` reader - Pulse Width prescaler"]
pub type PwprscR = crate::FieldReader;
#[doc = "Field `PWPRSC` writer - Pulse Width prescaler"]
pub type PwprscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Index Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Index Direction"]
    #[inline(always)]
    pub fn idir(&self) -> IdirR {
        IdirR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Index Blanking"]
    #[inline(always)]
    pub fn iblk(&self) -> IblkR {
        IblkR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - First Index"]
    #[inline(always)]
    pub fn fidx(&self) -> FidxR {
        FidxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Index Positioning"]
    #[inline(always)]
    pub fn ipos(&self) -> IposR {
        IposR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Pulse width"]
    #[inline(always)]
    pub fn pw(&self) -> PwR {
        PwR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - Pulse Width prescaler"]
    #[inline(always)]
    pub fn pwprsc(&self) -> PwprscR {
        PwprscR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECR")
            .field("ie", &self.ie())
            .field("idir", &self.idir())
            .field("iblk", &self.iblk())
            .field("fidx", &self.fidx())
            .field("ipos", &self.ipos())
            .field("pw", &self.pw())
            .field("pwprsc", &self.pwprsc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Index Enable"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<EcrSpec> {
        IeW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Index Direction"]
    #[inline(always)]
    pub fn idir(&mut self) -> IdirW<EcrSpec> {
        IdirW::new(self, 1)
    }
    #[doc = "Bits 3:4 - Index Blanking"]
    #[inline(always)]
    pub fn iblk(&mut self) -> IblkW<EcrSpec> {
        IblkW::new(self, 3)
    }
    #[doc = "Bit 5 - First Index"]
    #[inline(always)]
    pub fn fidx(&mut self) -> FidxW<EcrSpec> {
        FidxW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Index Positioning"]
    #[inline(always)]
    pub fn ipos(&mut self) -> IposW<EcrSpec> {
        IposW::new(self, 6)
    }
    #[doc = "Bits 16:23 - Pulse width"]
    #[inline(always)]
    pub fn pw(&mut self) -> PwW<EcrSpec> {
        PwW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Pulse Width prescaler"]
    #[inline(always)]
    pub fn pwprsc(&mut self) -> PwprscW<EcrSpec> {
        PwprscW::new(self, 24)
    }
}
#[doc = "DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcrSpec;
impl crate::RegisterSpec for EcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecr::R`](R) reader structure"]
impl crate::Readable for EcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ecr::W`](W) writer structure"]
impl crate::Writable for EcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for EcrSpec {
    const RESET_VALUE: u32 = 0;
}

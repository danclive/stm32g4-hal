#[doc = "Register `DTFR` reader"]
pub type R = crate::R<DtfrSpec>;
#[doc = "Register `DTFR` writer"]
pub type W = crate::W<DtfrSpec>;
#[doc = "Field `DTRx` reader - Deadtime Rising value"]
pub type DtrxR = crate::FieldReader<u16>;
#[doc = "Field `DTRx` writer - Deadtime Rising value"]
pub type DtrxW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SDTRx` reader - Sign Deadtime Rising value"]
pub type SdtrxR = crate::BitReader;
#[doc = "Field `SDTRx` writer - Sign Deadtime Rising value"]
pub type SdtrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTPRSC` reader - Deadtime Prescaler"]
pub type DtprscR = crate::FieldReader;
#[doc = "Field `DTPRSC` writer - Deadtime Prescaler"]
pub type DtprscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DTRSLKx` reader - Deadtime Rising Sign Lock"]
pub type DtrslkxR = crate::BitReader;
#[doc = "Field `DTRSLKx` writer - Deadtime Rising Sign Lock"]
pub type DtrslkxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTRLKx` reader - Deadtime Rising Lock"]
pub type DtrlkxR = crate::BitReader;
#[doc = "Field `DTRLKx` writer - Deadtime Rising Lock"]
pub type DtrlkxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTFx` reader - Deadtime Falling value"]
pub type DtfxR = crate::FieldReader<u16>;
#[doc = "Field `DTFx` writer - Deadtime Falling value"]
pub type DtfxW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SDTFx` reader - Sign Deadtime Falling value"]
pub type SdtfxR = crate::BitReader;
#[doc = "Field `SDTFx` writer - Sign Deadtime Falling value"]
pub type SdtfxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTFSLKx` reader - Deadtime Falling Sign Lock"]
pub type DtfslkxR = crate::BitReader;
#[doc = "Field `DTFSLKx` writer - Deadtime Falling Sign Lock"]
pub type DtfslkxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTFLKx` reader - Deadtime Falling Lock"]
pub type DtflkxR = crate::BitReader;
#[doc = "Field `DTFLKx` writer - Deadtime Falling Lock"]
pub type DtflkxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Deadtime Rising value"]
    #[inline(always)]
    pub fn dtrx(&self) -> DtrxR {
        DtrxR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Sign Deadtime Rising value"]
    #[inline(always)]
    pub fn sdtrx(&self) -> SdtrxR {
        SdtrxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Deadtime Prescaler"]
    #[inline(always)]
    pub fn dtprsc(&self) -> DtprscR {
        DtprscR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 14 - Deadtime Rising Sign Lock"]
    #[inline(always)]
    pub fn dtrslkx(&self) -> DtrslkxR {
        DtrslkxR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Deadtime Rising Lock"]
    #[inline(always)]
    pub fn dtrlkx(&self) -> DtrlkxR {
        DtrlkxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:24 - Deadtime Falling value"]
    #[inline(always)]
    pub fn dtfx(&self) -> DtfxR {
        DtfxR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 25 - Sign Deadtime Falling value"]
    #[inline(always)]
    pub fn sdtfx(&self) -> SdtfxR {
        SdtfxR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - Deadtime Falling Sign Lock"]
    #[inline(always)]
    pub fn dtfslkx(&self) -> DtfslkxR {
        DtfslkxR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Deadtime Falling Lock"]
    #[inline(always)]
    pub fn dtflkx(&self) -> DtflkxR {
        DtflkxR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTFR")
            .field("dtflkx", &self.dtflkx())
            .field("dtfslkx", &self.dtfslkx())
            .field("sdtfx", &self.sdtfx())
            .field("dtfx", &self.dtfx())
            .field("dtrlkx", &self.dtrlkx())
            .field("dtrslkx", &self.dtrslkx())
            .field("dtprsc", &self.dtprsc())
            .field("sdtrx", &self.sdtrx())
            .field("dtrx", &self.dtrx())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - Deadtime Rising value"]
    #[inline(always)]
    #[must_use]
    pub fn dtrx(&mut self) -> DtrxW<DtfrSpec> {
        DtrxW::new(self, 0)
    }
    #[doc = "Bit 9 - Sign Deadtime Rising value"]
    #[inline(always)]
    #[must_use]
    pub fn sdtrx(&mut self) -> SdtrxW<DtfrSpec> {
        SdtrxW::new(self, 9)
    }
    #[doc = "Bits 10:12 - Deadtime Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn dtprsc(&mut self) -> DtprscW<DtfrSpec> {
        DtprscW::new(self, 10)
    }
    #[doc = "Bit 14 - Deadtime Rising Sign Lock"]
    #[inline(always)]
    #[must_use]
    pub fn dtrslkx(&mut self) -> DtrslkxW<DtfrSpec> {
        DtrslkxW::new(self, 14)
    }
    #[doc = "Bit 15 - Deadtime Rising Lock"]
    #[inline(always)]
    #[must_use]
    pub fn dtrlkx(&mut self) -> DtrlkxW<DtfrSpec> {
        DtrlkxW::new(self, 15)
    }
    #[doc = "Bits 16:24 - Deadtime Falling value"]
    #[inline(always)]
    #[must_use]
    pub fn dtfx(&mut self) -> DtfxW<DtfrSpec> {
        DtfxW::new(self, 16)
    }
    #[doc = "Bit 25 - Sign Deadtime Falling value"]
    #[inline(always)]
    #[must_use]
    pub fn sdtfx(&mut self) -> SdtfxW<DtfrSpec> {
        SdtfxW::new(self, 25)
    }
    #[doc = "Bit 30 - Deadtime Falling Sign Lock"]
    #[inline(always)]
    #[must_use]
    pub fn dtfslkx(&mut self) -> DtfslkxW<DtfrSpec> {
        DtfslkxW::new(self, 30)
    }
    #[doc = "Bit 31 - Deadtime Falling Lock"]
    #[inline(always)]
    #[must_use]
    pub fn dtflkx(&mut self) -> DtflkxW<DtfrSpec> {
        DtflkxW::new(self, 31)
    }
}
#[doc = "Timerx Deadtime Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtfrSpec;
impl crate::RegisterSpec for DtfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtfr::R`](R) reader structure"]
impl crate::Readable for DtfrSpec {}
#[doc = "`write(|w| ..)` method takes [`dtfr::W`](W) writer structure"]
impl crate::Writable for DtfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTFR to value 0"]
impl crate::Resettable for DtfrSpec {
    const RESET_VALUE: u32 = 0;
}

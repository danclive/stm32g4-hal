#[doc = "Register `CHPFR` reader"]
pub type R = crate::R<ChpfrSpec>;
#[doc = "Register `CHPFR` writer"]
pub type W = crate::W<ChpfrSpec>;
#[doc = "Field `CHPFRQ` reader - Timerx carrier frequency value"]
pub type ChpfrqR = crate::FieldReader;
#[doc = "Field `CHPFRQ` writer - Timerx carrier frequency value"]
pub type ChpfrqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHPDTY` reader - Timerx chopper duty cycle value"]
pub type ChpdtyR = crate::FieldReader;
#[doc = "Field `CHPDTY` writer - Timerx chopper duty cycle value"]
pub type ChpdtyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STRTPW` reader - STRTPW"]
pub type StrtpwR = crate::FieldReader;
#[doc = "Field `STRTPW` writer - STRTPW"]
pub type StrtpwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Timerx carrier frequency value"]
    #[inline(always)]
    pub fn chpfrq(&self) -> ChpfrqR {
        ChpfrqR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Timerx chopper duty cycle value"]
    #[inline(always)]
    pub fn chpdty(&self) -> ChpdtyR {
        ChpdtyR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:10 - STRTPW"]
    #[inline(always)]
    pub fn strtpw(&self) -> StrtpwR {
        StrtpwR::new(((self.bits >> 7) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHPFR")
            .field("strtpw", &self.strtpw())
            .field("chpdty", &self.chpdty())
            .field("chpfrq", &self.chpfrq())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Timerx carrier frequency value"]
    #[inline(always)]
    #[must_use]
    pub fn chpfrq(&mut self) -> ChpfrqW<ChpfrSpec> {
        ChpfrqW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Timerx chopper duty cycle value"]
    #[inline(always)]
    #[must_use]
    pub fn chpdty(&mut self) -> ChpdtyW<ChpfrSpec> {
        ChpdtyW::new(self, 4)
    }
    #[doc = "Bits 7:10 - STRTPW"]
    #[inline(always)]
    #[must_use]
    pub fn strtpw(&mut self) -> StrtpwW<ChpfrSpec> {
        StrtpwW::new(self, 7)
    }
}
#[doc = "Timerx Chopper Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chpfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChpfrSpec;
impl crate::RegisterSpec for ChpfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chpfr::R`](R) reader structure"]
impl crate::Readable for ChpfrSpec {}
#[doc = "`write(|w| ..)` method takes [`chpfr::W`](W) writer structure"]
impl crate::Writable for ChpfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHPFR to value 0"]
impl crate::Resettable for ChpfrSpec {
    const RESET_VALUE: u32 = 0;
}

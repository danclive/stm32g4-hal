#[doc = "Register `CHPBR` reader"]
pub type R = crate::R<ChpbrSpec>;
#[doc = "Register `CHPBR` writer"]
pub type W = crate::W<ChpbrSpec>;
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
        f.debug_struct("CHPBR")
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
    pub fn chpfrq(&mut self) -> ChpfrqW<ChpbrSpec> {
        ChpfrqW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Timerx chopper duty cycle value"]
    #[inline(always)]
    #[must_use]
    pub fn chpdty(&mut self) -> ChpdtyW<ChpbrSpec> {
        ChpdtyW::new(self, 4)
    }
    #[doc = "Bits 7:10 - STRTPW"]
    #[inline(always)]
    #[must_use]
    pub fn strtpw(&mut self) -> StrtpwW<ChpbrSpec> {
        StrtpwW::new(self, 7)
    }
}
#[doc = "Timerx Chopper Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chpbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChpbrSpec;
impl crate::RegisterSpec for ChpbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chpbr::R`](R) reader structure"]
impl crate::Readable for ChpbrSpec {}
#[doc = "`write(|w| ..)` method takes [`chpbr::W`](W) writer structure"]
impl crate::Writable for ChpbrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHPBR to value 0"]
impl crate::Resettable for ChpbrSpec {
    const RESET_VALUE: u32 = 0;
}

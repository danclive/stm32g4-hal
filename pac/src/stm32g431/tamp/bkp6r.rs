#[doc = "Register `BKP6R` reader"]
pub type R = crate::R<Bkp6rSpec>;
#[doc = "Register `BKP6R` writer"]
pub type W = crate::W<Bkp6rSpec>;
#[doc = "Field `BKP` reader - BKP"]
pub type BkpR = crate::FieldReader<u32>;
#[doc = "Field `BKP` writer - BKP"]
pub type BkpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BkpR {
        BkpR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BKP6R").field("bkp", &self.bkp()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BkpW<Bkp6rSpec> {
        BkpW::new(self, 0)
    }
}
#[doc = "TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp6r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp6r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bkp6rSpec;
impl crate::RegisterSpec for Bkp6rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp6r::R`](R) reader structure"]
impl crate::Readable for Bkp6rSpec {}
#[doc = "`write(|w| ..)` method takes [`bkp6r::W`](W) writer structure"]
impl crate::Writable for Bkp6rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BKP6R to value 0"]
impl crate::Resettable for Bkp6rSpec {}

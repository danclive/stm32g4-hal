#[doc = "Register `BKP5R` reader"]
pub type R = crate::R<Bkp5rSpec>;
#[doc = "Register `BKP5R` writer"]
pub type W = crate::W<Bkp5rSpec>;
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
        f.debug_struct("BKP5R").field("bkp", &self.bkp()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BkpW<Bkp5rSpec> {
        BkpW::new(self, 0)
    }
}
#[doc = "TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp5r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp5r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bkp5rSpec;
impl crate::RegisterSpec for Bkp5rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp5r::R`](R) reader structure"]
impl crate::Readable for Bkp5rSpec {}
#[doc = "`write(|w| ..)` method takes [`bkp5r::W`](W) writer structure"]
impl crate::Writable for Bkp5rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BKP5R to value 0"]
impl crate::Resettable for Bkp5rSpec {}

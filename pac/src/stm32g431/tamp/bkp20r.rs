#[doc = "Register `BKP20R` reader"]
pub type R = crate::R<Bkp20rSpec>;
#[doc = "Register `BKP20R` writer"]
pub type W = crate::W<Bkp20rSpec>;
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
        f.debug_struct("BKP20R").field("bkp", &self.bkp()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BkpW<Bkp20rSpec> {
        BkpW::new(self, 0)
    }
}
#[doc = "TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp20r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp20r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bkp20rSpec;
impl crate::RegisterSpec for Bkp20rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp20r::R`](R) reader structure"]
impl crate::Readable for Bkp20rSpec {}
#[doc = "`write(|w| ..)` method takes [`bkp20r::W`](W) writer structure"]
impl crate::Writable for Bkp20rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BKP20R to value 0"]
impl crate::Resettable for Bkp20rSpec {
    const RESET_VALUE: u32 = 0;
}

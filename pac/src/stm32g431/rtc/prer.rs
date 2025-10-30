#[doc = "Register `PRER` reader"]
pub type R = crate::R<PrerSpec>;
#[doc = "Register `PRER` writer"]
pub type W = crate::W<PrerSpec>;
#[doc = "Field `PREDIV_S` reader - Synchronous prescaler factor"]
pub type PredivSR = crate::FieldReader<u16>;
#[doc = "Field `PREDIV_S` writer - Synchronous prescaler factor"]
pub type PredivSW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `PREDIV_A` reader - Asynchronous prescaler factor"]
pub type PredivAR = crate::FieldReader;
#[doc = "Field `PREDIV_A` writer - Asynchronous prescaler factor"]
pub type PredivAW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:14 - Synchronous prescaler factor"]
    #[inline(always)]
    pub fn prediv_s(&self) -> PredivSR {
        PredivSR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:22 - Asynchronous prescaler factor"]
    #[inline(always)]
    pub fn prediv_a(&self) -> PredivAR {
        PredivAR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRER")
            .field("prediv_a", &self.prediv_a())
            .field("prediv_s", &self.prediv_s())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:14 - Synchronous prescaler factor"]
    #[inline(always)]
    pub fn prediv_s(&mut self) -> PredivSW<'_, PrerSpec> {
        PredivSW::new(self, 0)
    }
    #[doc = "Bits 16:22 - Asynchronous prescaler factor"]
    #[inline(always)]
    pub fn prediv_a(&mut self) -> PredivAW<'_, PrerSpec> {
        PredivAW::new(self, 16)
    }
}
#[doc = "prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`prer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrerSpec;
impl crate::RegisterSpec for PrerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prer::R`](R) reader structure"]
impl crate::Readable for PrerSpec {}
#[doc = "`write(|w| ..)` method takes [`prer::W`](W) writer structure"]
impl crate::Writable for PrerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRER to value 0x007f_00ff"]
impl crate::Resettable for PrerSpec {
    const RESET_VALUE: u32 = 0x007f_00ff;
}

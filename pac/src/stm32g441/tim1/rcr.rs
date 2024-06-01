#[doc = "Register `RCR` reader"]
pub type R = crate::R<RcrSpec>;
#[doc = "Register `RCR` writer"]
pub type W = crate::W<RcrSpec>;
#[doc = "Field `REP` reader - Repetition counter value"]
pub type RepR = crate::FieldReader<u16>;
#[doc = "Field `REP` writer - Repetition counter value"]
pub type RepW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Repetition counter value"]
    #[inline(always)]
    pub fn rep(&self) -> RepR {
        RepR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCR").field("rep", &self.rep()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Repetition counter value"]
    #[inline(always)]
    #[must_use]
    pub fn rep(&mut self) -> RepW<RcrSpec> {
        RepW::new(self, 0)
    }
}
#[doc = "repetition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcrSpec;
impl crate::RegisterSpec for RcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcr::R`](R) reader structure"]
impl crate::Readable for RcrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcr::W`](W) writer structure"]
impl crate::Writable for RcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCR to value 0"]
impl crate::Resettable for RcrSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `MREP` reader"]
pub type R = crate::R<MrepSpec>;
#[doc = "Register `MREP` writer"]
pub type W = crate::W<MrepSpec>;
#[doc = "Field `MREP` reader - Master Timer Repetition counter value"]
pub type MrepR = crate::FieldReader;
#[doc = "Field `MREP` writer - Master Timer Repetition counter value"]
pub type MrepW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Master Timer Repetition counter value"]
    #[inline(always)]
    pub fn mrep(&self) -> MrepR {
        MrepR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MREP").field("mrep", &self.mrep()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Master Timer Repetition counter value"]
    #[inline(always)]
    #[must_use]
    pub fn mrep(&mut self) -> MrepW<MrepSpec> {
        MrepW::new(self, 0)
    }
}
#[doc = "Master Timer Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrep::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrep::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrepSpec;
impl crate::RegisterSpec for MrepSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrep::R`](R) reader structure"]
impl crate::Readable for MrepSpec {}
#[doc = "`write(|w| ..)` method takes [`mrep::W`](W) writer structure"]
impl crate::Writable for MrepSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MREP to value 0"]
impl crate::Resettable for MrepSpec {
    const RESET_VALUE: u32 = 0;
}

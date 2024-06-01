#[doc = "Register `CMP4CR` reader"]
pub type R = crate::R<Cmp4crSpec>;
#[doc = "Register `CMP4CR` writer"]
pub type W = crate::W<Cmp4crSpec>;
#[doc = "Field `CMP4x` reader - Timerx Compare 4 value"]
pub type Cmp4xR = crate::FieldReader<u16>;
#[doc = "Field `CMP4x` writer - Timerx Compare 4 value"]
pub type Cmp4xW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 4 value"]
    #[inline(always)]
    pub fn cmp4x(&self) -> Cmp4xR {
        Cmp4xR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMP4CR")
            .field("cmp4x", &self.cmp4x())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 4 value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp4x(&mut self) -> Cmp4xW<Cmp4crSpec> {
        Cmp4xW::new(self, 0)
    }
}
#[doc = "Timerx Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp4cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp4cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp4crSpec;
impl crate::RegisterSpec for Cmp4crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp4cr::R`](R) reader structure"]
impl crate::Readable for Cmp4crSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp4cr::W`](W) writer structure"]
impl crate::Writable for Cmp4crSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP4CR to value 0"]
impl crate::Resettable for Cmp4crSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CMP1CR` reader"]
pub type R = crate::R<Cmp1crSpec>;
#[doc = "Register `CMP1CR` writer"]
pub type W = crate::W<Cmp1crSpec>;
#[doc = "Field `CMP1x` reader - Timerx Compare 1 value"]
pub type Cmp1xR = crate::FieldReader<u16>;
#[doc = "Field `CMP1x` writer - Timerx Compare 1 value"]
pub type Cmp1xW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 1 value"]
    #[inline(always)]
    pub fn cmp1x(&self) -> Cmp1xR {
        Cmp1xR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMP1CR")
            .field("cmp1x", &self.cmp1x())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 1 value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1x(&mut self) -> Cmp1xW<Cmp1crSpec> {
        Cmp1xW::new(self, 0)
    }
}
#[doc = "Timerx Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp1crSpec;
impl crate::RegisterSpec for Cmp1crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp1cr::R`](R) reader structure"]
impl crate::Readable for Cmp1crSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp1cr::W`](W) writer structure"]
impl crate::Writable for Cmp1crSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP1CR to value 0"]
impl crate::Resettable for Cmp1crSpec {
    const RESET_VALUE: u32 = 0;
}

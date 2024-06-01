#[doc = "Register `CMP2DR` reader"]
pub type R = crate::R<Cmp2drSpec>;
#[doc = "Register `CMP2DR` writer"]
pub type W = crate::W<Cmp2drSpec>;
#[doc = "Field `CMP2x` reader - Timerx Compare 2 value"]
pub type Cmp2xR = crate::FieldReader<u16>;
#[doc = "Field `CMP2x` writer - Timerx Compare 2 value"]
pub type Cmp2xW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 2 value"]
    #[inline(always)]
    pub fn cmp2x(&self) -> Cmp2xR {
        Cmp2xR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMP2DR")
            .field("cmp2x", &self.cmp2x())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 2 value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2x(&mut self) -> Cmp2xW<Cmp2drSpec> {
        Cmp2xW::new(self, 0)
    }
}
#[doc = "Timerx Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp2dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp2dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp2drSpec;
impl crate::RegisterSpec for Cmp2drSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp2dr::R`](R) reader structure"]
impl crate::Readable for Cmp2drSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp2dr::W`](W) writer structure"]
impl crate::Writable for Cmp2drSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP2DR to value 0"]
impl crate::Resettable for Cmp2drSpec {
    const RESET_VALUE: u32 = 0;
}

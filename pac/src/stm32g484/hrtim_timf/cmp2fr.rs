#[doc = "Register `CMP2FR` reader"]
pub type R = crate::R<Cmp2frSpec>;
#[doc = "Register `CMP2FR` writer"]
pub type W = crate::W<Cmp2frSpec>;
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
        f.debug_struct("CMP2FR")
            .field("cmp2x", &self.cmp2x())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 2 value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2x(&mut self) -> Cmp2xW<Cmp2frSpec> {
        Cmp2xW::new(self, 0)
    }
}
#[doc = "Timerx Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp2fr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp2fr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp2frSpec;
impl crate::RegisterSpec for Cmp2frSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp2fr::R`](R) reader structure"]
impl crate::Readable for Cmp2frSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp2fr::W`](W) writer structure"]
impl crate::Writable for Cmp2frSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP2FR to value 0"]
impl crate::Resettable for Cmp2frSpec {
    const RESET_VALUE: u32 = 0;
}

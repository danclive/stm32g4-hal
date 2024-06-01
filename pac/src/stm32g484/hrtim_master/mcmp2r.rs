#[doc = "Register `MCMP2R` reader"]
pub type R = crate::R<Mcmp2rSpec>;
#[doc = "Register `MCMP2R` writer"]
pub type W = crate::W<Mcmp2rSpec>;
#[doc = "Field `MCMP2` reader - Master Timer Compare 2 value"]
pub type Mcmp2R = crate::FieldReader<u16>;
#[doc = "Field `MCMP2` writer - Master Timer Compare 2 value"]
pub type Mcmp2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Master Timer Compare 2 value"]
    #[inline(always)]
    pub fn mcmp2(&self) -> Mcmp2R {
        Mcmp2R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCMP2R")
            .field("mcmp2", &self.mcmp2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Timer Compare 2 value"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp2(&mut self) -> Mcmp2W<Mcmp2rSpec> {
        Mcmp2W::new(self, 0)
    }
}
#[doc = "Master Timer Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcmp2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcmp2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcmp2rSpec;
impl crate::RegisterSpec for Mcmp2rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcmp2r::R`](R) reader structure"]
impl crate::Readable for Mcmp2rSpec {}
#[doc = "`write(|w| ..)` method takes [`mcmp2r::W`](W) writer structure"]
impl crate::Writable for Mcmp2rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCMP2R to value 0"]
impl crate::Resettable for Mcmp2rSpec {
    const RESET_VALUE: u32 = 0;
}

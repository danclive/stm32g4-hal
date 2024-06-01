#[doc = "Register `MCMP1R` reader"]
pub type R = crate::R<Mcmp1rSpec>;
#[doc = "Register `MCMP1R` writer"]
pub type W = crate::W<Mcmp1rSpec>;
#[doc = "Field `MCMP1` reader - Master Timer Compare 1 value"]
pub type Mcmp1R = crate::FieldReader<u16>;
#[doc = "Field `MCMP1` writer - Master Timer Compare 1 value"]
pub type Mcmp1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Master Timer Compare 1 value"]
    #[inline(always)]
    pub fn mcmp1(&self) -> Mcmp1R {
        Mcmp1R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCMP1R")
            .field("mcmp1", &self.mcmp1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Timer Compare 1 value"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp1(&mut self) -> Mcmp1W<Mcmp1rSpec> {
        Mcmp1W::new(self, 0)
    }
}
#[doc = "Master Timer Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcmp1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcmp1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcmp1rSpec;
impl crate::RegisterSpec for Mcmp1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcmp1r::R`](R) reader structure"]
impl crate::Readable for Mcmp1rSpec {}
#[doc = "`write(|w| ..)` method takes [`mcmp1r::W`](W) writer structure"]
impl crate::Writable for Mcmp1rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCMP1R to value 0"]
impl crate::Resettable for Mcmp1rSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `MCMP4R` reader"]
pub type R = crate::R<Mcmp4rSpec>;
#[doc = "Register `MCMP4R` writer"]
pub type W = crate::W<Mcmp4rSpec>;
#[doc = "Field `MCMP4` reader - Master Timer Compare 4 value"]
pub type Mcmp4R = crate::FieldReader<u16>;
#[doc = "Field `MCMP4` writer - Master Timer Compare 4 value"]
pub type Mcmp4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Master Timer Compare 4 value"]
    #[inline(always)]
    pub fn mcmp4(&self) -> Mcmp4R {
        Mcmp4R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCMP4R")
            .field("mcmp4", &self.mcmp4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Timer Compare 4 value"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp4(&mut self) -> Mcmp4W<Mcmp4rSpec> {
        Mcmp4W::new(self, 0)
    }
}
#[doc = "Master Timer Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcmp4r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcmp4r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcmp4rSpec;
impl crate::RegisterSpec for Mcmp4rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcmp4r::R`](R) reader structure"]
impl crate::Readable for Mcmp4rSpec {}
#[doc = "`write(|w| ..)` method takes [`mcmp4r::W`](W) writer structure"]
impl crate::Writable for Mcmp4rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCMP4R to value 0"]
impl crate::Resettable for Mcmp4rSpec {
    const RESET_VALUE: u32 = 0;
}

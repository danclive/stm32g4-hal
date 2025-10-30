#[doc = "Register `FTSR2` reader"]
pub type R = crate::R<Ftsr2Spec>;
#[doc = "Register `FTSR2` writer"]
pub type W = crate::W<Ftsr2Spec>;
#[doc = "Field `FT35` reader - Falling trigger event configuration bit of line 35"]
pub type Ft35R = crate::BitReader;
#[doc = "Field `FT35` writer - Falling trigger event configuration bit of line 35"]
pub type Ft35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT36` reader - Falling trigger event configuration bit of line 36"]
pub type Ft36R = crate::BitReader;
#[doc = "Field `FT36` writer - Falling trigger event configuration bit of line 36"]
pub type Ft36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT37` reader - Falling trigger event configuration bit of line 37"]
pub type Ft37R = crate::BitReader;
#[doc = "Field `FT37` writer - Falling trigger event configuration bit of line 37"]
pub type Ft37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT38` reader - Falling trigger event configuration bit of line 38"]
pub type Ft38R = crate::BitReader;
#[doc = "Field `FT38` writer - Falling trigger event configuration bit of line 38"]
pub type Ft38W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Falling trigger event configuration bit of line 35"]
    #[inline(always)]
    pub fn ft35(&self) -> Ft35R {
        Ft35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling trigger event configuration bit of line 36"]
    #[inline(always)]
    pub fn ft36(&self) -> Ft36R {
        Ft36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling trigger event configuration bit of line 37"]
    #[inline(always)]
    pub fn ft37(&self) -> Ft37R {
        Ft37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Falling trigger event configuration bit of line 38"]
    #[inline(always)]
    pub fn ft38(&self) -> Ft38R {
        Ft38R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FTSR2")
            .field("ft35", &self.ft35())
            .field("ft36", &self.ft36())
            .field("ft37", &self.ft37())
            .field("ft38", &self.ft38())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - Falling trigger event configuration bit of line 35"]
    #[inline(always)]
    pub fn ft35(&mut self) -> Ft35W<'_, Ftsr2Spec> {
        Ft35W::new(self, 3)
    }
    #[doc = "Bit 4 - Falling trigger event configuration bit of line 36"]
    #[inline(always)]
    pub fn ft36(&mut self) -> Ft36W<'_, Ftsr2Spec> {
        Ft36W::new(self, 4)
    }
    #[doc = "Bit 5 - Falling trigger event configuration bit of line 37"]
    #[inline(always)]
    pub fn ft37(&mut self) -> Ft37W<'_, Ftsr2Spec> {
        Ft37W::new(self, 5)
    }
    #[doc = "Bit 6 - Falling trigger event configuration bit of line 38"]
    #[inline(always)]
    pub fn ft38(&mut self) -> Ft38W<'_, Ftsr2Spec> {
        Ft38W::new(self, 6)
    }
}
#[doc = "Falling Trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`ftsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ftsr2Spec;
impl crate::RegisterSpec for Ftsr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftsr2::R`](R) reader structure"]
impl crate::Readable for Ftsr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ftsr2::W`](W) writer structure"]
impl crate::Writable for Ftsr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FTSR2 to value 0"]
impl crate::Resettable for Ftsr2Spec {}

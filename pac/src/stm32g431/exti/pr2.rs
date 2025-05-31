#[doc = "Register `PR2` reader"]
pub type R = crate::R<Pr2Spec>;
#[doc = "Register `PR2` writer"]
pub type W = crate::W<Pr2Spec>;
#[doc = "Field `PIF35` reader - Pending interrupt flag on line 35"]
pub type Pif35R = crate::BitReader;
#[doc = "Field `PIF35` writer - Pending interrupt flag on line 35"]
pub type Pif35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF36` reader - Pending interrupt flag on line 36"]
pub type Pif36R = crate::BitReader;
#[doc = "Field `PIF36` writer - Pending interrupt flag on line 36"]
pub type Pif36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF37` reader - Pending interrupt flag on line 37"]
pub type Pif37R = crate::BitReader;
#[doc = "Field `PIF37` writer - Pending interrupt flag on line 37"]
pub type Pif37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF38` reader - Pending interrupt flag on line 38"]
pub type Pif38R = crate::BitReader;
#[doc = "Field `PIF38` writer - Pending interrupt flag on line 38"]
pub type Pif38W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Pending interrupt flag on line 35"]
    #[inline(always)]
    pub fn pif35(&self) -> Pif35R {
        Pif35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pending interrupt flag on line 36"]
    #[inline(always)]
    pub fn pif36(&self) -> Pif36R {
        Pif36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pending interrupt flag on line 37"]
    #[inline(always)]
    pub fn pif37(&self) -> Pif37R {
        Pif37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pending interrupt flag on line 38"]
    #[inline(always)]
    pub fn pif38(&self) -> Pif38R {
        Pif38R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PR2")
            .field("pif35", &self.pif35())
            .field("pif36", &self.pif36())
            .field("pif37", &self.pif37())
            .field("pif38", &self.pif38())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - Pending interrupt flag on line 35"]
    #[inline(always)]
    pub fn pif35(&mut self) -> Pif35W<Pr2Spec> {
        Pif35W::new(self, 3)
    }
    #[doc = "Bit 4 - Pending interrupt flag on line 36"]
    #[inline(always)]
    pub fn pif36(&mut self) -> Pif36W<Pr2Spec> {
        Pif36W::new(self, 4)
    }
    #[doc = "Bit 5 - Pending interrupt flag on line 37"]
    #[inline(always)]
    pub fn pif37(&mut self) -> Pif37W<Pr2Spec> {
        Pif37W::new(self, 5)
    }
    #[doc = "Bit 6 - Pending interrupt flag on line 38"]
    #[inline(always)]
    pub fn pif38(&mut self) -> Pif38W<Pr2Spec> {
        Pif38W::new(self, 6)
    }
}
#[doc = "Pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`pr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr2Spec;
impl crate::RegisterSpec for Pr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr2::R`](R) reader structure"]
impl crate::Readable for Pr2Spec {}
#[doc = "`write(|w| ..)` method takes [`pr2::W`](W) writer structure"]
impl crate::Writable for Pr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PR2 to value 0"]
impl crate::Resettable for Pr2Spec {}

#[doc = "Register `SWIER2` reader"]
pub type R = crate::R<Swier2Spec>;
#[doc = "Register `SWIER2` writer"]
pub type W = crate::W<Swier2Spec>;
#[doc = "Field `SWI35` reader - Software interrupt on line 35"]
pub type Swi35R = crate::BitReader;
#[doc = "Field `SWI35` writer - Software interrupt on line 35"]
pub type Swi35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI36` reader - Software interrupt on line 36"]
pub type Swi36R = crate::BitReader;
#[doc = "Field `SWI36` writer - Software interrupt on line 36"]
pub type Swi36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI37` reader - Software interrupt on line 37"]
pub type Swi37R = crate::BitReader;
#[doc = "Field `SWI37` writer - Software interrupt on line 37"]
pub type Swi37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI38` reader - Software interrupt on line 38"]
pub type Swi38R = crate::BitReader;
#[doc = "Field `SWI38` writer - Software interrupt on line 38"]
pub type Swi38W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Software interrupt on line 35"]
    #[inline(always)]
    pub fn swi35(&self) -> Swi35R {
        Swi35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software interrupt on line 36"]
    #[inline(always)]
    pub fn swi36(&self) -> Swi36R {
        Swi36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software interrupt on line 37"]
    #[inline(always)]
    pub fn swi37(&self) -> Swi37R {
        Swi37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software interrupt on line 38"]
    #[inline(always)]
    pub fn swi38(&self) -> Swi38R {
        Swi38R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWIER2")
            .field("swi35", &self.swi35())
            .field("swi36", &self.swi36())
            .field("swi37", &self.swi37())
            .field("swi38", &self.swi38())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - Software interrupt on line 35"]
    #[inline(always)]
    #[must_use]
    pub fn swi35(&mut self) -> Swi35W<Swier2Spec> {
        Swi35W::new(self, 3)
    }
    #[doc = "Bit 4 - Software interrupt on line 36"]
    #[inline(always)]
    #[must_use]
    pub fn swi36(&mut self) -> Swi36W<Swier2Spec> {
        Swi36W::new(self, 4)
    }
    #[doc = "Bit 5 - Software interrupt on line 37"]
    #[inline(always)]
    #[must_use]
    pub fn swi37(&mut self) -> Swi37W<Swier2Spec> {
        Swi37W::new(self, 5)
    }
    #[doc = "Bit 6 - Software interrupt on line 38"]
    #[inline(always)]
    #[must_use]
    pub fn swi38(&mut self) -> Swi38W<Swier2Spec> {
        Swi38W::new(self, 6)
    }
}
#[doc = "Software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swier2Spec;
impl crate::RegisterSpec for Swier2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swier2::R`](R) reader structure"]
impl crate::Readable for Swier2Spec {}
#[doc = "`write(|w| ..)` method takes [`swier2::W`](W) writer structure"]
impl crate::Writable for Swier2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWIER2 to value 0"]
impl crate::Resettable for Swier2Spec {
    const RESET_VALUE: u32 = 0;
}

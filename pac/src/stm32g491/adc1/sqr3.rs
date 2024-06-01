#[doc = "Register `SQR3` reader"]
pub type R = crate::R<Sqr3Spec>;
#[doc = "Register `SQR3` writer"]
pub type W = crate::W<Sqr3Spec>;
#[doc = "Field `SQ10` reader - SQ10"]
pub type Sq10R = crate::FieldReader;
#[doc = "Field `SQ10` writer - SQ10"]
pub type Sq10W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ11` reader - SQ11"]
pub type Sq11R = crate::FieldReader;
#[doc = "Field `SQ11` writer - SQ11"]
pub type Sq11W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ12` reader - SQ12"]
pub type Sq12R = crate::FieldReader;
#[doc = "Field `SQ12` writer - SQ12"]
pub type Sq12W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ13` reader - SQ13"]
pub type Sq13R = crate::FieldReader;
#[doc = "Field `SQ13` writer - SQ13"]
pub type Sq13W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ14` reader - SQ14"]
pub type Sq14R = crate::FieldReader;
#[doc = "Field `SQ14` writer - SQ14"]
pub type Sq14W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - SQ10"]
    #[inline(always)]
    pub fn sq10(&self) -> Sq10R {
        Sq10R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - SQ11"]
    #[inline(always)]
    pub fn sq11(&self) -> Sq11R {
        Sq11R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - SQ12"]
    #[inline(always)]
    pub fn sq12(&self) -> Sq12R {
        Sq12R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - SQ13"]
    #[inline(always)]
    pub fn sq13(&self) -> Sq13R {
        Sq13R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - SQ14"]
    #[inline(always)]
    pub fn sq14(&self) -> Sq14R {
        Sq14R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SQR3")
            .field("sq14", &self.sq14())
            .field("sq13", &self.sq13())
            .field("sq12", &self.sq12())
            .field("sq11", &self.sq11())
            .field("sq10", &self.sq10())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - SQ10"]
    #[inline(always)]
    #[must_use]
    pub fn sq10(&mut self) -> Sq10W<Sqr3Spec> {
        Sq10W::new(self, 0)
    }
    #[doc = "Bits 6:10 - SQ11"]
    #[inline(always)]
    #[must_use]
    pub fn sq11(&mut self) -> Sq11W<Sqr3Spec> {
        Sq11W::new(self, 6)
    }
    #[doc = "Bits 12:16 - SQ12"]
    #[inline(always)]
    #[must_use]
    pub fn sq12(&mut self) -> Sq12W<Sqr3Spec> {
        Sq12W::new(self, 12)
    }
    #[doc = "Bits 18:22 - SQ13"]
    #[inline(always)]
    #[must_use]
    pub fn sq13(&mut self) -> Sq13W<Sqr3Spec> {
        Sq13W::new(self, 18)
    }
    #[doc = "Bits 24:28 - SQ14"]
    #[inline(always)]
    #[must_use]
    pub fn sq14(&mut self) -> Sq14W<Sqr3Spec> {
        Sq14W::new(self, 24)
    }
}
#[doc = "regular sequence register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sqr3Spec;
impl crate::RegisterSpec for Sqr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr3::R`](R) reader structure"]
impl crate::Readable for Sqr3Spec {}
#[doc = "`write(|w| ..)` method takes [`sqr3::W`](W) writer structure"]
impl crate::Writable for Sqr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SQR3 to value 0"]
impl crate::Resettable for Sqr3Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `SMPR2` reader"]
pub type R = crate::R<Smpr2Spec>;
#[doc = "Register `SMPR2` writer"]
pub type W = crate::W<Smpr2Spec>;
#[doc = "Field `SMP10` reader - SMP10"]
pub type Smp10R = crate::FieldReader;
#[doc = "Field `SMP10` writer - SMP10"]
pub type Smp10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP11` reader - SMP11"]
pub type Smp11R = crate::FieldReader;
#[doc = "Field `SMP11` writer - SMP11"]
pub type Smp11W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP12` reader - SMP12"]
pub type Smp12R = crate::FieldReader;
#[doc = "Field `SMP12` writer - SMP12"]
pub type Smp12W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP13` reader - SMP13"]
pub type Smp13R = crate::FieldReader;
#[doc = "Field `SMP13` writer - SMP13"]
pub type Smp13W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP14` reader - SMP14"]
pub type Smp14R = crate::FieldReader;
#[doc = "Field `SMP14` writer - SMP14"]
pub type Smp14W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP15` reader - SMP15"]
pub type Smp15R = crate::FieldReader;
#[doc = "Field `SMP15` writer - SMP15"]
pub type Smp15W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP16` reader - SMP16"]
pub type Smp16R = crate::FieldReader;
#[doc = "Field `SMP16` writer - SMP16"]
pub type Smp16W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP17` reader - SMP17"]
pub type Smp17R = crate::FieldReader;
#[doc = "Field `SMP17` writer - SMP17"]
pub type Smp17W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP18` reader - SMP18"]
pub type Smp18R = crate::FieldReader;
#[doc = "Field `SMP18` writer - SMP18"]
pub type Smp18W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SMP10"]
    #[inline(always)]
    pub fn smp10(&self) -> Smp10R {
        Smp10R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - SMP11"]
    #[inline(always)]
    pub fn smp11(&self) -> Smp11R {
        Smp11R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - SMP12"]
    #[inline(always)]
    pub fn smp12(&self) -> Smp12R {
        Smp12R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - SMP13"]
    #[inline(always)]
    pub fn smp13(&self) -> Smp13R {
        Smp13R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - SMP14"]
    #[inline(always)]
    pub fn smp14(&self) -> Smp14R {
        Smp14R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - SMP15"]
    #[inline(always)]
    pub fn smp15(&self) -> Smp15R {
        Smp15R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - SMP16"]
    #[inline(always)]
    pub fn smp16(&self) -> Smp16R {
        Smp16R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - SMP17"]
    #[inline(always)]
    pub fn smp17(&self) -> Smp17R {
        Smp17R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - SMP18"]
    #[inline(always)]
    pub fn smp18(&self) -> Smp18R {
        Smp18R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR2")
            .field("smp18", &self.smp18())
            .field("smp17", &self.smp17())
            .field("smp16", &self.smp16())
            .field("smp15", &self.smp15())
            .field("smp14", &self.smp14())
            .field("smp13", &self.smp13())
            .field("smp12", &self.smp12())
            .field("smp11", &self.smp11())
            .field("smp10", &self.smp10())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - SMP10"]
    #[inline(always)]
    pub fn smp10(&mut self) -> Smp10W<'_, Smpr2Spec> {
        Smp10W::new(self, 0)
    }
    #[doc = "Bits 3:5 - SMP11"]
    #[inline(always)]
    pub fn smp11(&mut self) -> Smp11W<'_, Smpr2Spec> {
        Smp11W::new(self, 3)
    }
    #[doc = "Bits 6:8 - SMP12"]
    #[inline(always)]
    pub fn smp12(&mut self) -> Smp12W<'_, Smpr2Spec> {
        Smp12W::new(self, 6)
    }
    #[doc = "Bits 9:11 - SMP13"]
    #[inline(always)]
    pub fn smp13(&mut self) -> Smp13W<'_, Smpr2Spec> {
        Smp13W::new(self, 9)
    }
    #[doc = "Bits 12:14 - SMP14"]
    #[inline(always)]
    pub fn smp14(&mut self) -> Smp14W<'_, Smpr2Spec> {
        Smp14W::new(self, 12)
    }
    #[doc = "Bits 15:17 - SMP15"]
    #[inline(always)]
    pub fn smp15(&mut self) -> Smp15W<'_, Smpr2Spec> {
        Smp15W::new(self, 15)
    }
    #[doc = "Bits 18:20 - SMP16"]
    #[inline(always)]
    pub fn smp16(&mut self) -> Smp16W<'_, Smpr2Spec> {
        Smp16W::new(self, 18)
    }
    #[doc = "Bits 21:23 - SMP17"]
    #[inline(always)]
    pub fn smp17(&mut self) -> Smp17W<'_, Smpr2Spec> {
        Smp17W::new(self, 21)
    }
    #[doc = "Bits 24:26 - SMP18"]
    #[inline(always)]
    pub fn smp18(&mut self) -> Smp18W<'_, Smpr2Spec> {
        Smp18W::new(self, 24)
    }
}
#[doc = "sample time register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`smpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Smpr2Spec;
impl crate::RegisterSpec for Smpr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr2::R`](R) reader structure"]
impl crate::Readable for Smpr2Spec {}
#[doc = "`write(|w| ..)` method takes [`smpr2::W`](W) writer structure"]
impl crate::Writable for Smpr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMPR2 to value 0"]
impl crate::Resettable for Smpr2Spec {}

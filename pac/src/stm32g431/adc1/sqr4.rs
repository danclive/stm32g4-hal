#[doc = "Register `SQR4` reader"]
pub type R = crate::R<Sqr4Spec>;
#[doc = "Register `SQR4` writer"]
pub type W = crate::W<Sqr4Spec>;
#[doc = "Field `SQ15` reader - SQ15"]
pub type Sq15R = crate::FieldReader;
#[doc = "Field `SQ15` writer - SQ15"]
pub type Sq15W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ16` reader - SQ16"]
pub type Sq16R = crate::FieldReader;
#[doc = "Field `SQ16` writer - SQ16"]
pub type Sq16W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - SQ15"]
    #[inline(always)]
    pub fn sq15(&self) -> Sq15R {
        Sq15R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - SQ16"]
    #[inline(always)]
    pub fn sq16(&self) -> Sq16R {
        Sq16R::new(((self.bits >> 6) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SQR4")
            .field("sq16", &self.sq16())
            .field("sq15", &self.sq15())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - SQ15"]
    #[inline(always)]
    pub fn sq15(&mut self) -> Sq15W<Sqr4Spec> {
        Sq15W::new(self, 0)
    }
    #[doc = "Bits 6:10 - SQ16"]
    #[inline(always)]
    pub fn sq16(&mut self) -> Sq16W<Sqr4Spec> {
        Sq16W::new(self, 6)
    }
}
#[doc = "regular sequence register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sqr4Spec;
impl crate::RegisterSpec for Sqr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr4::R`](R) reader structure"]
impl crate::Readable for Sqr4Spec {}
#[doc = "`write(|w| ..)` method takes [`sqr4::W`](W) writer structure"]
impl crate::Writable for Sqr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SQR4 to value 0"]
impl crate::Resettable for Sqr4Spec {
    const RESET_VALUE: u32 = 0;
}

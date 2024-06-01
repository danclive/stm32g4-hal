#[doc = "Register `EECR3` reader"]
pub type R = crate::R<Eecr3Spec>;
#[doc = "Register `EECR3` writer"]
pub type W = crate::W<Eecr3Spec>;
#[doc = "Field `EE6F` reader - EE6F"]
pub type Ee6fR = crate::FieldReader;
#[doc = "Field `EE6F` writer - EE6F"]
pub type Ee6fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EE7F` reader - EE7F"]
pub type Ee7fR = crate::FieldReader;
#[doc = "Field `EE7F` writer - EE7F"]
pub type Ee7fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EE8F` reader - EE8F"]
pub type Ee8fR = crate::FieldReader;
#[doc = "Field `EE8F` writer - EE8F"]
pub type Ee8fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EE9F` reader - EE9F"]
pub type Ee9fR = crate::FieldReader;
#[doc = "Field `EE9F` writer - EE9F"]
pub type Ee9fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EE10F` reader - EE10F"]
pub type Ee10fR = crate::FieldReader;
#[doc = "Field `EE10F` writer - EE10F"]
pub type Ee10fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EEVSD` reader - EEVSD"]
pub type EevsdR = crate::FieldReader;
#[doc = "Field `EEVSD` writer - EEVSD"]
pub type EevsdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - EE6F"]
    #[inline(always)]
    pub fn ee6f(&self) -> Ee6fR {
        Ee6fR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - EE7F"]
    #[inline(always)]
    pub fn ee7f(&self) -> Ee7fR {
        Ee7fR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EE8F"]
    #[inline(always)]
    pub fn ee8f(&self) -> Ee8fR {
        Ee8fR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - EE9F"]
    #[inline(always)]
    pub fn ee9f(&self) -> Ee9fR {
        Ee9fR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - EE10F"]
    #[inline(always)]
    pub fn ee10f(&self) -> Ee10fR {
        Ee10fR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 30:31 - EEVSD"]
    #[inline(always)]
    pub fn eevsd(&self) -> EevsdR {
        EevsdR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EECR3")
            .field("ee6f", &self.ee6f())
            .field("ee7f", &self.ee7f())
            .field("ee8f", &self.ee8f())
            .field("ee9f", &self.ee9f())
            .field("ee10f", &self.ee10f())
            .field("eevsd", &self.eevsd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - EE6F"]
    #[inline(always)]
    #[must_use]
    pub fn ee6f(&mut self) -> Ee6fW<Eecr3Spec> {
        Ee6fW::new(self, 0)
    }
    #[doc = "Bits 6:9 - EE7F"]
    #[inline(always)]
    #[must_use]
    pub fn ee7f(&mut self) -> Ee7fW<Eecr3Spec> {
        Ee7fW::new(self, 6)
    }
    #[doc = "Bits 12:15 - EE8F"]
    #[inline(always)]
    #[must_use]
    pub fn ee8f(&mut self) -> Ee8fW<Eecr3Spec> {
        Ee8fW::new(self, 12)
    }
    #[doc = "Bits 18:21 - EE9F"]
    #[inline(always)]
    #[must_use]
    pub fn ee9f(&mut self) -> Ee9fW<Eecr3Spec> {
        Ee9fW::new(self, 18)
    }
    #[doc = "Bits 24:27 - EE10F"]
    #[inline(always)]
    #[must_use]
    pub fn ee10f(&mut self) -> Ee10fW<Eecr3Spec> {
        Ee10fW::new(self, 24)
    }
    #[doc = "Bits 30:31 - EEVSD"]
    #[inline(always)]
    #[must_use]
    pub fn eevsd(&mut self) -> EevsdW<Eecr3Spec> {
        EevsdW::new(self, 30)
    }
}
#[doc = "Timer External Event Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eecr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eecr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eecr3Spec;
impl crate::RegisterSpec for Eecr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eecr3::R`](R) reader structure"]
impl crate::Readable for Eecr3Spec {}
#[doc = "`write(|w| ..)` method takes [`eecr3::W`](W) writer structure"]
impl crate::Writable for Eecr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EECR3 to value 0"]
impl crate::Resettable for Eecr3Spec {
    const RESET_VALUE: u32 = 0;
}

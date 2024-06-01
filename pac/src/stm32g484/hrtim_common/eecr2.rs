#[doc = "Register `EECR2` reader"]
pub type R = crate::R<Eecr2Spec>;
#[doc = "Register `EECR2` writer"]
pub type W = crate::W<Eecr2Spec>;
#[doc = "Field `EE6SRC` reader - EE6SRC"]
pub type Ee6srcR = crate::FieldReader;
#[doc = "Field `EE6SRC` writer - EE6SRC"]
pub type Ee6srcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE6POL` reader - EE6POL"]
pub type Ee6polR = crate::BitReader;
#[doc = "Field `EE6POL` writer - EE6POL"]
pub type Ee6polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE6SNS` reader - EE6SNS"]
pub type Ee6snsR = crate::FieldReader;
#[doc = "Field `EE6SNS` writer - EE6SNS"]
pub type Ee6snsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE7SRC` reader - EE7SRC"]
pub type Ee7srcR = crate::FieldReader;
#[doc = "Field `EE7SRC` writer - EE7SRC"]
pub type Ee7srcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE7POL` reader - EE7POL"]
pub type Ee7polR = crate::BitReader;
#[doc = "Field `EE7POL` writer - EE7POL"]
pub type Ee7polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE7SNS` reader - EE7SNS"]
pub type Ee7snsR = crate::FieldReader;
#[doc = "Field `EE7SNS` writer - EE7SNS"]
pub type Ee7snsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE8SRC` reader - EE8SRC"]
pub type Ee8srcR = crate::FieldReader;
#[doc = "Field `EE8SRC` writer - EE8SRC"]
pub type Ee8srcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE8POL` reader - EE8POL"]
pub type Ee8polR = crate::BitReader;
#[doc = "Field `EE8POL` writer - EE8POL"]
pub type Ee8polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE8SNS` reader - EE8SNS"]
pub type Ee8snsR = crate::FieldReader;
#[doc = "Field `EE8SNS` writer - EE8SNS"]
pub type Ee8snsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE9SRC` reader - EE9SRC"]
pub type Ee9srcR = crate::FieldReader;
#[doc = "Field `EE9SRC` writer - EE9SRC"]
pub type Ee9srcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE9POL` reader - EE9POL"]
pub type Ee9polR = crate::BitReader;
#[doc = "Field `EE9POL` writer - EE9POL"]
pub type Ee9polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE9SNS` reader - EE9SNS"]
pub type Ee9snsR = crate::FieldReader;
#[doc = "Field `EE9SNS` writer - EE9SNS"]
pub type Ee9snsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE10SRC` reader - EE10SRC"]
pub type Ee10srcR = crate::FieldReader;
#[doc = "Field `EE10SRC` writer - EE10SRC"]
pub type Ee10srcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE10POL` reader - EE10POL"]
pub type Ee10polR = crate::BitReader;
#[doc = "Field `EE10POL` writer - EE10POL"]
pub type Ee10polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE10SNS` reader - EE10SNS"]
pub type Ee10snsR = crate::FieldReader;
#[doc = "Field `EE10SNS` writer - EE10SNS"]
pub type Ee10snsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - EE6SRC"]
    #[inline(always)]
    pub fn ee6src(&self) -> Ee6srcR {
        Ee6srcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - EE6POL"]
    #[inline(always)]
    pub fn ee6pol(&self) -> Ee6polR {
        Ee6polR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - EE6SNS"]
    #[inline(always)]
    pub fn ee6sns(&self) -> Ee6snsR {
        Ee6snsR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 6:7 - EE7SRC"]
    #[inline(always)]
    pub fn ee7src(&self) -> Ee7srcR {
        Ee7srcR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - EE7POL"]
    #[inline(always)]
    pub fn ee7pol(&self) -> Ee7polR {
        Ee7polR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - EE7SNS"]
    #[inline(always)]
    pub fn ee7sns(&self) -> Ee7snsR {
        Ee7snsR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 12:13 - EE8SRC"]
    #[inline(always)]
    pub fn ee8src(&self) -> Ee8srcR {
        Ee8srcR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - EE8POL"]
    #[inline(always)]
    pub fn ee8pol(&self) -> Ee8polR {
        Ee8polR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - EE8SNS"]
    #[inline(always)]
    pub fn ee8sns(&self) -> Ee8snsR {
        Ee8snsR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 18:19 - EE9SRC"]
    #[inline(always)]
    pub fn ee9src(&self) -> Ee9srcR {
        Ee9srcR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - EE9POL"]
    #[inline(always)]
    pub fn ee9pol(&self) -> Ee9polR {
        Ee9polR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - EE9SNS"]
    #[inline(always)]
    pub fn ee9sns(&self) -> Ee9snsR {
        Ee9snsR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 24:25 - EE10SRC"]
    #[inline(always)]
    pub fn ee10src(&self) -> Ee10srcR {
        Ee10srcR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - EE10POL"]
    #[inline(always)]
    pub fn ee10pol(&self) -> Ee10polR {
        Ee10polR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - EE10SNS"]
    #[inline(always)]
    pub fn ee10sns(&self) -> Ee10snsR {
        Ee10snsR::new(((self.bits >> 27) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EECR2")
            .field("ee6src", &self.ee6src())
            .field("ee6pol", &self.ee6pol())
            .field("ee6sns", &self.ee6sns())
            .field("ee7src", &self.ee7src())
            .field("ee7pol", &self.ee7pol())
            .field("ee7sns", &self.ee7sns())
            .field("ee8src", &self.ee8src())
            .field("ee8pol", &self.ee8pol())
            .field("ee8sns", &self.ee8sns())
            .field("ee9src", &self.ee9src())
            .field("ee9pol", &self.ee9pol())
            .field("ee9sns", &self.ee9sns())
            .field("ee10src", &self.ee10src())
            .field("ee10pol", &self.ee10pol())
            .field("ee10sns", &self.ee10sns())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - EE6SRC"]
    #[inline(always)]
    #[must_use]
    pub fn ee6src(&mut self) -> Ee6srcW<Eecr2Spec> {
        Ee6srcW::new(self, 0)
    }
    #[doc = "Bit 2 - EE6POL"]
    #[inline(always)]
    #[must_use]
    pub fn ee6pol(&mut self) -> Ee6polW<Eecr2Spec> {
        Ee6polW::new(self, 2)
    }
    #[doc = "Bits 3:4 - EE6SNS"]
    #[inline(always)]
    #[must_use]
    pub fn ee6sns(&mut self) -> Ee6snsW<Eecr2Spec> {
        Ee6snsW::new(self, 3)
    }
    #[doc = "Bits 6:7 - EE7SRC"]
    #[inline(always)]
    #[must_use]
    pub fn ee7src(&mut self) -> Ee7srcW<Eecr2Spec> {
        Ee7srcW::new(self, 6)
    }
    #[doc = "Bit 8 - EE7POL"]
    #[inline(always)]
    #[must_use]
    pub fn ee7pol(&mut self) -> Ee7polW<Eecr2Spec> {
        Ee7polW::new(self, 8)
    }
    #[doc = "Bits 9:10 - EE7SNS"]
    #[inline(always)]
    #[must_use]
    pub fn ee7sns(&mut self) -> Ee7snsW<Eecr2Spec> {
        Ee7snsW::new(self, 9)
    }
    #[doc = "Bits 12:13 - EE8SRC"]
    #[inline(always)]
    #[must_use]
    pub fn ee8src(&mut self) -> Ee8srcW<Eecr2Spec> {
        Ee8srcW::new(self, 12)
    }
    #[doc = "Bit 14 - EE8POL"]
    #[inline(always)]
    #[must_use]
    pub fn ee8pol(&mut self) -> Ee8polW<Eecr2Spec> {
        Ee8polW::new(self, 14)
    }
    #[doc = "Bits 15:16 - EE8SNS"]
    #[inline(always)]
    #[must_use]
    pub fn ee8sns(&mut self) -> Ee8snsW<Eecr2Spec> {
        Ee8snsW::new(self, 15)
    }
    #[doc = "Bits 18:19 - EE9SRC"]
    #[inline(always)]
    #[must_use]
    pub fn ee9src(&mut self) -> Ee9srcW<Eecr2Spec> {
        Ee9srcW::new(self, 18)
    }
    #[doc = "Bit 20 - EE9POL"]
    #[inline(always)]
    #[must_use]
    pub fn ee9pol(&mut self) -> Ee9polW<Eecr2Spec> {
        Ee9polW::new(self, 20)
    }
    #[doc = "Bits 21:22 - EE9SNS"]
    #[inline(always)]
    #[must_use]
    pub fn ee9sns(&mut self) -> Ee9snsW<Eecr2Spec> {
        Ee9snsW::new(self, 21)
    }
    #[doc = "Bits 24:25 - EE10SRC"]
    #[inline(always)]
    #[must_use]
    pub fn ee10src(&mut self) -> Ee10srcW<Eecr2Spec> {
        Ee10srcW::new(self, 24)
    }
    #[doc = "Bit 26 - EE10POL"]
    #[inline(always)]
    #[must_use]
    pub fn ee10pol(&mut self) -> Ee10polW<Eecr2Spec> {
        Ee10polW::new(self, 26)
    }
    #[doc = "Bits 27:28 - EE10SNS"]
    #[inline(always)]
    #[must_use]
    pub fn ee10sns(&mut self) -> Ee10snsW<Eecr2Spec> {
        Ee10snsW::new(self, 27)
    }
}
#[doc = "Timer External Event Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eecr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eecr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eecr2Spec;
impl crate::RegisterSpec for Eecr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eecr2::R`](R) reader structure"]
impl crate::Readable for Eecr2Spec {}
#[doc = "`write(|w| ..)` method takes [`eecr2::W`](W) writer structure"]
impl crate::Writable for Eecr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EECR2 to value 0"]
impl crate::Resettable for Eecr2Spec {
    const RESET_VALUE: u32 = 0;
}

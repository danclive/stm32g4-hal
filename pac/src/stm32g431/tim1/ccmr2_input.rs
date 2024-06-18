#[doc = "Register `CCMR2_Input` reader"]
pub type R = crate::R<Ccmr2InputSpec>;
#[doc = "Register `CCMR2_Input` writer"]
pub type W = crate::W<Ccmr2InputSpec>;
#[doc = "Field `CC3S` reader - Capture/compare 3 selection"]
pub type Cc3sR = crate::FieldReader;
#[doc = "Field `CC3S` writer - Capture/compare 3 selection"]
pub type Cc3sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC3PSC` reader - Input capture 3 prescaler"]
pub type Ic3pscR = crate::FieldReader;
#[doc = "Field `IC3PSC` writer - Input capture 3 prescaler"]
pub type Ic3pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC3F` reader - Input capture 3 filter"]
pub type Ic3fR = crate::FieldReader;
#[doc = "Field `IC3F` writer - Input capture 3 filter"]
pub type Ic3fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CC4S` reader - Capture/Compare 4 selection"]
pub type Cc4sR = crate::FieldReader;
#[doc = "Field `CC4S` writer - Capture/Compare 4 selection"]
pub type Cc4sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC4PSC` reader - Input capture 4 prescaler"]
pub type Ic4pscR = crate::FieldReader;
#[doc = "Field `IC4PSC` writer - Input capture 4 prescaler"]
pub type Ic4pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC4F` reader - Input capture 4 filter"]
pub type Ic4fR = crate::FieldReader;
#[doc = "Field `IC4F` writer - Input capture 4 filter"]
pub type Ic4fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Capture/compare 3 selection"]
    #[inline(always)]
    pub fn cc3s(&self) -> Cc3sR {
        Cc3sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler"]
    #[inline(always)]
    pub fn ic3psc(&self) -> Ic3pscR {
        Ic3pscR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 3 filter"]
    #[inline(always)]
    pub fn ic3f(&self) -> Ic3fR {
        Ic3fR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    pub fn cc4s(&self) -> Cc4sR {
        Cc4sR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler"]
    #[inline(always)]
    pub fn ic4psc(&self) -> Ic4pscR {
        Ic4pscR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Input capture 4 filter"]
    #[inline(always)]
    pub fn ic4f(&self) -> Ic4fR {
        Ic4fR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR2_Input")
            .field("ic4f", &self.ic4f())
            .field("ic4psc", &self.ic4psc())
            .field("cc4s", &self.cc4s())
            .field("ic3f", &self.ic3f())
            .field("ic3psc", &self.ic3psc())
            .field("cc3s", &self.cc3s())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/compare 3 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc3s(&mut self) -> Cc3sW<Ccmr2InputSpec> {
        Cc3sW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ic3psc(&mut self) -> Ic3pscW<Ccmr2InputSpec> {
        Ic3pscW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Input capture 3 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ic3f(&mut self) -> Ic3fW<Ccmr2InputSpec> {
        Ic3fW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc4s(&mut self) -> Cc4sW<Ccmr2InputSpec> {
        Cc4sW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ic4psc(&mut self) -> Ic4pscW<Ccmr2InputSpec> {
        Ic4pscW::new(self, 10)
    }
    #[doc = "Bits 12:15 - Input capture 4 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ic4f(&mut self) -> Ic4fW<Ccmr2InputSpec> {
        Ic4fW::new(self, 12)
    }
}
#[doc = "capture/compare mode register 2 (input mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr2InputSpec;
impl crate::RegisterSpec for Ccmr2InputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr2_input::R`](R) reader structure"]
impl crate::Readable for Ccmr2InputSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmr2_input::W`](W) writer structure"]
impl crate::Writable for Ccmr2InputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMR2_Input to value 0"]
impl crate::Resettable for Ccmr2InputSpec {
    const RESET_VALUE: u32 = 0;
}

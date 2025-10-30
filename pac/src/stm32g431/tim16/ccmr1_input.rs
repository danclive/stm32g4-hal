#[doc = "Register `CCMR1_Input` reader"]
pub type R = crate::R<Ccmr1InputSpec>;
#[doc = "Register `CCMR1_Input` writer"]
pub type W = crate::W<Ccmr1InputSpec>;
#[doc = "Field `CC1S` reader - Capture/Compare 1 selection"]
pub type Cc1sR = crate::FieldReader;
#[doc = "Field `CC1S` writer - Capture/Compare 1 selection"]
pub type Cc1sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC1PSC` reader - Input capture 1 prescaler"]
pub type Ic1pscR = crate::FieldReader;
#[doc = "Field `IC1PSC` writer - Input capture 1 prescaler"]
pub type Ic1pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC1F` reader - Input capture 1 filter"]
pub type Ic1fR = crate::FieldReader;
#[doc = "Field `IC1F` writer - Input capture 1 filter"]
pub type Ic1fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&self) -> Cc1sR {
        Cc1sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline(always)]
    pub fn ic1psc(&self) -> Ic1pscR {
        Ic1pscR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 1 filter"]
    #[inline(always)]
    pub fn ic1f(&self) -> Ic1fR {
        Ic1fR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR1_Input")
            .field("ic1f", &self.ic1f())
            .field("ic1psc", &self.ic1psc())
            .field("cc1s", &self.cc1s())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&mut self) -> Cc1sW<'_, Ccmr1InputSpec> {
        Cc1sW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline(always)]
    pub fn ic1psc(&mut self) -> Ic1pscW<'_, Ccmr1InputSpec> {
        Ic1pscW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Input capture 1 filter"]
    #[inline(always)]
    pub fn ic1f(&mut self) -> Ic1fW<'_, Ccmr1InputSpec> {
        Ic1fW::new(self, 4)
    }
}
#[doc = "capture/compare mode register 1 (input mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr1InputSpec;
impl crate::RegisterSpec for Ccmr1InputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1_input::R`](R) reader structure"]
impl crate::Readable for Ccmr1InputSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmr1_input::W`](W) writer structure"]
impl crate::Writable for Ccmr1InputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMR1_Input to value 0"]
impl crate::Resettable for Ccmr1InputSpec {}

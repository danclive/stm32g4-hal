#[doc = "Register `ADCPS1` reader"]
pub type R = crate::R<Adcps1Spec>;
#[doc = "Register `ADCPS1` writer"]
pub type W = crate::W<Adcps1Spec>;
#[doc = "Field `ADC1PSC` reader - ADC1PSC"]
pub type Adc1pscR = crate::FieldReader;
#[doc = "Field `ADC1PSC` writer - ADC1PSC"]
pub type Adc1pscW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC2PSC` reader - ADC2PSC"]
pub type Adc2pscR = crate::FieldReader;
#[doc = "Field `ADC2PSC` writer - ADC2PSC"]
pub type Adc2pscW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC3PSC` reader - ADC3PSC"]
pub type Adc3pscR = crate::FieldReader;
#[doc = "Field `ADC3PSC` writer - ADC3PSC"]
pub type Adc3pscW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC4PSC` reader - ADC4PSC"]
pub type Adc4pscR = crate::FieldReader;
#[doc = "Field `ADC4PSC` writer - ADC4PSC"]
pub type Adc4pscW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC5PSC` reader - ADC5PSC"]
pub type Adc5pscR = crate::FieldReader;
#[doc = "Field `ADC5PSC` writer - ADC5PSC"]
pub type Adc5pscW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - ADC1PSC"]
    #[inline(always)]
    pub fn adc1psc(&self) -> Adc1pscR {
        Adc1pscR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - ADC2PSC"]
    #[inline(always)]
    pub fn adc2psc(&self) -> Adc2pscR {
        Adc2pscR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - ADC3PSC"]
    #[inline(always)]
    pub fn adc3psc(&self) -> Adc3pscR {
        Adc3pscR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - ADC4PSC"]
    #[inline(always)]
    pub fn adc4psc(&self) -> Adc4pscR {
        Adc4pscR::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - ADC5PSC"]
    #[inline(always)]
    pub fn adc5psc(&self) -> Adc5pscR {
        Adc5pscR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCPS1")
            .field("adc5psc", &self.adc5psc())
            .field("adc4psc", &self.adc4psc())
            .field("adc3psc", &self.adc3psc())
            .field("adc2psc", &self.adc2psc())
            .field("adc1psc", &self.adc1psc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - ADC1PSC"]
    #[inline(always)]
    #[must_use]
    pub fn adc1psc(&mut self) -> Adc1pscW<Adcps1Spec> {
        Adc1pscW::new(self, 0)
    }
    #[doc = "Bits 6:10 - ADC2PSC"]
    #[inline(always)]
    #[must_use]
    pub fn adc2psc(&mut self) -> Adc2pscW<Adcps1Spec> {
        Adc2pscW::new(self, 6)
    }
    #[doc = "Bits 12:16 - ADC3PSC"]
    #[inline(always)]
    #[must_use]
    pub fn adc3psc(&mut self) -> Adc3pscW<Adcps1Spec> {
        Adc3pscW::new(self, 12)
    }
    #[doc = "Bits 18:22 - ADC4PSC"]
    #[inline(always)]
    #[must_use]
    pub fn adc4psc(&mut self) -> Adc4pscW<Adcps1Spec> {
        Adc4pscW::new(self, 18)
    }
    #[doc = "Bits 24:28 - ADC5PSC"]
    #[inline(always)]
    #[must_use]
    pub fn adc5psc(&mut self) -> Adc5pscW<Adcps1Spec> {
        Adc5pscW::new(self, 24)
    }
}
#[doc = "HRTIM ADC Post Scaler Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcps1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcps1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcps1Spec;
impl crate::RegisterSpec for Adcps1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcps1::R`](R) reader structure"]
impl crate::Readable for Adcps1Spec {}
#[doc = "`write(|w| ..)` method takes [`adcps1::W`](W) writer structure"]
impl crate::Writable for Adcps1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCPS1 to value 0"]
impl crate::Resettable for Adcps1Spec {
    const RESET_VALUE: u32 = 0;
}

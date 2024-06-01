#[doc = "Register `ADCPS2` reader"]
pub type R = crate::R<Adcps2Spec>;
#[doc = "Register `ADCPS2` writer"]
pub type W = crate::W<Adcps2Spec>;
#[doc = "Field `ADC6PSC` reader - ADC6PSC"]
pub type Adc6pscR = crate::FieldReader;
#[doc = "Field `ADC6PSC` writer - ADC6PSC"]
pub type Adc6pscW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC7PSC` reader - ADC7PSC"]
pub type Adc7pscR = crate::FieldReader;
#[doc = "Field `ADC7PSC` writer - ADC7PSC"]
pub type Adc7pscW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC8PSC` reader - ADC8PSC"]
pub type Adc8pscR = crate::FieldReader;
#[doc = "Field `ADC8PSC` writer - ADC8PSC"]
pub type Adc8pscW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC9PSC` reader - ADC9PSC"]
pub type Adc9pscR = crate::FieldReader;
#[doc = "Field `ADC9PSC` writer - ADC9PSC"]
pub type Adc9pscW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC10PSC` reader - ADC10PSC"]
pub type Adc10pscR = crate::FieldReader;
#[doc = "Field `ADC10PSC` writer - ADC10PSC"]
pub type Adc10pscW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - ADC6PSC"]
    #[inline(always)]
    pub fn adc6psc(&self) -> Adc6pscR {
        Adc6pscR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - ADC7PSC"]
    #[inline(always)]
    pub fn adc7psc(&self) -> Adc7pscR {
        Adc7pscR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - ADC8PSC"]
    #[inline(always)]
    pub fn adc8psc(&self) -> Adc8pscR {
        Adc8pscR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - ADC9PSC"]
    #[inline(always)]
    pub fn adc9psc(&self) -> Adc9pscR {
        Adc9pscR::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - ADC10PSC"]
    #[inline(always)]
    pub fn adc10psc(&self) -> Adc10pscR {
        Adc10pscR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCPS2")
            .field("adc10psc", &self.adc10psc())
            .field("adc9psc", &self.adc9psc())
            .field("adc8psc", &self.adc8psc())
            .field("adc7psc", &self.adc7psc())
            .field("adc6psc", &self.adc6psc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - ADC6PSC"]
    #[inline(always)]
    #[must_use]
    pub fn adc6psc(&mut self) -> Adc6pscW<Adcps2Spec> {
        Adc6pscW::new(self, 0)
    }
    #[doc = "Bits 6:10 - ADC7PSC"]
    #[inline(always)]
    #[must_use]
    pub fn adc7psc(&mut self) -> Adc7pscW<Adcps2Spec> {
        Adc7pscW::new(self, 6)
    }
    #[doc = "Bits 12:16 - ADC8PSC"]
    #[inline(always)]
    #[must_use]
    pub fn adc8psc(&mut self) -> Adc8pscW<Adcps2Spec> {
        Adc8pscW::new(self, 12)
    }
    #[doc = "Bits 18:22 - ADC9PSC"]
    #[inline(always)]
    #[must_use]
    pub fn adc9psc(&mut self) -> Adc9pscW<Adcps2Spec> {
        Adc9pscW::new(self, 18)
    }
    #[doc = "Bits 24:28 - ADC10PSC"]
    #[inline(always)]
    #[must_use]
    pub fn adc10psc(&mut self) -> Adc10pscW<Adcps2Spec> {
        Adc10pscW::new(self, 24)
    }
}
#[doc = "HRTIM ADC Post Scaler Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcps2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcps2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcps2Spec;
impl crate::RegisterSpec for Adcps2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcps2::R`](R) reader structure"]
impl crate::Readable for Adcps2Spec {}
#[doc = "`write(|w| ..)` method takes [`adcps2::W`](W) writer structure"]
impl crate::Writable for Adcps2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCPS2 to value 0"]
impl crate::Resettable for Adcps2Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `ADCER` reader"]
pub type R = crate::R<AdcerSpec>;
#[doc = "Register `ADCER` writer"]
pub type W = crate::W<AdcerSpec>;
#[doc = "Field `ADC5TRG` reader - ADC5TRG"]
pub type Adc5trgR = crate::FieldReader;
#[doc = "Field `ADC5TRG` writer - ADC5TRG"]
pub type Adc5trgW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC6TRG` reader - ADC6TRG"]
pub type Adc6trgR = crate::FieldReader;
#[doc = "Field `ADC6TRG` writer - ADC6TRG"]
pub type Adc6trgW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC7TRG` reader - ADC7TRG"]
pub type Adc7trgR = crate::FieldReader;
#[doc = "Field `ADC7TRG` writer - ADC7TRG"]
pub type Adc7trgW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC8TRG` reader - ADC8TRG"]
pub type Adc8trgR = crate::FieldReader;
#[doc = "Field `ADC8TRG` writer - ADC8TRG"]
pub type Adc8trgW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC9TRG` reader - ADC9TRG"]
pub type Adc9trgR = crate::FieldReader;
#[doc = "Field `ADC9TRG` writer - ADC9TRG"]
pub type Adc9trgW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC10TRG` reader - ADC10TRG"]
pub type Adc10trgR = crate::FieldReader;
#[doc = "Field `ADC10TRG` writer - ADC10TRG"]
pub type Adc10trgW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - ADC5TRG"]
    #[inline(always)]
    pub fn adc5trg(&self) -> Adc5trgR {
        Adc5trgR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - ADC6TRG"]
    #[inline(always)]
    pub fn adc6trg(&self) -> Adc6trgR {
        Adc6trgR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - ADC7TRG"]
    #[inline(always)]
    pub fn adc7trg(&self) -> Adc7trgR {
        Adc7trgR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - ADC8TRG"]
    #[inline(always)]
    pub fn adc8trg(&self) -> Adc8trgR {
        Adc8trgR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - ADC9TRG"]
    #[inline(always)]
    pub fn adc9trg(&self) -> Adc9trgR {
        Adc9trgR::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 26:30 - ADC10TRG"]
    #[inline(always)]
    pub fn adc10trg(&self) -> Adc10trgR {
        Adc10trgR::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCER")
            .field("adc10trg", &self.adc10trg())
            .field("adc9trg", &self.adc9trg())
            .field("adc8trg", &self.adc8trg())
            .field("adc7trg", &self.adc7trg())
            .field("adc6trg", &self.adc6trg())
            .field("adc5trg", &self.adc5trg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - ADC5TRG"]
    #[inline(always)]
    #[must_use]
    pub fn adc5trg(&mut self) -> Adc5trgW<AdcerSpec> {
        Adc5trgW::new(self, 0)
    }
    #[doc = "Bits 5:9 - ADC6TRG"]
    #[inline(always)]
    #[must_use]
    pub fn adc6trg(&mut self) -> Adc6trgW<AdcerSpec> {
        Adc6trgW::new(self, 5)
    }
    #[doc = "Bits 10:14 - ADC7TRG"]
    #[inline(always)]
    #[must_use]
    pub fn adc7trg(&mut self) -> Adc7trgW<AdcerSpec> {
        Adc7trgW::new(self, 10)
    }
    #[doc = "Bits 16:20 - ADC8TRG"]
    #[inline(always)]
    #[must_use]
    pub fn adc8trg(&mut self) -> Adc8trgW<AdcerSpec> {
        Adc8trgW::new(self, 16)
    }
    #[doc = "Bits 21:25 - ADC9TRG"]
    #[inline(always)]
    #[must_use]
    pub fn adc9trg(&mut self) -> Adc9trgW<AdcerSpec> {
        Adc9trgW::new(self, 21)
    }
    #[doc = "Bits 26:30 - ADC10TRG"]
    #[inline(always)]
    #[must_use]
    pub fn adc10trg(&mut self) -> Adc10trgW<AdcerSpec> {
        Adc10trgW::new(self, 26)
    }
}
#[doc = "HRTIM ADC Extended Trigger Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcerSpec;
impl crate::RegisterSpec for AdcerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcer::R`](R) reader structure"]
impl crate::Readable for AdcerSpec {}
#[doc = "`write(|w| ..)` method takes [`adcer::W`](W) writer structure"]
impl crate::Writable for AdcerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCER to value 0"]
impl crate::Resettable for AdcerSpec {
    const RESET_VALUE: u32 = 0;
}

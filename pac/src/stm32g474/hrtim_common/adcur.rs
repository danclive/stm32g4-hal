#[doc = "Register `ADCUR` reader"]
pub type R = crate::R<AdcurSpec>;
#[doc = "Register `ADCUR` writer"]
pub type W = crate::W<AdcurSpec>;
#[doc = "Field `AD5USRC` reader - AD5USRC"]
pub type Ad5usrcR = crate::FieldReader;
#[doc = "Field `AD5USRC` writer - AD5USRC"]
pub type Ad5usrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AD6USRC` reader - AD6USRC"]
pub type Ad6usrcR = crate::FieldReader;
#[doc = "Field `AD6USRC` writer - AD6USRC"]
pub type Ad6usrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AD7USRC` reader - AD7USRC"]
pub type Ad7usrcR = crate::FieldReader;
#[doc = "Field `AD7USRC` writer - AD7USRC"]
pub type Ad7usrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AD8USRC` reader - AD8USRC"]
pub type Ad8usrcR = crate::FieldReader;
#[doc = "Field `AD8USRC` writer - AD8USRC"]
pub type Ad8usrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AD9USRC` reader - AD9USRC"]
pub type Ad9usrcR = crate::FieldReader;
#[doc = "Field `AD9USRC` writer - AD9USRC"]
pub type Ad9usrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AD10USRC` reader - AD10USRC"]
pub type Ad10usrcR = crate::FieldReader;
#[doc = "Field `AD10USRC` writer - AD10USRC"]
pub type Ad10usrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - AD5USRC"]
    #[inline(always)]
    pub fn ad5usrc(&self) -> Ad5usrcR {
        Ad5usrcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - AD6USRC"]
    #[inline(always)]
    pub fn ad6usrc(&self) -> Ad6usrcR {
        Ad6usrcR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - AD7USRC"]
    #[inline(always)]
    pub fn ad7usrc(&self) -> Ad7usrcR {
        Ad7usrcR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - AD8USRC"]
    #[inline(always)]
    pub fn ad8usrc(&self) -> Ad8usrcR {
        Ad8usrcR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - AD9USRC"]
    #[inline(always)]
    pub fn ad9usrc(&self) -> Ad9usrcR {
        Ad9usrcR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - AD10USRC"]
    #[inline(always)]
    pub fn ad10usrc(&self) -> Ad10usrcR {
        Ad10usrcR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCUR")
            .field("ad10usrc", &self.ad10usrc())
            .field("ad9usrc", &self.ad9usrc())
            .field("ad8usrc", &self.ad8usrc())
            .field("ad7usrc", &self.ad7usrc())
            .field("ad6usrc", &self.ad6usrc())
            .field("ad5usrc", &self.ad5usrc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - AD5USRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad5usrc(&mut self) -> Ad5usrcW<AdcurSpec> {
        Ad5usrcW::new(self, 0)
    }
    #[doc = "Bits 4:6 - AD6USRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad6usrc(&mut self) -> Ad6usrcW<AdcurSpec> {
        Ad6usrcW::new(self, 4)
    }
    #[doc = "Bits 8:10 - AD7USRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad7usrc(&mut self) -> Ad7usrcW<AdcurSpec> {
        Ad7usrcW::new(self, 8)
    }
    #[doc = "Bits 12:14 - AD8USRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad8usrc(&mut self) -> Ad8usrcW<AdcurSpec> {
        Ad8usrcW::new(self, 12)
    }
    #[doc = "Bits 16:18 - AD9USRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad9usrc(&mut self) -> Ad9usrcW<AdcurSpec> {
        Ad9usrcW::new(self, 16)
    }
    #[doc = "Bits 20:22 - AD10USRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad10usrc(&mut self) -> Ad10usrcW<AdcurSpec> {
        Ad10usrcW::new(self, 20)
    }
}
#[doc = "HRTIM ADC Trigger Update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcurSpec;
impl crate::RegisterSpec for AdcurSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcur::R`](R) reader structure"]
impl crate::Readable for AdcurSpec {}
#[doc = "`write(|w| ..)` method takes [`adcur::W`](W) writer structure"]
impl crate::Writable for AdcurSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCUR to value 0"]
impl crate::Resettable for AdcurSpec {
    const RESET_VALUE: u32 = 0;
}

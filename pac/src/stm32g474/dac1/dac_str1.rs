#[doc = "Register `DAC_STR1` reader"]
pub type R = crate::R<DacStr1Spec>;
#[doc = "Register `DAC_STR1` writer"]
pub type W = crate::W<DacStr1Spec>;
#[doc = "Field `STRSTDATA1` reader - DAC Channel 1 Sawtooth reset value"]
pub type Strstdata1R = crate::FieldReader<u16>;
#[doc = "Field `STRSTDATA1` writer - DAC Channel 1 Sawtooth reset value"]
pub type Strstdata1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `STDIR1` reader - DAC Channel1 Sawtooth direction setting"]
pub type Stdir1R = crate::BitReader;
#[doc = "Field `STDIR1` writer - DAC Channel1 Sawtooth direction setting"]
pub type Stdir1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STINCDATA1` reader - DAC CH1 Sawtooth increment value (12.4 bit format)"]
pub type Stincdata1R = crate::FieldReader<u16>;
#[doc = "Field `STINCDATA1` writer - DAC CH1 Sawtooth increment value (12.4 bit format)"]
pub type Stincdata1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC Channel 1 Sawtooth reset value"]
    #[inline(always)]
    pub fn strstdata1(&self) -> Strstdata1R {
        Strstdata1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - DAC Channel1 Sawtooth direction setting"]
    #[inline(always)]
    pub fn stdir1(&self) -> Stdir1R {
        Stdir1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:31 - DAC CH1 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    pub fn stincdata1(&self) -> Stincdata1R {
        Stincdata1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_STR1")
            .field("strstdata1", &self.strstdata1())
            .field("stdir1", &self.stdir1())
            .field("stincdata1", &self.stincdata1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC Channel 1 Sawtooth reset value"]
    #[inline(always)]
    #[must_use]
    pub fn strstdata1(&mut self) -> Strstdata1W<DacStr1Spec> {
        Strstdata1W::new(self, 0)
    }
    #[doc = "Bit 12 - DAC Channel1 Sawtooth direction setting"]
    #[inline(always)]
    #[must_use]
    pub fn stdir1(&mut self) -> Stdir1W<DacStr1Spec> {
        Stdir1W::new(self, 12)
    }
    #[doc = "Bits 16:31 - DAC CH1 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    #[must_use]
    pub fn stincdata1(&mut self) -> Stincdata1W<DacStr1Spec> {
        Stincdata1W::new(self, 16)
    }
}
#[doc = "Sawtooth register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_str1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_str1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacStr1Spec;
impl crate::RegisterSpec for DacStr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_str1::R`](R) reader structure"]
impl crate::Readable for DacStr1Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_str1::W`](W) writer structure"]
impl crate::Writable for DacStr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_STR1 to value 0"]
impl crate::Resettable for DacStr1Spec {
    const RESET_VALUE: u32 = 0;
}

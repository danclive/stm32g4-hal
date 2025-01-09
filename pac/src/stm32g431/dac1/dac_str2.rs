#[doc = "Register `DAC_STR2` reader"]
pub type R = crate::R<DacStr2Spec>;
#[doc = "Register `DAC_STR2` writer"]
pub type W = crate::W<DacStr2Spec>;
#[doc = "Field `STRSTDATA2` reader - DAC Channel 2 Sawtooth reset value"]
pub type Strstdata2R = crate::FieldReader<u16>;
#[doc = "Field `STRSTDATA2` writer - DAC Channel 2 Sawtooth reset value"]
pub type Strstdata2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `STDIR2` reader - DAC Channel2 Sawtooth direction setting"]
pub type Stdir2R = crate::BitReader;
#[doc = "Field `STDIR2` writer - DAC Channel2 Sawtooth direction setting"]
pub type Stdir2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STINCDATA2` reader - DAC CH2 Sawtooth increment value (12.4 bit format)"]
pub type Stincdata2R = crate::FieldReader<u16>;
#[doc = "Field `STINCDATA2` writer - DAC CH2 Sawtooth increment value (12.4 bit format)"]
pub type Stincdata2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC Channel 2 Sawtooth reset value"]
    #[inline(always)]
    pub fn strstdata2(&self) -> Strstdata2R {
        Strstdata2R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - DAC Channel2 Sawtooth direction setting"]
    #[inline(always)]
    pub fn stdir2(&self) -> Stdir2R {
        Stdir2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:31 - DAC CH2 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    pub fn stincdata2(&self) -> Stincdata2R {
        Stincdata2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_STR2")
            .field("strstdata2", &self.strstdata2())
            .field("stdir2", &self.stdir2())
            .field("stincdata2", &self.stincdata2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC Channel 2 Sawtooth reset value"]
    #[inline(always)]
    pub fn strstdata2(&mut self) -> Strstdata2W<DacStr2Spec> {
        Strstdata2W::new(self, 0)
    }
    #[doc = "Bit 12 - DAC Channel2 Sawtooth direction setting"]
    #[inline(always)]
    pub fn stdir2(&mut self) -> Stdir2W<DacStr2Spec> {
        Stdir2W::new(self, 12)
    }
    #[doc = "Bits 16:31 - DAC CH2 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    pub fn stincdata2(&mut self) -> Stincdata2W<DacStr2Spec> {
        Stincdata2W::new(self, 16)
    }
}
#[doc = "Sawtooth register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_str2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_str2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacStr2Spec;
impl crate::RegisterSpec for DacStr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_str2::R`](R) reader structure"]
impl crate::Readable for DacStr2Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_str2::W`](W) writer structure"]
impl crate::Writable for DacStr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_STR2 to value 0"]
impl crate::Resettable for DacStr2Spec {
    const RESET_VALUE: u32 = 0;
}

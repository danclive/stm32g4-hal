#[doc = "Register `FLTCR` reader"]
pub type R = crate::R<FltcrSpec>;
#[doc = "Register `FLTCR` writer"]
pub type W = crate::W<FltcrSpec>;
#[doc = "Field `TAMPFREQ` reader - TAMPFREQ"]
pub type TampfreqR = crate::FieldReader;
#[doc = "Field `TAMPFREQ` writer - TAMPFREQ"]
pub type TampfreqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TAMPFLT` reader - TAMPFLT"]
pub type TampfltR = crate::FieldReader;
#[doc = "Field `TAMPFLT` writer - TAMPFLT"]
pub type TampfltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TAMPPRCH` reader - TAMPPRCH"]
pub type TampprchR = crate::FieldReader;
#[doc = "Field `TAMPPRCH` writer - TAMPPRCH"]
pub type TampprchW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TAMPPUDIS` reader - TAMPPUDIS"]
pub type TamppudisR = crate::BitReader;
#[doc = "Field `TAMPPUDIS` writer - TAMPPUDIS"]
pub type TamppudisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - TAMPFREQ"]
    #[inline(always)]
    pub fn tampfreq(&self) -> TampfreqR {
        TampfreqR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - TAMPFLT"]
    #[inline(always)]
    pub fn tampflt(&self) -> TampfltR {
        TampfltR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - TAMPPRCH"]
    #[inline(always)]
    pub fn tampprch(&self) -> TampprchR {
        TampprchR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - TAMPPUDIS"]
    #[inline(always)]
    pub fn tamppudis(&self) -> TamppudisR {
        TamppudisR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLTCR")
            .field("tampfreq", &self.tampfreq())
            .field("tampflt", &self.tampflt())
            .field("tampprch", &self.tampprch())
            .field("tamppudis", &self.tamppudis())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - TAMPFREQ"]
    #[inline(always)]
    pub fn tampfreq(&mut self) -> TampfreqW<FltcrSpec> {
        TampfreqW::new(self, 0)
    }
    #[doc = "Bits 3:4 - TAMPFLT"]
    #[inline(always)]
    pub fn tampflt(&mut self) -> TampfltW<FltcrSpec> {
        TampfltW::new(self, 3)
    }
    #[doc = "Bits 5:6 - TAMPPRCH"]
    #[inline(always)]
    pub fn tampprch(&mut self) -> TampprchW<FltcrSpec> {
        TampprchW::new(self, 5)
    }
    #[doc = "Bit 7 - TAMPPUDIS"]
    #[inline(always)]
    pub fn tamppudis(&mut self) -> TamppudisW<FltcrSpec> {
        TamppudisW::new(self, 7)
    }
}
#[doc = "TAMP filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fltcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FltcrSpec;
impl crate::RegisterSpec for FltcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltcr::R`](R) reader structure"]
impl crate::Readable for FltcrSpec {}
#[doc = "`write(|w| ..)` method takes [`fltcr::W`](W) writer structure"]
impl crate::Writable for FltcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLTCR to value 0"]
impl crate::Resettable for FltcrSpec {
    const RESET_VALUE: u32 = 0;
}

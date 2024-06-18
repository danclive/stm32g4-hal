#[doc = "Register `DAC_DOR2` reader"]
pub type R = crate::R<DacDor2Spec>;
#[doc = "Field `DACC2DOR` reader - DAC channel2 data output These bits are read-only, they contain data output for DAC channel2."]
pub type Dacc2dorR = crate::FieldReader<u16>;
#[doc = "Field `DACC2DORB` reader - DAC channel2 data output"]
pub type Dacc2dorbR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - DAC channel2 data output These bits are read-only, they contain data output for DAC channel2."]
    #[inline(always)]
    pub fn dacc2dor(&self) -> Dacc2dorR {
        Dacc2dorR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - DAC channel2 data output"]
    #[inline(always)]
    pub fn dacc2dorb(&self) -> Dacc2dorbR {
        Dacc2dorbR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_DOR2")
            .field("dacc2dor", &self.dacc2dor())
            .field("dacc2dorb", &self.dacc2dorb())
            .finish()
    }
}
#[doc = "DAC channel2 data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_dor2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacDor2Spec;
impl crate::RegisterSpec for DacDor2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_dor2::R`](R) reader structure"]
impl crate::Readable for DacDor2Spec {}
#[doc = "`reset()` method sets DAC_DOR2 to value 0"]
impl crate::Resettable for DacDor2Spec {
    const RESET_VALUE: u32 = 0;
}

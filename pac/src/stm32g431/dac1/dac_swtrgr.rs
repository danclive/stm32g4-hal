#[doc = "Register `DAC_SWTRGR` writer"]
pub type W = crate::W<DacSwtrgrSpec>;
#[doc = "Field `SWTRIG1` writer - DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register."]
pub type Swtrig1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIG2` writer - DAC channel2 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register."]
pub type Swtrig2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIGB1` writer - DAC channel1 software trigger B"]
pub type Swtrigb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIGB2` writer - DAC channel2 software trigger B"]
pub type Swtrigb2W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<DacSwtrgrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register."]
    #[inline(always)]
    pub fn swtrig1(&mut self) -> Swtrig1W<DacSwtrgrSpec> {
        Swtrig1W::new(self, 0)
    }
    #[doc = "Bit 1 - DAC channel2 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register."]
    #[inline(always)]
    pub fn swtrig2(&mut self) -> Swtrig2W<DacSwtrgrSpec> {
        Swtrig2W::new(self, 1)
    }
    #[doc = "Bit 16 - DAC channel1 software trigger B"]
    #[inline(always)]
    pub fn swtrigb1(&mut self) -> Swtrigb1W<DacSwtrgrSpec> {
        Swtrigb1W::new(self, 16)
    }
    #[doc = "Bit 17 - DAC channel2 software trigger B"]
    #[inline(always)]
    pub fn swtrigb2(&mut self) -> Swtrigb2W<DacSwtrgrSpec> {
        Swtrigb2W::new(self, 17)
    }
}
#[doc = "DAC software trigger register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_swtrgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacSwtrgrSpec;
impl crate::RegisterSpec for DacSwtrgrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dac_swtrgr::W`](W) writer structure"]
impl crate::Writable for DacSwtrgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_SWTRGR to value 0"]
impl crate::Resettable for DacSwtrgrSpec {
    const RESET_VALUE: u32 = 0;
}

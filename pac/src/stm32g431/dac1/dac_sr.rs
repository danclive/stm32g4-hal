#[doc = "Register `DAC_SR` reader"]
pub type R = crate::R<DacSrSpec>;
#[doc = "Register `DAC_SR` writer"]
pub type W = crate::W<DacSrSpec>;
#[doc = "Field `DAC1RDY` reader - DAC channel1 ready status bit"]
pub type Dac1rdyR = crate::BitReader;
#[doc = "Field `DAC1RDY` writer - DAC channel1 ready status bit"]
pub type Dac1rdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DORSTAT1` reader - DAC channel1 output register status bit"]
pub type Dorstat1R = crate::BitReader;
#[doc = "Field `DORSTAT1` writer - DAC channel1 output register status bit"]
pub type Dorstat1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAUDR1` reader - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
pub type Dmaudr1R = crate::BitReader;
#[doc = "Field `DMAUDR1` writer - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
pub type Dmaudr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL_FLAG1` reader - DAC Channel 1 calibration offset status This bit is set and cleared by hardware"]
pub type CalFlag1R = crate::BitReader;
#[doc = "Field `BWST1` reader - DAC Channel 1 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3LSI periods of synchronization)."]
pub type Bwst1R = crate::BitReader;
#[doc = "Field `DAC2RDY` reader - DAC channel 2 ready status bit"]
pub type Dac2rdyR = crate::BitReader;
#[doc = "Field `DAC2RDY` writer - DAC channel 2 ready status bit"]
pub type Dac2rdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DORSTAT2` reader - DAC channel 2 output register status bit"]
pub type Dorstat2R = crate::BitReader;
#[doc = "Field `DORSTAT2` writer - DAC channel 2 output register status bit"]
pub type Dorstat2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAUDR2` reader - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
pub type Dmaudr2R = crate::BitReader;
#[doc = "Field `DMAUDR2` writer - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
pub type Dmaudr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL_FLAG2` reader - DAC Channel 2 calibration offset status This bit is set and cleared by hardware"]
pub type CalFlag2R = crate::BitReader;
#[doc = "Field `BWST2` reader - DAC Channel 2 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR2, It is cleared by hardware when the write operation of DAC_SHSR2 is complete. (It takes about 3 LSI periods of synchronization)."]
pub type Bwst2R = crate::BitReader;
impl R {
    #[doc = "Bit 11 - DAC channel1 ready status bit"]
    #[inline(always)]
    pub fn dac1rdy(&self) -> Dac1rdyR {
        Dac1rdyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DAC channel1 output register status bit"]
    #[inline(always)]
    pub fn dorstat1(&self) -> Dorstat1R {
        Dorstat1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    pub fn dmaudr1(&self) -> Dmaudr1R {
        Dmaudr1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DAC Channel 1 calibration offset status This bit is set and cleared by hardware"]
    #[inline(always)]
    pub fn cal_flag1(&self) -> CalFlag1R {
        CalFlag1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DAC Channel 1 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3LSI periods of synchronization)."]
    #[inline(always)]
    pub fn bwst1(&self) -> Bwst1R {
        Bwst1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 27 - DAC channel 2 ready status bit"]
    #[inline(always)]
    pub fn dac2rdy(&self) -> Dac2rdyR {
        Dac2rdyR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DAC channel 2 output register status bit"]
    #[inline(always)]
    pub fn dorstat2(&self) -> Dorstat2R {
        Dorstat2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    pub fn dmaudr2(&self) -> Dmaudr2R {
        Dmaudr2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DAC Channel 2 calibration offset status This bit is set and cleared by hardware"]
    #[inline(always)]
    pub fn cal_flag2(&self) -> CalFlag2R {
        CalFlag2R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DAC Channel 2 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR2, It is cleared by hardware when the write operation of DAC_SHSR2 is complete. (It takes about 3 LSI periods of synchronization)."]
    #[inline(always)]
    pub fn bwst2(&self) -> Bwst2R {
        Bwst2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_SR")
            .field("dac1rdy", &self.dac1rdy())
            .field("dorstat1", &self.dorstat1())
            .field("dmaudr1", &self.dmaudr1())
            .field("cal_flag1", &self.cal_flag1())
            .field("bwst1", &self.bwst1())
            .field("dac2rdy", &self.dac2rdy())
            .field("dorstat2", &self.dorstat2())
            .field("dmaudr2", &self.dmaudr2())
            .field("cal_flag2", &self.cal_flag2())
            .field("bwst2", &self.bwst2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 11 - DAC channel1 ready status bit"]
    #[inline(always)]
    pub fn dac1rdy(&mut self) -> Dac1rdyW<DacSrSpec> {
        Dac1rdyW::new(self, 11)
    }
    #[doc = "Bit 12 - DAC channel1 output register status bit"]
    #[inline(always)]
    pub fn dorstat1(&mut self) -> Dorstat1W<DacSrSpec> {
        Dorstat1W::new(self, 12)
    }
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    pub fn dmaudr1(&mut self) -> Dmaudr1W<DacSrSpec> {
        Dmaudr1W::new(self, 13)
    }
    #[doc = "Bit 27 - DAC channel 2 ready status bit"]
    #[inline(always)]
    pub fn dac2rdy(&mut self) -> Dac2rdyW<DacSrSpec> {
        Dac2rdyW::new(self, 27)
    }
    #[doc = "Bit 28 - DAC channel 2 output register status bit"]
    #[inline(always)]
    pub fn dorstat2(&mut self) -> Dorstat2W<DacSrSpec> {
        Dorstat2W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    pub fn dmaudr2(&mut self) -> Dmaudr2W<DacSrSpec> {
        Dmaudr2W::new(self, 29)
    }
}
#[doc = "DAC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacSrSpec;
impl crate::RegisterSpec for DacSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_sr::R`](R) reader structure"]
impl crate::Readable for DacSrSpec {}
#[doc = "`write(|w| ..)` method takes [`dac_sr::W`](W) writer structure"]
impl crate::Writable for DacSrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAC_SR to value 0"]
impl crate::Resettable for DacSrSpec {}

#[doc = "Register `DAC_CR` reader"]
pub type R = crate::R<DacCrSpec>;
#[doc = "Register `DAC_CR` writer"]
pub type W = crate::W<DacCrSpec>;
#[doc = "Field `EN1` reader - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1."]
pub type En1R = crate::BitReader;
#[doc = "Field `EN1` writer - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1."]
pub type En1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEN1` reader - DAC channel1 trigger enable"]
pub type Ten1R = crate::BitReader;
#[doc = "Field `TEN1` writer - DAC channel1 trigger enable"]
pub type Ten1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEL1` reader - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
pub type Tsel1R = crate::FieldReader;
#[doc = "Field `TSEL1` writer - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
pub type Tsel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WAVE1` reader - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
pub type Wave1R = crate::FieldReader;
#[doc = "Field `WAVE1` writer - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
pub type Wave1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAMP1` reader - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
pub type Mamp1R = crate::FieldReader;
#[doc = "Field `MAMP1` writer - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
pub type Mamp1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMAEN1` reader - DAC channel1 DMA enable This bit is set and cleared by software."]
pub type Dmaen1R = crate::BitReader;
#[doc = "Field `DMAEN1` writer - DAC channel1 DMA enable This bit is set and cleared by software."]
pub type Dmaen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAUDRIE1` reader - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software."]
pub type Dmaudrie1R = crate::BitReader;
#[doc = "Field `DMAUDRIE1` writer - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software."]
pub type Dmaudrie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEN1` reader - DAC Channel 1 calibration enable This bit is set and cleared by software to enable/disable DAC channel 1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
pub type Cen1R = crate::BitReader;
#[doc = "Field `CEN1` writer - DAC Channel 1 calibration enable This bit is set and cleared by software to enable/disable DAC channel 1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
pub type Cen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN2` reader - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2."]
pub type En2R = crate::BitReader;
#[doc = "Field `EN2` writer - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2."]
pub type En2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEN2` reader - DAC channel2 trigger enable"]
pub type Ten2R = crate::BitReader;
#[doc = "Field `TEN2` writer - DAC channel2 trigger enable"]
pub type Ten2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEL2` reader - DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)."]
pub type Tsel2R = crate::FieldReader;
#[doc = "Field `TSEL2` writer - DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)."]
pub type Tsel2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WAVE2` reader - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)"]
pub type Wave2R = crate::FieldReader;
#[doc = "Field `WAVE2` writer - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)"]
pub type Wave2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAMP2` reader - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
pub type Mamp2R = crate::FieldReader;
#[doc = "Field `MAMP2` writer - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
pub type Mamp2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMAEN2` reader - DAC channel2 DMA enable This bit is set and cleared by software."]
pub type Dmaen2R = crate::BitReader;
#[doc = "Field `DMAEN2` writer - DAC channel2 DMA enable This bit is set and cleared by software."]
pub type Dmaen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAUDRIE2` reader - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software."]
pub type Dmaudrie2R = crate::BitReader;
#[doc = "Field `DMAUDRIE2` writer - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software."]
pub type Dmaudrie2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEN2` reader - DAC Channel 2 calibration enable This bit is set and cleared by software to enable/disable DAC channel 2 calibration, it can be written only if bit EN2=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
pub type Cen2R = crate::BitReader;
#[doc = "Field `CEN2` writer - DAC Channel 2 calibration enable This bit is set and cleared by software to enable/disable DAC channel 2 calibration, it can be written only if bit EN2=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
pub type Cen2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1."]
    #[inline(always)]
    pub fn en1(&self) -> En1R {
        En1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC channel1 trigger enable"]
    #[inline(always)]
    pub fn ten1(&self) -> Ten1R {
        Ten1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    pub fn tsel1(&self) -> Tsel1R {
        Tsel1R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    pub fn wave1(&self) -> Wave1R {
        Wave1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    pub fn mamp1(&self) -> Mamp1R {
        Mamp1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dmaen1(&self) -> Dmaen1R {
        Dmaen1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dmaudrie1(&self) -> Dmaudrie1R {
        Dmaudrie1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DAC Channel 1 calibration enable This bit is set and cleared by software to enable/disable DAC channel 1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
    #[inline(always)]
    pub fn cen1(&self) -> Cen1R {
        Cen1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2."]
    #[inline(always)]
    pub fn en2(&self) -> En2R {
        En2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC channel2 trigger enable"]
    #[inline(always)]
    pub fn ten2(&self) -> Ten2R {
        Ten2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)."]
    #[inline(always)]
    pub fn tsel2(&self) -> Tsel2R {
        Tsel2R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)"]
    #[inline(always)]
    pub fn wave2(&self) -> Wave2R {
        Wave2R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    pub fn mamp2(&self) -> Mamp2R {
        Mamp2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - DAC channel2 DMA enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dmaen2(&self) -> Dmaen2R {
        Dmaen2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dmaudrie2(&self) -> Dmaudrie2R {
        Dmaudrie2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DAC Channel 2 calibration enable This bit is set and cleared by software to enable/disable DAC channel 2 calibration, it can be written only if bit EN2=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
    #[inline(always)]
    pub fn cen2(&self) -> Cen2R {
        Cen2R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_CR")
            .field("en1", &self.en1())
            .field("ten1", &self.ten1())
            .field("tsel1", &self.tsel1())
            .field("wave1", &self.wave1())
            .field("mamp1", &self.mamp1())
            .field("dmaen1", &self.dmaen1())
            .field("dmaudrie1", &self.dmaudrie1())
            .field("cen1", &self.cen1())
            .field("en2", &self.en2())
            .field("ten2", &self.ten2())
            .field("tsel2", &self.tsel2())
            .field("wave2", &self.wave2())
            .field("mamp2", &self.mamp2())
            .field("dmaen2", &self.dmaen2())
            .field("dmaudrie2", &self.dmaudrie2())
            .field("cen2", &self.cen2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1."]
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> En1W<DacCrSpec> {
        En1W::new(self, 0)
    }
    #[doc = "Bit 1 - DAC channel1 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten1(&mut self) -> Ten1W<DacCrSpec> {
        Ten1W::new(self, 1)
    }
    #[doc = "Bits 2:5 - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    #[must_use]
    pub fn tsel1(&mut self) -> Tsel1W<DacCrSpec> {
        Tsel1W::new(self, 2)
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    #[must_use]
    pub fn wave1(&mut self) -> Wave1W<DacCrSpec> {
        Wave1W::new(self, 6)
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    #[must_use]
    pub fn mamp1(&mut self) -> Mamp1W<DacCrSpec> {
        Mamp1W::new(self, 8)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dmaen1(&mut self) -> Dmaen1W<DacCrSpec> {
        Dmaen1W::new(self, 12)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dmaudrie1(&mut self) -> Dmaudrie1W<DacCrSpec> {
        Dmaudrie1W::new(self, 13)
    }
    #[doc = "Bit 14 - DAC Channel 1 calibration enable This bit is set and cleared by software to enable/disable DAC channel 1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn cen1(&mut self) -> Cen1W<DacCrSpec> {
        Cen1W::new(self, 14)
    }
    #[doc = "Bit 16 - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2."]
    #[inline(always)]
    #[must_use]
    pub fn en2(&mut self) -> En2W<DacCrSpec> {
        En2W::new(self, 16)
    }
    #[doc = "Bit 17 - DAC channel2 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten2(&mut self) -> Ten2W<DacCrSpec> {
        Ten2W::new(self, 17)
    }
    #[doc = "Bits 18:21 - DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)."]
    #[inline(always)]
    #[must_use]
    pub fn tsel2(&mut self) -> Tsel2W<DacCrSpec> {
        Tsel2W::new(self, 18)
    }
    #[doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)"]
    #[inline(always)]
    #[must_use]
    pub fn wave2(&mut self) -> Wave2W<DacCrSpec> {
        Wave2W::new(self, 22)
    }
    #[doc = "Bits 24:27 - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    #[must_use]
    pub fn mamp2(&mut self) -> Mamp2W<DacCrSpec> {
        Mamp2W::new(self, 24)
    }
    #[doc = "Bit 28 - DAC channel2 DMA enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dmaen2(&mut self) -> Dmaen2W<DacCrSpec> {
        Dmaen2W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dmaudrie2(&mut self) -> Dmaudrie2W<DacCrSpec> {
        Dmaudrie2W::new(self, 29)
    }
    #[doc = "Bit 30 - DAC Channel 2 calibration enable This bit is set and cleared by software to enable/disable DAC channel 2 calibration, it can be written only if bit EN2=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn cen2(&mut self) -> Cen2W<DacCrSpec> {
        Cen2W::new(self, 30)
    }
}
#[doc = "DAC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacCrSpec;
impl crate::RegisterSpec for DacCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_cr::R`](R) reader structure"]
impl crate::Readable for DacCrSpec {}
#[doc = "`write(|w| ..)` method takes [`dac_cr::W`](W) writer structure"]
impl crate::Writable for DacCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_CR to value 0"]
impl crate::Resettable for DacCrSpec {
    const RESET_VALUE: u32 = 0;
}

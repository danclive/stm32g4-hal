#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `CCPC` reader - Capture/compare preloaded control"]
pub type CcpcR = crate::BitReader;
#[doc = "Field `CCPC` writer - Capture/compare preloaded control"]
pub type CcpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCUS` reader - Capture/compare control update selection"]
pub type CcusR = crate::BitReader;
#[doc = "Field `CCUS` writer - Capture/compare control update selection"]
pub type CcusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCDS` reader - Capture/compare DMA selection"]
pub type CcdsR = crate::BitReader;
#[doc = "Field `CCDS` writer - Capture/compare DMA selection"]
pub type CcdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMS` reader - Master mode selection"]
pub type MmsR = crate::FieldReader;
#[doc = "Field `MMS` writer - Master mode selection"]
pub type MmsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TI1S` reader - TI1 selection"]
pub type Ti1sR = crate::BitReader;
#[doc = "Field `TI1S` writer - TI1 selection"]
pub type Ti1sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS1` reader - Output Idle state 1"]
pub type Ois1R = crate::BitReader;
#[doc = "Field `OIS1` writer - Output Idle state 1"]
pub type Ois1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS1N` reader - Output Idle state 1"]
pub type Ois1nR = crate::BitReader;
#[doc = "Field `OIS1N` writer - Output Idle state 1"]
pub type Ois1nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS2` reader - Output Idle state 2"]
pub type Ois2R = crate::BitReader;
#[doc = "Field `OIS2` writer - Output Idle state 2"]
pub type Ois2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS2N` reader - Output Idle state 2"]
pub type Ois2nR = crate::BitReader;
#[doc = "Field `OIS2N` writer - Output Idle state 2"]
pub type Ois2nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS3` reader - Output Idle state 3"]
pub type Ois3R = crate::BitReader;
#[doc = "Field `OIS3` writer - Output Idle state 3"]
pub type Ois3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS3N` reader - Output Idle state 3"]
pub type Ois3nR = crate::BitReader;
#[doc = "Field `OIS3N` writer - Output Idle state 3"]
pub type Ois3nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS4` reader - Output Idle state 4"]
pub type Ois4R = crate::BitReader;
#[doc = "Field `OIS4` writer - Output Idle state 4"]
pub type Ois4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS4N` reader - Output Idle state 4 (OC4N output)"]
pub type Ois4nR = crate::BitReader;
#[doc = "Field `OIS4N` writer - Output Idle state 4 (OC4N output)"]
pub type Ois4nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS5` reader - Output Idle state 5 (OC5 output)"]
pub type Ois5R = crate::BitReader;
#[doc = "Field `OIS5` writer - Output Idle state 5 (OC5 output)"]
pub type Ois5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS6` reader - Output Idle state 6 (OC6 output)"]
pub type Ois6R = crate::BitReader;
#[doc = "Field `OIS6` writer - Output Idle state 6 (OC6 output)"]
pub type Ois6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMS2` reader - Master mode selection 2"]
pub type Mms2R = crate::FieldReader;
#[doc = "Field `MMS2` writer - Master mode selection 2"]
pub type Mms2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MMS_3` reader - Master mode selection - bit 3"]
pub type Mms3R = crate::BitReader;
#[doc = "Field `MMS_3` writer - Master mode selection - bit 3"]
pub type Mms3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture/compare preloaded control"]
    #[inline(always)]
    pub fn ccpc(&self) -> CcpcR {
        CcpcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection"]
    #[inline(always)]
    pub fn ccus(&self) -> CcusR {
        CcusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&self) -> CcdsR {
        CcdsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    pub fn mms(&self) -> MmsR {
        MmsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    pub fn ti1s(&self) -> Ti1sR {
        Ti1sR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1(&self) -> Ois1R {
        Ois1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1n(&self) -> Ois1nR {
        Ois1nR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output Idle state 2"]
    #[inline(always)]
    pub fn ois2(&self) -> Ois2R {
        Ois2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output Idle state 2"]
    #[inline(always)]
    pub fn ois2n(&self) -> Ois2nR {
        Ois2nR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output Idle state 3"]
    #[inline(always)]
    pub fn ois3(&self) -> Ois3R {
        Ois3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output Idle state 3"]
    #[inline(always)]
    pub fn ois3n(&self) -> Ois3nR {
        Ois3nR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output Idle state 4"]
    #[inline(always)]
    pub fn ois4(&self) -> Ois4R {
        Ois4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Output Idle state 4 (OC4N output)"]
    #[inline(always)]
    pub fn ois4n(&self) -> Ois4nR {
        Ois4nR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output Idle state 5 (OC5 output)"]
    #[inline(always)]
    pub fn ois5(&self) -> Ois5R {
        Ois5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Output Idle state 6 (OC6 output)"]
    #[inline(always)]
    pub fn ois6(&self) -> Ois6R {
        Ois6R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Master mode selection 2"]
    #[inline(always)]
    pub fn mms2(&self) -> Mms2R {
        Mms2R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 25 - Master mode selection - bit 3"]
    #[inline(always)]
    pub fn mms_3(&self) -> Mms3R {
        Mms3R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("mms_3", &self.mms_3())
            .field("mms2", &self.mms2())
            .field("ois6", &self.ois6())
            .field("ois5", &self.ois5())
            .field("ois4n", &self.ois4n())
            .field("ois4", &self.ois4())
            .field("ois3n", &self.ois3n())
            .field("ois3", &self.ois3())
            .field("ois2n", &self.ois2n())
            .field("ois2", &self.ois2())
            .field("ois1n", &self.ois1n())
            .field("ois1", &self.ois1())
            .field("ti1s", &self.ti1s())
            .field("mms", &self.mms())
            .field("ccds", &self.ccds())
            .field("ccus", &self.ccus())
            .field("ccpc", &self.ccpc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare preloaded control"]
    #[inline(always)]
    pub fn ccpc(&mut self) -> CcpcW<Cr2Spec> {
        CcpcW::new(self, 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection"]
    #[inline(always)]
    pub fn ccus(&mut self) -> CcusW<Cr2Spec> {
        CcusW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&mut self) -> CcdsW<Cr2Spec> {
        CcdsW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    pub fn mms(&mut self) -> MmsW<Cr2Spec> {
        MmsW::new(self, 4)
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    pub fn ti1s(&mut self) -> Ti1sW<Cr2Spec> {
        Ti1sW::new(self, 7)
    }
    #[doc = "Bit 8 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1(&mut self) -> Ois1W<Cr2Spec> {
        Ois1W::new(self, 8)
    }
    #[doc = "Bit 9 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1n(&mut self) -> Ois1nW<Cr2Spec> {
        Ois1nW::new(self, 9)
    }
    #[doc = "Bit 10 - Output Idle state 2"]
    #[inline(always)]
    pub fn ois2(&mut self) -> Ois2W<Cr2Spec> {
        Ois2W::new(self, 10)
    }
    #[doc = "Bit 11 - Output Idle state 2"]
    #[inline(always)]
    pub fn ois2n(&mut self) -> Ois2nW<Cr2Spec> {
        Ois2nW::new(self, 11)
    }
    #[doc = "Bit 12 - Output Idle state 3"]
    #[inline(always)]
    pub fn ois3(&mut self) -> Ois3W<Cr2Spec> {
        Ois3W::new(self, 12)
    }
    #[doc = "Bit 13 - Output Idle state 3"]
    #[inline(always)]
    pub fn ois3n(&mut self) -> Ois3nW<Cr2Spec> {
        Ois3nW::new(self, 13)
    }
    #[doc = "Bit 14 - Output Idle state 4"]
    #[inline(always)]
    pub fn ois4(&mut self) -> Ois4W<Cr2Spec> {
        Ois4W::new(self, 14)
    }
    #[doc = "Bit 15 - Output Idle state 4 (OC4N output)"]
    #[inline(always)]
    pub fn ois4n(&mut self) -> Ois4nW<Cr2Spec> {
        Ois4nW::new(self, 15)
    }
    #[doc = "Bit 16 - Output Idle state 5 (OC5 output)"]
    #[inline(always)]
    pub fn ois5(&mut self) -> Ois5W<Cr2Spec> {
        Ois5W::new(self, 16)
    }
    #[doc = "Bit 18 - Output Idle state 6 (OC6 output)"]
    #[inline(always)]
    pub fn ois6(&mut self) -> Ois6W<Cr2Spec> {
        Ois6W::new(self, 18)
    }
    #[doc = "Bits 20:23 - Master mode selection 2"]
    #[inline(always)]
    pub fn mms2(&mut self) -> Mms2W<Cr2Spec> {
        Mms2W::new(self, 20)
    }
    #[doc = "Bit 25 - Master mode selection - bit 3"]
    #[inline(always)]
    pub fn mms_3(&mut self) -> Mms3W<Cr2Spec> {
        Mms3W::new(self, 25)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {
    const RESET_VALUE: u32 = 0;
}

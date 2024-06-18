#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `SYNCOKIE` reader - SYNC event OK interrupt enable"]
pub type SyncokieR = crate::BitReader;
#[doc = "Field `SYNCOKIE` writer - SYNC event OK interrupt enable"]
pub type SyncokieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCWARNIE` reader - SYNC warning interrupt enable"]
pub type SyncwarnieR = crate::BitReader;
#[doc = "Field `SYNCWARNIE` writer - SYNC warning interrupt enable"]
pub type SyncwarnieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Synchronization or trimming error interrupt enable"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - Synchronization or trimming error interrupt enable"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESYNCIE` reader - Expected SYNC interrupt enable"]
pub type EsyncieR = crate::BitReader;
#[doc = "Field `ESYNCIE` writer - Expected SYNC interrupt enable"]
pub type EsyncieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEN` reader - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
pub type CenR = crate::BitReader;
#[doc = "Field `CEN` writer - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOTRIMEN` reader - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details."]
pub type AutotrimenR = crate::BitReader;
#[doc = "Field `AUTOTRIMEN` writer - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details."]
pub type AutotrimenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWSYNC` reader - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware."]
pub type SwsyncR = crate::BitReader;
#[doc = "Field `SWSYNC` writer - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware."]
pub type SwsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIM` reader - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is around 67 kHz between two consecutive TRIM steps. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only."]
pub type TrimR = crate::FieldReader;
#[doc = "Field `TRIM` writer - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is around 67 kHz between two consecutive TRIM steps. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only."]
pub type TrimW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - SYNC event OK interrupt enable"]
    #[inline(always)]
    pub fn syncokie(&self) -> SyncokieR {
        SyncokieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNC warning interrupt enable"]
    #[inline(always)]
    pub fn syncwarnie(&self) -> SyncwarnieR {
        SyncwarnieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization or trimming error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expected SYNC interrupt enable"]
    #[inline(always)]
    pub fn esyncie(&self) -> EsyncieR {
        EsyncieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details."]
    #[inline(always)]
    pub fn autotrimen(&self) -> AutotrimenR {
        AutotrimenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware."]
    #[inline(always)]
    pub fn swsync(&self) -> SwsyncR {
        SwsyncR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is around 67 kHz between two consecutive TRIM steps. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only."]
    #[inline(always)]
    pub fn trim(&self) -> TrimR {
        TrimR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("syncokie", &self.syncokie())
            .field("syncwarnie", &self.syncwarnie())
            .field("errie", &self.errie())
            .field("esyncie", &self.esyncie())
            .field("cen", &self.cen())
            .field("autotrimen", &self.autotrimen())
            .field("swsync", &self.swsync())
            .field("trim", &self.trim())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SYNC event OK interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncokie(&mut self) -> SyncokieW<CrSpec> {
        SyncokieW::new(self, 0)
    }
    #[doc = "Bit 1 - SYNC warning interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncwarnie(&mut self) -> SyncwarnieW<CrSpec> {
        SyncwarnieW::new(self, 1)
    }
    #[doc = "Bit 2 - Synchronization or trimming error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ErrieW<CrSpec> {
        ErrieW::new(self, 2)
    }
    #[doc = "Bit 3 - Expected SYNC interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn esyncie(&mut self) -> EsyncieW<CrSpec> {
        EsyncieW::new(self, 3)
    }
    #[doc = "Bit 5 - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CenW<CrSpec> {
        CenW::new(self, 5)
    }
    #[doc = "Bit 6 - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details."]
    #[inline(always)]
    #[must_use]
    pub fn autotrimen(&mut self) -> AutotrimenW<CrSpec> {
        AutotrimenW::new(self, 6)
    }
    #[doc = "Bit 7 - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn swsync(&mut self) -> SwsyncW<CrSpec> {
        SwsyncW::new(self, 7)
    }
    #[doc = "Bits 8:14 - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is around 67 kHz between two consecutive TRIM steps. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only."]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TrimW<CrSpec> {
        TrimW::new(self, 8)
    }
}
#[doc = "CRS control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0x4000"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x4000;
}

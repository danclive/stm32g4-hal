#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `RELOAD` reader - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to Section7.3.3: Frequency error measurement for more details about counter behavior."]
pub type ReloadR = crate::FieldReader<u16>;
#[doc = "Field `RELOAD` writer - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to Section7.3.3: Frequency error measurement for more details about counter behavior."]
pub type ReloadW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FELIM` reader - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\\[15:0\\]
bits of the CRS_ISR register. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details about FECAP evaluation."]
pub type FelimR = crate::FieldReader;
#[doc = "Field `FELIM` writer - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\\[15:0\\]
bits of the CRS_ISR register. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details about FECAP evaluation."]
pub type FelimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SYNCDIV` reader - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal."]
pub type SyncdivR = crate::FieldReader;
#[doc = "Field `SYNCDIV` writer - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal."]
pub type SyncdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SYNCSRC` reader - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source. Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF will not be generated by the host. No SYNC signal will therefore be provided to the CRS to calibrate the HSI48 on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs should be used as SYNC signal."]
pub type SyncsrcR = crate::FieldReader;
#[doc = "Field `SYNCSRC` writer - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source. Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF will not be generated by the host. No SYNC signal will therefore be provided to the CRS to calibrate the HSI48 on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs should be used as SYNC signal."]
pub type SyncsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYNCPOL` reader - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source."]
pub type SyncpolR = crate::BitReader;
#[doc = "Field `SYNCPOL` writer - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source."]
pub type SyncpolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to Section7.3.3: Frequency error measurement for more details about counter behavior."]
    #[inline(always)]
    pub fn reload(&self) -> ReloadR {
        ReloadR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\\[15:0\\]
bits of the CRS_ISR register. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details about FECAP evaluation."]
    #[inline(always)]
    pub fn felim(&self) -> FelimR {
        FelimR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal."]
    #[inline(always)]
    pub fn syncdiv(&self) -> SyncdivR {
        SyncdivR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source. Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF will not be generated by the host. No SYNC signal will therefore be provided to the CRS to calibrate the HSI48 on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs should be used as SYNC signal."]
    #[inline(always)]
    pub fn syncsrc(&self) -> SyncsrcR {
        SyncsrcR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source."]
    #[inline(always)]
    pub fn syncpol(&self) -> SyncpolR {
        SyncpolR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("reload", &self.reload())
            .field("felim", &self.felim())
            .field("syncdiv", &self.syncdiv())
            .field("syncsrc", &self.syncsrc())
            .field("syncpol", &self.syncpol())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to Section7.3.3: Frequency error measurement for more details about counter behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> ReloadW<CfgrSpec> {
        ReloadW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\\[15:0\\]
bits of the CRS_ISR register. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details about FECAP evaluation."]
    #[inline(always)]
    #[must_use]
    pub fn felim(&mut self) -> FelimW<CfgrSpec> {
        FelimW::new(self, 16)
    }
    #[doc = "Bits 24:26 - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal."]
    #[inline(always)]
    #[must_use]
    pub fn syncdiv(&mut self) -> SyncdivW<CfgrSpec> {
        SyncdivW::new(self, 24)
    }
    #[doc = "Bits 28:29 - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source. Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF will not be generated by the host. No SYNC signal will therefore be provided to the CRS to calibrate the HSI48 on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs should be used as SYNC signal."]
    #[inline(always)]
    #[must_use]
    pub fn syncsrc(&mut self) -> SyncsrcW<CfgrSpec> {
        SyncsrcW::new(self, 28)
    }
    #[doc = "Bit 31 - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source."]
    #[inline(always)]
    #[must_use]
    pub fn syncpol(&mut self) -> SyncpolW<CfgrSpec> {
        SyncpolW::new(self, 31)
    }
}
#[doc = "This register can be written only when the frequency error counter is disabled (CEN bit is cleared in CRS_CR). When the counter is enabled, this register is write-protected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0x2022_bb7f"]
impl crate::Resettable for CfgrSpec {
    const RESET_VALUE: u32 = 0x2022_bb7f;
}

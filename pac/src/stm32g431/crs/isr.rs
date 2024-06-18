#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `SYNCOKF` reader - SYNC event OK flag This flag is set by hardware when the measured frequency error is smaller than FELIM * 3. This means that either no adjustment of the TRIM value is needed or that an adjustment by one trimming step is enough to compensate the frequency error. An interrupt is generated if the SYNCOKIE bit is set in the CRS_CR register. It is cleared by software by setting the SYNCOKC bit in the CRS_ICR register."]
pub type SyncokfR = crate::BitReader;
#[doc = "Field `SYNCWARNF` reader - SYNC warning flag This flag is set by hardware when the measured frequency error is greater than or equal to FELIM * 3, but smaller than FELIM * 128. This means that to compensate the frequency error, the TRIM value must be adjusted by two steps or more. An interrupt is generated if the SYNCWARNIE bit is set in the CRS_CR register. It is cleared by software by setting the SYNCWARNC bit in the CRS_ICR register."]
pub type SyncwarnfR = crate::BitReader;
#[doc = "Field `ERRF` reader - Error flag This flag is set by hardware in case of any synchronization or trimming error. It is the logical OR of the TRIMOVF, SYNCMISS and SYNCERR bits. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software in reaction to setting the ERRC bit in the CRS_ICR register, which clears the TRIMOVF, SYNCMISS and SYNCERR bits."]
pub type ErrfR = crate::BitReader;
#[doc = "Field `ESYNCF` reader - Expected SYNC flag This flag is set by hardware when the frequency error counter reached a zero value. An interrupt is generated if the ESYNCIE bit is set in the CRS_CR register. It is cleared by software by setting the ESYNCC bit in the CRS_ICR register."]
pub type EsyncfR = crate::BitReader;
#[doc = "Field `SYNCERR` reader - SYNC error This flag is set by hardware when the SYNC pulse arrives before the ESYNC event and the measured frequency error is greater than or equal to FELIM * 128. This means that the frequency error is too big (internal frequency too low) to be compensated by adjusting the TRIM value, and that some other action should be taken. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register."]
pub type SyncerrR = crate::BitReader;
#[doc = "Field `SYNCMISS` reader - SYNC missed This flag is set by hardware when the frequency error counter reached value FELIM * 128 and no SYNC was detected, meaning either that a SYNC pulse was missed or that the frequency error is too big (internal frequency too high) to be compensated by adjusting the TRIM value, and that some other action should be taken. At this point, the frequency error counter is stopped (waiting for a next SYNC) and an interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register."]
pub type SyncmissR = crate::BitReader;
#[doc = "Field `TRIMOVF` reader - Trimming overflow or underflow This flag is set by hardware when the automatic trimming tries to over- or under-flow the TRIM value. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register."]
pub type TrimovfR = crate::BitReader;
#[doc = "Field `FEDIR` reader - Frequency error direction FEDIR is the counting direction of the frequency error counter latched in the time of the last SYNC event. It shows whether the actual frequency is below or above the target."]
pub type FedirR = crate::BitReader;
#[doc = "Field `FECAP` reader - Frequency error capture FECAP is the frequency error counter value latched in the time ofthe last SYNC event. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details about FECAP usage."]
pub type FecapR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - SYNC event OK flag This flag is set by hardware when the measured frequency error is smaller than FELIM * 3. This means that either no adjustment of the TRIM value is needed or that an adjustment by one trimming step is enough to compensate the frequency error. An interrupt is generated if the SYNCOKIE bit is set in the CRS_CR register. It is cleared by software by setting the SYNCOKC bit in the CRS_ICR register."]
    #[inline(always)]
    pub fn syncokf(&self) -> SyncokfR {
        SyncokfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNC warning flag This flag is set by hardware when the measured frequency error is greater than or equal to FELIM * 3, but smaller than FELIM * 128. This means that to compensate the frequency error, the TRIM value must be adjusted by two steps or more. An interrupt is generated if the SYNCWARNIE bit is set in the CRS_CR register. It is cleared by software by setting the SYNCWARNC bit in the CRS_ICR register."]
    #[inline(always)]
    pub fn syncwarnf(&self) -> SyncwarnfR {
        SyncwarnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error flag This flag is set by hardware in case of any synchronization or trimming error. It is the logical OR of the TRIMOVF, SYNCMISS and SYNCERR bits. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software in reaction to setting the ERRC bit in the CRS_ICR register, which clears the TRIMOVF, SYNCMISS and SYNCERR bits."]
    #[inline(always)]
    pub fn errf(&self) -> ErrfR {
        ErrfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expected SYNC flag This flag is set by hardware when the frequency error counter reached a zero value. An interrupt is generated if the ESYNCIE bit is set in the CRS_CR register. It is cleared by software by setting the ESYNCC bit in the CRS_ICR register."]
    #[inline(always)]
    pub fn esyncf(&self) -> EsyncfR {
        EsyncfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SYNC error This flag is set by hardware when the SYNC pulse arrives before the ESYNC event and the measured frequency error is greater than or equal to FELIM * 128. This means that the frequency error is too big (internal frequency too low) to be compensated by adjusting the TRIM value, and that some other action should be taken. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register."]
    #[inline(always)]
    pub fn syncerr(&self) -> SyncerrR {
        SyncerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SYNC missed This flag is set by hardware when the frequency error counter reached value FELIM * 128 and no SYNC was detected, meaning either that a SYNC pulse was missed or that the frequency error is too big (internal frequency too high) to be compensated by adjusting the TRIM value, and that some other action should be taken. At this point, the frequency error counter is stopped (waiting for a next SYNC) and an interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register."]
    #[inline(always)]
    pub fn syncmiss(&self) -> SyncmissR {
        SyncmissR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Trimming overflow or underflow This flag is set by hardware when the automatic trimming tries to over- or under-flow the TRIM value. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register."]
    #[inline(always)]
    pub fn trimovf(&self) -> TrimovfR {
        TrimovfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Frequency error direction FEDIR is the counting direction of the frequency error counter latched in the time of the last SYNC event. It shows whether the actual frequency is below or above the target."]
    #[inline(always)]
    pub fn fedir(&self) -> FedirR {
        FedirR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Frequency error capture FECAP is the frequency error counter value latched in the time ofthe last SYNC event. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details about FECAP usage."]
    #[inline(always)]
    pub fn fecap(&self) -> FecapR {
        FecapR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("syncokf", &self.syncokf())
            .field("syncwarnf", &self.syncwarnf())
            .field("errf", &self.errf())
            .field("esyncf", &self.esyncf())
            .field("syncerr", &self.syncerr())
            .field("syncmiss", &self.syncmiss())
            .field("trimovf", &self.trimovf())
            .field("fedir", &self.fedir())
            .field("fecap", &self.fecap())
            .finish()
    }
}
#[doc = "CRS interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0;
}

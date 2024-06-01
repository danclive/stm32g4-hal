#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Field `ADDRDY_MST` reader - ADDRDY_MST"]
pub type AddrdyMstR = crate::BitReader;
#[doc = "Field `EOSMP_MST` reader - EOSMP_MST"]
pub type EosmpMstR = crate::BitReader;
#[doc = "Field `EOC_MST` reader - EOC_MST"]
pub type EocMstR = crate::BitReader;
#[doc = "Field `EOS_MST` reader - EOS_MST"]
pub type EosMstR = crate::BitReader;
#[doc = "Field `OVR_MST` reader - OVR_MST"]
pub type OvrMstR = crate::BitReader;
#[doc = "Field `JEOC_MST` reader - JEOC_MST"]
pub type JeocMstR = crate::BitReader;
#[doc = "Field `JEOS_MST` reader - JEOS_MST"]
pub type JeosMstR = crate::BitReader;
#[doc = "Field `AWD1_MST` reader - AWD1_MST"]
pub type Awd1MstR = crate::BitReader;
#[doc = "Field `AWD2_MST` reader - AWD2_MST"]
pub type Awd2MstR = crate::BitReader;
#[doc = "Field `AWD3_MST` reader - AWD3_MST"]
pub type Awd3MstR = crate::BitReader;
#[doc = "Field `JQOVF_MST` reader - JQOVF_MST"]
pub type JqovfMstR = crate::BitReader;
#[doc = "Field `ADRDY_SLV` reader - ADRDY_SLV"]
pub type AdrdySlvR = crate::BitReader;
#[doc = "Field `EOSMP_SLV` reader - EOSMP_SLV"]
pub type EosmpSlvR = crate::BitReader;
#[doc = "Field `EOC_SLV` reader - End of regular conversion of the slave ADC"]
pub type EocSlvR = crate::BitReader;
#[doc = "Field `EOS_SLV` reader - End of regular sequence flag of the slave ADC"]
pub type EosSlvR = crate::BitReader;
#[doc = "Field `OVR_SLV` reader - Overrun flag of the slave ADC"]
pub type OvrSlvR = crate::BitReader;
#[doc = "Field `JEOC_SLV` reader - End of injected conversion flag of the slave ADC"]
pub type JeocSlvR = crate::BitReader;
#[doc = "Field `JEOS_SLV` reader - End of injected sequence flag of the slave ADC"]
pub type JeosSlvR = crate::BitReader;
#[doc = "Field `AWD1_SLV` reader - Analog watchdog 1 flag of the slave ADC"]
pub type Awd1SlvR = crate::BitReader;
#[doc = "Field `AWD2_SLV` reader - Analog watchdog 2 flag of the slave ADC"]
pub type Awd2SlvR = crate::BitReader;
#[doc = "Field `AWD3_SLV` reader - Analog watchdog 3 flag of the slave ADC"]
pub type Awd3SlvR = crate::BitReader;
#[doc = "Field `JQOVF_SLV` reader - Injected Context Queue Overflow flag of the slave ADC"]
pub type JqovfSlvR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ADDRDY_MST"]
    #[inline(always)]
    pub fn addrdy_mst(&self) -> AddrdyMstR {
        AddrdyMstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EOSMP_MST"]
    #[inline(always)]
    pub fn eosmp_mst(&self) -> EosmpMstR {
        EosmpMstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EOC_MST"]
    #[inline(always)]
    pub fn eoc_mst(&self) -> EocMstR {
        EocMstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOS_MST"]
    #[inline(always)]
    pub fn eos_mst(&self) -> EosMstR {
        EosMstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OVR_MST"]
    #[inline(always)]
    pub fn ovr_mst(&self) -> OvrMstR {
        OvrMstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JEOC_MST"]
    #[inline(always)]
    pub fn jeoc_mst(&self) -> JeocMstR {
        JeocMstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - JEOS_MST"]
    #[inline(always)]
    pub fn jeos_mst(&self) -> JeosMstR {
        JeosMstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AWD1_MST"]
    #[inline(always)]
    pub fn awd1_mst(&self) -> Awd1MstR {
        Awd1MstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AWD2_MST"]
    #[inline(always)]
    pub fn awd2_mst(&self) -> Awd2MstR {
        Awd2MstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AWD3_MST"]
    #[inline(always)]
    pub fn awd3_mst(&self) -> Awd3MstR {
        Awd3MstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - JQOVF_MST"]
    #[inline(always)]
    pub fn jqovf_mst(&self) -> JqovfMstR {
        JqovfMstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - ADRDY_SLV"]
    #[inline(always)]
    pub fn adrdy_slv(&self) -> AdrdySlvR {
        AdrdySlvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - EOSMP_SLV"]
    #[inline(always)]
    pub fn eosmp_slv(&self) -> EosmpSlvR {
        EosmpSlvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - End of regular conversion of the slave ADC"]
    #[inline(always)]
    pub fn eoc_slv(&self) -> EocSlvR {
        EocSlvR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - End of regular sequence flag of the slave ADC"]
    #[inline(always)]
    pub fn eos_slv(&self) -> EosSlvR {
        EosSlvR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Overrun flag of the slave ADC"]
    #[inline(always)]
    pub fn ovr_slv(&self) -> OvrSlvR {
        OvrSlvR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - End of injected conversion flag of the slave ADC"]
    #[inline(always)]
    pub fn jeoc_slv(&self) -> JeocSlvR {
        JeocSlvR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - End of injected sequence flag of the slave ADC"]
    #[inline(always)]
    pub fn jeos_slv(&self) -> JeosSlvR {
        JeosSlvR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog watchdog 1 flag of the slave ADC"]
    #[inline(always)]
    pub fn awd1_slv(&self) -> Awd1SlvR {
        Awd1SlvR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Analog watchdog 2 flag of the slave ADC"]
    #[inline(always)]
    pub fn awd2_slv(&self) -> Awd2SlvR {
        Awd2SlvR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Analog watchdog 3 flag of the slave ADC"]
    #[inline(always)]
    pub fn awd3_slv(&self) -> Awd3SlvR {
        Awd3SlvR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Injected Context Queue Overflow flag of the slave ADC"]
    #[inline(always)]
    pub fn jqovf_slv(&self) -> JqovfSlvR {
        JqovfSlvR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("addrdy_mst", &self.addrdy_mst())
            .field("eosmp_mst", &self.eosmp_mst())
            .field("eoc_mst", &self.eoc_mst())
            .field("eos_mst", &self.eos_mst())
            .field("ovr_mst", &self.ovr_mst())
            .field("jeoc_mst", &self.jeoc_mst())
            .field("jeos_mst", &self.jeos_mst())
            .field("awd1_mst", &self.awd1_mst())
            .field("awd2_mst", &self.awd2_mst())
            .field("awd3_mst", &self.awd3_mst())
            .field("jqovf_mst", &self.jqovf_mst())
            .field("adrdy_slv", &self.adrdy_slv())
            .field("eosmp_slv", &self.eosmp_slv())
            .field("eoc_slv", &self.eoc_slv())
            .field("eos_slv", &self.eos_slv())
            .field("ovr_slv", &self.ovr_slv())
            .field("jeoc_slv", &self.jeoc_slv())
            .field("jeos_slv", &self.jeos_slv())
            .field("awd1_slv", &self.awd1_slv())
            .field("awd2_slv", &self.awd2_slv())
            .field("awd3_slv", &self.awd3_slv())
            .field("jqovf_slv", &self.jqovf_slv())
            .finish()
    }
}
#[doc = "ADC Common status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {
    const RESET_VALUE: u32 = 0;
}

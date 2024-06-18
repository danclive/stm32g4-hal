#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `CCF` reader - Computation complete flag"]
pub type CcfR = crate::BitReader;
#[doc = "Field `RDERR` reader - Read error flag"]
pub type RderrR = crate::BitReader;
#[doc = "Field `WRERR` reader - Write error flag"]
pub type WrerrR = crate::BitReader;
#[doc = "Field `BUSY` reader - BUSY"]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Computation complete flag"]
    #[inline(always)]
    pub fn ccf(&self) -> CcfR {
        CcfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read error flag"]
    #[inline(always)]
    pub fn rderr(&self) -> RderrR {
        RderrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write error flag"]
    #[inline(always)]
    pub fn wrerr(&self) -> WrerrR {
        WrerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("busy", &self.busy())
            .field("wrerr", &self.wrerr())
            .field("rderr", &self.rderr())
            .field("ccf", &self.ccf())
            .finish()
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0;
}

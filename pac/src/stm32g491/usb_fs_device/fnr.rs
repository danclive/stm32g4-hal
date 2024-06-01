#[doc = "Register `FNR` reader"]
pub type R = crate::R<FnrSpec>;
#[doc = "Field `FN` reader - FN"]
pub type FnR = crate::FieldReader<u16>;
#[doc = "Field `LSOF` reader - LSOF"]
pub type LsofR = crate::FieldReader;
#[doc = "Field `LCK` reader - LCK"]
pub type LckR = crate::BitReader;
#[doc = "Field `RXDM` reader - RXDM"]
pub type RxdmR = crate::BitReader;
#[doc = "Field `RXDP` reader - RXDP"]
pub type RxdpR = crate::BitReader;
impl R {
    #[doc = "Bits 0:10 - FN"]
    #[inline(always)]
    pub fn fn_(&self) -> FnR {
        FnR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:12 - LSOF"]
    #[inline(always)]
    pub fn lsof(&self) -> LsofR {
        LsofR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - LCK"]
    #[inline(always)]
    pub fn lck(&self) -> LckR {
        LckR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RXDM"]
    #[inline(always)]
    pub fn rxdm(&self) -> RxdmR {
        RxdmR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RXDP"]
    #[inline(always)]
    pub fn rxdp(&self) -> RxdpR {
        RxdpR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FNR")
            .field("fn_", &self.fn_())
            .field("lsof", &self.lsof())
            .field("lck", &self.lck())
            .field("rxdm", &self.rxdm())
            .field("rxdp", &self.rxdp())
            .finish()
    }
}
#[doc = "USB frame number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fnr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FnrSpec;
impl crate::RegisterSpec for FnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fnr::R`](R) reader structure"]
impl crate::Readable for FnrSpec {}
#[doc = "`reset()` method sets FNR to value 0"]
impl crate::Resettable for FnrSpec {
    const RESET_VALUE: u32 = 0;
}

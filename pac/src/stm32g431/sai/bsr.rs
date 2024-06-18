#[doc = "Register `BSR` reader"]
pub type R = crate::R<BsrSpec>;
#[doc = "Field `OVRUDR` reader - Overrun / underrun"]
pub type OvrudrR = crate::BitReader;
#[doc = "Field `MUTEDET` reader - Mute detection"]
pub type MutedetR = crate::BitReader;
#[doc = "Field `WCKCFG` reader - Wrong clock configuration flag"]
pub type WckcfgR = crate::BitReader;
#[doc = "Field `FREQ` reader - FIFO request"]
pub type FreqR = crate::BitReader;
#[doc = "Field `CNRDY` reader - Codec not ready"]
pub type CnrdyR = crate::BitReader;
#[doc = "Field `AFSDET` reader - Anticipated frame synchronization detection"]
pub type AfsdetR = crate::BitReader;
#[doc = "Field `LFSDET` reader - Late frame synchronization detection"]
pub type LfsdetR = crate::BitReader;
#[doc = "Field `FLVL` reader - FIFO level threshold"]
pub type FlvlR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Overrun / underrun"]
    #[inline(always)]
    pub fn ovrudr(&self) -> OvrudrR {
        OvrudrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mute detection"]
    #[inline(always)]
    pub fn mutedet(&self) -> MutedetR {
        MutedetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wrong clock configuration flag"]
    #[inline(always)]
    pub fn wckcfg(&self) -> WckcfgR {
        WckcfgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO request"]
    #[inline(always)]
    pub fn freq(&self) -> FreqR {
        FreqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Codec not ready"]
    #[inline(always)]
    pub fn cnrdy(&self) -> CnrdyR {
        CnrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Anticipated frame synchronization detection"]
    #[inline(always)]
    pub fn afsdet(&self) -> AfsdetR {
        AfsdetR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Late frame synchronization detection"]
    #[inline(always)]
    pub fn lfsdet(&self) -> LfsdetR {
        LfsdetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:18 - FIFO level threshold"]
    #[inline(always)]
    pub fn flvl(&self) -> FlvlR {
        FlvlR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BSR")
            .field("flvl", &self.flvl())
            .field("lfsdet", &self.lfsdet())
            .field("afsdet", &self.afsdet())
            .field("cnrdy", &self.cnrdy())
            .field("freq", &self.freq())
            .field("wckcfg", &self.wckcfg())
            .field("mutedet", &self.mutedet())
            .field("ovrudr", &self.ovrudr())
            .finish()
    }
}
#[doc = "BStatus register\n\nYou can [`read`](crate::Reg::read) this register and get [`bsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BsrSpec;
impl crate::RegisterSpec for BsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsr::R`](R) reader structure"]
impl crate::Readable for BsrSpec {}
#[doc = "`reset()` method sets BSR to value 0"]
impl crate::Resettable for BsrSpec {
    const RESET_VALUE: u32 = 0;
}

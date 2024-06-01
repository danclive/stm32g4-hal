#[doc = "Register `HPMS` reader"]
pub type R = crate::R<HpmsSpec>;
#[doc = "Field `BIDX` reader - BIDX"]
pub type BidxR = crate::FieldReader;
#[doc = "Field `MSI` reader - MSI"]
pub type MsiR = crate::FieldReader;
#[doc = "Field `FIDX` reader - FIDX"]
pub type FidxR = crate::FieldReader;
#[doc = "Field `FLST` reader - FLST"]
pub type FlstR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - BIDX"]
    #[inline(always)]
    pub fn bidx(&self) -> BidxR {
        BidxR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 6:7 - MSI"]
    #[inline(always)]
    pub fn msi(&self) -> MsiR {
        MsiR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - FIDX"]
    #[inline(always)]
    pub fn fidx(&self) -> FidxR {
        FidxR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - FLST"]
    #[inline(always)]
    pub fn flst(&self) -> FlstR {
        FlstR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPMS")
            .field("bidx", &self.bidx())
            .field("msi", &self.msi())
            .field("fidx", &self.fidx())
            .field("flst", &self.flst())
            .finish()
    }
}
#[doc = "This register is updated every time a Message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpms::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpmsSpec;
impl crate::RegisterSpec for HpmsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpms::R`](R) reader structure"]
impl crate::Readable for HpmsSpec {}
#[doc = "`reset()` method sets HPMS to value 0"]
impl crate::Resettable for HpmsSpec {
    const RESET_VALUE: u32 = 0;
}

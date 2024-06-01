#[doc = "Register `TXFQS` reader"]
pub type R = crate::R<TxfqsSpec>;
#[doc = "Field `TFFL` reader - TFFL"]
pub type TfflR = crate::FieldReader;
#[doc = "Field `TFGI` reader - TFGI"]
pub type TfgiR = crate::FieldReader;
#[doc = "Field `TFQPI` reader - TFQPI"]
pub type TfqpiR = crate::FieldReader;
#[doc = "Field `TFQF` reader - TFQF"]
pub type TfqfR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - TFFL"]
    #[inline(always)]
    pub fn tffl(&self) -> TfflR {
        TfflR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - TFGI"]
    #[inline(always)]
    pub fn tfgi(&self) -> TfgiR {
        TfgiR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - TFQPI"]
    #[inline(always)]
    pub fn tfqpi(&self) -> TfqpiR {
        TfqpiR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 21 - TFQF"]
    #[inline(always)]
    pub fn tfqf(&self) -> TfqfR {
        TfqfR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXFQS")
            .field("tffl", &self.tffl())
            .field("tfgi", &self.tfgi())
            .field("tfqpi", &self.tfqpi())
            .field("tfqf", &self.tfqf())
            .finish()
    }
}
#[doc = "The Tx FIFO/Queue status is related to the pending Tx requests listed in register TXBRP. Therefore the effect of Add/Cancellation requests may be delayed due to a running Tx scan (TXBRP not yet updated).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfqs::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxfqsSpec;
impl crate::RegisterSpec for TxfqsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfqs::R`](R) reader structure"]
impl crate::Readable for TxfqsSpec {}
#[doc = "`reset()` method sets TXFQS to value 0x03"]
impl crate::Resettable for TxfqsSpec {
    const RESET_VALUE: u32 = 0x03;
}

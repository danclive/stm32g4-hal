#[doc = "Register `FDCAN_TXFQS` reader"]
pub type R = crate::R<FdcanTxfqsSpec>;
#[doc = "Field `TFFL` reader - Tx FIFO free level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC\\[TFQM\\]
= 1)."]
pub type TfflR = crate::FieldReader;
#[doc = "Field `TFGI` reader - Tx FIFO get index Tx FIFO read index pointer, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC.TFQM = 1)"]
pub type TfgiR = crate::FieldReader;
#[doc = "Field `TFQPI` reader - Tx FIFO/queue put index Tx FIFO/queue write index pointer, range 0 to 3"]
pub type TfqpiR = crate::FieldReader;
#[doc = "Tx FIFO/queue full\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfqf {
    #[doc = "0: Tx FIFO/queue not full"]
    B0x0 = 0,
    #[doc = "1: Tx FIFO/queue full"]
    B0x1 = 1,
}
impl From<Tfqf> for bool {
    #[inline(always)]
    fn from(variant: Tfqf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFQF` reader - Tx FIFO/queue full"]
pub type TfqfR = crate::BitReader<Tfqf>;
impl TfqfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfqf {
        match self.bits {
            false => Tfqf::B0x0,
            true => Tfqf::B0x1,
        }
    }
    #[doc = "Tx FIFO/queue not full"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tfqf::B0x0
    }
    #[doc = "Tx FIFO/queue full"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tfqf::B0x1
    }
}
impl R {
    #[doc = "Bits 0:2 - Tx FIFO free level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC\\[TFQM\\]
= 1)."]
    #[inline(always)]
    pub fn tffl(&self) -> TfflR {
        TfflR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - Tx FIFO get index Tx FIFO read index pointer, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC.TFQM = 1)"]
    #[inline(always)]
    pub fn tfgi(&self) -> TfgiR {
        TfgiR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Tx FIFO/queue put index Tx FIFO/queue write index pointer, range 0 to 3"]
    #[inline(always)]
    pub fn tfqpi(&self) -> TfqpiR {
        TfqpiR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 21 - Tx FIFO/queue full"]
    #[inline(always)]
    pub fn tfqf(&self) -> TfqfR {
        TfqfR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXFQS")
            .field("tffl", &self.tffl())
            .field("tfgi", &self.tfgi())
            .field("tfqpi", &self.tfqpi())
            .field("tfqf", &self.tfqf())
            .finish()
    }
}
#[doc = "FDCAN Tx FIFO/queue status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txfqs::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTxfqsSpec;
impl crate::RegisterSpec for FdcanTxfqsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txfqs::R`](R) reader structure"]
impl crate::Readable for FdcanTxfqsSpec {}
#[doc = "`reset()` method sets FDCAN_TXFQS to value 0x03"]
impl crate::Resettable for FdcanTxfqsSpec {
    const RESET_VALUE: u32 = 0x03;
}

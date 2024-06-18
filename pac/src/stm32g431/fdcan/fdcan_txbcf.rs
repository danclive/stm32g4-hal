#[doc = "Register `FDCAN_TXBCF` reader"]
pub type R = crate::R<FdcanTxbcfSpec>;
#[doc = "Cancellation finished Each Tx buffer has its own CF bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cf {
    #[doc = "0: No transmit buffer cancellation"]
    B0x0 = 0,
    #[doc = "1: Transmit buffer cancellation finished"]
    B0x1 = 1,
}
impl From<Cf> for u8 {
    #[inline(always)]
    fn from(variant: Cf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cf {
    type Ux = u8;
}
impl crate::IsEnum for Cf {}
#[doc = "Field `CF` reader - Cancellation finished Each Tx buffer has its own CF bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
pub type CfR = crate::FieldReader<Cf>;
impl CfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cf> {
        match self.bits {
            0 => Some(Cf::B0x0),
            1 => Some(Cf::B0x1),
            _ => None,
        }
    }
    #[doc = "No transmit buffer cancellation"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cf::B0x0
    }
    #[doc = "Transmit buffer cancellation finished"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cf::B0x1
    }
}
impl R {
    #[doc = "Bits 0:2 - Cancellation finished Each Tx buffer has its own CF bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
    #[inline(always)]
    pub fn cf(&self) -> CfR {
        CfR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBCF")
            .field("cf", &self.cf())
            .finish()
    }
}
#[doc = "FDCAN Tx buffer cancellation finished register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txbcf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTxbcfSpec;
impl crate::RegisterSpec for FdcanTxbcfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbcf::R`](R) reader structure"]
impl crate::Readable for FdcanTxbcfSpec {}
#[doc = "`reset()` method sets FDCAN_TXBCF to value 0"]
impl crate::Resettable for FdcanTxbcfSpec {
    const RESET_VALUE: u32 = 0;
}

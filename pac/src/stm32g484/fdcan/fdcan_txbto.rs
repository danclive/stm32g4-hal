#[doc = "Register `FDCAN_TXBTO` reader"]
pub type R = crate::R<FdcanTxbtoSpec>;
#[doc = "Transmission occurred. Each Tx buffer has its own TO bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum To {
    #[doc = "0: No transmission occurred"]
    B0x0 = 0,
    #[doc = "1: Transmission occurred"]
    B0x1 = 1,
}
impl From<To> for u8 {
    #[inline(always)]
    fn from(variant: To) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for To {
    type Ux = u8;
}
impl crate::IsEnum for To {}
#[doc = "Field `TO` reader - Transmission occurred. Each Tx buffer has its own TO bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
pub type ToR = crate::FieldReader<To>;
impl ToR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<To> {
        match self.bits {
            0 => Some(To::B0x0),
            1 => Some(To::B0x1),
            _ => None,
        }
    }
    #[doc = "No transmission occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == To::B0x0
    }
    #[doc = "Transmission occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == To::B0x1
    }
}
impl R {
    #[doc = "Bits 0:2 - Transmission occurred. Each Tx buffer has its own TO bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
    #[inline(always)]
    pub fn to(&self) -> ToR {
        ToR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBTO")
            .field("to", &self.to())
            .finish()
    }
}
#[doc = "FDCAN Tx buffer transmission occurred register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbto::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTxbtoSpec;
impl crate::RegisterSpec for FdcanTxbtoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbto::R`](R) reader structure"]
impl crate::Readable for FdcanTxbtoSpec {}
#[doc = "`reset()` method sets FDCAN_TXBTO to value 0"]
impl crate::Resettable for FdcanTxbtoSpec {
    const RESET_VALUE: u32 = 0;
}

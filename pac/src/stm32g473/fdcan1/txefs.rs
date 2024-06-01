#[doc = "Register `TXEFS` reader"]
pub type R = crate::R<TxefsSpec>;
#[doc = "Field `EFFL` reader - EFFL"]
pub type EfflR = crate::FieldReader;
#[doc = "Field `EFGI` reader - EFGI"]
pub type EfgiR = crate::FieldReader;
#[doc = "Field `EFPI` reader - EFPI"]
pub type EfpiR = crate::FieldReader;
#[doc = "Field `EFF` reader - EFF"]
pub type EffR = crate::BitReader;
#[doc = "Field `TEFL` reader - TEFL"]
pub type TeflR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - EFFL"]
    #[inline(always)]
    pub fn effl(&self) -> EfflR {
        EfflR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - EFGI"]
    #[inline(always)]
    pub fn efgi(&self) -> EfgiR {
        EfgiR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - EFPI"]
    #[inline(always)]
    pub fn efpi(&self) -> EfpiR {
        EfpiR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - EFF"]
    #[inline(always)]
    pub fn eff(&self) -> EffR {
        EffR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TEFL"]
    #[inline(always)]
    pub fn tefl(&self) -> TeflR {
        TeflR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXEFS")
            .field("effl", &self.effl())
            .field("efgi", &self.efgi())
            .field("efpi", &self.efpi())
            .field("eff", &self.eff())
            .field("tefl", &self.tefl())
            .finish()
    }
}
#[doc = "FDCAN Tx Event FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefs::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxefsSpec;
impl crate::RegisterSpec for TxefsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txefs::R`](R) reader structure"]
impl crate::Readable for TxefsSpec {}
#[doc = "`reset()` method sets TXEFS to value 0"]
impl crate::Resettable for TxefsSpec {
    const RESET_VALUE: u32 = 0;
}

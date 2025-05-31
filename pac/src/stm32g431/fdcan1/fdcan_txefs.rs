#[doc = "Register `FDCAN_TXEFS` reader"]
pub type R = crate::R<FdcanTxefsSpec>;
#[doc = "Field `EFFL` reader - Event FIFO fill level Number of elements stored in Tx event FIFO, range 0 to 3."]
pub type EfflR = crate::FieldReader;
#[doc = "Field `EFGI` reader - Event FIFO get index Tx event FIFO read index pointer, range 0 to 3."]
pub type EfgiR = crate::FieldReader;
#[doc = "Field `EFPI` reader - Event FIFO put index Tx event FIFO write index pointer, range 0 to 3."]
pub type EfpiR = crate::FieldReader;
#[doc = "Event FIFO full\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eff {
    #[doc = "0: Tx event FIFO not full"]
    B0x0 = 0,
    #[doc = "1: Tx event FIFO full"]
    B0x1 = 1,
}
impl From<Eff> for bool {
    #[inline(always)]
    fn from(variant: Eff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EFF` reader - Event FIFO full"]
pub type EffR = crate::BitReader<Eff>;
impl EffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eff {
        match self.bits {
            false => Eff::B0x0,
            true => Eff::B0x1,
        }
    }
    #[doc = "Tx event FIFO not full"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eff::B0x0
    }
    #[doc = "Tx event FIFO full"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eff::B0x1
    }
}
#[doc = "Field `TEFL` reader - Tx event FIFO element lost This bit is a copy of interrupt flag IR\\[TEFL\\]. When IR\\[TEFL\\] is reset, this bit is also reset. 0 No Tx event FIFO element lost 1 Tx event FIFO element lost, also set after write attempt to Tx event FIFO of size 0."]
pub type TeflR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Event FIFO fill level Number of elements stored in Tx event FIFO, range 0 to 3."]
    #[inline(always)]
    pub fn effl(&self) -> EfflR {
        EfflR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - Event FIFO get index Tx event FIFO read index pointer, range 0 to 3."]
    #[inline(always)]
    pub fn efgi(&self) -> EfgiR {
        EfgiR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Event FIFO put index Tx event FIFO write index pointer, range 0 to 3."]
    #[inline(always)]
    pub fn efpi(&self) -> EfpiR {
        EfpiR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Event FIFO full"]
    #[inline(always)]
    pub fn eff(&self) -> EffR {
        EffR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Tx event FIFO element lost This bit is a copy of interrupt flag IR\\[TEFL\\]. When IR\\[TEFL\\] is reset, this bit is also reset. 0 No Tx event FIFO element lost 1 Tx event FIFO element lost, also set after write attempt to Tx event FIFO of size 0."]
    #[inline(always)]
    pub fn tefl(&self) -> TeflR {
        TeflR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXEFS")
            .field("effl", &self.effl())
            .field("efgi", &self.efgi())
            .field("efpi", &self.efpi())
            .field("eff", &self.eff())
            .field("tefl", &self.tefl())
            .finish()
    }
}
#[doc = "FDCAN Tx event FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txefs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTxefsSpec;
impl crate::RegisterSpec for FdcanTxefsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txefs::R`](R) reader structure"]
impl crate::Readable for FdcanTxefsSpec {}
#[doc = "`reset()` method sets FDCAN_TXEFS to value 0"]
impl crate::Resettable for FdcanTxefsSpec {}

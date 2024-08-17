#[doc = "Register `FDCAN_RXF1S` reader"]
pub type R = crate::R<FdcanRxf1sSpec>;
#[doc = "Field `F1FL` reader - Rx FIFO 1 fill level Number of elements stored in Rx FIFO 1, range 0 to 3."]
pub type F1flR = crate::FieldReader;
#[doc = "Field `F1GI` reader - Rx FIFO 1 get index Rx FIFO 1 read index pointer, range 0 to 2."]
pub type F1giR = crate::FieldReader;
#[doc = "Field `F1PI` reader - Rx FIFO 1 put index Rx FIFO 1 write index pointer, range 0 to 2."]
pub type F1piR = crate::FieldReader;
#[doc = "Rx FIFO 1 full\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F1f {
    #[doc = "0: Rx FIFO 1 not full"]
    B0x0 = 0,
    #[doc = "1: Rx FIFO 1 full"]
    B0x1 = 1,
}
impl From<F1f> for bool {
    #[inline(always)]
    fn from(variant: F1f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F1F` reader - Rx FIFO 1 full"]
pub type F1fR = crate::BitReader<F1f>;
impl F1fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> F1f {
        match self.bits {
            false => F1f::B0x0,
            true => F1f::B0x1,
        }
    }
    #[doc = "Rx FIFO 1 not full"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == F1f::B0x0
    }
    #[doc = "Rx FIFO 1 full"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == F1f::B0x1
    }
}
#[doc = "Rx FIFO 1 message lost This bit is a copy of interrupt flag IR\\[RF1L\\]. When IR\\[RF1L\\]
is reset, this bit is also reset.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf1l {
    #[doc = "0: No Rx FIFO 1 message lost"]
    B0x0 = 0,
    #[doc = "1: Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size 0"]
    B0x1 = 1,
}
impl From<Rf1l> for bool {
    #[inline(always)]
    fn from(variant: Rf1l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF1L` reader - Rx FIFO 1 message lost This bit is a copy of interrupt flag IR\\[RF1L\\]. When IR\\[RF1L\\]
is reset, this bit is also reset."]
pub type Rf1lR = crate::BitReader<Rf1l>;
impl Rf1lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rf1l {
        match self.bits {
            false => Rf1l::B0x0,
            true => Rf1l::B0x1,
        }
    }
    #[doc = "No Rx FIFO 1 message lost"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rf1l::B0x0
    }
    #[doc = "Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size 0"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rf1l::B0x1
    }
}
impl R {
    #[doc = "Bits 0:3 - Rx FIFO 1 fill level Number of elements stored in Rx FIFO 1, range 0 to 3."]
    #[inline(always)]
    pub fn f1fl(&self) -> F1flR {
        F1flR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Rx FIFO 1 get index Rx FIFO 1 read index pointer, range 0 to 2."]
    #[inline(always)]
    pub fn f1gi(&self) -> F1giR {
        F1giR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Rx FIFO 1 put index Rx FIFO 1 write index pointer, range 0 to 2."]
    #[inline(always)]
    pub fn f1pi(&self) -> F1piR {
        F1piR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 1 full"]
    #[inline(always)]
    pub fn f1f(&self) -> F1fR {
        F1fR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 1 message lost This bit is a copy of interrupt flag IR\\[RF1L\\]. When IR\\[RF1L\\]
is reset, this bit is also reset."]
    #[inline(always)]
    pub fn rf1l(&self) -> Rf1lR {
        Rf1lR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXF1S")
            .field("f1fl", &self.f1fl())
            .field("f1gi", &self.f1gi())
            .field("f1pi", &self.f1pi())
            .field("f1f", &self.f1f())
            .field("rf1l", &self.rf1l())
            .finish()
    }
}
#[doc = "FDCAN Rx FIFO 1 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rxf1s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanRxf1sSpec;
impl crate::RegisterSpec for FdcanRxf1sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rxf1s::R`](R) reader structure"]
impl crate::Readable for FdcanRxf1sSpec {}
#[doc = "`reset()` method sets FDCAN_RXF1S to value 0"]
impl crate::Resettable for FdcanRxf1sSpec {
    const RESET_VALUE: u32 = 0;
}

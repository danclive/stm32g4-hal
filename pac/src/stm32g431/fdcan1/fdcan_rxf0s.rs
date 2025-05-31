#[doc = "Register `FDCAN_RXF0S` reader"]
pub type R = crate::R<FdcanRxf0sSpec>;
#[doc = "Field `F0FL` reader - Rx FIFO 0 fill level Number of elements stored in Rx FIFO 0, range 0 to 3."]
pub type F0flR = crate::FieldReader;
#[doc = "Field `F0GI` reader - Rx FIFO 0 get index Rx FIFO 0 read index pointer, range 0 to 2."]
pub type F0giR = crate::FieldReader;
#[doc = "Field `F0PI` reader - Rx FIFO 0 put index Rx FIFO 0 write index pointer, range 0 to 2."]
pub type F0piR = crate::FieldReader;
#[doc = "Rx FIFO 0 full\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F0f {
    #[doc = "0: Rx FIFO 0 not full"]
    B0x0 = 0,
    #[doc = "1: Rx FIFO 0 full"]
    B0x1 = 1,
}
impl From<F0f> for bool {
    #[inline(always)]
    fn from(variant: F0f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F0F` reader - Rx FIFO 0 full"]
pub type F0fR = crate::BitReader<F0f>;
impl F0fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> F0f {
        match self.bits {
            false => F0f::B0x0,
            true => F0f::B0x1,
        }
    }
    #[doc = "Rx FIFO 0 not full"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == F0f::B0x0
    }
    #[doc = "Rx FIFO 0 full"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == F0f::B0x1
    }
}
#[doc = "Rx FIFO 0 message lost This bit is a copy of interrupt flag IR\\[RF0L\\]. When IR\\[RF0L\\] is reset, this bit is also reset.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf0l {
    #[doc = "0: No Rx FIFO 0 message lost"]
    B0x0 = 0,
    #[doc = "1: Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size 0"]
    B0x1 = 1,
}
impl From<Rf0l> for bool {
    #[inline(always)]
    fn from(variant: Rf0l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF0L` reader - Rx FIFO 0 message lost This bit is a copy of interrupt flag IR\\[RF0L\\]. When IR\\[RF0L\\] is reset, this bit is also reset."]
pub type Rf0lR = crate::BitReader<Rf0l>;
impl Rf0lR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rf0l {
        match self.bits {
            false => Rf0l::B0x0,
            true => Rf0l::B0x1,
        }
    }
    #[doc = "No Rx FIFO 0 message lost"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rf0l::B0x0
    }
    #[doc = "Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size 0"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rf0l::B0x1
    }
}
impl R {
    #[doc = "Bits 0:3 - Rx FIFO 0 fill level Number of elements stored in Rx FIFO 0, range 0 to 3."]
    #[inline(always)]
    pub fn f0fl(&self) -> F0flR {
        F0flR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Rx FIFO 0 get index Rx FIFO 0 read index pointer, range 0 to 2."]
    #[inline(always)]
    pub fn f0gi(&self) -> F0giR {
        F0giR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Rx FIFO 0 put index Rx FIFO 0 write index pointer, range 0 to 2."]
    #[inline(always)]
    pub fn f0pi(&self) -> F0piR {
        F0piR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 0 full"]
    #[inline(always)]
    pub fn f0f(&self) -> F0fR {
        F0fR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 0 message lost This bit is a copy of interrupt flag IR\\[RF0L\\]. When IR\\[RF0L\\] is reset, this bit is also reset."]
    #[inline(always)]
    pub fn rf0l(&self) -> Rf0lR {
        Rf0lR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXF0S")
            .field("f0fl", &self.f0fl())
            .field("f0gi", &self.f0gi())
            .field("f0pi", &self.f0pi())
            .field("f0f", &self.f0f())
            .field("rf0l", &self.rf0l())
            .finish()
    }
}
#[doc = "FDCAN Rx FIFO 0 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rxf0s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanRxf0sSpec;
impl crate::RegisterSpec for FdcanRxf0sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rxf0s::R`](R) reader structure"]
impl crate::Readable for FdcanRxf0sSpec {}
#[doc = "`reset()` method sets FDCAN_RXF0S to value 0"]
impl crate::Resettable for FdcanRxf0sSpec {}

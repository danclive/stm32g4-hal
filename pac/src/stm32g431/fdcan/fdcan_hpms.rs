#[doc = "Register `FDCAN_HPMS` reader"]
pub type R = crate::R<FdcanHpmsSpec>;
#[doc = "Field `BIDX` reader - Buffer index Index of Rx FIFO element to which the message was stored. Only valid when MSI\\[1\\]
= 1."]
pub type BidxR = crate::FieldReader;
#[doc = "Message storage indicator\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Msi {
    #[doc = "0: No FIFO selected"]
    B0x0 = 0,
    #[doc = "1: FIFO overrun"]
    B0x1 = 1,
    #[doc = "2: Message stored in FIFO 0"]
    B0x2 = 2,
    #[doc = "3: Message stored in FIFO 1"]
    B0x3 = 3,
}
impl From<Msi> for u8 {
    #[inline(always)]
    fn from(variant: Msi) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Msi {
    type Ux = u8;
}
impl crate::IsEnum for Msi {}
#[doc = "Field `MSI` reader - Message storage indicator"]
pub type MsiR = crate::FieldReader<Msi>;
impl MsiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msi {
        match self.bits {
            0 => Msi::B0x0,
            1 => Msi::B0x1,
            2 => Msi::B0x2,
            3 => Msi::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "No FIFO selected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Msi::B0x0
    }
    #[doc = "FIFO overrun"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Msi::B0x1
    }
    #[doc = "Message stored in FIFO 0"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Msi::B0x2
    }
    #[doc = "Message stored in FIFO 1"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Msi::B0x3
    }
}
#[doc = "Field `FIDX` reader - Filter index Index of matching filter element. Range is 0 to RXGFC\\[LSS\\]
- 1 or RXGFC\\[LSE\\]
- 1."]
pub type FidxR = crate::FieldReader;
#[doc = "Filter list Indicates the filter list of the matching filter element.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flst {
    #[doc = "0: Standard filter list"]
    B0x0 = 0,
    #[doc = "1: Extended filter list"]
    B0x1 = 1,
}
impl From<Flst> for bool {
    #[inline(always)]
    fn from(variant: Flst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLST` reader - Filter list Indicates the filter list of the matching filter element."]
pub type FlstR = crate::BitReader<Flst>;
impl FlstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flst {
        match self.bits {
            false => Flst::B0x0,
            true => Flst::B0x1,
        }
    }
    #[doc = "Standard filter list"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Flst::B0x0
    }
    #[doc = "Extended filter list"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Flst::B0x1
    }
}
impl R {
    #[doc = "Bits 0:2 - Buffer index Index of Rx FIFO element to which the message was stored. Only valid when MSI\\[1\\]
= 1."]
    #[inline(always)]
    pub fn bidx(&self) -> BidxR {
        BidxR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 6:7 - Message storage indicator"]
    #[inline(always)]
    pub fn msi(&self) -> MsiR {
        MsiR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Filter index Index of matching filter element. Range is 0 to RXGFC\\[LSS\\]
- 1 or RXGFC\\[LSE\\]
- 1."]
    #[inline(always)]
    pub fn fidx(&self) -> FidxR {
        FidxR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Filter list Indicates the filter list of the matching filter element."]
    #[inline(always)]
    pub fn flst(&self) -> FlstR {
        FlstR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_HPMS")
            .field("bidx", &self.bidx())
            .field("msi", &self.msi())
            .field("fidx", &self.fidx())
            .field("flst", &self.flst())
            .finish()
    }
}
#[doc = "FDCAN high-priority message status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_hpms::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanHpmsSpec;
impl crate::RegisterSpec for FdcanHpmsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_hpms::R`](R) reader structure"]
impl crate::Readable for FdcanHpmsSpec {}
#[doc = "`reset()` method sets FDCAN_HPMS to value 0"]
impl crate::Resettable for FdcanHpmsSpec {
    const RESET_VALUE: u32 = 0;
}

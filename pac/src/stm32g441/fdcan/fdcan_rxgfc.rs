#[doc = "Register `FDCAN_RXGFC` reader"]
pub type R = crate::R<FdcanRxgfcSpec>;
#[doc = "Register `FDCAN_RXGFC` writer"]
pub type W = crate::W<FdcanRxgfcSpec>;
#[doc = "Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rrfe {
    #[doc = "0: Filter remote frames with 29-bit standard IDs"]
    B0x0 = 0,
    #[doc = "1: Reject all remote frames with 29-bit standard IDs"]
    B0x1 = 1,
}
impl From<Rrfe> for bool {
    #[inline(always)]
    fn from(variant: Rrfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRFE` reader - Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type RrfeR = crate::BitReader<Rrfe>;
impl RrfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rrfe {
        match self.bits {
            false => Rrfe::B0x0,
            true => Rrfe::B0x1,
        }
    }
    #[doc = "Filter remote frames with 29-bit standard IDs"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rrfe::B0x0
    }
    #[doc = "Reject all remote frames with 29-bit standard IDs"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rrfe::B0x1
    }
}
#[doc = "Field `RRFE` writer - Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type RrfeW<'a, REG> = crate::BitWriter<'a, REG, Rrfe>;
impl<'a, REG> RrfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter remote frames with 29-bit standard IDs"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rrfe::B0x0)
    }
    #[doc = "Reject all remote frames with 29-bit standard IDs"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rrfe::B0x1)
    }
}
#[doc = "Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rrfs {
    #[doc = "0: Filter remote frames with 11-bit standard IDs"]
    B0x0 = 0,
    #[doc = "1: Reject all remote frames with 11-bit standard IDs"]
    B0x1 = 1,
}
impl From<Rrfs> for bool {
    #[inline(always)]
    fn from(variant: Rrfs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRFS` reader - Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type RrfsR = crate::BitReader<Rrfs>;
impl RrfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rrfs {
        match self.bits {
            false => Rrfs::B0x0,
            true => Rrfs::B0x1,
        }
    }
    #[doc = "Filter remote frames with 11-bit standard IDs"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rrfs::B0x0
    }
    #[doc = "Reject all remote frames with 11-bit standard IDs"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rrfs::B0x1
    }
}
#[doc = "Field `RRFS` writer - Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type RrfsW<'a, REG> = crate::BitWriter<'a, REG, Rrfs>;
impl<'a, REG> RrfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter remote frames with 11-bit standard IDs"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rrfs::B0x0)
    }
    #[doc = "Reject all remote frames with 11-bit standard IDs"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rrfs::B0x1)
    }
}
#[doc = "Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Anfe {
    #[doc = "0: Accept in Rx FIFO 0"]
    B0x0 = 0,
    #[doc = "1: Accept in Rx FIFO 1"]
    B0x1 = 1,
    #[doc = "2: Reject"]
    B0x2 = 2,
    #[doc = "3: Reject"]
    B0x3 = 3,
}
impl From<Anfe> for u8 {
    #[inline(always)]
    fn from(variant: Anfe) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Anfe {
    type Ux = u8;
}
impl crate::IsEnum for Anfe {}
#[doc = "Field `ANFE` reader - Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type AnfeR = crate::FieldReader<Anfe>;
impl AnfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Anfe {
        match self.bits {
            0 => Anfe::B0x0,
            1 => Anfe::B0x1,
            2 => Anfe::B0x2,
            3 => Anfe::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Anfe::B0x0
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Anfe::B0x1
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Anfe::B0x2
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Anfe::B0x3
    }
}
#[doc = "Field `ANFE` writer - Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type AnfeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Anfe, crate::Safe>;
impl<'a, REG> AnfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Anfe::B0x0)
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Anfe::B0x1)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Anfe::B0x2)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Anfe::B0x3)
    }
}
#[doc = "Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Anfs {
    #[doc = "0: Accept in Rx FIFO 0"]
    B0x0 = 0,
    #[doc = "1: Accept in Rx FIFO 1"]
    B0x1 = 1,
    #[doc = "2: Reject"]
    B0x2 = 2,
    #[doc = "3: Reject"]
    B0x3 = 3,
}
impl From<Anfs> for u8 {
    #[inline(always)]
    fn from(variant: Anfs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Anfs {
    type Ux = u8;
}
impl crate::IsEnum for Anfs {}
#[doc = "Field `ANFS` reader - Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type AnfsR = crate::FieldReader<Anfs>;
impl AnfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Anfs {
        match self.bits {
            0 => Anfs::B0x0,
            1 => Anfs::B0x1,
            2 => Anfs::B0x2,
            3 => Anfs::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Anfs::B0x0
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Anfs::B0x1
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Anfs::B0x2
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Anfs::B0x3
    }
}
#[doc = "Field `ANFS` writer - Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type AnfsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Anfs, crate::Safe>;
impl<'a, REG> AnfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Anfs::B0x0)
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Anfs::B0x1)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Anfs::B0x2)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Anfs::B0x3)
    }
}
#[doc = "Field `F1OM` reader - FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type F1omR = crate::BitReader;
#[doc = "Field `F1OM` writer - FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type F1omW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F0OM` reader - FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type F0omR = crate::BitReader;
#[doc = "Field `F0OM` writer - FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type F0omW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "List size standard 1 to 28: Number of standard message ID filter elements >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lss {
    #[doc = "0: No standard message ID filter"]
    B0x0 = 0,
}
impl From<Lss> for u8 {
    #[inline(always)]
    fn from(variant: Lss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lss {
    type Ux = u8;
}
impl crate::IsEnum for Lss {}
#[doc = "Field `LSS` reader - List size standard 1 to 28: Number of standard message ID filter elements >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type LssR = crate::FieldReader<Lss>;
impl LssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lss> {
        match self.bits {
            0 => Some(Lss::B0x0),
            _ => None,
        }
    }
    #[doc = "No standard message ID filter"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lss::B0x0
    }
}
#[doc = "Field `LSS` writer - List size standard 1 to 28: Number of standard message ID filter elements >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type LssW<'a, REG> = crate::FieldWriter<'a, REG, 5, Lss>;
impl<'a, REG> LssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No standard message ID filter"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lss::B0x0)
    }
}
#[doc = "List size extended 1 to 8: Number of extended message ID filter elements >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lse {
    #[doc = "0: No extended message ID filter"]
    B0x0 = 0,
}
impl From<Lse> for u8 {
    #[inline(always)]
    fn from(variant: Lse) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lse {
    type Ux = u8;
}
impl crate::IsEnum for Lse {}
#[doc = "Field `LSE` reader - List size extended 1 to 8: Number of extended message ID filter elements >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type LseR = crate::FieldReader<Lse>;
impl LseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lse> {
        match self.bits {
            0 => Some(Lse::B0x0),
            _ => None,
        }
    }
    #[doc = "No extended message ID filter"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lse::B0x0
    }
}
#[doc = "Field `LSE` writer - List size extended 1 to 8: Number of extended message ID filter elements >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type LseW<'a, REG> = crate::FieldWriter<'a, REG, 4, Lse>;
impl<'a, REG> LseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No extended message ID filter"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lse::B0x0)
    }
}
impl R {
    #[doc = "Bit 0 - Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn rrfe(&self) -> RrfeR {
        RrfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn rrfs(&self) -> RrfsR {
        RrfsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn anfe(&self) -> AnfeR {
        AnfeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn anfs(&self) -> AnfsR {
        AnfsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn f1om(&self) -> F1omR {
        F1omR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn f0om(&self) -> F0omR {
        F0omR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:20 - List size standard 1 to 28: Number of standard message ID filter elements >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn lss(&self) -> LssR {
        LssR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - List size extended 1 to 8: Number of extended message ID filter elements >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn lse(&self) -> LseR {
        LseR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXGFC")
            .field("rrfe", &self.rrfe())
            .field("rrfs", &self.rrfs())
            .field("anfe", &self.anfe())
            .field("anfs", &self.anfs())
            .field("f1om", &self.f1om())
            .field("f0om", &self.f0om())
            .field("lss", &self.lss())
            .field("lse", &self.lse())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn rrfe(&mut self) -> RrfeW<FdcanRxgfcSpec> {
        RrfeW::new(self, 0)
    }
    #[doc = "Bit 1 - Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn rrfs(&mut self) -> RrfsW<FdcanRxgfcSpec> {
        RrfsW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn anfe(&mut self) -> AnfeW<FdcanRxgfcSpec> {
        AnfeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn anfs(&mut self) -> AnfsW<FdcanRxgfcSpec> {
        AnfsW::new(self, 4)
    }
    #[doc = "Bit 8 - FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn f1om(&mut self) -> F1omW<FdcanRxgfcSpec> {
        F1omW::new(self, 8)
    }
    #[doc = "Bit 9 - FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn f0om(&mut self) -> F0omW<FdcanRxgfcSpec> {
        F0omW::new(self, 9)
    }
    #[doc = "Bits 16:20 - List size standard 1 to 28: Number of standard message ID filter elements >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn lss(&mut self) -> LssW<FdcanRxgfcSpec> {
        LssW::new(self, 16)
    }
    #[doc = "Bits 24:27 - List size extended 1 to 8: Number of extended message ID filter elements >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn lse(&mut self) -> LseW<FdcanRxgfcSpec> {
        LseW::new(self, 24)
    }
}
#[doc = "FDCAN global filter configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxgfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxgfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanRxgfcSpec;
impl crate::RegisterSpec for FdcanRxgfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rxgfc::R`](R) reader structure"]
impl crate::Readable for FdcanRxgfcSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_rxgfc::W`](W) writer structure"]
impl crate::Writable for FdcanRxgfcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_RXGFC to value 0"]
impl crate::Resettable for FdcanRxgfcSpec {
    const RESET_VALUE: u32 = 0;
}

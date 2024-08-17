#[doc = "Register `FDCAN_IR` reader"]
pub type R = crate::R<FdcanIrSpec>;
#[doc = "Register `FDCAN_IR` writer"]
pub type W = crate::W<FdcanIrSpec>;
#[doc = "Rx FIFO 0 new message\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf0n {
    #[doc = "0: No new message written to Rx FIFO 0"]
    B0x0 = 0,
    #[doc = "1: New message written to Rx FIFO 0"]
    B0x1 = 1,
}
impl From<Rf0n> for bool {
    #[inline(always)]
    fn from(variant: Rf0n) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF0N` reader - Rx FIFO 0 new message"]
pub type Rf0nR = crate::BitReader<Rf0n>;
impl Rf0nR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rf0n {
        match self.bits {
            false => Rf0n::B0x0,
            true => Rf0n::B0x1,
        }
    }
    #[doc = "No new message written to Rx FIFO 0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rf0n::B0x0
    }
    #[doc = "New message written to Rx FIFO 0"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rf0n::B0x1
    }
}
#[doc = "Field `RF0N` writer - Rx FIFO 0 new message"]
pub type Rf0nW<'a, REG> = crate::BitWriter<'a, REG, Rf0n>;
impl<'a, REG> Rf0nW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No new message written to Rx FIFO 0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rf0n::B0x0)
    }
    #[doc = "New message written to Rx FIFO 0"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rf0n::B0x1)
    }
}
#[doc = "Rx FIFO 0 full\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf0f {
    #[doc = "0: Rx FIFO 0 not full"]
    B0x0 = 0,
    #[doc = "1: Rx FIFO 0 full"]
    B0x1 = 1,
}
impl From<Rf0f> for bool {
    #[inline(always)]
    fn from(variant: Rf0f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF0F` reader - Rx FIFO 0 full"]
pub type Rf0fR = crate::BitReader<Rf0f>;
impl Rf0fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rf0f {
        match self.bits {
            false => Rf0f::B0x0,
            true => Rf0f::B0x1,
        }
    }
    #[doc = "Rx FIFO 0 not full"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rf0f::B0x0
    }
    #[doc = "Rx FIFO 0 full"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rf0f::B0x1
    }
}
#[doc = "Field `RF0F` writer - Rx FIFO 0 full"]
pub type Rf0fW<'a, REG> = crate::BitWriter<'a, REG, Rf0f>;
impl<'a, REG> Rf0fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rx FIFO 0 not full"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rf0f::B0x0)
    }
    #[doc = "Rx FIFO 0 full"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rf0f::B0x1)
    }
}
#[doc = "Rx FIFO 0 message lost\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf0l {
    #[doc = "0: No Rx FIFO 0 message lost"]
    B0x0 = 0,
    #[doc = "1: Rx FIFO 0 message lost"]
    B0x1 = 1,
}
impl From<Rf0l> for bool {
    #[inline(always)]
    fn from(variant: Rf0l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF0L` reader - Rx FIFO 0 message lost"]
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
    #[doc = "Rx FIFO 0 message lost"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rf0l::B0x1
    }
}
#[doc = "Field `RF0L` writer - Rx FIFO 0 message lost"]
pub type Rf0lW<'a, REG> = crate::BitWriter<'a, REG, Rf0l>;
impl<'a, REG> Rf0lW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Rx FIFO 0 message lost"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rf0l::B0x0)
    }
    #[doc = "Rx FIFO 0 message lost"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rf0l::B0x1)
    }
}
#[doc = "Rx FIFO 1 new message\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf1n {
    #[doc = "0: No new message written to Rx FIFO 1"]
    B0x0 = 0,
    #[doc = "1: New message written to Rx FIFO 1"]
    B0x1 = 1,
}
impl From<Rf1n> for bool {
    #[inline(always)]
    fn from(variant: Rf1n) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF1N` reader - Rx FIFO 1 new message"]
pub type Rf1nR = crate::BitReader<Rf1n>;
impl Rf1nR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rf1n {
        match self.bits {
            false => Rf1n::B0x0,
            true => Rf1n::B0x1,
        }
    }
    #[doc = "No new message written to Rx FIFO 1"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rf1n::B0x0
    }
    #[doc = "New message written to Rx FIFO 1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rf1n::B0x1
    }
}
#[doc = "Field `RF1N` writer - Rx FIFO 1 new message"]
pub type Rf1nW<'a, REG> = crate::BitWriter<'a, REG, Rf1n>;
impl<'a, REG> Rf1nW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No new message written to Rx FIFO 1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rf1n::B0x0)
    }
    #[doc = "New message written to Rx FIFO 1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rf1n::B0x1)
    }
}
#[doc = "Rx FIFO 1 full\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf1f {
    #[doc = "0: Rx FIFO 1 not full"]
    B0x0 = 0,
    #[doc = "1: Rx FIFO 1 full"]
    B0x1 = 1,
}
impl From<Rf1f> for bool {
    #[inline(always)]
    fn from(variant: Rf1f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF1F` reader - Rx FIFO 1 full"]
pub type Rf1fR = crate::BitReader<Rf1f>;
impl Rf1fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rf1f {
        match self.bits {
            false => Rf1f::B0x0,
            true => Rf1f::B0x1,
        }
    }
    #[doc = "Rx FIFO 1 not full"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rf1f::B0x0
    }
    #[doc = "Rx FIFO 1 full"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rf1f::B0x1
    }
}
#[doc = "Field `RF1F` writer - Rx FIFO 1 full"]
pub type Rf1fW<'a, REG> = crate::BitWriter<'a, REG, Rf1f>;
impl<'a, REG> Rf1fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rx FIFO 1 not full"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rf1f::B0x0)
    }
    #[doc = "Rx FIFO 1 full"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rf1f::B0x1)
    }
}
#[doc = "Rx FIFO 1 message lost\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf1l {
    #[doc = "0: No Rx FIFO 1 message lost"]
    B0x0 = 0,
    #[doc = "1: Rx FIFO 1 message lost"]
    B0x1 = 1,
}
impl From<Rf1l> for bool {
    #[inline(always)]
    fn from(variant: Rf1l) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF1L` reader - Rx FIFO 1 message lost"]
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
    #[doc = "Rx FIFO 1 message lost"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rf1l::B0x1
    }
}
#[doc = "Field `RF1L` writer - Rx FIFO 1 message lost"]
pub type Rf1lW<'a, REG> = crate::BitWriter<'a, REG, Rf1l>;
impl<'a, REG> Rf1lW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Rx FIFO 1 message lost"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rf1l::B0x0)
    }
    #[doc = "Rx FIFO 1 message lost"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rf1l::B0x1)
    }
}
#[doc = "High-priority message\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hpm {
    #[doc = "0: No high-priority message received"]
    B0x0 = 0,
    #[doc = "1: High-priority message received"]
    B0x1 = 1,
}
impl From<Hpm> for bool {
    #[inline(always)]
    fn from(variant: Hpm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HPM` reader - High-priority message"]
pub type HpmR = crate::BitReader<Hpm>;
impl HpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hpm {
        match self.bits {
            false => Hpm::B0x0,
            true => Hpm::B0x1,
        }
    }
    #[doc = "No high-priority message received"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hpm::B0x0
    }
    #[doc = "High-priority message received"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hpm::B0x1
    }
}
#[doc = "Field `HPM` writer - High-priority message"]
pub type HpmW<'a, REG> = crate::BitWriter<'a, REG, Hpm>;
impl<'a, REG> HpmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No high-priority message received"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hpm::B0x0)
    }
    #[doc = "High-priority message received"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hpm::B0x1)
    }
}
#[doc = "Transmission completed\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc {
    #[doc = "0: No transmission completed"]
    B0x0 = 0,
    #[doc = "1: Transmission completed"]
    B0x1 = 1,
}
impl From<Tc> for bool {
    #[inline(always)]
    fn from(variant: Tc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC` reader - Transmission completed"]
pub type TcR = crate::BitReader<Tc>;
impl TcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc {
        match self.bits {
            false => Tc::B0x0,
            true => Tc::B0x1,
        }
    }
    #[doc = "No transmission completed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tc::B0x0
    }
    #[doc = "Transmission completed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tc::B0x1
    }
}
#[doc = "Field `TC` writer - Transmission completed"]
pub type TcW<'a, REG> = crate::BitWriter<'a, REG, Tc>;
impl<'a, REG> TcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No transmission completed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tc::B0x0)
    }
    #[doc = "Transmission completed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tc::B0x1)
    }
}
#[doc = "Transmission cancellation finished\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcf {
    #[doc = "0: No transmission cancellation finished"]
    B0x0 = 0,
    #[doc = "1: Transmission cancellation finished"]
    B0x1 = 1,
}
impl From<Tcf> for bool {
    #[inline(always)]
    fn from(variant: Tcf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCF` reader - Transmission cancellation finished"]
pub type TcfR = crate::BitReader<Tcf>;
impl TcfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcf {
        match self.bits {
            false => Tcf::B0x0,
            true => Tcf::B0x1,
        }
    }
    #[doc = "No transmission cancellation finished"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tcf::B0x0
    }
    #[doc = "Transmission cancellation finished"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tcf::B0x1
    }
}
#[doc = "Field `TCF` writer - Transmission cancellation finished"]
pub type TcfW<'a, REG> = crate::BitWriter<'a, REG, Tcf>;
impl<'a, REG> TcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No transmission cancellation finished"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcf::B0x0)
    }
    #[doc = "Transmission cancellation finished"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcf::B0x1)
    }
}
#[doc = "Tx FIFO empty\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfe {
    #[doc = "0: Tx FIFO non-empty"]
    B0x0 = 0,
    #[doc = "1: Tx FIFO empty"]
    B0x1 = 1,
}
impl From<Tfe> for bool {
    #[inline(always)]
    fn from(variant: Tfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFE` reader - Tx FIFO empty"]
pub type TfeR = crate::BitReader<Tfe>;
impl TfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfe {
        match self.bits {
            false => Tfe::B0x0,
            true => Tfe::B0x1,
        }
    }
    #[doc = "Tx FIFO non-empty"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tfe::B0x0
    }
    #[doc = "Tx FIFO empty"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tfe::B0x1
    }
}
#[doc = "Field `TFE` writer - Tx FIFO empty"]
pub type TfeW<'a, REG> = crate::BitWriter<'a, REG, Tfe>;
impl<'a, REG> TfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx FIFO non-empty"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tfe::B0x0)
    }
    #[doc = "Tx FIFO empty"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tfe::B0x1)
    }
}
#[doc = "Tx event FIFO New Entry\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tefn {
    #[doc = "0: Tx event FIFO unchanged"]
    B0x0 = 0,
    #[doc = "1: Tx handler wrote Tx event FIFO element."]
    B0x1 = 1,
}
impl From<Tefn> for bool {
    #[inline(always)]
    fn from(variant: Tefn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEFN` reader - Tx event FIFO New Entry"]
pub type TefnR = crate::BitReader<Tefn>;
impl TefnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tefn {
        match self.bits {
            false => Tefn::B0x0,
            true => Tefn::B0x1,
        }
    }
    #[doc = "Tx event FIFO unchanged"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tefn::B0x0
    }
    #[doc = "Tx handler wrote Tx event FIFO element."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tefn::B0x1
    }
}
#[doc = "Field `TEFN` writer - Tx event FIFO New Entry"]
pub type TefnW<'a, REG> = crate::BitWriter<'a, REG, Tefn>;
impl<'a, REG> TefnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx event FIFO unchanged"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tefn::B0x0)
    }
    #[doc = "Tx handler wrote Tx event FIFO element."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tefn::B0x1)
    }
}
#[doc = "Tx event FIFO full\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Teff {
    #[doc = "0: Tx event FIFO Not full"]
    B0x0 = 0,
    #[doc = "1: Tx event FIFO full"]
    B0x1 = 1,
}
impl From<Teff> for bool {
    #[inline(always)]
    fn from(variant: Teff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEFF` reader - Tx event FIFO full"]
pub type TeffR = crate::BitReader<Teff>;
impl TeffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Teff {
        match self.bits {
            false => Teff::B0x0,
            true => Teff::B0x1,
        }
    }
    #[doc = "Tx event FIFO Not full"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Teff::B0x0
    }
    #[doc = "Tx event FIFO full"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Teff::B0x1
    }
}
#[doc = "Field `TEFF` writer - Tx event FIFO full"]
pub type TeffW<'a, REG> = crate::BitWriter<'a, REG, Teff>;
impl<'a, REG> TeffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx event FIFO Not full"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Teff::B0x0)
    }
    #[doc = "Tx event FIFO full"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Teff::B0x1)
    }
}
#[doc = "Tx event FIFO element lost\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tefl {
    #[doc = "0: No Tx event FIFO element lost"]
    B0x0 = 0,
    #[doc = "1: Tx event FIFO element lost"]
    B0x1 = 1,
}
impl From<Tefl> for bool {
    #[inline(always)]
    fn from(variant: Tefl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEFL` reader - Tx event FIFO element lost"]
pub type TeflR = crate::BitReader<Tefl>;
impl TeflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tefl {
        match self.bits {
            false => Tefl::B0x0,
            true => Tefl::B0x1,
        }
    }
    #[doc = "No Tx event FIFO element lost"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tefl::B0x0
    }
    #[doc = "Tx event FIFO element lost"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tefl::B0x1
    }
}
#[doc = "Field `TEFL` writer - Tx event FIFO element lost"]
pub type TeflW<'a, REG> = crate::BitWriter<'a, REG, Tefl>;
impl<'a, REG> TeflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Tx event FIFO element lost"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tefl::B0x0)
    }
    #[doc = "Tx event FIFO element lost"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tefl::B0x1)
    }
}
#[doc = "Timestamp wraparound\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsw {
    #[doc = "0: No timestamp counter wrap-around"]
    B0x0 = 0,
    #[doc = "1: Timestamp counter wrapped around"]
    B0x1 = 1,
}
impl From<Tsw> for bool {
    #[inline(always)]
    fn from(variant: Tsw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSW` reader - Timestamp wraparound"]
pub type TswR = crate::BitReader<Tsw>;
impl TswR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsw {
        match self.bits {
            false => Tsw::B0x0,
            true => Tsw::B0x1,
        }
    }
    #[doc = "No timestamp counter wrap-around"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tsw::B0x0
    }
    #[doc = "Timestamp counter wrapped around"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tsw::B0x1
    }
}
#[doc = "Field `TSW` writer - Timestamp wraparound"]
pub type TswW<'a, REG> = crate::BitWriter<'a, REG, Tsw>;
impl<'a, REG> TswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No timestamp counter wrap-around"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsw::B0x0)
    }
    #[doc = "Timestamp counter wrapped around"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsw::B0x1)
    }
}
#[doc = "Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see Restricted operation mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mraf {
    #[doc = "0: No Message RAM access failure occurred"]
    B0x0 = 0,
    #[doc = "1: Message RAM access failure occurred"]
    B0x1 = 1,
}
impl From<Mraf> for bool {
    #[inline(always)]
    fn from(variant: Mraf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRAF` reader - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see Restricted operation mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM."]
pub type MrafR = crate::BitReader<Mraf>;
impl MrafR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mraf {
        match self.bits {
            false => Mraf::B0x0,
            true => Mraf::B0x1,
        }
    }
    #[doc = "No Message RAM access failure occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mraf::B0x0
    }
    #[doc = "Message RAM access failure occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mraf::B0x1
    }
}
#[doc = "Field `MRAF` writer - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see Restricted operation mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM."]
pub type MrafW<'a, REG> = crate::BitWriter<'a, REG, Mraf>;
impl<'a, REG> MrafW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Message RAM access failure occurred"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mraf::B0x0)
    }
    #[doc = "Message RAM access failure occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mraf::B0x1)
    }
}
#[doc = "Timeout occurred\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Too {
    #[doc = "0: No timeout"]
    B0x0 = 0,
    #[doc = "1: Timeout reached"]
    B0x1 = 1,
}
impl From<Too> for bool {
    #[inline(always)]
    fn from(variant: Too) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOO` reader - Timeout occurred"]
pub type TooR = crate::BitReader<Too>;
impl TooR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Too {
        match self.bits {
            false => Too::B0x0,
            true => Too::B0x1,
        }
    }
    #[doc = "No timeout"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Too::B0x0
    }
    #[doc = "Timeout reached"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Too::B0x1
    }
}
#[doc = "Field `TOO` writer - Timeout occurred"]
pub type TooW<'a, REG> = crate::BitWriter<'a, REG, Too>;
impl<'a, REG> TooW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No timeout"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Too::B0x0)
    }
    #[doc = "Timeout reached"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Too::B0x1)
    }
}
#[doc = "Error logging overflow\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Elo {
    #[doc = "0: CAN error logging counter did not overflow."]
    B0x0 = 0,
    #[doc = "1: Overflow of CAN error logging counter occurred."]
    B0x1 = 1,
}
impl From<Elo> for bool {
    #[inline(always)]
    fn from(variant: Elo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ELO` reader - Error logging overflow"]
pub type EloR = crate::BitReader<Elo>;
impl EloR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Elo {
        match self.bits {
            false => Elo::B0x0,
            true => Elo::B0x1,
        }
    }
    #[doc = "CAN error logging counter did not overflow."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Elo::B0x0
    }
    #[doc = "Overflow of CAN error logging counter occurred."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Elo::B0x1
    }
}
#[doc = "Field `ELO` writer - Error logging overflow"]
pub type EloW<'a, REG> = crate::BitWriter<'a, REG, Elo>;
impl<'a, REG> EloW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CAN error logging counter did not overflow."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Elo::B0x0)
    }
    #[doc = "Overflow of CAN error logging counter occurred."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Elo::B0x1)
    }
}
#[doc = "Error passive\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep {
    #[doc = "0: Error_Passive status unchanged"]
    B0x0 = 0,
    #[doc = "1: Error_Passive status changed"]
    B0x1 = 1,
}
impl From<Ep> for bool {
    #[inline(always)]
    fn from(variant: Ep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP` reader - Error passive"]
pub type EpR = crate::BitReader<Ep>;
impl EpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep {
        match self.bits {
            false => Ep::B0x0,
            true => Ep::B0x1,
        }
    }
    #[doc = "Error_Passive status unchanged"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ep::B0x0
    }
    #[doc = "Error_Passive status changed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ep::B0x1
    }
}
#[doc = "Field `EP` writer - Error passive"]
pub type EpW<'a, REG> = crate::BitWriter<'a, REG, Ep>;
impl<'a, REG> EpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error_Passive status unchanged"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ep::B0x0)
    }
    #[doc = "Error_Passive status changed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ep::B0x1)
    }
}
#[doc = "Warning status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ew {
    #[doc = "0: Error_Warning status unchanged"]
    B0x0 = 0,
    #[doc = "1: Error_Warning status changed"]
    B0x1 = 1,
}
impl From<Ew> for bool {
    #[inline(always)]
    fn from(variant: Ew) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EW` reader - Warning status"]
pub type EwR = crate::BitReader<Ew>;
impl EwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ew {
        match self.bits {
            false => Ew::B0x0,
            true => Ew::B0x1,
        }
    }
    #[doc = "Error_Warning status unchanged"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ew::B0x0
    }
    #[doc = "Error_Warning status changed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ew::B0x1
    }
}
#[doc = "Field `EW` writer - Warning status"]
pub type EwW<'a, REG> = crate::BitWriter<'a, REG, Ew>;
impl<'a, REG> EwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error_Warning status unchanged"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ew::B0x0)
    }
    #[doc = "Error_Warning status changed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ew::B0x1)
    }
}
#[doc = "Bus_Off status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bo {
    #[doc = "0: Bus_Off status unchanged"]
    B0x0 = 0,
    #[doc = "1: Bus_Off status changed"]
    B0x1 = 1,
}
impl From<Bo> for bool {
    #[inline(always)]
    fn from(variant: Bo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BO` reader - Bus_Off status"]
pub type BoR = crate::BitReader<Bo>;
impl BoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bo {
        match self.bits {
            false => Bo::B0x0,
            true => Bo::B0x1,
        }
    }
    #[doc = "Bus_Off status unchanged"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bo::B0x0
    }
    #[doc = "Bus_Off status changed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bo::B0x1
    }
}
#[doc = "Field `BO` writer - Bus_Off status"]
pub type BoW<'a, REG> = crate::BitWriter<'a, REG, Bo>;
impl<'a, REG> BoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus_Off status unchanged"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bo::B0x0)
    }
    #[doc = "Bus_Off status changed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bo::B0x1)
    }
}
#[doc = "Watchdog interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdi {
    #[doc = "0: No message RAM watchdog event occurred"]
    B0x0 = 0,
    #[doc = "1: Message RAM watchdog event due to missing READY"]
    B0x1 = 1,
}
impl From<Wdi> for bool {
    #[inline(always)]
    fn from(variant: Wdi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDI` reader - Watchdog interrupt"]
pub type WdiR = crate::BitReader<Wdi>;
impl WdiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdi {
        match self.bits {
            false => Wdi::B0x0,
            true => Wdi::B0x1,
        }
    }
    #[doc = "No message RAM watchdog event occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wdi::B0x0
    }
    #[doc = "Message RAM watchdog event due to missing READY"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wdi::B0x1
    }
}
#[doc = "Field `WDI` writer - Watchdog interrupt"]
pub type WdiW<'a, REG> = crate::BitWriter<'a, REG, Wdi>;
impl<'a, REG> WdiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No message RAM watchdog event occurred"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdi::B0x0)
    }
    #[doc = "Message RAM watchdog event due to missing READY"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdi::B0x1)
    }
}
#[doc = "Protocol error in arbitration phase (nominal bit time is used)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pea {
    #[doc = "0: No protocol error in arbitration phase"]
    B0x0 = 0,
    #[doc = "1: Protocol error in arbitration phase detected (PSR.LEC different from 0,7)"]
    B0x1 = 1,
}
impl From<Pea> for bool {
    #[inline(always)]
    fn from(variant: Pea) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEA` reader - Protocol error in arbitration phase (nominal bit time is used)"]
pub type PeaR = crate::BitReader<Pea>;
impl PeaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pea {
        match self.bits {
            false => Pea::B0x0,
            true => Pea::B0x1,
        }
    }
    #[doc = "No protocol error in arbitration phase"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pea::B0x0
    }
    #[doc = "Protocol error in arbitration phase detected (PSR.LEC different from 0,7)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pea::B0x1
    }
}
#[doc = "Field `PEA` writer - Protocol error in arbitration phase (nominal bit time is used)"]
pub type PeaW<'a, REG> = crate::BitWriter<'a, REG, Pea>;
impl<'a, REG> PeaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No protocol error in arbitration phase"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pea::B0x0)
    }
    #[doc = "Protocol error in arbitration phase detected (PSR.LEC different from 0,7)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pea::B0x1)
    }
}
#[doc = "Protocol error in data phase (data bit time is used)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ped {
    #[doc = "0: No protocol error in data phase"]
    B0x0 = 0,
    #[doc = "1: Protocol error in data phase detected (PSR.DLEC different from 0,7)"]
    B0x1 = 1,
}
impl From<Ped> for bool {
    #[inline(always)]
    fn from(variant: Ped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PED` reader - Protocol error in data phase (data bit time is used)"]
pub type PedR = crate::BitReader<Ped>;
impl PedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ped {
        match self.bits {
            false => Ped::B0x0,
            true => Ped::B0x1,
        }
    }
    #[doc = "No protocol error in data phase"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ped::B0x0
    }
    #[doc = "Protocol error in data phase detected (PSR.DLEC different from 0,7)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ped::B0x1
    }
}
#[doc = "Field `PED` writer - Protocol error in data phase (data bit time is used)"]
pub type PedW<'a, REG> = crate::BitWriter<'a, REG, Ped>;
impl<'a, REG> PedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No protocol error in data phase"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ped::B0x0)
    }
    #[doc = "Protocol error in data phase detected (PSR.DLEC different from 0,7)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ped::B0x1)
    }
}
#[doc = "Access to reserved address\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ara {
    #[doc = "0: No access to reserved address occurred"]
    B0x0 = 0,
    #[doc = "1: Access to reserved address occurred"]
    B0x1 = 1,
}
impl From<Ara> for bool {
    #[inline(always)]
    fn from(variant: Ara) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARA` reader - Access to reserved address"]
pub type AraR = crate::BitReader<Ara>;
impl AraR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ara {
        match self.bits {
            false => Ara::B0x0,
            true => Ara::B0x1,
        }
    }
    #[doc = "No access to reserved address occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ara::B0x0
    }
    #[doc = "Access to reserved address occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ara::B0x1
    }
}
#[doc = "Field `ARA` writer - Access to reserved address"]
pub type AraW<'a, REG> = crate::BitWriter<'a, REG, Ara>;
impl<'a, REG> AraW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No access to reserved address occurred"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ara::B0x0)
    }
    #[doc = "Access to reserved address occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ara::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 new message"]
    #[inline(always)]
    pub fn rf0n(&self) -> Rf0nR {
        Rf0nR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 full"]
    #[inline(always)]
    pub fn rf0f(&self) -> Rf0fR {
        Rf0fR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 message lost"]
    #[inline(always)]
    pub fn rf0l(&self) -> Rf0lR {
        Rf0lR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 1 new message"]
    #[inline(always)]
    pub fn rf1n(&self) -> Rf1nR {
        Rf1nR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 full"]
    #[inline(always)]
    pub fn rf1f(&self) -> Rf1fR {
        Rf1fR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 message lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> Rf1lR {
        Rf1lR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - High-priority message"]
    #[inline(always)]
    pub fn hpm(&self) -> HpmR {
        HpmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmission completed"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmission cancellation finished"]
    #[inline(always)]
    pub fn tcf(&self) -> TcfR {
        TcfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tx FIFO empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx event FIFO New Entry"]
    #[inline(always)]
    pub fn tefn(&self) -> TefnR {
        TefnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx event FIFO full"]
    #[inline(always)]
    pub fn teff(&self) -> TeffR {
        TeffR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx event FIFO element lost"]
    #[inline(always)]
    pub fn tefl(&self) -> TeflR {
        TeflR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timestamp wraparound"]
    #[inline(always)]
    pub fn tsw(&self) -> TswR {
        TswR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see Restricted operation mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM."]
    #[inline(always)]
    pub fn mraf(&self) -> MrafR {
        MrafR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timeout occurred"]
    #[inline(always)]
    pub fn too(&self) -> TooR {
        TooR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Error logging overflow"]
    #[inline(always)]
    pub fn elo(&self) -> EloR {
        EloR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Error passive"]
    #[inline(always)]
    pub fn ep(&self) -> EpR {
        EpR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Warning status"]
    #[inline(always)]
    pub fn ew(&self) -> EwR {
        EwR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bus_Off status"]
    #[inline(always)]
    pub fn bo(&self) -> BoR {
        BoR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Watchdog interrupt"]
    #[inline(always)]
    pub fn wdi(&self) -> WdiR {
        WdiR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Protocol error in arbitration phase (nominal bit time is used)"]
    #[inline(always)]
    pub fn pea(&self) -> PeaR {
        PeaR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Protocol error in data phase (data bit time is used)"]
    #[inline(always)]
    pub fn ped(&self) -> PedR {
        PedR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Access to reserved address"]
    #[inline(always)]
    pub fn ara(&self) -> AraR {
        AraR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_IR")
            .field("rf0n", &self.rf0n())
            .field("rf0f", &self.rf0f())
            .field("rf0l", &self.rf0l())
            .field("rf1n", &self.rf1n())
            .field("rf1f", &self.rf1f())
            .field("rf1l", &self.rf1l())
            .field("hpm", &self.hpm())
            .field("tc", &self.tc())
            .field("tcf", &self.tcf())
            .field("tfe", &self.tfe())
            .field("tefn", &self.tefn())
            .field("teff", &self.teff())
            .field("tefl", &self.tefl())
            .field("tsw", &self.tsw())
            .field("mraf", &self.mraf())
            .field("too", &self.too())
            .field("elo", &self.elo())
            .field("ep", &self.ep())
            .field("ew", &self.ew())
            .field("bo", &self.bo())
            .field("wdi", &self.wdi())
            .field("pea", &self.pea())
            .field("ped", &self.ped())
            .field("ara", &self.ara())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 new message"]
    #[inline(always)]
    #[must_use]
    pub fn rf0n(&mut self) -> Rf0nW<FdcanIrSpec> {
        Rf0nW::new(self, 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 full"]
    #[inline(always)]
    #[must_use]
    pub fn rf0f(&mut self) -> Rf0fW<FdcanIrSpec> {
        Rf0fW::new(self, 1)
    }
    #[doc = "Bit 2 - Rx FIFO 0 message lost"]
    #[inline(always)]
    #[must_use]
    pub fn rf0l(&mut self) -> Rf0lW<FdcanIrSpec> {
        Rf0lW::new(self, 2)
    }
    #[doc = "Bit 3 - Rx FIFO 1 new message"]
    #[inline(always)]
    #[must_use]
    pub fn rf1n(&mut self) -> Rf1nW<FdcanIrSpec> {
        Rf1nW::new(self, 3)
    }
    #[doc = "Bit 4 - Rx FIFO 1 full"]
    #[inline(always)]
    #[must_use]
    pub fn rf1f(&mut self) -> Rf1fW<FdcanIrSpec> {
        Rf1fW::new(self, 4)
    }
    #[doc = "Bit 5 - Rx FIFO 1 message lost"]
    #[inline(always)]
    #[must_use]
    pub fn rf1l(&mut self) -> Rf1lW<FdcanIrSpec> {
        Rf1lW::new(self, 5)
    }
    #[doc = "Bit 6 - High-priority message"]
    #[inline(always)]
    #[must_use]
    pub fn hpm(&mut self) -> HpmW<FdcanIrSpec> {
        HpmW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmission completed"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TcW<FdcanIrSpec> {
        TcW::new(self, 7)
    }
    #[doc = "Bit 8 - Transmission cancellation finished"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TcfW<FdcanIrSpec> {
        TcfW::new(self, 8)
    }
    #[doc = "Bit 9 - Tx FIFO empty"]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TfeW<FdcanIrSpec> {
        TfeW::new(self, 9)
    }
    #[doc = "Bit 10 - Tx event FIFO New Entry"]
    #[inline(always)]
    #[must_use]
    pub fn tefn(&mut self) -> TefnW<FdcanIrSpec> {
        TefnW::new(self, 10)
    }
    #[doc = "Bit 11 - Tx event FIFO full"]
    #[inline(always)]
    #[must_use]
    pub fn teff(&mut self) -> TeffW<FdcanIrSpec> {
        TeffW::new(self, 11)
    }
    #[doc = "Bit 12 - Tx event FIFO element lost"]
    #[inline(always)]
    #[must_use]
    pub fn tefl(&mut self) -> TeflW<FdcanIrSpec> {
        TeflW::new(self, 12)
    }
    #[doc = "Bit 13 - Timestamp wraparound"]
    #[inline(always)]
    #[must_use]
    pub fn tsw(&mut self) -> TswW<FdcanIrSpec> {
        TswW::new(self, 13)
    }
    #[doc = "Bit 14 - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see Restricted operation mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM."]
    #[inline(always)]
    #[must_use]
    pub fn mraf(&mut self) -> MrafW<FdcanIrSpec> {
        MrafW::new(self, 14)
    }
    #[doc = "Bit 15 - Timeout occurred"]
    #[inline(always)]
    #[must_use]
    pub fn too(&mut self) -> TooW<FdcanIrSpec> {
        TooW::new(self, 15)
    }
    #[doc = "Bit 16 - Error logging overflow"]
    #[inline(always)]
    #[must_use]
    pub fn elo(&mut self) -> EloW<FdcanIrSpec> {
        EloW::new(self, 16)
    }
    #[doc = "Bit 17 - Error passive"]
    #[inline(always)]
    #[must_use]
    pub fn ep(&mut self) -> EpW<FdcanIrSpec> {
        EpW::new(self, 17)
    }
    #[doc = "Bit 18 - Warning status"]
    #[inline(always)]
    #[must_use]
    pub fn ew(&mut self) -> EwW<FdcanIrSpec> {
        EwW::new(self, 18)
    }
    #[doc = "Bit 19 - Bus_Off status"]
    #[inline(always)]
    #[must_use]
    pub fn bo(&mut self) -> BoW<FdcanIrSpec> {
        BoW::new(self, 19)
    }
    #[doc = "Bit 20 - Watchdog interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wdi(&mut self) -> WdiW<FdcanIrSpec> {
        WdiW::new(self, 20)
    }
    #[doc = "Bit 21 - Protocol error in arbitration phase (nominal bit time is used)"]
    #[inline(always)]
    #[must_use]
    pub fn pea(&mut self) -> PeaW<FdcanIrSpec> {
        PeaW::new(self, 21)
    }
    #[doc = "Bit 22 - Protocol error in data phase (data bit time is used)"]
    #[inline(always)]
    #[must_use]
    pub fn ped(&mut self) -> PedW<FdcanIrSpec> {
        PedW::new(self, 22)
    }
    #[doc = "Bit 23 - Access to reserved address"]
    #[inline(always)]
    #[must_use]
    pub fn ara(&mut self) -> AraW<FdcanIrSpec> {
        AraW::new(self, 23)
    }
}
#[doc = "FDCAN interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanIrSpec;
impl crate::RegisterSpec for FdcanIrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ir::R`](R) reader structure"]
impl crate::Readable for FdcanIrSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ir::W`](W) writer structure"]
impl crate::Writable for FdcanIrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_IR to value 0"]
impl crate::Resettable for FdcanIrSpec {
    const RESET_VALUE: u32 = 0;
}

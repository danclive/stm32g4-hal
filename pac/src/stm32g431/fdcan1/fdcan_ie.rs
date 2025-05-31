#[doc = "Register `FDCAN_IE` reader"]
pub type R = crate::R<FdcanIeSpec>;
#[doc = "Register `FDCAN_IE` writer"]
pub type W = crate::W<FdcanIeSpec>;
#[doc = "Rx FIFO 0 new message interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf0ne {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Rf0ne> for bool {
    #[inline(always)]
    fn from(variant: Rf0ne) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF0NE` reader - Rx FIFO 0 new message interrupt enable"]
pub type Rf0neR = crate::BitReader<Rf0ne>;
impl Rf0neR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rf0ne {
        match self.bits {
            false => Rf0ne::B0x0,
            true => Rf0ne::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rf0ne::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rf0ne::B0x1
    }
}
#[doc = "Field `RF0NE` writer - Rx FIFO 0 new message interrupt enable"]
pub type Rf0neW<'a, REG> = crate::BitWriter<'a, REG, Rf0ne>;
impl<'a, REG> Rf0neW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rf0ne::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rf0ne::B0x1)
    }
}
#[doc = "Rx FIFO 0 full interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf0fe {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Rf0fe> for bool {
    #[inline(always)]
    fn from(variant: Rf0fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF0FE` reader - Rx FIFO 0 full interrupt enable"]
pub type Rf0feR = crate::BitReader<Rf0fe>;
impl Rf0feR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rf0fe {
        match self.bits {
            false => Rf0fe::B0x0,
            true => Rf0fe::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rf0fe::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rf0fe::B0x1
    }
}
#[doc = "Field `RF0FE` writer - Rx FIFO 0 full interrupt enable"]
pub type Rf0feW<'a, REG> = crate::BitWriter<'a, REG, Rf0fe>;
impl<'a, REG> Rf0feW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rf0fe::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rf0fe::B0x1)
    }
}
#[doc = "Rx FIFO 0 message lost interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf0le {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Rf0le> for bool {
    #[inline(always)]
    fn from(variant: Rf0le) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF0LE` reader - Rx FIFO 0 message lost interrupt enable"]
pub type Rf0leR = crate::BitReader<Rf0le>;
impl Rf0leR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rf0le {
        match self.bits {
            false => Rf0le::B0x0,
            true => Rf0le::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rf0le::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rf0le::B0x1
    }
}
#[doc = "Field `RF0LE` writer - Rx FIFO 0 message lost interrupt enable"]
pub type Rf0leW<'a, REG> = crate::BitWriter<'a, REG, Rf0le>;
impl<'a, REG> Rf0leW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rf0le::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rf0le::B0x1)
    }
}
#[doc = "Rx FIFO 1 new message interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf1ne {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Rf1ne> for bool {
    #[inline(always)]
    fn from(variant: Rf1ne) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF1NE` reader - Rx FIFO 1 new message interrupt enable"]
pub type Rf1neR = crate::BitReader<Rf1ne>;
impl Rf1neR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rf1ne {
        match self.bits {
            false => Rf1ne::B0x0,
            true => Rf1ne::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rf1ne::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rf1ne::B0x1
    }
}
#[doc = "Field `RF1NE` writer - Rx FIFO 1 new message interrupt enable"]
pub type Rf1neW<'a, REG> = crate::BitWriter<'a, REG, Rf1ne>;
impl<'a, REG> Rf1neW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rf1ne::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rf1ne::B0x1)
    }
}
#[doc = "Rx FIFO 1 full interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf1fe {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Rf1fe> for bool {
    #[inline(always)]
    fn from(variant: Rf1fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF1FE` reader - Rx FIFO 1 full interrupt enable"]
pub type Rf1feR = crate::BitReader<Rf1fe>;
impl Rf1feR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rf1fe {
        match self.bits {
            false => Rf1fe::B0x0,
            true => Rf1fe::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rf1fe::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rf1fe::B0x1
    }
}
#[doc = "Field `RF1FE` writer - Rx FIFO 1 full interrupt enable"]
pub type Rf1feW<'a, REG> = crate::BitWriter<'a, REG, Rf1fe>;
impl<'a, REG> Rf1feW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rf1fe::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rf1fe::B0x1)
    }
}
#[doc = "Rx FIFO 1 message lost interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf1le {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Rf1le> for bool {
    #[inline(always)]
    fn from(variant: Rf1le) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF1LE` reader - Rx FIFO 1 message lost interrupt enable"]
pub type Rf1leR = crate::BitReader<Rf1le>;
impl Rf1leR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rf1le {
        match self.bits {
            false => Rf1le::B0x0,
            true => Rf1le::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rf1le::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rf1le::B0x1
    }
}
#[doc = "Field `RF1LE` writer - Rx FIFO 1 message lost interrupt enable"]
pub type Rf1leW<'a, REG> = crate::BitWriter<'a, REG, Rf1le>;
impl<'a, REG> Rf1leW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rf1le::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rf1le::B0x1)
    }
}
#[doc = "High-priority message interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hpme {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Hpme> for bool {
    #[inline(always)]
    fn from(variant: Hpme) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HPME` reader - High-priority message interrupt enable"]
pub type HpmeR = crate::BitReader<Hpme>;
impl HpmeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hpme {
        match self.bits {
            false => Hpme::B0x0,
            true => Hpme::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hpme::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hpme::B0x1
    }
}
#[doc = "Field `HPME` writer - High-priority message interrupt enable"]
pub type HpmeW<'a, REG> = crate::BitWriter<'a, REG, Hpme>;
impl<'a, REG> HpmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hpme::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hpme::B0x1)
    }
}
#[doc = "Transmission completed interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tce {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Tce> for bool {
    #[inline(always)]
    fn from(variant: Tce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCE` reader - Transmission completed interrupt enable"]
pub type TceR = crate::BitReader<Tce>;
impl TceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tce {
        match self.bits {
            false => Tce::B0x0,
            true => Tce::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tce::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tce::B0x1
    }
}
#[doc = "Field `TCE` writer - Transmission completed interrupt enable"]
pub type TceW<'a, REG> = crate::BitWriter<'a, REG, Tce>;
impl<'a, REG> TceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tce::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tce::B0x1)
    }
}
#[doc = "Transmission cancellation finished interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcfe {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Tcfe> for bool {
    #[inline(always)]
    fn from(variant: Tcfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCFE` reader - Transmission cancellation finished interrupt enable"]
pub type TcfeR = crate::BitReader<Tcfe>;
impl TcfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcfe {
        match self.bits {
            false => Tcfe::B0x0,
            true => Tcfe::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tcfe::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tcfe::B0x1
    }
}
#[doc = "Field `TCFE` writer - Transmission cancellation finished interrupt enable"]
pub type TcfeW<'a, REG> = crate::BitWriter<'a, REG, Tcfe>;
impl<'a, REG> TcfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfe::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcfe::B0x1)
    }
}
#[doc = "Tx FIFO empty interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfee {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Tfee> for bool {
    #[inline(always)]
    fn from(variant: Tfee) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFEE` reader - Tx FIFO empty interrupt enable"]
pub type TfeeR = crate::BitReader<Tfee>;
impl TfeeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfee {
        match self.bits {
            false => Tfee::B0x0,
            true => Tfee::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tfee::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tfee::B0x1
    }
}
#[doc = "Field `TFEE` writer - Tx FIFO empty interrupt enable"]
pub type TfeeW<'a, REG> = crate::BitWriter<'a, REG, Tfee>;
impl<'a, REG> TfeeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tfee::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tfee::B0x1)
    }
}
#[doc = "Tx event FIFO new entry interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tefne {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Tefne> for bool {
    #[inline(always)]
    fn from(variant: Tefne) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEFNE` reader - Tx event FIFO new entry interrupt enable"]
pub type TefneR = crate::BitReader<Tefne>;
impl TefneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tefne {
        match self.bits {
            false => Tefne::B0x0,
            true => Tefne::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tefne::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tefne::B0x1
    }
}
#[doc = "Field `TEFNE` writer - Tx event FIFO new entry interrupt enable"]
pub type TefneW<'a, REG> = crate::BitWriter<'a, REG, Tefne>;
impl<'a, REG> TefneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tefne::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tefne::B0x1)
    }
}
#[doc = "Tx event FIFO full interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Teffe {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Teffe> for bool {
    #[inline(always)]
    fn from(variant: Teffe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEFFE` reader - Tx event FIFO full interrupt enable"]
pub type TeffeR = crate::BitReader<Teffe>;
impl TeffeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Teffe {
        match self.bits {
            false => Teffe::B0x0,
            true => Teffe::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Teffe::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Teffe::B0x1
    }
}
#[doc = "Field `TEFFE` writer - Tx event FIFO full interrupt enable"]
pub type TeffeW<'a, REG> = crate::BitWriter<'a, REG, Teffe>;
impl<'a, REG> TeffeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Teffe::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Teffe::B0x1)
    }
}
#[doc = "Tx event FIFO element lost interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tefle {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Tefle> for bool {
    #[inline(always)]
    fn from(variant: Tefle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEFLE` reader - Tx event FIFO element lost interrupt enable"]
pub type TefleR = crate::BitReader<Tefle>;
impl TefleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tefle {
        match self.bits {
            false => Tefle::B0x0,
            true => Tefle::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tefle::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tefle::B0x1
    }
}
#[doc = "Field `TEFLE` writer - Tx event FIFO element lost interrupt enable"]
pub type TefleW<'a, REG> = crate::BitWriter<'a, REG, Tefle>;
impl<'a, REG> TefleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tefle::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tefle::B0x1)
    }
}
#[doc = "Timestamp wraparound interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tswe {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Tswe> for bool {
    #[inline(always)]
    fn from(variant: Tswe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSWE` reader - Timestamp wraparound interrupt enable"]
pub type TsweR = crate::BitReader<Tswe>;
impl TsweR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tswe {
        match self.bits {
            false => Tswe::B0x0,
            true => Tswe::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tswe::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tswe::B0x1
    }
}
#[doc = "Field `TSWE` writer - Timestamp wraparound interrupt enable"]
pub type TsweW<'a, REG> = crate::BitWriter<'a, REG, Tswe>;
impl<'a, REG> TsweW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tswe::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tswe::B0x1)
    }
}
#[doc = "Message RAM access failure interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mrafe {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Mrafe> for bool {
    #[inline(always)]
    fn from(variant: Mrafe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRAFE` reader - Message RAM access failure interrupt enable"]
pub type MrafeR = crate::BitReader<Mrafe>;
impl MrafeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mrafe {
        match self.bits {
            false => Mrafe::B0x0,
            true => Mrafe::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mrafe::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mrafe::B0x1
    }
}
#[doc = "Field `MRAFE` writer - Message RAM access failure interrupt enable"]
pub type MrafeW<'a, REG> = crate::BitWriter<'a, REG, Mrafe>;
impl<'a, REG> MrafeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mrafe::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mrafe::B0x1)
    }
}
#[doc = "Timeout occurred interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tooe {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Tooe> for bool {
    #[inline(always)]
    fn from(variant: Tooe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOOE` reader - Timeout occurred interrupt enable"]
pub type TooeR = crate::BitReader<Tooe>;
impl TooeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tooe {
        match self.bits {
            false => Tooe::B0x0,
            true => Tooe::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tooe::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tooe::B0x1
    }
}
#[doc = "Field `TOOE` writer - Timeout occurred interrupt enable"]
pub type TooeW<'a, REG> = crate::BitWriter<'a, REG, Tooe>;
impl<'a, REG> TooeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tooe::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tooe::B0x1)
    }
}
#[doc = "Error logging overflow interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eloe {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Eloe> for bool {
    #[inline(always)]
    fn from(variant: Eloe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ELOE` reader - Error logging overflow interrupt enable"]
pub type EloeR = crate::BitReader<Eloe>;
impl EloeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eloe {
        match self.bits {
            false => Eloe::B0x0,
            true => Eloe::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eloe::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eloe::B0x1
    }
}
#[doc = "Field `ELOE` writer - Error logging overflow interrupt enable"]
pub type EloeW<'a, REG> = crate::BitWriter<'a, REG, Eloe>;
impl<'a, REG> EloeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Eloe::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Eloe::B0x1)
    }
}
#[doc = "Error passive interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epe {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Epe> for bool {
    #[inline(always)]
    fn from(variant: Epe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPE` reader - Error passive interrupt enable"]
pub type EpeR = crate::BitReader<Epe>;
impl EpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epe {
        match self.bits {
            false => Epe::B0x0,
            true => Epe::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Epe::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Epe::B0x1
    }
}
#[doc = "Field `EPE` writer - Error passive interrupt enable"]
pub type EpeW<'a, REG> = crate::BitWriter<'a, REG, Epe>;
impl<'a, REG> EpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Epe::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Epe::B0x1)
    }
}
#[doc = "Warning status interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ewe {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Ewe> for bool {
    #[inline(always)]
    fn from(variant: Ewe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWE` reader - Warning status interrupt enable"]
pub type EweR = crate::BitReader<Ewe>;
impl EweR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ewe {
        match self.bits {
            false => Ewe::B0x0,
            true => Ewe::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ewe::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ewe::B0x1
    }
}
#[doc = "Field `EWE` writer - Warning status interrupt enable"]
pub type EweW<'a, REG> = crate::BitWriter<'a, REG, Ewe>;
impl<'a, REG> EweW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ewe::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ewe::B0x1)
    }
}
#[doc = "Bus_Off status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boe {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Boe> for bool {
    #[inline(always)]
    fn from(variant: Boe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOE` reader - Bus_Off status"]
pub type BoeR = crate::BitReader<Boe>;
impl BoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Boe {
        match self.bits {
            false => Boe::B0x0,
            true => Boe::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Boe::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Boe::B0x1
    }
}
#[doc = "Field `BOE` writer - Bus_Off status"]
pub type BoeW<'a, REG> = crate::BitWriter<'a, REG, Boe>;
impl<'a, REG> BoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Boe::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Boe::B0x1)
    }
}
#[doc = "Watchdog interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdie {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Wdie> for bool {
    #[inline(always)]
    fn from(variant: Wdie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDIE` reader - Watchdog interrupt enable"]
pub type WdieR = crate::BitReader<Wdie>;
impl WdieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdie {
        match self.bits {
            false => Wdie::B0x0,
            true => Wdie::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wdie::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wdie::B0x1
    }
}
#[doc = "Field `WDIE` writer - Watchdog interrupt enable"]
pub type WdieW<'a, REG> = crate::BitWriter<'a, REG, Wdie>;
impl<'a, REG> WdieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdie::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdie::B0x1)
    }
}
#[doc = "Field `PEAE` reader - Protocol error in arbitration phase enable"]
pub type PeaeR = crate::BitReader;
#[doc = "Field `PEAE` writer - Protocol error in arbitration phase enable"]
pub type PeaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEDE` reader - Protocol error in data phase enable"]
pub type PedeR = crate::BitReader;
#[doc = "Field `PEDE` writer - Protocol error in data phase enable"]
pub type PedeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARAE` reader - Access to reserved address enable"]
pub type AraeR = crate::BitReader;
#[doc = "Field `ARAE` writer - Access to reserved address enable"]
pub type AraeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 new message interrupt enable"]
    #[inline(always)]
    pub fn rf0ne(&self) -> Rf0neR {
        Rf0neR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 full interrupt enable"]
    #[inline(always)]
    pub fn rf0fe(&self) -> Rf0feR {
        Rf0feR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 message lost interrupt enable"]
    #[inline(always)]
    pub fn rf0le(&self) -> Rf0leR {
        Rf0leR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 1 new message interrupt enable"]
    #[inline(always)]
    pub fn rf1ne(&self) -> Rf1neR {
        Rf1neR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 full interrupt enable"]
    #[inline(always)]
    pub fn rf1fe(&self) -> Rf1feR {
        Rf1feR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 message lost interrupt enable"]
    #[inline(always)]
    pub fn rf1le(&self) -> Rf1leR {
        Rf1leR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - High-priority message interrupt enable"]
    #[inline(always)]
    pub fn hpme(&self) -> HpmeR {
        HpmeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmission completed interrupt enable"]
    #[inline(always)]
    pub fn tce(&self) -> TceR {
        TceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmission cancellation finished interrupt enable"]
    #[inline(always)]
    pub fn tcfe(&self) -> TcfeR {
        TcfeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tx FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn tfee(&self) -> TfeeR {
        TfeeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx event FIFO new entry interrupt enable"]
    #[inline(always)]
    pub fn tefne(&self) -> TefneR {
        TefneR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx event FIFO full interrupt enable"]
    #[inline(always)]
    pub fn teffe(&self) -> TeffeR {
        TeffeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx event FIFO element lost interrupt enable"]
    #[inline(always)]
    pub fn tefle(&self) -> TefleR {
        TefleR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timestamp wraparound interrupt enable"]
    #[inline(always)]
    pub fn tswe(&self) -> TsweR {
        TsweR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Message RAM access failure interrupt enable"]
    #[inline(always)]
    pub fn mrafe(&self) -> MrafeR {
        MrafeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timeout occurred interrupt enable"]
    #[inline(always)]
    pub fn tooe(&self) -> TooeR {
        TooeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Error logging overflow interrupt enable"]
    #[inline(always)]
    pub fn eloe(&self) -> EloeR {
        EloeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Error passive interrupt enable"]
    #[inline(always)]
    pub fn epe(&self) -> EpeR {
        EpeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Warning status interrupt enable"]
    #[inline(always)]
    pub fn ewe(&self) -> EweR {
        EweR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bus_Off status"]
    #[inline(always)]
    pub fn boe(&self) -> BoeR {
        BoeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Watchdog interrupt enable"]
    #[inline(always)]
    pub fn wdie(&self) -> WdieR {
        WdieR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Protocol error in arbitration phase enable"]
    #[inline(always)]
    pub fn peae(&self) -> PeaeR {
        PeaeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Protocol error in data phase enable"]
    #[inline(always)]
    pub fn pede(&self) -> PedeR {
        PedeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Access to reserved address enable"]
    #[inline(always)]
    pub fn arae(&self) -> AraeR {
        AraeR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_IE")
            .field("rf0ne", &self.rf0ne())
            .field("rf0fe", &self.rf0fe())
            .field("rf0le", &self.rf0le())
            .field("rf1ne", &self.rf1ne())
            .field("rf1fe", &self.rf1fe())
            .field("rf1le", &self.rf1le())
            .field("hpme", &self.hpme())
            .field("tce", &self.tce())
            .field("tcfe", &self.tcfe())
            .field("tfee", &self.tfee())
            .field("tefne", &self.tefne())
            .field("teffe", &self.teffe())
            .field("tefle", &self.tefle())
            .field("tswe", &self.tswe())
            .field("mrafe", &self.mrafe())
            .field("tooe", &self.tooe())
            .field("eloe", &self.eloe())
            .field("epe", &self.epe())
            .field("ewe", &self.ewe())
            .field("boe", &self.boe())
            .field("wdie", &self.wdie())
            .field("peae", &self.peae())
            .field("pede", &self.pede())
            .field("arae", &self.arae())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 new message interrupt enable"]
    #[inline(always)]
    pub fn rf0ne(&mut self) -> Rf0neW<FdcanIeSpec> {
        Rf0neW::new(self, 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 full interrupt enable"]
    #[inline(always)]
    pub fn rf0fe(&mut self) -> Rf0feW<FdcanIeSpec> {
        Rf0feW::new(self, 1)
    }
    #[doc = "Bit 2 - Rx FIFO 0 message lost interrupt enable"]
    #[inline(always)]
    pub fn rf0le(&mut self) -> Rf0leW<FdcanIeSpec> {
        Rf0leW::new(self, 2)
    }
    #[doc = "Bit 3 - Rx FIFO 1 new message interrupt enable"]
    #[inline(always)]
    pub fn rf1ne(&mut self) -> Rf1neW<FdcanIeSpec> {
        Rf1neW::new(self, 3)
    }
    #[doc = "Bit 4 - Rx FIFO 1 full interrupt enable"]
    #[inline(always)]
    pub fn rf1fe(&mut self) -> Rf1feW<FdcanIeSpec> {
        Rf1feW::new(self, 4)
    }
    #[doc = "Bit 5 - Rx FIFO 1 message lost interrupt enable"]
    #[inline(always)]
    pub fn rf1le(&mut self) -> Rf1leW<FdcanIeSpec> {
        Rf1leW::new(self, 5)
    }
    #[doc = "Bit 6 - High-priority message interrupt enable"]
    #[inline(always)]
    pub fn hpme(&mut self) -> HpmeW<FdcanIeSpec> {
        HpmeW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmission completed interrupt enable"]
    #[inline(always)]
    pub fn tce(&mut self) -> TceW<FdcanIeSpec> {
        TceW::new(self, 7)
    }
    #[doc = "Bit 8 - Transmission cancellation finished interrupt enable"]
    #[inline(always)]
    pub fn tcfe(&mut self) -> TcfeW<FdcanIeSpec> {
        TcfeW::new(self, 8)
    }
    #[doc = "Bit 9 - Tx FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn tfee(&mut self) -> TfeeW<FdcanIeSpec> {
        TfeeW::new(self, 9)
    }
    #[doc = "Bit 10 - Tx event FIFO new entry interrupt enable"]
    #[inline(always)]
    pub fn tefne(&mut self) -> TefneW<FdcanIeSpec> {
        TefneW::new(self, 10)
    }
    #[doc = "Bit 11 - Tx event FIFO full interrupt enable"]
    #[inline(always)]
    pub fn teffe(&mut self) -> TeffeW<FdcanIeSpec> {
        TeffeW::new(self, 11)
    }
    #[doc = "Bit 12 - Tx event FIFO element lost interrupt enable"]
    #[inline(always)]
    pub fn tefle(&mut self) -> TefleW<FdcanIeSpec> {
        TefleW::new(self, 12)
    }
    #[doc = "Bit 13 - Timestamp wraparound interrupt enable"]
    #[inline(always)]
    pub fn tswe(&mut self) -> TsweW<FdcanIeSpec> {
        TsweW::new(self, 13)
    }
    #[doc = "Bit 14 - Message RAM access failure interrupt enable"]
    #[inline(always)]
    pub fn mrafe(&mut self) -> MrafeW<FdcanIeSpec> {
        MrafeW::new(self, 14)
    }
    #[doc = "Bit 15 - Timeout occurred interrupt enable"]
    #[inline(always)]
    pub fn tooe(&mut self) -> TooeW<FdcanIeSpec> {
        TooeW::new(self, 15)
    }
    #[doc = "Bit 16 - Error logging overflow interrupt enable"]
    #[inline(always)]
    pub fn eloe(&mut self) -> EloeW<FdcanIeSpec> {
        EloeW::new(self, 16)
    }
    #[doc = "Bit 17 - Error passive interrupt enable"]
    #[inline(always)]
    pub fn epe(&mut self) -> EpeW<FdcanIeSpec> {
        EpeW::new(self, 17)
    }
    #[doc = "Bit 18 - Warning status interrupt enable"]
    #[inline(always)]
    pub fn ewe(&mut self) -> EweW<FdcanIeSpec> {
        EweW::new(self, 18)
    }
    #[doc = "Bit 19 - Bus_Off status"]
    #[inline(always)]
    pub fn boe(&mut self) -> BoeW<FdcanIeSpec> {
        BoeW::new(self, 19)
    }
    #[doc = "Bit 20 - Watchdog interrupt enable"]
    #[inline(always)]
    pub fn wdie(&mut self) -> WdieW<FdcanIeSpec> {
        WdieW::new(self, 20)
    }
    #[doc = "Bit 21 - Protocol error in arbitration phase enable"]
    #[inline(always)]
    pub fn peae(&mut self) -> PeaeW<FdcanIeSpec> {
        PeaeW::new(self, 21)
    }
    #[doc = "Bit 22 - Protocol error in data phase enable"]
    #[inline(always)]
    pub fn pede(&mut self) -> PedeW<FdcanIeSpec> {
        PedeW::new(self, 22)
    }
    #[doc = "Bit 23 - Access to reserved address enable"]
    #[inline(always)]
    pub fn arae(&mut self) -> AraeW<FdcanIeSpec> {
        AraeW::new(self, 23)
    }
}
#[doc = "FDCAN interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanIeSpec;
impl crate::RegisterSpec for FdcanIeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ie::R`](R) reader structure"]
impl crate::Readable for FdcanIeSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ie::W`](W) writer structure"]
impl crate::Writable for FdcanIeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_IE to value 0"]
impl crate::Resettable for FdcanIeSpec {}

#[doc = "Register `FDCAN_CCCR` reader"]
pub type R = crate::R<FdcanCccrSpec>;
#[doc = "Register `FDCAN_CCCR` writer"]
pub type W = crate::W<FdcanCccrSpec>;
#[doc = "Initialization\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Init {
    #[doc = "0: Normal operation"]
    B0x0 = 0,
    #[doc = "1: Initialization started"]
    B0x1 = 1,
}
impl From<Init> for bool {
    #[inline(always)]
    fn from(variant: Init) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIT` reader - Initialization"]
pub type InitR = crate::BitReader<Init>;
impl InitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Init {
        match self.bits {
            false => Init::B0x0,
            true => Init::B0x1,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Init::B0x0
    }
    #[doc = "Initialization started"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Init::B0x1
    }
}
#[doc = "Field `INIT` writer - Initialization"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG, Init>;
impl<'a, REG> InitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Init::B0x0)
    }
    #[doc = "Initialization started"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Init::B0x1)
    }
}
#[doc = "Configuration change enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cce {
    #[doc = "0: The CPU has no write access to the protected configuration registers."]
    B0x0 = 0,
    #[doc = "1: The CPU has write access to the protected configuration registers (while CCCR.INIT = 1)."]
    B0x1 = 1,
}
impl From<Cce> for bool {
    #[inline(always)]
    fn from(variant: Cce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCE` reader - Configuration change enable"]
pub type CceR = crate::BitReader<Cce>;
impl CceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cce {
        match self.bits {
            false => Cce::B0x0,
            true => Cce::B0x1,
        }
    }
    #[doc = "The CPU has no write access to the protected configuration registers."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cce::B0x0
    }
    #[doc = "The CPU has write access to the protected configuration registers (while CCCR.INIT = 1)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cce::B0x1
    }
}
#[doc = "Field `CCE` writer - Configuration change enable"]
pub type CceW<'a, REG> = crate::BitWriter<'a, REG, Cce>;
impl<'a, REG> CceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The CPU has no write access to the protected configuration registers."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cce::B0x0)
    }
    #[doc = "The CPU has write access to the protected configuration registers (while CCCR.INIT = 1)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cce::B0x1)
    }
}
#[doc = "ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asm {
    #[doc = "0: Normal CAN operation"]
    B0x0 = 0,
    #[doc = "1: Restricted operation Mode active"]
    B0x1 = 1,
}
impl From<Asm> for bool {
    #[inline(always)]
    fn from(variant: Asm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASM` reader - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time."]
pub type AsmR = crate::BitReader<Asm>;
impl AsmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asm {
        match self.bits {
            false => Asm::B0x0,
            true => Asm::B0x1,
        }
    }
    #[doc = "Normal CAN operation"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Asm::B0x0
    }
    #[doc = "Restricted operation Mode active"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Asm::B0x1
    }
}
#[doc = "Field `ASM` writer - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time."]
pub type AsmW<'a, REG> = crate::BitWriter<'a, REG, Asm>;
impl<'a, REG> AsmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal CAN operation"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Asm::B0x0)
    }
    #[doc = "Restricted operation Mode active"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Asm::B0x1)
    }
}
#[doc = "Clock stop acknowledge\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csa {
    #[doc = "0: No clock stop acknowledged"]
    B0x0 = 0,
    #[doc = "1: FDCAN may be set in power down by stopping APB clock and kernel clock."]
    B0x1 = 1,
}
impl From<Csa> for bool {
    #[inline(always)]
    fn from(variant: Csa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSA` reader - Clock stop acknowledge"]
pub type CsaR = crate::BitReader<Csa>;
impl CsaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csa {
        match self.bits {
            false => Csa::B0x0,
            true => Csa::B0x1,
        }
    }
    #[doc = "No clock stop acknowledged"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Csa::B0x0
    }
    #[doc = "FDCAN may be set in power down by stopping APB clock and kernel clock."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Csa::B0x1
    }
}
#[doc = "Clock stop request\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csr {
    #[doc = "0: No clock stop requested"]
    B0x0 = 0,
    #[doc = "1: Clock stop requested. When clock stop is requested, first INIT and then CSA is set after all pending transfer requests have been completed and the CAN bus reached idle."]
    B0x1 = 1,
}
impl From<Csr> for bool {
    #[inline(always)]
    fn from(variant: Csr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSR` reader - Clock stop request"]
pub type CsrR = crate::BitReader<Csr>;
impl CsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csr {
        match self.bits {
            false => Csr::B0x0,
            true => Csr::B0x1,
        }
    }
    #[doc = "No clock stop requested"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Csr::B0x0
    }
    #[doc = "Clock stop requested. When clock stop is requested, first INIT and then CSA is set after all pending transfer requests have been completed and the CAN bus reached idle."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Csr::B0x1
    }
}
#[doc = "Field `CSR` writer - Clock stop request"]
pub type CsrW<'a, REG> = crate::BitWriter<'a, REG, Csr>;
impl<'a, REG> CsrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No clock stop requested"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Csr::B0x0)
    }
    #[doc = "Clock stop requested. When clock stop is requested, first INIT and then CSA is set after all pending transfer requests have been completed and the CAN bus reached idle."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Csr::B0x1)
    }
}
#[doc = "Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mon {
    #[doc = "0: Bus monitoring mode disabled"]
    B0x0 = 0,
    #[doc = "1: Bus monitoring mode enabled"]
    B0x1 = 1,
}
impl From<Mon> for bool {
    #[inline(always)]
    fn from(variant: Mon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MON` reader - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time."]
pub type MonR = crate::BitReader<Mon>;
impl MonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mon {
        match self.bits {
            false => Mon::B0x0,
            true => Mon::B0x1,
        }
    }
    #[doc = "Bus monitoring mode disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mon::B0x0
    }
    #[doc = "Bus monitoring mode enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mon::B0x1
    }
}
#[doc = "Field `MON` writer - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time."]
pub type MonW<'a, REG> = crate::BitWriter<'a, REG, Mon>;
impl<'a, REG> MonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus monitoring mode disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mon::B0x0)
    }
    #[doc = "Bus monitoring mode enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mon::B0x1)
    }
}
#[doc = "Disable automatic retransmission\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dar {
    #[doc = "0: Automatic retransmission of messages not transmitted successfully enabled"]
    B0x0 = 0,
    #[doc = "1: Automatic retransmission disabled"]
    B0x1 = 1,
}
impl From<Dar> for bool {
    #[inline(always)]
    fn from(variant: Dar) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAR` reader - Disable automatic retransmission"]
pub type DarR = crate::BitReader<Dar>;
impl DarR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dar {
        match self.bits {
            false => Dar::B0x0,
            true => Dar::B0x1,
        }
    }
    #[doc = "Automatic retransmission of messages not transmitted successfully enabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dar::B0x0
    }
    #[doc = "Automatic retransmission disabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dar::B0x1
    }
}
#[doc = "Field `DAR` writer - Disable automatic retransmission"]
pub type DarW<'a, REG> = crate::BitWriter<'a, REG, Dar>;
impl<'a, REG> DarW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic retransmission of messages not transmitted successfully enabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dar::B0x0)
    }
    #[doc = "Automatic retransmission disabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dar::B0x1)
    }
}
#[doc = "Test mode enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Test {
    #[doc = "0: Normal operation, register TEST holds reset values"]
    B0x0 = 0,
    #[doc = "1: Test Mode, write access to register TEST enabled"]
    B0x1 = 1,
}
impl From<Test> for bool {
    #[inline(always)]
    fn from(variant: Test) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEST` reader - Test mode enable"]
pub type TestR = crate::BitReader<Test>;
impl TestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Test {
        match self.bits {
            false => Test::B0x0,
            true => Test::B0x1,
        }
    }
    #[doc = "Normal operation, register TEST holds reset values"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Test::B0x0
    }
    #[doc = "Test Mode, write access to register TEST enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Test::B0x1
    }
}
#[doc = "Field `TEST` writer - Test mode enable"]
pub type TestW<'a, REG> = crate::BitWriter<'a, REG, Test>;
impl<'a, REG> TestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation, register TEST holds reset values"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Test::B0x0)
    }
    #[doc = "Test Mode, write access to register TEST enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Test::B0x1)
    }
}
#[doc = "FD operation enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fdoe {
    #[doc = "0: FD operation disabled"]
    B0x0 = 0,
    #[doc = "1: FD operation enabled"]
    B0x1 = 1,
}
impl From<Fdoe> for bool {
    #[inline(always)]
    fn from(variant: Fdoe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDOE` reader - FD operation enable"]
pub type FdoeR = crate::BitReader<Fdoe>;
impl FdoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fdoe {
        match self.bits {
            false => Fdoe::B0x0,
            true => Fdoe::B0x1,
        }
    }
    #[doc = "FD operation disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Fdoe::B0x0
    }
    #[doc = "FD operation enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Fdoe::B0x1
    }
}
#[doc = "Field `FDOE` writer - FD operation enable"]
pub type FdoeW<'a, REG> = crate::BitWriter<'a, REG, Fdoe>;
impl<'a, REG> FdoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FD operation disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Fdoe::B0x0)
    }
    #[doc = "FD operation enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Fdoe::B0x1)
    }
}
#[doc = "FDCAN bit rate switching\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brse {
    #[doc = "0: Bit rate switching for transmissions disabled"]
    B0x0 = 0,
    #[doc = "1: Bit rate switching for transmissions enabled"]
    B0x1 = 1,
}
impl From<Brse> for bool {
    #[inline(always)]
    fn from(variant: Brse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRSE` reader - FDCAN bit rate switching"]
pub type BrseR = crate::BitReader<Brse>;
impl BrseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brse {
        match self.bits {
            false => Brse::B0x0,
            true => Brse::B0x1,
        }
    }
    #[doc = "Bit rate switching for transmissions disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Brse::B0x0
    }
    #[doc = "Bit rate switching for transmissions enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Brse::B0x1
    }
}
#[doc = "Field `BRSE` writer - FDCAN bit rate switching"]
pub type BrseW<'a, REG> = crate::BitWriter<'a, REG, Brse>;
impl<'a, REG> BrseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit rate switching for transmissions disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Brse::B0x0)
    }
    #[doc = "Bit rate switching for transmissions enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Brse::B0x1)
    }
}
#[doc = "Protocol exception handling disable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pxhd {
    #[doc = "0: Protocol exception handling enabled"]
    B0x0 = 0,
    #[doc = "1: Protocol exception handling disabled"]
    B0x1 = 1,
}
impl From<Pxhd> for bool {
    #[inline(always)]
    fn from(variant: Pxhd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PXHD` reader - Protocol exception handling disable"]
pub type PxhdR = crate::BitReader<Pxhd>;
impl PxhdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pxhd {
        match self.bits {
            false => Pxhd::B0x0,
            true => Pxhd::B0x1,
        }
    }
    #[doc = "Protocol exception handling enabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pxhd::B0x0
    }
    #[doc = "Protocol exception handling disabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pxhd::B0x1
    }
}
#[doc = "Field `PXHD` writer - Protocol exception handling disable"]
pub type PxhdW<'a, REG> = crate::BitWriter<'a, REG, Pxhd>;
impl<'a, REG> PxhdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protocol exception handling enabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pxhd::B0x0)
    }
    #[doc = "Protocol exception handling disabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pxhd::B0x1)
    }
}
#[doc = "Edge filtering during bus integration\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Efbi {
    #[doc = "0: Edge filtering disabled"]
    B0x0 = 0,
    #[doc = "1: Two consecutive dominant tq required to detect an edge for hard synchronization"]
    B0x1 = 1,
}
impl From<Efbi> for bool {
    #[inline(always)]
    fn from(variant: Efbi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EFBI` reader - Edge filtering during bus integration"]
pub type EfbiR = crate::BitReader<Efbi>;
impl EfbiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Efbi {
        match self.bits {
            false => Efbi::B0x0,
            true => Efbi::B0x1,
        }
    }
    #[doc = "Edge filtering disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Efbi::B0x0
    }
    #[doc = "Two consecutive dominant tq required to detect an edge for hard synchronization"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Efbi::B0x1
    }
}
#[doc = "Field `EFBI` writer - Edge filtering during bus integration"]
pub type EfbiW<'a, REG> = crate::BitWriter<'a, REG, Efbi>;
impl<'a, REG> EfbiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Edge filtering disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Efbi::B0x0)
    }
    #[doc = "Two consecutive dominant tq required to detect an edge for hard synchronization"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Efbi::B0x1)
    }
}
#[doc = "If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txp {
    #[doc = "0: disabled"]
    B0x0 = 0,
    #[doc = "1: enabled"]
    B0x1 = 1,
}
impl From<Txp> for bool {
    #[inline(always)]
    fn from(variant: Txp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXP` reader - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame."]
pub type TxpR = crate::BitReader<Txp>;
impl TxpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txp {
        match self.bits {
            false => Txp::B0x0,
            true => Txp::B0x1,
        }
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Txp::B0x0
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Txp::B0x1
    }
}
#[doc = "Field `TXP` writer - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame."]
pub type TxpW<'a, REG> = crate::BitWriter<'a, REG, Txp>;
impl<'a, REG> TxpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Txp::B0x0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Txp::B0x1)
    }
}
#[doc = "Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Niso {
    #[doc = "0: CAN FD frame format according to ISO11898-1"]
    B0x0 = 0,
    #[doc = "1: CAN FD frame format according to Bosch CAN FD Specification V1.0"]
    B0x1 = 1,
}
impl From<Niso> for bool {
    #[inline(always)]
    fn from(variant: Niso) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NISO` reader - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0."]
pub type NisoR = crate::BitReader<Niso>;
impl NisoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Niso {
        match self.bits {
            false => Niso::B0x0,
            true => Niso::B0x1,
        }
    }
    #[doc = "CAN FD frame format according to ISO11898-1"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Niso::B0x0
    }
    #[doc = "CAN FD frame format according to Bosch CAN FD Specification V1.0"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Niso::B0x1
    }
}
#[doc = "Field `NISO` writer - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0."]
pub type NisoW<'a, REG> = crate::BitWriter<'a, REG, Niso>;
impl<'a, REG> NisoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CAN FD frame format according to ISO11898-1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Niso::B0x0)
    }
    #[doc = "CAN FD frame format according to Bosch CAN FD Specification V1.0"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Niso::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Initialization"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuration change enable"]
    #[inline(always)]
    pub fn cce(&self) -> CceR {
        CceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time."]
    #[inline(always)]
    pub fn asm(&self) -> AsmR {
        AsmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock stop acknowledge"]
    #[inline(always)]
    pub fn csa(&self) -> CsaR {
        CsaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock stop request"]
    #[inline(always)]
    pub fn csr(&self) -> CsrR {
        CsrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time."]
    #[inline(always)]
    pub fn mon(&self) -> MonR {
        MonR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable automatic retransmission"]
    #[inline(always)]
    pub fn dar(&self) -> DarR {
        DarR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test mode enable"]
    #[inline(always)]
    pub fn test(&self) -> TestR {
        TestR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FD operation enable"]
    #[inline(always)]
    pub fn fdoe(&self) -> FdoeR {
        FdoeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FDCAN bit rate switching"]
    #[inline(always)]
    pub fn brse(&self) -> BrseR {
        BrseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Protocol exception handling disable"]
    #[inline(always)]
    pub fn pxhd(&self) -> PxhdR {
        PxhdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Edge filtering during bus integration"]
    #[inline(always)]
    pub fn efbi(&self) -> EfbiR {
        EfbiR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame."]
    #[inline(always)]
    pub fn txp(&self) -> TxpR {
        TxpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0."]
    #[inline(always)]
    pub fn niso(&self) -> NisoR {
        NisoR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_CCCR")
            .field("init", &self.init())
            .field("cce", &self.cce())
            .field("asm", &self.asm())
            .field("csa", &self.csa())
            .field("csr", &self.csr())
            .field("mon", &self.mon())
            .field("dar", &self.dar())
            .field("test", &self.test())
            .field("fdoe", &self.fdoe())
            .field("brse", &self.brse())
            .field("pxhd", &self.pxhd())
            .field("efbi", &self.efbi())
            .field("txp", &self.txp())
            .field("niso", &self.niso())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Initialization"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> InitW<FdcanCccrSpec> {
        InitW::new(self, 0)
    }
    #[doc = "Bit 1 - Configuration change enable"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CceW<FdcanCccrSpec> {
        CceW::new(self, 1)
    }
    #[doc = "Bit 2 - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time."]
    #[inline(always)]
    #[must_use]
    pub fn asm(&mut self) -> AsmW<FdcanCccrSpec> {
        AsmW::new(self, 2)
    }
    #[doc = "Bit 4 - Clock stop request"]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CsrW<FdcanCccrSpec> {
        CsrW::new(self, 4)
    }
    #[doc = "Bit 5 - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time."]
    #[inline(always)]
    #[must_use]
    pub fn mon(&mut self) -> MonW<FdcanCccrSpec> {
        MonW::new(self, 5)
    }
    #[doc = "Bit 6 - Disable automatic retransmission"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DarW<FdcanCccrSpec> {
        DarW::new(self, 6)
    }
    #[doc = "Bit 7 - Test mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn test(&mut self) -> TestW<FdcanCccrSpec> {
        TestW::new(self, 7)
    }
    #[doc = "Bit 8 - FD operation enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdoe(&mut self) -> FdoeW<FdcanCccrSpec> {
        FdoeW::new(self, 8)
    }
    #[doc = "Bit 9 - FDCAN bit rate switching"]
    #[inline(always)]
    #[must_use]
    pub fn brse(&mut self) -> BrseW<FdcanCccrSpec> {
        BrseW::new(self, 9)
    }
    #[doc = "Bit 12 - Protocol exception handling disable"]
    #[inline(always)]
    #[must_use]
    pub fn pxhd(&mut self) -> PxhdW<FdcanCccrSpec> {
        PxhdW::new(self, 12)
    }
    #[doc = "Bit 13 - Edge filtering during bus integration"]
    #[inline(always)]
    #[must_use]
    pub fn efbi(&mut self) -> EfbiW<FdcanCccrSpec> {
        EfbiW::new(self, 13)
    }
    #[doc = "Bit 14 - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame."]
    #[inline(always)]
    #[must_use]
    pub fn txp(&mut self) -> TxpW<FdcanCccrSpec> {
        TxpW::new(self, 14)
    }
    #[doc = "Bit 15 - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0."]
    #[inline(always)]
    #[must_use]
    pub fn niso(&mut self) -> NisoW<FdcanCccrSpec> {
        NisoW::new(self, 15)
    }
}
#[doc = "FDCAN CC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_cccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_cccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanCccrSpec;
impl crate::RegisterSpec for FdcanCccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_cccr::R`](R) reader structure"]
impl crate::Readable for FdcanCccrSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_cccr::W`](W) writer structure"]
impl crate::Writable for FdcanCccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_CCCR to value 0x01"]
impl crate::Resettable for FdcanCccrSpec {
    const RESET_VALUE: u32 = 0x01;
}

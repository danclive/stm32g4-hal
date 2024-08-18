#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Act {
    #[doc = "Synchronizing: node is synchronizing on CAN communication."]
    B_0X0 = 0x0,
    #[doc = "Idle: node is neither receiver nor transmitter."]
    B_0X1 = 0x01,
    #[doc = "Receiver: node is operating as receiver."]
    B_0X2 = 0x02,
    #[doc = "Transmitter: node is operating as transmitter."]
    B_0X3 = 0x03,
}
impl Act {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Act {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Act {
    #[inline(always)]
    fn from(val: u8) -> Act {
        Act::from_bits(val)
    }
}
impl From<Act> for u8 {
    #[inline(always)]
    fn from(val: Act) -> u8 {
        Act::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Anfe {
    #[doc = "Accept in Rx FIFO 0"]
    B_0X0 = 0x0,
    #[doc = "Accept in Rx FIFO 1"]
    B_0X1 = 0x01,
    #[doc = "Reject"]
    B_0X2 = 0x02,
    #[doc = "Reject"]
    B_0X3 = 0x03,
}
impl Anfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Anfe {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Anfe {
    #[inline(always)]
    fn from(val: u8) -> Anfe {
        Anfe::from_bits(val)
    }
}
impl From<Anfe> for u8 {
    #[inline(always)]
    fn from(val: Anfe) -> u8 {
        Anfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Anfs {
    #[doc = "Accept in Rx FIFO 0"]
    B_0X0 = 0x0,
    #[doc = "Accept in Rx FIFO 1"]
    B_0X1 = 0x01,
    #[doc = "Reject"]
    B_0X2 = 0x02,
    #[doc = "Reject"]
    B_0X3 = 0x03,
}
impl Anfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Anfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Anfs {
    #[inline(always)]
    fn from(val: u8) -> Anfs {
        Anfs::from_bits(val)
    }
}
impl From<Anfs> for u8 {
    #[inline(always)]
    fn from(val: Anfs) -> u8 {
        Anfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ar {
    #[doc = "No transmission request added"]
    B_0X0 = 0x0,
    #[doc = "Transmission requested added."]
    B_0X1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ar {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ar {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ar {
    #[inline(always)]
    fn from(val: u8) -> Ar {
        Ar::from_bits(val)
    }
}
impl From<Ar> for u8 {
    #[inline(always)]
    fn from(val: Ar) -> u8 {
        Ar::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ara {
    #[doc = "No access to reserved address occurred"]
    B_0X0 = 0x0,
    #[doc = "Access to reserved address occurred"]
    B_0X1 = 0x01,
}
impl Ara {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ara {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ara {
    #[inline(always)]
    fn from(val: u8) -> Ara {
        Ara::from_bits(val)
    }
}
impl From<Ara> for u8 {
    #[inline(always)]
    fn from(val: Ara) -> u8 {
        Ara::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Asm {
    #[doc = "Normal CAN operation"]
    B_0X0 = 0x0,
    #[doc = "Restricted operation Mode active"]
    B_0X1 = 0x01,
}
impl Asm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Asm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Asm {
    #[inline(always)]
    fn from(val: u8) -> Asm {
        Asm::from_bits(val)
    }
}
impl From<Asm> for u8 {
    #[inline(always)]
    fn from(val: Asm) -> u8 {
        Asm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Boe {
    #[doc = "Interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt enabled"]
    B_0X1 = 0x01,
}
impl Boe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Boe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Boe {
    #[inline(always)]
    fn from(val: u8) -> Boe {
        Boe::from_bits(val)
    }
}
impl From<Boe> for u8 {
    #[inline(always)]
    fn from(val: Boe) -> u8 {
        Boe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Brse {
    #[doc = "Bit rate switching for transmissions disabled"]
    B_0X0 = 0x0,
    #[doc = "Bit rate switching for transmissions enabled"]
    B_0X1 = 0x01,
}
impl Brse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Brse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Brse {
    #[inline(always)]
    fn from(val: u8) -> Brse {
        Brse::from_bits(val)
    }
}
impl From<Brse> for u8 {
    #[inline(always)]
    fn from(val: Brse) -> u8 {
        Brse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cce {
    #[doc = "The CPU has no write access to the protected configuration registers."]
    B_0X0 = 0x0,
    #[doc = "The CPU has write access to the protected configuration registers (while CCCR.INIT = 1)."]
    B_0X1 = 0x01,
}
impl Cce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cce {
    #[inline(always)]
    fn from(val: u8) -> Cce {
        Cce::from_bits(val)
    }
}
impl From<Cce> for u8 {
    #[inline(always)]
    fn from(val: Cce) -> u8 {
        Cce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cf {
    #[doc = "No transmit buffer cancellation"]
    B_0X0 = 0x0,
    #[doc = "Transmit buffer cancellation finished"]
    B_0X1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cf {
    #[inline(always)]
    fn from(val: u8) -> Cf {
        Cf::from_bits(val)
    }
}
impl From<Cf> for u8 {
    #[inline(always)]
    fn from(val: Cf) -> u8 {
        Cf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cfie {
    #[doc = "Cancellation finished interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Cancellation finished interrupt enabled"]
    B_0X1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cfie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfie {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfie {
    #[inline(always)]
    fn from(val: u8) -> Cfie {
        Cfie::from_bits(val)
    }
}
impl From<Cfie> for u8 {
    #[inline(always)]
    fn from(val: Cfie) -> u8 {
        Cfie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cr {
    #[doc = "No cancellation pending"]
    B_0X0 = 0x0,
    #[doc = "Cancellation pending"]
    B_0X1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cr {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cr {
    #[inline(always)]
    fn from(val: u8) -> Cr {
        Cr::from_bits(val)
    }
}
impl From<Cr> for u8 {
    #[inline(always)]
    fn from(val: Cr) -> u8 {
        Cr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Csa {
    #[doc = "No clock stop acknowledged"]
    B_0X0 = 0x0,
    #[doc = "FDCAN may be set in power down by stopping APB clock and kernel clock."]
    B_0X1 = 0x01,
}
impl Csa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csa {
    #[inline(always)]
    fn from(val: u8) -> Csa {
        Csa::from_bits(val)
    }
}
impl From<Csa> for u8 {
    #[inline(always)]
    fn from(val: Csa) -> u8 {
        Csa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Csr {
    #[doc = "No clock stop requested"]
    B_0X0 = 0x0,
    #[doc = "Clock stop requested. When clock stop is requested, first INIT and then CSA is set after all pending transfer requests have been completed and the CAN bus reached idle."]
    B_0X1 = 0x01,
}
impl Csr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csr {
    #[inline(always)]
    fn from(val: u8) -> Csr {
        Csr::from_bits(val)
    }
}
impl From<Csr> for u8 {
    #[inline(always)]
    fn from(val: Csr) -> u8 {
        Csr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dar {
    #[doc = "Automatic retransmission of messages not transmitted successfully enabled"]
    B_0X0 = 0x0,
    #[doc = "Automatic retransmission disabled"]
    B_0X1 = 0x01,
}
impl Dar {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dar {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dar {
    #[inline(always)]
    fn from(val: u8) -> Dar {
        Dar::from_bits(val)
    }
}
impl From<Dar> for u8 {
    #[inline(always)]
    fn from(val: Dar) -> u8 {
        Dar::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Efbi {
    #[doc = "Edge filtering disabled"]
    B_0X0 = 0x0,
    #[doc = "Two consecutive dominant tq required to detect an edge for hard synchronization"]
    B_0X1 = 0x01,
}
impl Efbi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Efbi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Efbi {
    #[inline(always)]
    fn from(val: u8) -> Efbi {
        Efbi::from_bits(val)
    }
}
impl From<Efbi> for u8 {
    #[inline(always)]
    fn from(val: Efbi) -> u8 {
        Efbi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Eff {
    #[doc = "Tx event FIFO not full"]
    B_0X0 = 0x0,
    #[doc = "Tx event FIFO full"]
    B_0X1 = 0x01,
}
impl Eff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eff {
    #[inline(always)]
    fn from(val: u8) -> Eff {
        Eff::from_bits(val)
    }
}
impl From<Eff> for u8 {
    #[inline(always)]
    fn from(val: Eff) -> u8 {
        Eff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Eint0 {
    #[doc = "Interrupt line fdcan_intr1_it disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt line fdcan_intr1_it enabled"]
    B_0X1 = 0x01,
}
impl Eint0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eint0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eint0 {
    #[inline(always)]
    fn from(val: u8) -> Eint0 {
        Eint0::from_bits(val)
    }
}
impl From<Eint0> for u8 {
    #[inline(always)]
    fn from(val: Eint0) -> u8 {
        Eint0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Eint1 {
    #[doc = "Interrupt line fdcan_intr0_it disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt line fdcan_intr0_it enabled"]
    B_0X1 = 0x01,
}
impl Eint1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eint1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eint1 {
    #[inline(always)]
    fn from(val: u8) -> Eint1 {
        Eint1::from_bits(val)
    }
}
impl From<Eint1> for u8 {
    #[inline(always)]
    fn from(val: Eint1) -> u8 {
        Eint1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Elo {
    #[doc = "CAN error logging counter did not overflow."]
    B_0X0 = 0x0,
    #[doc = "Overflow of CAN error logging counter occurred."]
    B_0X1 = 0x01,
}
impl Elo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Elo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Elo {
    #[inline(always)]
    fn from(val: u8) -> Elo {
        Elo::from_bits(val)
    }
}
impl From<Elo> for u8 {
    #[inline(always)]
    fn from(val: Elo) -> u8 {
        Elo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Eloe {
    #[doc = "Interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt enabled"]
    B_0X1 = 0x01,
}
impl Eloe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eloe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eloe {
    #[inline(always)]
    fn from(val: u8) -> Eloe {
        Eloe::from_bits(val)
    }
}
impl From<Eloe> for u8 {
    #[inline(always)]
    fn from(val: Eloe) -> u8 {
        Eloe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Epe {
    #[doc = "Interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt enabled"]
    B_0X1 = 0x01,
}
impl Epe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Epe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Epe {
    #[inline(always)]
    fn from(val: u8) -> Epe {
        Epe::from_bits(val)
    }
}
impl From<Epe> for u8 {
    #[inline(always)]
    fn from(val: Epe) -> u8 {
        Epe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Etoc {
    #[doc = "Timeout counter disabled"]
    B_0X0 = 0x0,
    #[doc = "Timeout counter enabled"]
    B_0X1 = 0x01,
}
impl Etoc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Etoc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Etoc {
    #[inline(always)]
    fn from(val: u8) -> Etoc {
        Etoc::from_bits(val)
    }
}
impl From<Etoc> for u8 {
    #[inline(always)]
    fn from(val: Etoc) -> u8 {
        Etoc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ewe {
    #[doc = "Interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt enabled"]
    B_0X1 = 0x01,
}
impl Ewe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ewe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ewe {
    #[inline(always)]
    fn from(val: u8) -> Ewe {
        Ewe::from_bits(val)
    }
}
impl From<Ewe> for u8 {
    #[inline(always)]
    fn from(val: Ewe) -> u8 {
        Ewe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum F0f {
    #[doc = "Rx FIFO 0 not full"]
    B_0X0 = 0x0,
    #[doc = "Rx FIFO 0 full"]
    B_0X1 = 0x01,
}
impl F0f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> F0f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for F0f {
    #[inline(always)]
    fn from(val: u8) -> F0f {
        F0f::from_bits(val)
    }
}
impl From<F0f> for u8 {
    #[inline(always)]
    fn from(val: F0f) -> u8 {
        F0f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum F1f {
    #[doc = "Rx FIFO 1 not full"]
    B_0X0 = 0x0,
    #[doc = "Rx FIFO 1 full"]
    B_0X1 = 0x01,
}
impl F1f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> F1f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for F1f {
    #[inline(always)]
    fn from(val: u8) -> F1f {
        F1f::from_bits(val)
    }
}
impl From<F1f> for u8 {
    #[inline(always)]
    fn from(val: F1f) -> u8 {
        F1f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FdcanIrBo {
    #[doc = "Bus_Off status unchanged"]
    B_0X0 = 0x0,
    #[doc = "Bus_Off status changed"]
    B_0X1 = 0x01,
}
impl FdcanIrBo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FdcanIrBo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FdcanIrBo {
    #[inline(always)]
    fn from(val: u8) -> FdcanIrBo {
        FdcanIrBo::from_bits(val)
    }
}
impl From<FdcanIrBo> for u8 {
    #[inline(always)]
    fn from(val: FdcanIrBo) -> u8 {
        FdcanIrBo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FdcanIrEp {
    #[doc = "Error_Passive status unchanged"]
    B_0X0 = 0x0,
    #[doc = "Error_Passive status changed"]
    B_0X1 = 0x01,
}
impl FdcanIrEp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FdcanIrEp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FdcanIrEp {
    #[inline(always)]
    fn from(val: u8) -> FdcanIrEp {
        FdcanIrEp::from_bits(val)
    }
}
impl From<FdcanIrEp> for u8 {
    #[inline(always)]
    fn from(val: FdcanIrEp) -> u8 {
        FdcanIrEp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FdcanIrEw {
    #[doc = "Error_Warning status unchanged"]
    B_0X0 = 0x0,
    #[doc = "Error_Warning status changed"]
    B_0X1 = 0x01,
}
impl FdcanIrEw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FdcanIrEw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FdcanIrEw {
    #[inline(always)]
    fn from(val: u8) -> FdcanIrEw {
        FdcanIrEw::from_bits(val)
    }
}
impl From<FdcanIrEw> for u8 {
    #[inline(always)]
    fn from(val: FdcanIrEw) -> u8 {
        FdcanIrEw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FdcanIrRf0l {
    #[doc = "No Rx FIFO 0 message lost"]
    B_0X0 = 0x0,
    #[doc = "Rx FIFO 0 message lost"]
    B_0X1 = 0x01,
}
impl FdcanIrRf0l {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FdcanIrRf0l {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FdcanIrRf0l {
    #[inline(always)]
    fn from(val: u8) -> FdcanIrRf0l {
        FdcanIrRf0l::from_bits(val)
    }
}
impl From<FdcanIrRf0l> for u8 {
    #[inline(always)]
    fn from(val: FdcanIrRf0l) -> u8 {
        FdcanIrRf0l::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FdcanIrRf1l {
    #[doc = "No Rx FIFO 1 message lost"]
    B_0X0 = 0x0,
    #[doc = "Rx FIFO 1 message lost"]
    B_0X1 = 0x01,
}
impl FdcanIrRf1l {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FdcanIrRf1l {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FdcanIrRf1l {
    #[inline(always)]
    fn from(val: u8) -> FdcanIrRf1l {
        FdcanIrRf1l::from_bits(val)
    }
}
impl From<FdcanIrRf1l> for u8 {
    #[inline(always)]
    fn from(val: FdcanIrRf1l) -> u8 {
        FdcanIrRf1l::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FdcanPsrBo {
    #[doc = "The FDCAN is not Bus_Off."]
    B_0X0 = 0x0,
    #[doc = "The FDCAN is in Bus_Off state."]
    B_0X1 = 0x01,
}
impl FdcanPsrBo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FdcanPsrBo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FdcanPsrBo {
    #[inline(always)]
    fn from(val: u8) -> FdcanPsrBo {
        FdcanPsrBo::from_bits(val)
    }
}
impl From<FdcanPsrBo> for u8 {
    #[inline(always)]
    fn from(val: FdcanPsrBo) -> u8 {
        FdcanPsrBo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FdcanPsrEp {
    #[doc = "The FDCAN is in the Error_Active state. It normally takes part in bus communication and sends an active error flag when an error has been detected."]
    B_0X0 = 0x0,
    #[doc = "The FDCAN is in the Error_Passive state."]
    B_0X1 = 0x01,
}
impl FdcanPsrEp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FdcanPsrEp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FdcanPsrEp {
    #[inline(always)]
    fn from(val: u8) -> FdcanPsrEp {
        FdcanPsrEp::from_bits(val)
    }
}
impl From<FdcanPsrEp> for u8 {
    #[inline(always)]
    fn from(val: FdcanPsrEp) -> u8 {
        FdcanPsrEp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FdcanPsrEw {
    #[doc = "Both error counters are below the Error_Warning limit of 96."]
    B_0X0 = 0x0,
    #[doc = "At least one of error counter has reached the Error_Warning limit of 96."]
    B_0X1 = 0x01,
}
impl FdcanPsrEw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FdcanPsrEw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FdcanPsrEw {
    #[inline(always)]
    fn from(val: u8) -> FdcanPsrEw {
        FdcanPsrEw::from_bits(val)
    }
}
impl From<FdcanPsrEw> for u8 {
    #[inline(always)]
    fn from(val: FdcanPsrEw) -> u8 {
        FdcanPsrEw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FdcanRxf0sRf0l {
    #[doc = "No Rx FIFO 0 message lost"]
    B_0X0 = 0x0,
    #[doc = "Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size 0"]
    B_0X1 = 0x01,
}
impl FdcanRxf0sRf0l {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FdcanRxf0sRf0l {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FdcanRxf0sRf0l {
    #[inline(always)]
    fn from(val: u8) -> FdcanRxf0sRf0l {
        FdcanRxf0sRf0l::from_bits(val)
    }
}
impl From<FdcanRxf0sRf0l> for u8 {
    #[inline(always)]
    fn from(val: FdcanRxf0sRf0l) -> u8 {
        FdcanRxf0sRf0l::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FdcanRxf1sRf1l {
    #[doc = "No Rx FIFO 1 message lost"]
    B_0X0 = 0x0,
    #[doc = "Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size 0"]
    B_0X1 = 0x01,
}
impl FdcanRxf1sRf1l {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FdcanRxf1sRf1l {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FdcanRxf1sRf1l {
    #[inline(always)]
    fn from(val: u8) -> FdcanRxf1sRf1l {
        FdcanRxf1sRf1l::from_bits(val)
    }
}
impl From<FdcanRxf1sRf1l> for u8 {
    #[inline(always)]
    fn from(val: FdcanRxf1sRf1l) -> u8 {
        FdcanRxf1sRf1l::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Fdoe {
    #[doc = "FD operation disabled"]
    B_0X0 = 0x0,
    #[doc = "FD operation enabled"]
    B_0X1 = 0x01,
}
impl Fdoe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fdoe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fdoe {
    #[inline(always)]
    fn from(val: u8) -> Fdoe {
        Fdoe::from_bits(val)
    }
}
impl From<Fdoe> for u8 {
    #[inline(always)]
    fn from(val: Fdoe) -> u8 {
        Fdoe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Flst {
    #[doc = "Standard filter list"]
    B_0X0 = 0x0,
    #[doc = "Extended filter list"]
    B_0X1 = 0x01,
}
impl Flst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flst {
    #[inline(always)]
    fn from(val: u8) -> Flst {
        Flst::from_bits(val)
    }
}
impl From<Flst> for u8 {
    #[inline(always)]
    fn from(val: Flst) -> u8 {
        Flst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hpm {
    #[doc = "No high-priority message received"]
    B_0X0 = 0x0,
    #[doc = "High-priority message received"]
    B_0X1 = 0x01,
}
impl Hpm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hpm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hpm {
    #[inline(always)]
    fn from(val: u8) -> Hpm {
        Hpm::from_bits(val)
    }
}
impl From<Hpm> for u8 {
    #[inline(always)]
    fn from(val: Hpm) -> u8 {
        Hpm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hpme {
    #[doc = "Interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt enabled"]
    B_0X1 = 0x01,
}
impl Hpme {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hpme {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hpme {
    #[inline(always)]
    fn from(val: u8) -> Hpme {
        Hpme::from_bits(val)
    }
}
impl From<Hpme> for u8 {
    #[inline(always)]
    fn from(val: Hpme) -> u8 {
        Hpme::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Init {
    #[doc = "Normal operation"]
    B_0X0 = 0x0,
    #[doc = "Initialization started"]
    B_0X1 = 0x01,
}
impl Init {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Init {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Init {
    #[inline(always)]
    fn from(val: u8) -> Init {
        Init::from_bits(val)
    }
}
impl From<Init> for u8 {
    #[inline(always)]
    fn from(val: Init) -> u8 {
        Init::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lbck {
    #[doc = "Reset value, Loop Back mode is disabled"]
    B_0X0 = 0x0,
    #[doc = "Loop Back mode is enabled (see Power down (Sleep mode))"]
    B_0X1 = 0x01,
}
impl Lbck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lbck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lbck {
    #[inline(always)]
    fn from(val: u8) -> Lbck {
        Lbck::from_bits(val)
    }
}
impl From<Lbck> for u8 {
    #[inline(always)]
    fn from(val: Lbck) -> u8 {
        Lbck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lec {
    #[doc = "No Error: No error occurred since LEC has been reset by successful reception or transmission."]
    B_0X0 = 0x0,
    #[doc = "Stuff Error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    B_0X1 = 0x01,
    #[doc = "Form Error: A fixed format part of a received frame has the wrong format."]
    B_0X2 = 0x02,
    #[doc = "AckError: The message transmitted by the FDCAN was not acknowledged by another node."]
    B_0X3 = 0x03,
    #[doc = "Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1), but the monitored bus value was dominant."]
    B_0X4 = 0x04,
    #[doc = "Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0), but the monitored bus value was recessive. During Bus_Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    B_0X5 = 0x05,
    #[doc = "CRCError: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data."]
    B_0X6 = 0x06,
    #[doc = "NoChange: Any read access to the Protocol status register re-initializes the LEC to 7. When the LEC shows the value 7, no CAN bus event was detected since the last CPU read access to the Protocol status register."]
    B_0X7 = 0x07,
}
impl Lec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lec {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lec {
    #[inline(always)]
    fn from(val: u8) -> Lec {
        Lec::from_bits(val)
    }
}
impl From<Lec> for u8 {
    #[inline(always)]
    fn from(val: Lec) -> u8 {
        Lec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lse {
    #[doc = "No extended message ID filter"]
    B_0X0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Lse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lse {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lse {
    #[inline(always)]
    fn from(val: u8) -> Lse {
        Lse::from_bits(val)
    }
}
impl From<Lse> for u8 {
    #[inline(always)]
    fn from(val: Lse) -> u8 {
        Lse::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Lss(pub u8);
impl Lss {
    #[doc = "No standard message ID filter"]
    pub const B_0X0: Self = Self(0x0);
}
impl Lss {
    pub const fn from_bits(val: u8) -> Lss {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Lss {
    #[inline(always)]
    fn from(val: u8) -> Lss {
        Lss::from_bits(val)
    }
}
impl From<Lss> for u8 {
    #[inline(always)]
    fn from(val: Lss) -> u8 {
        Lss::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mon {
    #[doc = "Bus monitoring mode disabled"]
    B_0X0 = 0x0,
    #[doc = "Bus monitoring mode enabled"]
    B_0X1 = 0x01,
}
impl Mon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mon {
    #[inline(always)]
    fn from(val: u8) -> Mon {
        Mon::from_bits(val)
    }
}
impl From<Mon> for u8 {
    #[inline(always)]
    fn from(val: Mon) -> u8 {
        Mon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mraf {
    #[doc = "No Message RAM access failure occurred"]
    B_0X0 = 0x0,
    #[doc = "Message RAM access failure occurred"]
    B_0X1 = 0x01,
}
impl Mraf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mraf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mraf {
    #[inline(always)]
    fn from(val: u8) -> Mraf {
        Mraf::from_bits(val)
    }
}
impl From<Mraf> for u8 {
    #[inline(always)]
    fn from(val: Mraf) -> u8 {
        Mraf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mrafe {
    #[doc = "Interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt enabled"]
    B_0X1 = 0x01,
}
impl Mrafe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mrafe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mrafe {
    #[inline(always)]
    fn from(val: u8) -> Mrafe {
        Mrafe::from_bits(val)
    }
}
impl From<Mrafe> for u8 {
    #[inline(always)]
    fn from(val: Mrafe) -> u8 {
        Mrafe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Msi {
    #[doc = "No FIFO selected"]
    B_0X0 = 0x0,
    #[doc = "FIFO overrun"]
    B_0X1 = 0x01,
    #[doc = "Message stored in FIFO 0"]
    B_0X2 = 0x02,
    #[doc = "Message stored in FIFO 1"]
    B_0X3 = 0x03,
}
impl Msi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Msi {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Msi {
    #[inline(always)]
    fn from(val: u8) -> Msi {
        Msi::from_bits(val)
    }
}
impl From<Msi> for u8 {
    #[inline(always)]
    fn from(val: Msi) -> u8 {
        Msi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Niso {
    #[doc = "CAN FD frame format according to ISO11898-1"]
    B_0X0 = 0x0,
    #[doc = "CAN FD frame format according to Bosch CAN FD Specification V1.0"]
    B_0X1 = 0x01,
}
impl Niso {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Niso {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Niso {
    #[inline(always)]
    fn from(val: u8) -> Niso {
        Niso::from_bits(val)
    }
}
impl From<Niso> for u8 {
    #[inline(always)]
    fn from(val: Niso) -> u8 {
        Niso::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pdiv {
    #[doc = "Divide by 1"]
    B_0X0 = 0x0,
    #[doc = "Divide by 2"]
    B_0X1 = 0x01,
    #[doc = "Divide by 4"]
    B_0X2 = 0x02,
    #[doc = "Divide by 6"]
    B_0X3 = 0x03,
    #[doc = "Divide by 8"]
    B_0X4 = 0x04,
    #[doc = "Divide by 10"]
    B_0X5 = 0x05,
    #[doc = "Divide by 12"]
    B_0X6 = 0x06,
    #[doc = "Divide by 14"]
    B_0X7 = 0x07,
    #[doc = "Divide by 16"]
    B_0X8 = 0x08,
    #[doc = "Divide by 18"]
    B_0X9 = 0x09,
    #[doc = "Divide by 20"]
    B_0XA = 0x0a,
    #[doc = "Divide by 22"]
    B_0XB = 0x0b,
    #[doc = "Divide by 24"]
    B_0XC = 0x0c,
    #[doc = "Divide by 26"]
    B_0XD = 0x0d,
    #[doc = "Divide by 28"]
    B_0XE = 0x0e,
    #[doc = "Divide by 30"]
    B_0XF = 0x0f,
}
impl Pdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdiv {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdiv {
    #[inline(always)]
    fn from(val: u8) -> Pdiv {
        Pdiv::from_bits(val)
    }
}
impl From<Pdiv> for u8 {
    #[inline(always)]
    fn from(val: Pdiv) -> u8 {
        Pdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pea {
    #[doc = "No protocol error in arbitration phase"]
    B_0X0 = 0x0,
    #[doc = "Protocol error in arbitration phase detected (PSR.LEC different from 0,7)"]
    B_0X1 = 0x01,
}
impl Pea {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pea {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pea {
    #[inline(always)]
    fn from(val: u8) -> Pea {
        Pea::from_bits(val)
    }
}
impl From<Pea> for u8 {
    #[inline(always)]
    fn from(val: Pea) -> u8 {
        Pea::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ped {
    #[doc = "No protocol error in data phase"]
    B_0X0 = 0x0,
    #[doc = "Protocol error in data phase detected (PSR.DLEC different from 0,7)"]
    B_0X1 = 0x01,
}
impl Ped {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ped {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ped {
    #[inline(always)]
    fn from(val: u8) -> Ped {
        Ped::from_bits(val)
    }
}
impl From<Ped> for u8 {
    #[inline(always)]
    fn from(val: Ped) -> u8 {
        Ped::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pxe {
    #[doc = "No protocol exception event occurred since last read access"]
    B_0X0 = 0x0,
    #[doc = "Protocol exception event occurred"]
    B_0X1 = 0x01,
}
impl Pxe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pxe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pxe {
    #[inline(always)]
    fn from(val: u8) -> Pxe {
        Pxe::from_bits(val)
    }
}
impl From<Pxe> for u8 {
    #[inline(always)]
    fn from(val: Pxe) -> u8 {
        Pxe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pxhd {
    #[doc = "Protocol exception handling enabled"]
    B_0X0 = 0x0,
    #[doc = "Protocol exception handling disabled"]
    B_0X1 = 0x01,
}
impl Pxhd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pxhd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pxhd {
    #[inline(always)]
    fn from(val: u8) -> Pxhd {
        Pxhd::from_bits(val)
    }
}
impl From<Pxhd> for u8 {
    #[inline(always)]
    fn from(val: Pxhd) -> u8 {
        Pxhd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rbrs {
    #[doc = "Last received FDCAN message did not have its BRS flag set."]
    B_0X0 = 0x0,
    #[doc = "Last received FDCAN message had its BRS flag set."]
    B_0X1 = 0x01,
}
impl Rbrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rbrs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rbrs {
    #[inline(always)]
    fn from(val: u8) -> Rbrs {
        Rbrs::from_bits(val)
    }
}
impl From<Rbrs> for u8 {
    #[inline(always)]
    fn from(val: Rbrs) -> u8 {
        Rbrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Redl {
    #[doc = "Since this bit was reset by the CPU, no FDCAN message has been received."]
    B_0X0 = 0x0,
    #[doc = "Message in FDCAN format with EDL flag set has been received."]
    B_0X1 = 0x01,
}
impl Redl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Redl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Redl {
    #[inline(always)]
    fn from(val: u8) -> Redl {
        Redl::from_bits(val)
    }
}
impl From<Redl> for u8 {
    #[inline(always)]
    fn from(val: Redl) -> u8 {
        Redl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Resi {
    #[doc = "Last received FDCAN message did not have its ESI flag set."]
    B_0X0 = 0x0,
    #[doc = "Last received FDCAN message had its ESI flag set."]
    B_0X1 = 0x01,
}
impl Resi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resi {
    #[inline(always)]
    fn from(val: u8) -> Resi {
        Resi::from_bits(val)
    }
}
impl From<Resi> for u8 {
    #[inline(always)]
    fn from(val: Resi) -> u8 {
        Resi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rf0f {
    #[doc = "Rx FIFO 0 not full"]
    B_0X0 = 0x0,
    #[doc = "Rx FIFO 0 full"]
    B_0X1 = 0x01,
}
impl Rf0f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rf0f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rf0f {
    #[inline(always)]
    fn from(val: u8) -> Rf0f {
        Rf0f::from_bits(val)
    }
}
impl From<Rf0f> for u8 {
    #[inline(always)]
    fn from(val: Rf0f) -> u8 {
        Rf0f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rf0fe {
    #[doc = "Interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt enabled"]
    B_0X1 = 0x01,
}
impl Rf0fe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rf0fe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rf0fe {
    #[inline(always)]
    fn from(val: u8) -> Rf0fe {
        Rf0fe::from_bits(val)
    }
}
impl From<Rf0fe> for u8 {
    #[inline(always)]
    fn from(val: Rf0fe) -> u8 {
        Rf0fe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rf0le {
    #[doc = "Interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt enabled"]
    B_0X1 = 0x01,
}
impl Rf0le {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rf0le {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rf0le {
    #[inline(always)]
    fn from(val: u8) -> Rf0le {
        Rf0le::from_bits(val)
    }
}
impl From<Rf0le> for u8 {
    #[inline(always)]
    fn from(val: Rf0le) -> u8 {
        Rf0le::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rf0n {
    #[doc = "No new message written to Rx FIFO 0"]
    B_0X0 = 0x0,
    #[doc = "New message written to Rx FIFO 0"]
    B_0X1 = 0x01,
}
impl Rf0n {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rf0n {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rf0n {
    #[inline(always)]
    fn from(val: u8) -> Rf0n {
        Rf0n::from_bits(val)
    }
}
impl From<Rf0n> for u8 {
    #[inline(always)]
    fn from(val: Rf0n) -> u8 {
        Rf0n::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rf0ne {
    #[doc = "Interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt enabled"]
    B_0X1 = 0x01,
}
impl Rf0ne {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rf0ne {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rf0ne {
    #[inline(always)]
    fn from(val: u8) -> Rf0ne {
        Rf0ne::from_bits(val)
    }
}
impl From<Rf0ne> for u8 {
    #[inline(always)]
    fn from(val: Rf0ne) -> u8 {
        Rf0ne::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rf1f {
    #[doc = "Rx FIFO 1 not full"]
    B_0X0 = 0x0,
    #[doc = "Rx FIFO 1 full"]
    B_0X1 = 0x01,
}
impl Rf1f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rf1f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rf1f {
    #[inline(always)]
    fn from(val: u8) -> Rf1f {
        Rf1f::from_bits(val)
    }
}
impl From<Rf1f> for u8 {
    #[inline(always)]
    fn from(val: Rf1f) -> u8 {
        Rf1f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rf1fe {
    #[doc = "Interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt enabled"]
    B_0X1 = 0x01,
}
impl Rf1fe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rf1fe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rf1fe {
    #[inline(always)]
    fn from(val: u8) -> Rf1fe {
        Rf1fe::from_bits(val)
    }
}
impl From<Rf1fe> for u8 {
    #[inline(always)]
    fn from(val: Rf1fe) -> u8 {
        Rf1fe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rf1le {
    #[doc = "Interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt enabled"]
    B_0X1 = 0x01,
}
impl Rf1le {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rf1le {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rf1le {
    #[inline(always)]
    fn from(val: u8) -> Rf1le {
        Rf1le::from_bits(val)
    }
}
impl From<Rf1le> for u8 {
    #[inline(always)]
    fn from(val: Rf1le) -> u8 {
        Rf1le::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rf1n {
    #[doc = "No new message written to Rx FIFO 1"]
    B_0X0 = 0x0,
    #[doc = "New message written to Rx FIFO 1"]
    B_0X1 = 0x01,
}
impl Rf1n {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rf1n {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rf1n {
    #[inline(always)]
    fn from(val: u8) -> Rf1n {
        Rf1n::from_bits(val)
    }
}
impl From<Rf1n> for u8 {
    #[inline(always)]
    fn from(val: Rf1n) -> u8 {
        Rf1n::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rf1ne {
    #[doc = "Interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt enabled"]
    B_0X1 = 0x01,
}
impl Rf1ne {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rf1ne {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rf1ne {
    #[inline(always)]
    fn from(val: u8) -> Rf1ne {
        Rf1ne::from_bits(val)
    }
}
impl From<Rf1ne> for u8 {
    #[inline(always)]
    fn from(val: Rf1ne) -> u8 {
        Rf1ne::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rp {
    #[doc = "The receive error counter is below the error passive level of 128."]
    B_0X0 = 0x0,
    #[doc = "The receive error counter has reached the error passive level of 128."]
    B_0X1 = 0x01,
}
impl Rp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rp {
    #[inline(always)]
    fn from(val: u8) -> Rp {
        Rp::from_bits(val)
    }
}
impl From<Rp> for u8 {
    #[inline(always)]
    fn from(val: Rp) -> u8 {
        Rp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rrfe {
    #[doc = "Filter remote frames with 29-bit standard IDs"]
    B_0X0 = 0x0,
    #[doc = "Reject all remote frames with 29-bit standard IDs"]
    B_0X1 = 0x01,
}
impl Rrfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rrfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rrfe {
    #[inline(always)]
    fn from(val: u8) -> Rrfe {
        Rrfe::from_bits(val)
    }
}
impl From<Rrfe> for u8 {
    #[inline(always)]
    fn from(val: Rrfe) -> u8 {
        Rrfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rrfs {
    #[doc = "Filter remote frames with 11-bit standard IDs"]
    B_0X0 = 0x0,
    #[doc = "Reject all remote frames with 11-bit standard IDs"]
    B_0X1 = 0x01,
}
impl Rrfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rrfs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rrfs {
    #[inline(always)]
    fn from(val: u8) -> Rrfs {
        Rrfs::from_bits(val)
    }
}
impl From<Rrfs> for u8 {
    #[inline(always)]
    fn from(val: Rrfs) -> u8 {
        Rrfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rx {
    #[doc = "The CAN bus is dominant (FDCANx_RX = 0)"]
    B_0X0 = 0x0,
    #[doc = "The CAN bus is recessive (FDCANx_RX = 1)"]
    B_0X1 = 0x01,
}
impl Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rx {
    #[inline(always)]
    fn from(val: u8) -> Rx {
        Rx::from_bits(val)
    }
}
impl From<Rx> for u8 {
    #[inline(always)]
    fn from(val: Rx) -> u8 {
        Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tc {
    #[doc = "No transmission completed"]
    B_0X0 = 0x0,
    #[doc = "Transmission completed"]
    B_0X1 = 0x01,
}
impl Tc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tc {
    #[inline(always)]
    fn from(val: u8) -> Tc {
        Tc::from_bits(val)
    }
}
impl From<Tc> for u8 {
    #[inline(always)]
    fn from(val: Tc) -> u8 {
        Tc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tce {
    #[doc = "Interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt enabled"]
    B_0X1 = 0x01,
}
impl Tce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tce {
    #[inline(always)]
    fn from(val: u8) -> Tce {
        Tce::from_bits(val)
    }
}
impl From<Tce> for u8 {
    #[inline(always)]
    fn from(val: Tce) -> u8 {
        Tce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tcf {
    #[doc = "No transmission cancellation finished"]
    B_0X0 = 0x0,
    #[doc = "Transmission cancellation finished"]
    B_0X1 = 0x01,
}
impl Tcf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcf {
    #[inline(always)]
    fn from(val: u8) -> Tcf {
        Tcf::from_bits(val)
    }
}
impl From<Tcf> for u8 {
    #[inline(always)]
    fn from(val: Tcf) -> u8 {
        Tcf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tcfe {
    #[doc = "Interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt enabled"]
    B_0X1 = 0x01,
}
impl Tcfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcfe {
    #[inline(always)]
    fn from(val: u8) -> Tcfe {
        Tcfe::from_bits(val)
    }
}
impl From<Tcfe> for u8 {
    #[inline(always)]
    fn from(val: Tcfe) -> u8 {
        Tcfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tdc {
    #[doc = "Transceiver delay compensation disabled"]
    B_0X0 = 0x0,
    #[doc = "Transceiver delay compensation enabled"]
    B_0X1 = 0x01,
}
impl Tdc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdc {
    #[inline(always)]
    fn from(val: u8) -> Tdc {
        Tdc::from_bits(val)
    }
}
impl From<Tdc> for u8 {
    #[inline(always)]
    fn from(val: Tdc) -> u8 {
        Tdc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Teff {
    #[doc = "Tx event FIFO Not full"]
    B_0X0 = 0x0,
    #[doc = "Tx event FIFO full"]
    B_0X1 = 0x01,
}
impl Teff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Teff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Teff {
    #[inline(always)]
    fn from(val: u8) -> Teff {
        Teff::from_bits(val)
    }
}
impl From<Teff> for u8 {
    #[inline(always)]
    fn from(val: Teff) -> u8 {
        Teff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Teffe {
    #[doc = "Interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt enabled"]
    B_0X1 = 0x01,
}
impl Teffe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Teffe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Teffe {
    #[inline(always)]
    fn from(val: u8) -> Teffe {
        Teffe::from_bits(val)
    }
}
impl From<Teffe> for u8 {
    #[inline(always)]
    fn from(val: Teffe) -> u8 {
        Teffe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tefl {
    #[doc = "No Tx event FIFO element lost"]
    B_0X0 = 0x0,
    #[doc = "Tx event FIFO element lost"]
    B_0X1 = 0x01,
}
impl Tefl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tefl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tefl {
    #[inline(always)]
    fn from(val: u8) -> Tefl {
        Tefl::from_bits(val)
    }
}
impl From<Tefl> for u8 {
    #[inline(always)]
    fn from(val: Tefl) -> u8 {
        Tefl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tefle {
    #[doc = "Interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt enabled"]
    B_0X1 = 0x01,
}
impl Tefle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tefle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tefle {
    #[inline(always)]
    fn from(val: u8) -> Tefle {
        Tefle::from_bits(val)
    }
}
impl From<Tefle> for u8 {
    #[inline(always)]
    fn from(val: Tefle) -> u8 {
        Tefle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tefn {
    #[doc = "Tx event FIFO unchanged"]
    B_0X0 = 0x0,
    #[doc = "Tx handler wrote Tx event FIFO element."]
    B_0X1 = 0x01,
}
impl Tefn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tefn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tefn {
    #[inline(always)]
    fn from(val: u8) -> Tefn {
        Tefn::from_bits(val)
    }
}
impl From<Tefn> for u8 {
    #[inline(always)]
    fn from(val: Tefn) -> u8 {
        Tefn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tefne {
    #[doc = "Interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt enabled"]
    B_0X1 = 0x01,
}
impl Tefne {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tefne {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tefne {
    #[inline(always)]
    fn from(val: u8) -> Tefne {
        Tefne::from_bits(val)
    }
}
impl From<Tefne> for u8 {
    #[inline(always)]
    fn from(val: Tefne) -> u8 {
        Tefne::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Test {
    #[doc = "Normal operation, register TEST holds reset values"]
    B_0X0 = 0x0,
    #[doc = "Test Mode, write access to register TEST enabled"]
    B_0X1 = 0x01,
}
impl Test {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Test {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Test {
    #[inline(always)]
    fn from(val: u8) -> Test {
        Test::from_bits(val)
    }
}
impl From<Test> for u8 {
    #[inline(always)]
    fn from(val: Test) -> u8 {
        Test::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tfe {
    #[doc = "Tx FIFO non-empty"]
    B_0X0 = 0x0,
    #[doc = "Tx FIFO empty"]
    B_0X1 = 0x01,
}
impl Tfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfe {
    #[inline(always)]
    fn from(val: u8) -> Tfe {
        Tfe::from_bits(val)
    }
}
impl From<Tfe> for u8 {
    #[inline(always)]
    fn from(val: Tfe) -> u8 {
        Tfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tfee {
    #[doc = "Interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt enabled"]
    B_0X1 = 0x01,
}
impl Tfee {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfee {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfee {
    #[inline(always)]
    fn from(val: u8) -> Tfee {
        Tfee::from_bits(val)
    }
}
impl From<Tfee> for u8 {
    #[inline(always)]
    fn from(val: Tfee) -> u8 {
        Tfee::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tfqf {
    #[doc = "Tx FIFO/queue not full"]
    B_0X0 = 0x0,
    #[doc = "Tx FIFO/queue full"]
    B_0X1 = 0x01,
}
impl Tfqf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfqf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfqf {
    #[inline(always)]
    fn from(val: u8) -> Tfqf {
        Tfqf::from_bits(val)
    }
}
impl From<Tfqf> for u8 {
    #[inline(always)]
    fn from(val: Tfqf) -> u8 {
        Tfqf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tfqm {
    #[doc = "Tx FIFO operation"]
    B_0X0 = 0x0,
    #[doc = "Tx queue operation."]
    B_0X1 = 0x01,
}
impl Tfqm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfqm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfqm {
    #[inline(always)]
    fn from(val: u8) -> Tfqm {
        Tfqm::from_bits(val)
    }
}
impl From<Tfqm> for u8 {
    #[inline(always)]
    fn from(val: Tfqm) -> u8 {
        Tfqm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tie {
    #[doc = "Transmission interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Transmission interrupt enable"]
    B_0X1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tie {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tie {
    #[inline(always)]
    fn from(val: u8) -> Tie {
        Tie::from_bits(val)
    }
}
impl From<Tie> for u8 {
    #[inline(always)]
    fn from(val: Tie) -> u8 {
        Tie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum To {
    #[doc = "No transmission occurred"]
    B_0X0 = 0x0,
    #[doc = "Transmission occurred"]
    B_0X1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl To {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> To {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for To {
    #[inline(always)]
    fn from(val: u8) -> To {
        To::from_bits(val)
    }
}
impl From<To> for u8 {
    #[inline(always)]
    fn from(val: To) -> u8 {
        To::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Too {
    #[doc = "No timeout"]
    B_0X0 = 0x0,
    #[doc = "Timeout reached"]
    B_0X1 = 0x01,
}
impl Too {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Too {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Too {
    #[inline(always)]
    fn from(val: u8) -> Too {
        Too::from_bits(val)
    }
}
impl From<Too> for u8 {
    #[inline(always)]
    fn from(val: Too) -> u8 {
        Too::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tooe {
    #[doc = "Interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt enabled"]
    B_0X1 = 0x01,
}
impl Tooe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tooe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tooe {
    #[inline(always)]
    fn from(val: u8) -> Tooe {
        Tooe::from_bits(val)
    }
}
impl From<Tooe> for u8 {
    #[inline(always)]
    fn from(val: Tooe) -> u8 {
        Tooe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tos {
    #[doc = "Continuous operation"]
    B_0X0 = 0x0,
    #[doc = "Timeout controlled by Tx event FIFO"]
    B_0X1 = 0x01,
    #[doc = "Timeout controlled by Rx FIFO 0"]
    B_0X2 = 0x02,
    #[doc = "Timeout controlled by Rx FIFO 1"]
    B_0X3 = 0x03,
}
impl Tos {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tos {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tos {
    #[inline(always)]
    fn from(val: u8) -> Tos {
        Tos::from_bits(val)
    }
}
impl From<Tos> for u8 {
    #[inline(always)]
    fn from(val: Tos) -> u8 {
        Tos::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Trp {
    #[doc = "No transmission request pending"]
    B_0X0 = 0x0,
    #[doc = "Transmission request pending"]
    B_0X1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Trp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trp {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trp {
    #[inline(always)]
    fn from(val: u8) -> Trp {
        Trp::from_bits(val)
    }
}
impl From<Trp> for u8 {
    #[inline(always)]
    fn from(val: Trp) -> u8 {
        Trp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tss {
    #[doc = "Timestamp counter value always 0x0000"]
    B_0X0 = 0x0,
    #[doc = "Timestamp counter value incremented according to TCP"]
    B_0X1 = 0x01,
    #[doc = "External timestamp counter from TIM3 value (tim3_cnt\\[0:15\\])"]
    B_0X2 = 0x02,
    #[doc = "Same as 00."]
    B_0X3 = 0x03,
}
impl Tss {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tss {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tss {
    #[inline(always)]
    fn from(val: u8) -> Tss {
        Tss::from_bits(val)
    }
}
impl From<Tss> for u8 {
    #[inline(always)]
    fn from(val: Tss) -> u8 {
        Tss::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tsw {
    #[doc = "No timestamp counter wrap-around"]
    B_0X0 = 0x0,
    #[doc = "Timestamp counter wrapped around"]
    B_0X1 = 0x01,
}
impl Tsw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsw {
    #[inline(always)]
    fn from(val: u8) -> Tsw {
        Tsw::from_bits(val)
    }
}
impl From<Tsw> for u8 {
    #[inline(always)]
    fn from(val: Tsw) -> u8 {
        Tsw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tswe {
    #[doc = "Interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt enabled"]
    B_0X1 = 0x01,
}
impl Tswe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tswe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tswe {
    #[inline(always)]
    fn from(val: u8) -> Tswe {
        Tswe::from_bits(val)
    }
}
impl From<Tswe> for u8 {
    #[inline(always)]
    fn from(val: Tswe) -> u8 {
        Tswe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tx {
    #[doc = "Reset value, FDCANx_TX TX is controlled by the CAN core, updated at the end of the CAN bit time"]
    B_0X0 = 0x0,
    #[doc = "Sample point can be monitored at pin FDCANx_TX"]
    B_0X1 = 0x01,
    #[doc = "Dominant (0) level at pin FDCANx_TX"]
    B_0X2 = 0x02,
    #[doc = "Recessive (1) at pin FDCANx_TX"]
    B_0X3 = 0x03,
}
impl Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tx {
    #[inline(always)]
    fn from(val: u8) -> Tx {
        Tx::from_bits(val)
    }
}
impl From<Tx> for u8 {
    #[inline(always)]
    fn from(val: Tx) -> u8 {
        Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Txp {
    #[doc = "disabled"]
    B_0X0 = 0x0,
    #[doc = "enabled"]
    B_0X1 = 0x01,
}
impl Txp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txp {
    #[inline(always)]
    fn from(val: u8) -> Txp {
        Txp::from_bits(val)
    }
}
impl From<Txp> for u8 {
    #[inline(always)]
    fn from(val: Txp) -> u8 {
        Txp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Wdi {
    #[doc = "No message RAM watchdog event occurred"]
    B_0X0 = 0x0,
    #[doc = "Message RAM watchdog event due to missing READY"]
    B_0X1 = 0x01,
}
impl Wdi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdi {
    #[inline(always)]
    fn from(val: u8) -> Wdi {
        Wdi::from_bits(val)
    }
}
impl From<Wdi> for u8 {
    #[inline(always)]
    fn from(val: Wdi) -> u8 {
        Wdi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Wdie {
    #[doc = "Interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "Interrupt enabled"]
    B_0X1 = 0x01,
}
impl Wdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdie {
    #[inline(always)]
    fn from(val: u8) -> Wdie {
        Wdie::from_bits(val)
    }
}
impl From<Wdie> for u8 {
    #[inline(always)]
    fn from(val: Wdie) -> u8 {
        Wdie::to_bits(val)
    }
}

#[doc = "Register `FDCAN_PSR` reader"]
pub type R = crate::R<FdcanPsrSpec>;
#[doc = "Register `FDCAN_PSR` writer"]
pub type W = crate::W<FdcanPsrSpec>;
#[doc = "Last error code The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read.\n\nValue on reset: 7"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lec {
    #[doc = "0: No Error: No error occurred since LEC has been reset by successful reception or transmission."]
    B0x0 = 0,
    #[doc = "1: Stuff Error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    B0x1 = 1,
    #[doc = "2: Form Error: A fixed format part of a received frame has the wrong format."]
    B0x2 = 2,
    #[doc = "3: AckError: The message transmitted by the FDCAN was not acknowledged by another node."]
    B0x3 = 3,
    #[doc = "4: Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1), but the monitored bus value was dominant."]
    B0x4 = 4,
    #[doc = "5: Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0), but the monitored bus value was recessive. During Bus_Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    B0x5 = 5,
    #[doc = "6: CRCError: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data."]
    B0x6 = 6,
    #[doc = "7: NoChange: Any read access to the Protocol status register re-initializes the LEC to 7. When the LEC shows the value 7, no CAN bus event was detected since the last CPU read access to the Protocol status register."]
    B0x7 = 7,
}
impl From<Lec> for u8 {
    #[inline(always)]
    fn from(variant: Lec) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lec {
    type Ux = u8;
}
impl crate::IsEnum for Lec {}
#[doc = "Field `LEC` reader - Last error code The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read."]
pub type LecR = crate::FieldReader<Lec>;
impl LecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lec {
        match self.bits {
            0 => Lec::B0x0,
            1 => Lec::B0x1,
            2 => Lec::B0x2,
            3 => Lec::B0x3,
            4 => Lec::B0x4,
            5 => Lec::B0x5,
            6 => Lec::B0x6,
            7 => Lec::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "No Error: No error occurred since LEC has been reset by successful reception or transmission."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lec::B0x0
    }
    #[doc = "Stuff Error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lec::B0x1
    }
    #[doc = "Form Error: A fixed format part of a received frame has the wrong format."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Lec::B0x2
    }
    #[doc = "AckError: The message transmitted by the FDCAN was not acknowledged by another node."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Lec::B0x3
    }
    #[doc = "Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1), but the monitored bus value was dominant."]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Lec::B0x4
    }
    #[doc = "Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0), but the monitored bus value was recessive. During Bus_Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Lec::B0x5
    }
    #[doc = "CRCError: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data."]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Lec::B0x6
    }
    #[doc = "NoChange: Any read access to the Protocol status register re-initializes the LEC to 7. When the LEC shows the value 7, no CAN bus event was detected since the last CPU read access to the Protocol status register."]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Lec::B0x7
    }
}
#[doc = "Field `LEC` writer - Last error code The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read."]
pub type LecW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lec, crate::Safe>;
impl<'a, REG> LecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Error: No error occurred since LEC has been reset by successful reception or transmission."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::B0x0)
    }
    #[doc = "Stuff Error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::B0x1)
    }
    #[doc = "Form Error: A fixed format part of a received frame has the wrong format."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::B0x2)
    }
    #[doc = "AckError: The message transmitted by the FDCAN was not acknowledged by another node."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::B0x3)
    }
    #[doc = "Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1), but the monitored bus value was dominant."]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::B0x4)
    }
    #[doc = "Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0), but the monitored bus value was recessive. During Bus_Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::B0x5)
    }
    #[doc = "CRCError: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data."]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::B0x6)
    }
    #[doc = "NoChange: Any read access to the Protocol status register re-initializes the LEC to 7. When the LEC shows the value 7, no CAN bus event was detected since the last CPU read access to the Protocol status register."]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::B0x7)
    }
}
#[doc = "Activity Monitors the modules CAN communication state.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Act {
    #[doc = "0: Synchronizing: node is synchronizing on CAN communication."]
    B0x0 = 0,
    #[doc = "1: Idle: node is neither receiver nor transmitter."]
    B0x1 = 1,
    #[doc = "2: Receiver: node is operating as receiver."]
    B0x2 = 2,
    #[doc = "3: Transmitter: node is operating as transmitter."]
    B0x3 = 3,
}
impl From<Act> for u8 {
    #[inline(always)]
    fn from(variant: Act) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Act {
    type Ux = u8;
}
impl crate::IsEnum for Act {}
#[doc = "Field `ACT` reader - Activity Monitors the modules CAN communication state."]
pub type ActR = crate::FieldReader<Act>;
impl ActR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Act {
        match self.bits {
            0 => Act::B0x0,
            1 => Act::B0x1,
            2 => Act::B0x2,
            3 => Act::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Synchronizing: node is synchronizing on CAN communication."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Act::B0x0
    }
    #[doc = "Idle: node is neither receiver nor transmitter."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Act::B0x1
    }
    #[doc = "Receiver: node is operating as receiver."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Act::B0x2
    }
    #[doc = "Transmitter: node is operating as transmitter."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Act::B0x3
    }
}
#[doc = "Error passive\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep {
    #[doc = "0: The FDCAN is in the Error_Active state. It normally takes part in bus communication and sends an active error flag when an error has been detected."]
    B0x0 = 0,
    #[doc = "1: The FDCAN is in the Error_Passive state."]
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
    #[doc = "The FDCAN is in the Error_Active state. It normally takes part in bus communication and sends an active error flag when an error has been detected."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ep::B0x0
    }
    #[doc = "The FDCAN is in the Error_Passive state."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ep::B0x1
    }
}
#[doc = "Warning Sstatus\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ew {
    #[doc = "0: Both error counters are below the Error_Warning limit of 96."]
    B0x0 = 0,
    #[doc = "1: At least one of error counter has reached the Error_Warning limit of 96."]
    B0x1 = 1,
}
impl From<Ew> for bool {
    #[inline(always)]
    fn from(variant: Ew) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EW` reader - Warning Sstatus"]
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
    #[doc = "Both error counters are below the Error_Warning limit of 96."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ew::B0x0
    }
    #[doc = "At least one of error counter has reached the Error_Warning limit of 96."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ew::B0x1
    }
}
#[doc = "Bus_Off status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bo {
    #[doc = "0: The FDCAN is not Bus_Off."]
    B0x0 = 0,
    #[doc = "1: The FDCAN is in Bus_Off state."]
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
    #[doc = "The FDCAN is not Bus_Off."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bo::B0x0
    }
    #[doc = "The FDCAN is in Bus_Off state."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bo::B0x1
    }
}
#[doc = "Field `DLEC` reader - Data last error code Type of last error that occurred in the data phase of a FDCAN format frame with its BRS flag set. Coding is the same as for LEC. This field is cleared to 0 when a FDCAN format frame with its BRS flag set has been transferred (reception or transmission) without error. Access type is RS: set on read."]
pub type DlecR = crate::FieldReader;
#[doc = "Field `DLEC` writer - Data last error code Type of last error that occurred in the data phase of a FDCAN format frame with its BRS flag set. Coding is the same as for LEC. This field is cleared to 0 when a FDCAN format frame with its BRS flag set has been transferred (reception or transmission) without error. Access type is RS: set on read."]
pub type DlecW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "ESI flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resi {
    #[doc = "0: Last received FDCAN message did not have its ESI flag set."]
    B0x0 = 0,
    #[doc = "1: Last received FDCAN message had its ESI flag set."]
    B0x1 = 1,
}
impl From<Resi> for bool {
    #[inline(always)]
    fn from(variant: Resi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESI` reader - ESI flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
pub type ResiR = crate::BitReader<Resi>;
impl ResiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resi {
        match self.bits {
            false => Resi::B0x0,
            true => Resi::B0x1,
        }
    }
    #[doc = "Last received FDCAN message did not have its ESI flag set."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Resi::B0x0
    }
    #[doc = "Last received FDCAN message had its ESI flag set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Resi::B0x1
    }
}
#[doc = "Field `RESI` writer - ESI flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
pub type ResiW<'a, REG> = crate::BitWriter<'a, REG, Resi>;
impl<'a, REG> ResiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Last received FDCAN message did not have its ESI flag set."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Resi::B0x0)
    }
    #[doc = "Last received FDCAN message had its ESI flag set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Resi::B0x1)
    }
}
#[doc = "BRS flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rbrs {
    #[doc = "0: Last received FDCAN message did not have its BRS flag set."]
    B0x0 = 0,
    #[doc = "1: Last received FDCAN message had its BRS flag set."]
    B0x1 = 1,
}
impl From<Rbrs> for bool {
    #[inline(always)]
    fn from(variant: Rbrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBRS` reader - BRS flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
pub type RbrsR = crate::BitReader<Rbrs>;
impl RbrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rbrs {
        match self.bits {
            false => Rbrs::B0x0,
            true => Rbrs::B0x1,
        }
    }
    #[doc = "Last received FDCAN message did not have its BRS flag set."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rbrs::B0x0
    }
    #[doc = "Last received FDCAN message had its BRS flag set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rbrs::B0x1
    }
}
#[doc = "Field `RBRS` writer - BRS flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
pub type RbrsW<'a, REG> = crate::BitWriter<'a, REG, Rbrs>;
impl<'a, REG> RbrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Last received FDCAN message did not have its BRS flag set."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rbrs::B0x0)
    }
    #[doc = "Last received FDCAN message had its BRS flag set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rbrs::B0x1)
    }
}
#[doc = "Received FDCAN message This bit is set independent of acceptance filtering. Access type is RX: reset on read.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Redl {
    #[doc = "0: Since this bit was reset by the CPU, no FDCAN message has been received."]
    B0x0 = 0,
    #[doc = "1: Message in FDCAN format with EDL flag set has been received."]
    B0x1 = 1,
}
impl From<Redl> for bool {
    #[inline(always)]
    fn from(variant: Redl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REDL` reader - Received FDCAN message This bit is set independent of acceptance filtering. Access type is RX: reset on read."]
pub type RedlR = crate::BitReader<Redl>;
impl RedlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Redl {
        match self.bits {
            false => Redl::B0x0,
            true => Redl::B0x1,
        }
    }
    #[doc = "Since this bit was reset by the CPU, no FDCAN message has been received."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Redl::B0x0
    }
    #[doc = "Message in FDCAN format with EDL flag set has been received."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Redl::B0x1
    }
}
#[doc = "Field `REDL` writer - Received FDCAN message This bit is set independent of acceptance filtering. Access type is RX: reset on read."]
pub type RedlW<'a, REG> = crate::BitWriter<'a, REG, Redl>;
impl<'a, REG> RedlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Since this bit was reset by the CPU, no FDCAN message has been received."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Redl::B0x0)
    }
    #[doc = "Message in FDCAN format with EDL flag set has been received."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Redl::B0x1)
    }
}
#[doc = "Protocol exception event\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pxe {
    #[doc = "0: No protocol exception event occurred since last read access"]
    B0x0 = 0,
    #[doc = "1: Protocol exception event occurred"]
    B0x1 = 1,
}
impl From<Pxe> for bool {
    #[inline(always)]
    fn from(variant: Pxe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PXE` reader - Protocol exception event"]
pub type PxeR = crate::BitReader<Pxe>;
impl PxeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pxe {
        match self.bits {
            false => Pxe::B0x0,
            true => Pxe::B0x1,
        }
    }
    #[doc = "No protocol exception event occurred since last read access"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pxe::B0x0
    }
    #[doc = "Protocol exception event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pxe::B0x1
    }
}
#[doc = "Field `PXE` writer - Protocol exception event"]
pub type PxeW<'a, REG> = crate::BitWriter<'a, REG, Pxe>;
impl<'a, REG> PxeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No protocol exception event occurred since last read access"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pxe::B0x0)
    }
    #[doc = "Protocol exception event occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pxe::B0x1)
    }
}
#[doc = "Field `TDCV` reader - Transmitter delay compensation value Position of the secondary sample point, defined by the sum of the measured delay from FDCAN_TX to FDCAN_RX and TDCR.TDCO. The SSP position is, in the data phase, the number of minimum time quanta (mtq) between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq."]
pub type TdcvR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Last error code The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read."]
    #[inline(always)]
    pub fn lec(&self) -> LecR {
        LecR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Activity Monitors the modules CAN communication state."]
    #[inline(always)]
    pub fn act(&self) -> ActR {
        ActR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Error passive"]
    #[inline(always)]
    pub fn ep(&self) -> EpR {
        EpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Warning Sstatus"]
    #[inline(always)]
    pub fn ew(&self) -> EwR {
        EwR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus_Off status"]
    #[inline(always)]
    pub fn bo(&self) -> BoR {
        BoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Data last error code Type of last error that occurred in the data phase of a FDCAN format frame with its BRS flag set. Coding is the same as for LEC. This field is cleared to 0 when a FDCAN format frame with its BRS flag set has been transferred (reception or transmission) without error. Access type is RS: set on read."]
    #[inline(always)]
    pub fn dlec(&self) -> DlecR {
        DlecR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - ESI flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn resi(&self) -> ResiR {
        ResiR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BRS flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn rbrs(&self) -> RbrsR {
        RbrsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Received FDCAN message This bit is set independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn redl(&self) -> RedlR {
        RedlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protocol exception event"]
    #[inline(always)]
    pub fn pxe(&self) -> PxeR {
        PxeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Transmitter delay compensation value Position of the secondary sample point, defined by the sum of the measured delay from FDCAN_TX to FDCAN_RX and TDCR.TDCO. The SSP position is, in the data phase, the number of minimum time quanta (mtq) between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq."]
    #[inline(always)]
    pub fn tdcv(&self) -> TdcvR {
        TdcvR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_PSR")
            .field("lec", &self.lec())
            .field("act", &self.act())
            .field("ep", &self.ep())
            .field("ew", &self.ew())
            .field("bo", &self.bo())
            .field("dlec", &self.dlec())
            .field("resi", &self.resi())
            .field("rbrs", &self.rbrs())
            .field("redl", &self.redl())
            .field("pxe", &self.pxe())
            .field("tdcv", &self.tdcv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Last error code The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read."]
    #[inline(always)]
    pub fn lec(&mut self) -> LecW<'_, FdcanPsrSpec> {
        LecW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Data last error code Type of last error that occurred in the data phase of a FDCAN format frame with its BRS flag set. Coding is the same as for LEC. This field is cleared to 0 when a FDCAN format frame with its BRS flag set has been transferred (reception or transmission) without error. Access type is RS: set on read."]
    #[inline(always)]
    pub fn dlec(&mut self) -> DlecW<'_, FdcanPsrSpec> {
        DlecW::new(self, 8)
    }
    #[doc = "Bit 11 - ESI flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn resi(&mut self) -> ResiW<'_, FdcanPsrSpec> {
        ResiW::new(self, 11)
    }
    #[doc = "Bit 12 - BRS flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn rbrs(&mut self) -> RbrsW<'_, FdcanPsrSpec> {
        RbrsW::new(self, 12)
    }
    #[doc = "Bit 13 - Received FDCAN message This bit is set independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn redl(&mut self) -> RedlW<'_, FdcanPsrSpec> {
        RedlW::new(self, 13)
    }
    #[doc = "Bit 14 - Protocol exception event"]
    #[inline(always)]
    pub fn pxe(&mut self) -> PxeW<'_, FdcanPsrSpec> {
        PxeW::new(self, 14)
    }
}
#[doc = "FDCAN protocol status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_psr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_psr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanPsrSpec;
impl crate::RegisterSpec for FdcanPsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_psr::R`](R) reader structure"]
impl crate::Readable for FdcanPsrSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_psr::W`](W) writer structure"]
impl crate::Writable for FdcanPsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_PSR to value 0x0707"]
impl crate::Resettable for FdcanPsrSpec {
    const RESET_VALUE: u32 = 0x0707;
}

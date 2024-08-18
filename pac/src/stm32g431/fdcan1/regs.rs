#[doc = "FDCAN CC control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanCccr(pub u32);
impl FdcanCccr {
    #[doc = "Initialization"]
    #[inline(always)]
    pub const fn init(&self) -> super::vals::Init {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Init::from_bits(val as u8)
    }
    #[doc = "Initialization"]
    #[inline(always)]
    pub fn set_init(&mut self, val: super::vals::Init) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Configuration change enable"]
    #[inline(always)]
    pub const fn cce(&self) -> super::vals::Cce {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Cce::from_bits(val as u8)
    }
    #[doc = "Configuration change enable"]
    #[inline(always)]
    pub fn set_cce(&mut self, val: super::vals::Cce) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time."]
    #[inline(always)]
    pub const fn asm(&self) -> super::vals::Asm {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Asm::from_bits(val as u8)
    }
    #[doc = "ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time."]
    #[inline(always)]
    pub fn set_asm(&mut self, val: super::vals::Asm) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Clock stop acknowledge"]
    #[inline(always)]
    pub const fn csa(&self) -> super::vals::Csa {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Csa::from_bits(val as u8)
    }
    #[doc = "Clock stop acknowledge"]
    #[inline(always)]
    pub fn set_csa(&mut self, val: super::vals::Csa) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Clock stop request"]
    #[inline(always)]
    pub const fn csr(&self) -> super::vals::Csr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Csr::from_bits(val as u8)
    }
    #[doc = "Clock stop request"]
    #[inline(always)]
    pub fn set_csr(&mut self, val: super::vals::Csr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time."]
    #[inline(always)]
    pub const fn mon(&self) -> super::vals::Mon {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Mon::from_bits(val as u8)
    }
    #[doc = "Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time."]
    #[inline(always)]
    pub fn set_mon(&mut self, val: super::vals::Mon) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Disable automatic retransmission"]
    #[inline(always)]
    pub const fn dar(&self) -> super::vals::Dar {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dar::from_bits(val as u8)
    }
    #[doc = "Disable automatic retransmission"]
    #[inline(always)]
    pub fn set_dar(&mut self, val: super::vals::Dar) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Test mode enable"]
    #[inline(always)]
    pub const fn test(&self) -> super::vals::Test {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Test::from_bits(val as u8)
    }
    #[doc = "Test mode enable"]
    #[inline(always)]
    pub fn set_test(&mut self, val: super::vals::Test) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "FD operation enable"]
    #[inline(always)]
    pub const fn fdoe(&self) -> super::vals::Fdoe {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Fdoe::from_bits(val as u8)
    }
    #[doc = "FD operation enable"]
    #[inline(always)]
    pub fn set_fdoe(&mut self, val: super::vals::Fdoe) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "FDCAN bit rate switching"]
    #[inline(always)]
    pub const fn brse(&self) -> super::vals::Brse {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Brse::from_bits(val as u8)
    }
    #[doc = "FDCAN bit rate switching"]
    #[inline(always)]
    pub fn set_brse(&mut self, val: super::vals::Brse) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Protocol exception handling disable"]
    #[inline(always)]
    pub const fn pxhd(&self) -> super::vals::Pxhd {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pxhd::from_bits(val as u8)
    }
    #[doc = "Protocol exception handling disable"]
    #[inline(always)]
    pub fn set_pxhd(&mut self, val: super::vals::Pxhd) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Edge filtering during bus integration"]
    #[inline(always)]
    pub const fn efbi(&self) -> super::vals::Efbi {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Efbi::from_bits(val as u8)
    }
    #[doc = "Edge filtering during bus integration"]
    #[inline(always)]
    pub fn set_efbi(&mut self, val: super::vals::Efbi) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame."]
    #[inline(always)]
    pub const fn txp(&self) -> super::vals::Txp {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Txp::from_bits(val as u8)
    }
    #[doc = "If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame."]
    #[inline(always)]
    pub fn set_txp(&mut self, val: super::vals::Txp) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0."]
    #[inline(always)]
    pub const fn niso(&self) -> super::vals::Niso {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Niso::from_bits(val as u8)
    }
    #[doc = "Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0."]
    #[inline(always)]
    pub fn set_niso(&mut self, val: super::vals::Niso) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for FdcanCccr {
    #[inline(always)]
    fn default() -> FdcanCccr {
        FdcanCccr(0)
    }
}
#[doc = "FDCAN CFG clock divider register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanCkdiv(pub u32);
impl FdcanCkdiv {
    #[doc = "input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub const fn pdiv(&self) -> super::vals::Pdiv {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pdiv::from_bits(val as u8)
    }
    #[doc = "input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn set_pdiv(&mut self, val: super::vals::Pdiv) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for FdcanCkdiv {
    #[inline(always)]
    fn default() -> FdcanCkdiv {
        FdcanCkdiv(0)
    }
}
#[doc = "FDCAN core release register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanCrel(pub u32);
impl FdcanCrel {
    #[doc = "18"]
    #[inline(always)]
    pub const fn day(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "18"]
    #[inline(always)]
    pub fn set_day(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "12"]
    #[inline(always)]
    pub const fn mon(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "12"]
    #[inline(always)]
    pub fn set_mon(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "4"]
    #[inline(always)]
    pub const fn year(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn set_year(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "1"]
    #[inline(always)]
    pub const fn substep(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn set_substep(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "2"]
    #[inline(always)]
    pub const fn step(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn set_step(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "3"]
    #[inline(always)]
    pub const fn rel(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "3"]
    #[inline(always)]
    pub fn set_rel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for FdcanCrel {
    #[inline(always)]
    fn default() -> FdcanCrel {
        FdcanCrel(0)
    }
}
#[doc = "FDCAN data bit timing and prescaler register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanDbtp(pub u32);
impl FdcanDbtp {
    #[doc = "Synchronization jump width Must always be smaller than DTSEG2, valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1: t<sub>SJW</sub> = (DSJW + 1) x tq."]
    #[inline(always)]
    pub const fn dsjw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Synchronization jump width Must always be smaller than DTSEG2, valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1: t<sub>SJW</sub> = (DSJW + 1) x tq."]
    #[inline(always)]
    pub fn set_dsjw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Data time segment after sample point Valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1, i.e. t<sub>BS2</sub> = (DTSEG2 + 1) x tq."]
    #[inline(always)]
    pub const fn dtseg2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Data time segment after sample point Valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1, i.e. t<sub>BS2</sub> = (DTSEG2 + 1) x tq."]
    #[inline(always)]
    pub fn set_dtseg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Data time segment before sample point Valid values are 0 to 31. The value used by the hardware is the one programmed, incremented by 1, i.e. t<sub>BS1</sub> = (DTSEG1 + 1) x tq."]
    #[inline(always)]
    pub const fn dtseg1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Data time segment before sample point Valid values are 0 to 31. The value used by the hardware is the one programmed, incremented by 1, i.e. t<sub>BS1</sub> = (DTSEG1 + 1) x tq."]
    #[inline(always)]
    pub fn set_dtseg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Data bit rate prescaler The value by which the oscillator frequency is divided to generate the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are 0 to 31. The hardware interpreters this value as the value programmed plus 1."]
    #[inline(always)]
    pub const fn dbrp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Data bit rate prescaler The value by which the oscillator frequency is divided to generate the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are 0 to 31. The hardware interpreters this value as the value programmed plus 1."]
    #[inline(always)]
    pub fn set_dbrp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Transceiver delay compensation"]
    #[inline(always)]
    pub const fn tdc(&self) -> super::vals::Tdc {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Tdc::from_bits(val as u8)
    }
    #[doc = "Transceiver delay compensation"]
    #[inline(always)]
    pub fn set_tdc(&mut self, val: super::vals::Tdc) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for FdcanDbtp {
    #[inline(always)]
    fn default() -> FdcanDbtp {
        FdcanDbtp(0)
    }
}
#[doc = "FDCAN error counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanEcr(pub u32);
impl FdcanEcr {
    #[doc = "Transmit error counter Actual state of the transmit error counter, values between 0 and 255. When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented."]
    #[inline(always)]
    pub const fn tec(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit error counter Actual state of the transmit error counter, values between 0 and 255. When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented."]
    #[inline(always)]
    pub fn set_tec(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive error counter Actual state of the receive error counter, values between 0 and 127."]
    #[inline(always)]
    pub const fn rec(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Receive error counter Actual state of the receive error counter, values between 0 and 127."]
    #[inline(always)]
    pub fn set_rec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Receive error passive"]
    #[inline(always)]
    pub const fn rp(&self) -> super::vals::Rp {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Rp::from_bits(val as u8)
    }
    #[doc = "Receive error passive"]
    #[inline(always)]
    pub fn set_rp(&mut self, val: super::vals::Rp) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\\[ELO\\]. Access type is RX: reset on read."]
    #[inline(always)]
    pub const fn cel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\\[ELO\\]. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn set_cel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for FdcanEcr {
    #[inline(always)]
    fn default() -> FdcanEcr {
        FdcanEcr(0)
    }
}
#[doc = "FDCAN endian register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanEndn(pub u32);
impl FdcanEndn {
    #[doc = "Endianness test value The endianness test value is 0x8765 4321."]
    #[inline(always)]
    pub const fn etv(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Endianness test value The endianness test value is 0x8765 4321."]
    #[inline(always)]
    pub fn set_etv(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FdcanEndn {
    #[inline(always)]
    fn default() -> FdcanEndn {
        FdcanEndn(0)
    }
}
#[doc = "FDCAN high-priority message status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanHpms(pub u32);
impl FdcanHpms {
    #[doc = "Buffer index Index of Rx FIFO element to which the message was stored. Only valid when MSI\\[1\\] = 1."]
    #[inline(always)]
    pub const fn bidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Buffer index Index of Rx FIFO element to which the message was stored. Only valid when MSI\\[1\\] = 1."]
    #[inline(always)]
    pub fn set_bidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Message storage indicator"]
    #[inline(always)]
    pub const fn msi(&self) -> super::vals::Msi {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Msi::from_bits(val as u8)
    }
    #[doc = "Message storage indicator"]
    #[inline(always)]
    pub fn set_msi(&mut self, val: super::vals::Msi) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Filter index Index of matching filter element. Range is 0 to RXGFC\\[LSS\\] - 1 or RXGFC\\[LSE\\] - 1."]
    #[inline(always)]
    pub const fn fidx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Filter index Index of matching filter element. Range is 0 to RXGFC\\[LSS\\] - 1 or RXGFC\\[LSE\\] - 1."]
    #[inline(always)]
    pub fn set_fidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Filter list Indicates the filter list of the matching filter element."]
    #[inline(always)]
    pub const fn flst(&self) -> super::vals::Flst {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Flst::from_bits(val as u8)
    }
    #[doc = "Filter list Indicates the filter list of the matching filter element."]
    #[inline(always)]
    pub fn set_flst(&mut self, val: super::vals::Flst) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for FdcanHpms {
    #[inline(always)]
    fn default() -> FdcanHpms {
        FdcanHpms(0)
    }
}
#[doc = "FDCAN interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanIe(pub u32);
impl FdcanIe {
    #[doc = "Rx FIFO 0 new message interrupt enable"]
    #[inline(always)]
    pub const fn rf0ne(&self) -> super::vals::Rf0ne {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Rf0ne::from_bits(val as u8)
    }
    #[doc = "Rx FIFO 0 new message interrupt enable"]
    #[inline(always)]
    pub fn set_rf0ne(&mut self, val: super::vals::Rf0ne) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Rx FIFO 0 full interrupt enable"]
    #[inline(always)]
    pub const fn rf0fe(&self) -> super::vals::Rf0fe {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Rf0fe::from_bits(val as u8)
    }
    #[doc = "Rx FIFO 0 full interrupt enable"]
    #[inline(always)]
    pub fn set_rf0fe(&mut self, val: super::vals::Rf0fe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Rx FIFO 0 message lost interrupt enable"]
    #[inline(always)]
    pub const fn rf0le(&self) -> super::vals::Rf0le {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Rf0le::from_bits(val as u8)
    }
    #[doc = "Rx FIFO 0 message lost interrupt enable"]
    #[inline(always)]
    pub fn set_rf0le(&mut self, val: super::vals::Rf0le) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Rx FIFO 1 new message interrupt enable"]
    #[inline(always)]
    pub const fn rf1ne(&self) -> super::vals::Rf1ne {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Rf1ne::from_bits(val as u8)
    }
    #[doc = "Rx FIFO 1 new message interrupt enable"]
    #[inline(always)]
    pub fn set_rf1ne(&mut self, val: super::vals::Rf1ne) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Rx FIFO 1 full interrupt enable"]
    #[inline(always)]
    pub const fn rf1fe(&self) -> super::vals::Rf1fe {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Rf1fe::from_bits(val as u8)
    }
    #[doc = "Rx FIFO 1 full interrupt enable"]
    #[inline(always)]
    pub fn set_rf1fe(&mut self, val: super::vals::Rf1fe) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Rx FIFO 1 message lost interrupt enable"]
    #[inline(always)]
    pub const fn rf1le(&self) -> super::vals::Rf1le {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Rf1le::from_bits(val as u8)
    }
    #[doc = "Rx FIFO 1 message lost interrupt enable"]
    #[inline(always)]
    pub fn set_rf1le(&mut self, val: super::vals::Rf1le) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "High-priority message interrupt enable"]
    #[inline(always)]
    pub const fn hpme(&self) -> super::vals::Hpme {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Hpme::from_bits(val as u8)
    }
    #[doc = "High-priority message interrupt enable"]
    #[inline(always)]
    pub fn set_hpme(&mut self, val: super::vals::Hpme) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmission completed interrupt enable"]
    #[inline(always)]
    pub const fn tce(&self) -> super::vals::Tce {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Tce::from_bits(val as u8)
    }
    #[doc = "Transmission completed interrupt enable"]
    #[inline(always)]
    pub fn set_tce(&mut self, val: super::vals::Tce) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Transmission cancellation finished interrupt enable"]
    #[inline(always)]
    pub const fn tcfe(&self) -> super::vals::Tcfe {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Tcfe::from_bits(val as u8)
    }
    #[doc = "Transmission cancellation finished interrupt enable"]
    #[inline(always)]
    pub fn set_tcfe(&mut self, val: super::vals::Tcfe) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Tx FIFO empty interrupt enable"]
    #[inline(always)]
    pub const fn tfee(&self) -> super::vals::Tfee {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Tfee::from_bits(val as u8)
    }
    #[doc = "Tx FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn set_tfee(&mut self, val: super::vals::Tfee) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Tx event FIFO new entry interrupt enable"]
    #[inline(always)]
    pub const fn tefne(&self) -> super::vals::Tefne {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Tefne::from_bits(val as u8)
    }
    #[doc = "Tx event FIFO new entry interrupt enable"]
    #[inline(always)]
    pub fn set_tefne(&mut self, val: super::vals::Tefne) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Tx event FIFO full interrupt enable"]
    #[inline(always)]
    pub const fn teffe(&self) -> super::vals::Teffe {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Teffe::from_bits(val as u8)
    }
    #[doc = "Tx event FIFO full interrupt enable"]
    #[inline(always)]
    pub fn set_teffe(&mut self, val: super::vals::Teffe) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Tx event FIFO element lost interrupt enable"]
    #[inline(always)]
    pub const fn tefle(&self) -> super::vals::Tefle {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Tefle::from_bits(val as u8)
    }
    #[doc = "Tx event FIFO element lost interrupt enable"]
    #[inline(always)]
    pub fn set_tefle(&mut self, val: super::vals::Tefle) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Timestamp wraparound interrupt enable"]
    #[inline(always)]
    pub const fn tswe(&self) -> super::vals::Tswe {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Tswe::from_bits(val as u8)
    }
    #[doc = "Timestamp wraparound interrupt enable"]
    #[inline(always)]
    pub fn set_tswe(&mut self, val: super::vals::Tswe) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Message RAM access failure interrupt enable"]
    #[inline(always)]
    pub const fn mrafe(&self) -> super::vals::Mrafe {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Mrafe::from_bits(val as u8)
    }
    #[doc = "Message RAM access failure interrupt enable"]
    #[inline(always)]
    pub fn set_mrafe(&mut self, val: super::vals::Mrafe) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Timeout occurred interrupt enable"]
    #[inline(always)]
    pub const fn tooe(&self) -> super::vals::Tooe {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Tooe::from_bits(val as u8)
    }
    #[doc = "Timeout occurred interrupt enable"]
    #[inline(always)]
    pub fn set_tooe(&mut self, val: super::vals::Tooe) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Error logging overflow interrupt enable"]
    #[inline(always)]
    pub const fn eloe(&self) -> super::vals::Eloe {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Eloe::from_bits(val as u8)
    }
    #[doc = "Error logging overflow interrupt enable"]
    #[inline(always)]
    pub fn set_eloe(&mut self, val: super::vals::Eloe) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Error passive interrupt enable"]
    #[inline(always)]
    pub const fn epe(&self) -> super::vals::Epe {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Epe::from_bits(val as u8)
    }
    #[doc = "Error passive interrupt enable"]
    #[inline(always)]
    pub fn set_epe(&mut self, val: super::vals::Epe) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Warning status interrupt enable"]
    #[inline(always)]
    pub const fn ewe(&self) -> super::vals::Ewe {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Ewe::from_bits(val as u8)
    }
    #[doc = "Warning status interrupt enable"]
    #[inline(always)]
    pub fn set_ewe(&mut self, val: super::vals::Ewe) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Bus_Off status"]
    #[inline(always)]
    pub const fn boe(&self) -> super::vals::Boe {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Boe::from_bits(val as u8)
    }
    #[doc = "Bus_Off status"]
    #[inline(always)]
    pub fn set_boe(&mut self, val: super::vals::Boe) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Watchdog interrupt enable"]
    #[inline(always)]
    pub const fn wdie(&self) -> super::vals::Wdie {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Wdie::from_bits(val as u8)
    }
    #[doc = "Watchdog interrupt enable"]
    #[inline(always)]
    pub fn set_wdie(&mut self, val: super::vals::Wdie) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Protocol error in arbitration phase enable"]
    #[inline(always)]
    pub const fn peae(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol error in arbitration phase enable"]
    #[inline(always)]
    pub fn set_peae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Protocol error in data phase enable"]
    #[inline(always)]
    pub const fn pede(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol error in data phase enable"]
    #[inline(always)]
    pub fn set_pede(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Access to reserved address enable"]
    #[inline(always)]
    pub const fn arae(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Access to reserved address enable"]
    #[inline(always)]
    pub fn set_arae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for FdcanIe {
    #[inline(always)]
    fn default() -> FdcanIe {
        FdcanIe(0)
    }
}
#[doc = "FDCAN interrupt line enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanIle(pub u32);
impl FdcanIle {
    #[doc = "Enable interrupt line 0"]
    #[inline(always)]
    pub const fn eint0(&self) -> super::vals::Eint0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Eint0::from_bits(val as u8)
    }
    #[doc = "Enable interrupt line 0"]
    #[inline(always)]
    pub fn set_eint0(&mut self, val: super::vals::Eint0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable interrupt line 1"]
    #[inline(always)]
    pub const fn eint1(&self) -> super::vals::Eint1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Eint1::from_bits(val as u8)
    }
    #[doc = "Enable interrupt line 1"]
    #[inline(always)]
    pub fn set_eint1(&mut self, val: super::vals::Eint1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for FdcanIle {
    #[inline(always)]
    fn default() -> FdcanIle {
        FdcanIle(0)
    }
}
#[doc = "FDCAN interrupt line select register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanIls(pub u32);
impl FdcanIls {
    #[doc = "RX FIFO bit grouping the following interruption RF0LL: Rx FIFO 0 message lost interrupt line RF0FL: Rx FIFO 0 full interrupt line RF0NL: Rx FIFO 0 new message interrupt line"]
    #[inline(always)]
    pub const fn rxfifo0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO bit grouping the following interruption RF0LL: Rx FIFO 0 message lost interrupt line RF0FL: Rx FIFO 0 full interrupt line RF0NL: Rx FIFO 0 new message interrupt line"]
    #[inline(always)]
    pub fn set_rxfifo0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX FIFO bit grouping the following interruption RF1LL: Rx FIFO 1 message lost interrupt line RF1FL: Rx FIFO 1 full interrupt line RF1NL: Rx FIFO 1 new message interrupt line"]
    #[inline(always)]
    pub const fn rxfifo1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO bit grouping the following interruption RF1LL: Rx FIFO 1 message lost interrupt line RF1FL: Rx FIFO 1 full interrupt line RF1NL: Rx FIFO 1 new message interrupt line"]
    #[inline(always)]
    pub fn set_rxfifo1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Status message bit grouping the following interruption TCFL: Transmission cancellation finished interrupt line TCL: Transmission completed interrupt line HPML: High-priority message interrupt line"]
    #[inline(always)]
    pub const fn smsg(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Status message bit grouping the following interruption TCFL: Transmission cancellation finished interrupt line TCL: Transmission completed interrupt line HPML: High-priority message interrupt line"]
    #[inline(always)]
    pub fn set_smsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Tx FIFO ERROR grouping the following interruption TEFLL: Tx event FIFO element lost interrupt line TEFFL: Tx event FIFO full interrupt line TEFNL: Tx event FIFO new entry interrupt line TFEL: Tx FIFO empty interrupt line"]
    #[inline(always)]
    pub const fn tferr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO ERROR grouping the following interruption TEFLL: Tx event FIFO element lost interrupt line TEFFL: Tx event FIFO full interrupt line TEFNL: Tx event FIFO new entry interrupt line TFEL: Tx FIFO empty interrupt line"]
    #[inline(always)]
    pub fn set_tferr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt regrouping the following interruption TOOL: Timeout occurred interrupt line MRAFL: Message RAM access failure interrupt line TSWL: Timestamp wraparound interrupt line"]
    #[inline(always)]
    pub const fn misc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt regrouping the following interruption TOOL: Timeout occurred interrupt line MRAFL: Message RAM access failure interrupt line TSWL: Timestamp wraparound interrupt line"]
    #[inline(always)]
    pub fn set_misc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Bit and line error grouping the following interruption EPL Error passive interrupt line ELOL: Error logging overflow interrupt line"]
    #[inline(always)]
    pub const fn berr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Bit and line error grouping the following interruption EPL Error passive interrupt line ELOL: Error logging overflow interrupt line"]
    #[inline(always)]
    pub fn set_berr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Protocol error grouping the following interruption ARAL: Access to reserved address line PEDL: Protocol error in data phase line PEAL: Protocol error in arbitration phase line WDIL: Watchdog interrupt line BOL: Bus_Off status EWL: Warning status interrupt line"]
    #[inline(always)]
    pub const fn perr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol error grouping the following interruption ARAL: Access to reserved address line PEDL: Protocol error in data phase line PEAL: Protocol error in arbitration phase line WDIL: Watchdog interrupt line BOL: Bus_Off status EWL: Warning status interrupt line"]
    #[inline(always)]
    pub fn set_perr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for FdcanIls {
    #[inline(always)]
    fn default() -> FdcanIls {
        FdcanIls(0)
    }
}
#[doc = "FDCAN interrupt register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanIr(pub u32);
impl FdcanIr {
    #[doc = "Rx FIFO 0 new message"]
    #[inline(always)]
    pub const fn rf0n(&self) -> super::vals::Rf0n {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Rf0n::from_bits(val as u8)
    }
    #[doc = "Rx FIFO 0 new message"]
    #[inline(always)]
    pub fn set_rf0n(&mut self, val: super::vals::Rf0n) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Rx FIFO 0 full"]
    #[inline(always)]
    pub const fn rf0f(&self) -> super::vals::Rf0f {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Rf0f::from_bits(val as u8)
    }
    #[doc = "Rx FIFO 0 full"]
    #[inline(always)]
    pub fn set_rf0f(&mut self, val: super::vals::Rf0f) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Rx FIFO 0 message lost"]
    #[inline(always)]
    pub const fn rf0l(&self) -> super::vals::FdcanIrRf0l {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::FdcanIrRf0l::from_bits(val as u8)
    }
    #[doc = "Rx FIFO 0 message lost"]
    #[inline(always)]
    pub fn set_rf0l(&mut self, val: super::vals::FdcanIrRf0l) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Rx FIFO 1 new message"]
    #[inline(always)]
    pub const fn rf1n(&self) -> super::vals::Rf1n {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Rf1n::from_bits(val as u8)
    }
    #[doc = "Rx FIFO 1 new message"]
    #[inline(always)]
    pub fn set_rf1n(&mut self, val: super::vals::Rf1n) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Rx FIFO 1 full"]
    #[inline(always)]
    pub const fn rf1f(&self) -> super::vals::Rf1f {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Rf1f::from_bits(val as u8)
    }
    #[doc = "Rx FIFO 1 full"]
    #[inline(always)]
    pub fn set_rf1f(&mut self, val: super::vals::Rf1f) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Rx FIFO 1 message lost"]
    #[inline(always)]
    pub const fn rf1l(&self) -> super::vals::FdcanIrRf1l {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::FdcanIrRf1l::from_bits(val as u8)
    }
    #[doc = "Rx FIFO 1 message lost"]
    #[inline(always)]
    pub fn set_rf1l(&mut self, val: super::vals::FdcanIrRf1l) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "High-priority message"]
    #[inline(always)]
    pub const fn hpm(&self) -> super::vals::Hpm {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Hpm::from_bits(val as u8)
    }
    #[doc = "High-priority message"]
    #[inline(always)]
    pub fn set_hpm(&mut self, val: super::vals::Hpm) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmission completed"]
    #[inline(always)]
    pub const fn tc(&self) -> super::vals::Tc {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Tc::from_bits(val as u8)
    }
    #[doc = "Transmission completed"]
    #[inline(always)]
    pub fn set_tc(&mut self, val: super::vals::Tc) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Transmission cancellation finished"]
    #[inline(always)]
    pub const fn tcf(&self) -> super::vals::Tcf {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Tcf::from_bits(val as u8)
    }
    #[doc = "Transmission cancellation finished"]
    #[inline(always)]
    pub fn set_tcf(&mut self, val: super::vals::Tcf) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Tx FIFO empty"]
    #[inline(always)]
    pub const fn tfe(&self) -> super::vals::Tfe {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Tfe::from_bits(val as u8)
    }
    #[doc = "Tx FIFO empty"]
    #[inline(always)]
    pub fn set_tfe(&mut self, val: super::vals::Tfe) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Tx event FIFO New Entry"]
    #[inline(always)]
    pub const fn tefn(&self) -> super::vals::Tefn {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Tefn::from_bits(val as u8)
    }
    #[doc = "Tx event FIFO New Entry"]
    #[inline(always)]
    pub fn set_tefn(&mut self, val: super::vals::Tefn) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Tx event FIFO full"]
    #[inline(always)]
    pub const fn teff(&self) -> super::vals::Teff {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Teff::from_bits(val as u8)
    }
    #[doc = "Tx event FIFO full"]
    #[inline(always)]
    pub fn set_teff(&mut self, val: super::vals::Teff) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Tx event FIFO element lost"]
    #[inline(always)]
    pub const fn tefl(&self) -> super::vals::Tefl {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Tefl::from_bits(val as u8)
    }
    #[doc = "Tx event FIFO element lost"]
    #[inline(always)]
    pub fn set_tefl(&mut self, val: super::vals::Tefl) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Timestamp wraparound"]
    #[inline(always)]
    pub const fn tsw(&self) -> super::vals::Tsw {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Tsw::from_bits(val as u8)
    }
    #[doc = "Timestamp wraparound"]
    #[inline(always)]
    pub fn set_tsw(&mut self, val: super::vals::Tsw) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see Restricted operation mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM."]
    #[inline(always)]
    pub const fn mraf(&self) -> super::vals::Mraf {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Mraf::from_bits(val as u8)
    }
    #[doc = "Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see Restricted operation mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM."]
    #[inline(always)]
    pub fn set_mraf(&mut self, val: super::vals::Mraf) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Timeout occurred"]
    #[inline(always)]
    pub const fn too(&self) -> super::vals::Too {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Too::from_bits(val as u8)
    }
    #[doc = "Timeout occurred"]
    #[inline(always)]
    pub fn set_too(&mut self, val: super::vals::Too) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Error logging overflow"]
    #[inline(always)]
    pub const fn elo(&self) -> super::vals::Elo {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Elo::from_bits(val as u8)
    }
    #[doc = "Error logging overflow"]
    #[inline(always)]
    pub fn set_elo(&mut self, val: super::vals::Elo) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Error passive"]
    #[inline(always)]
    pub const fn ep(&self) -> super::vals::FdcanIrEp {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::FdcanIrEp::from_bits(val as u8)
    }
    #[doc = "Error passive"]
    #[inline(always)]
    pub fn set_ep(&mut self, val: super::vals::FdcanIrEp) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Warning status"]
    #[inline(always)]
    pub const fn ew(&self) -> super::vals::FdcanIrEw {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::FdcanIrEw::from_bits(val as u8)
    }
    #[doc = "Warning status"]
    #[inline(always)]
    pub fn set_ew(&mut self, val: super::vals::FdcanIrEw) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Bus_Off status"]
    #[inline(always)]
    pub const fn bo(&self) -> super::vals::FdcanIrBo {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::FdcanIrBo::from_bits(val as u8)
    }
    #[doc = "Bus_Off status"]
    #[inline(always)]
    pub fn set_bo(&mut self, val: super::vals::FdcanIrBo) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Watchdog interrupt"]
    #[inline(always)]
    pub const fn wdi(&self) -> super::vals::Wdi {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Wdi::from_bits(val as u8)
    }
    #[doc = "Watchdog interrupt"]
    #[inline(always)]
    pub fn set_wdi(&mut self, val: super::vals::Wdi) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Protocol error in arbitration phase (nominal bit time is used)"]
    #[inline(always)]
    pub const fn pea(&self) -> super::vals::Pea {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pea::from_bits(val as u8)
    }
    #[doc = "Protocol error in arbitration phase (nominal bit time is used)"]
    #[inline(always)]
    pub fn set_pea(&mut self, val: super::vals::Pea) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Protocol error in data phase (data bit time is used)"]
    #[inline(always)]
    pub const fn ped(&self) -> super::vals::Ped {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Ped::from_bits(val as u8)
    }
    #[doc = "Protocol error in data phase (data bit time is used)"]
    #[inline(always)]
    pub fn set_ped(&mut self, val: super::vals::Ped) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Access to reserved address"]
    #[inline(always)]
    pub const fn ara(&self) -> super::vals::Ara {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Ara::from_bits(val as u8)
    }
    #[doc = "Access to reserved address"]
    #[inline(always)]
    pub fn set_ara(&mut self, val: super::vals::Ara) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for FdcanIr {
    #[inline(always)]
    fn default() -> FdcanIr {
        FdcanIr(0)
    }
}
#[doc = "FDCAN nominal bit timing and prescaler register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanNbtp(pub u32);
impl FdcanNbtp {
    #[doc = "Nominal time segment after sample point Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub const fn ntseg2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Nominal time segment after sample point Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub fn set_ntseg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Nominal time segment before sample point Valid values are 0 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub const fn ntseg1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Nominal time segment before sample point Valid values are 0 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn set_ntseg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Bit rate prescaler Value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub const fn nbrp(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "Bit rate prescaler Value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn set_nbrp(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
    #[doc = "Nominal (re)synchronization jump width Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that the used value is the one programmed incremented by one. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub const fn nsjw(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Nominal (re)synchronization jump width Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that the used value is the one programmed incremented by one. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn set_nsjw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for FdcanNbtp {
    #[inline(always)]
    fn default() -> FdcanNbtp {
        FdcanNbtp(0)
    }
}
#[doc = "FDCAN protocol status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanPsr(pub u32);
impl FdcanPsr {
    #[doc = "Last error code The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read."]
    #[inline(always)]
    pub const fn lec(&self) -> super::vals::Lec {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Lec::from_bits(val as u8)
    }
    #[doc = "Last error code The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read."]
    #[inline(always)]
    pub fn set_lec(&mut self, val: super::vals::Lec) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Activity Monitors the modules CAN communication state."]
    #[inline(always)]
    pub const fn act(&self) -> super::vals::Act {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::Act::from_bits(val as u8)
    }
    #[doc = "Activity Monitors the modules CAN communication state."]
    #[inline(always)]
    pub fn set_act(&mut self, val: super::vals::Act) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Error passive"]
    #[inline(always)]
    pub const fn ep(&self) -> super::vals::FdcanPsrEp {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::FdcanPsrEp::from_bits(val as u8)
    }
    #[doc = "Error passive"]
    #[inline(always)]
    pub fn set_ep(&mut self, val: super::vals::FdcanPsrEp) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Warning Sstatus"]
    #[inline(always)]
    pub const fn ew(&self) -> super::vals::FdcanPsrEw {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::FdcanPsrEw::from_bits(val as u8)
    }
    #[doc = "Warning Sstatus"]
    #[inline(always)]
    pub fn set_ew(&mut self, val: super::vals::FdcanPsrEw) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Bus_Off status"]
    #[inline(always)]
    pub const fn bo(&self) -> super::vals::FdcanPsrBo {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::FdcanPsrBo::from_bits(val as u8)
    }
    #[doc = "Bus_Off status"]
    #[inline(always)]
    pub fn set_bo(&mut self, val: super::vals::FdcanPsrBo) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Data last error code Type of last error that occurred in the data phase of a FDCAN format frame with its BRS flag set. Coding is the same as for LEC. This field is cleared to 0 when a FDCAN format frame with its BRS flag set has been transferred (reception or transmission) without error. Access type is RS: set on read."]
    #[inline(always)]
    pub const fn dlec(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Data last error code Type of last error that occurred in the data phase of a FDCAN format frame with its BRS flag set. Coding is the same as for LEC. This field is cleared to 0 when a FDCAN format frame with its BRS flag set has been transferred (reception or transmission) without error. Access type is RS: set on read."]
    #[inline(always)]
    pub fn set_dlec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "ESI flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    pub const fn resi(&self) -> super::vals::Resi {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Resi::from_bits(val as u8)
    }
    #[doc = "ESI flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn set_resi(&mut self, val: super::vals::Resi) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "BRS flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    pub const fn rbrs(&self) -> super::vals::Rbrs {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Rbrs::from_bits(val as u8)
    }
    #[doc = "BRS flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn set_rbrs(&mut self, val: super::vals::Rbrs) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Received FDCAN message This bit is set independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    pub const fn redl(&self) -> super::vals::Redl {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Redl::from_bits(val as u8)
    }
    #[doc = "Received FDCAN message This bit is set independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn set_redl(&mut self, val: super::vals::Redl) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Protocol exception event"]
    #[inline(always)]
    pub const fn pxe(&self) -> super::vals::Pxe {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pxe::from_bits(val as u8)
    }
    #[doc = "Protocol exception event"]
    #[inline(always)]
    pub fn set_pxe(&mut self, val: super::vals::Pxe) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Transmitter delay compensation value Position of the secondary sample point, defined by the sum of the measured delay from FDCAN_TX to FDCAN_RX and TDCR.TDCO. The SSP position is, in the data phase, the number of minimum time quanta (mtq) between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq."]
    #[inline(always)]
    pub const fn tdcv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmitter delay compensation value Position of the secondary sample point, defined by the sum of the measured delay from FDCAN_TX to FDCAN_RX and TDCR.TDCO. The SSP position is, in the data phase, the number of minimum time quanta (mtq) between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq."]
    #[inline(always)]
    pub fn set_tdcv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for FdcanPsr {
    #[inline(always)]
    fn default() -> FdcanPsr {
        FdcanPsr(0)
    }
}
#[doc = "FDCAN RAM watchdog register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanRwd(pub u32);
impl FdcanRwd {
    #[doc = "Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of FDCAN_CCCR register are set to 1."]
    #[inline(always)]
    pub const fn wdc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of FDCAN_CCCR register are set to 1."]
    #[inline(always)]
    pub fn set_wdc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Watchdog value Actual message RAM watchdog counter value."]
    #[inline(always)]
    pub const fn wdv(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Watchdog value Actual message RAM watchdog counter value."]
    #[inline(always)]
    pub fn set_wdv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for FdcanRwd {
    #[inline(always)]
    fn default() -> FdcanRwd {
        FdcanRwd(0)
    }
}
#[doc = "CAN Rx FIFO 0 acknowledge register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanRxf0a(pub u32);
impl FdcanRxf0a {
    #[doc = "Rx FIFO 0 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This sets the Rx FIFO 0 get index RXF0S\\[F0GI\\] to F0AI + 1 and update the FIFO 0 fill level RXF0S\\[F0FL\\]."]
    #[inline(always)]
    pub const fn f0ai(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Rx FIFO 0 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This sets the Rx FIFO 0 get index RXF0S\\[F0GI\\] to F0AI + 1 and update the FIFO 0 fill level RXF0S\\[F0FL\\]."]
    #[inline(always)]
    pub fn set_f0ai(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for FdcanRxf0a {
    #[inline(always)]
    fn default() -> FdcanRxf0a {
        FdcanRxf0a(0)
    }
}
#[doc = "FDCAN Rx FIFO 0 status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanRxf0s(pub u32);
impl FdcanRxf0s {
    #[doc = "Rx FIFO 0 fill level Number of elements stored in Rx FIFO 0, range 0 to 3."]
    #[inline(always)]
    pub const fn f0fl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Rx FIFO 0 fill level Number of elements stored in Rx FIFO 0, range 0 to 3."]
    #[inline(always)]
    pub fn set_f0fl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Rx FIFO 0 get index Rx FIFO 0 read index pointer, range 0 to 2."]
    #[inline(always)]
    pub const fn f0gi(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Rx FIFO 0 get index Rx FIFO 0 read index pointer, range 0 to 2."]
    #[inline(always)]
    pub fn set_f0gi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Rx FIFO 0 put index Rx FIFO 0 write index pointer, range 0 to 2."]
    #[inline(always)]
    pub const fn f0pi(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Rx FIFO 0 put index Rx FIFO 0 write index pointer, range 0 to 2."]
    #[inline(always)]
    pub fn set_f0pi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Rx FIFO 0 full"]
    #[inline(always)]
    pub const fn f0f(&self) -> super::vals::F0f {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::F0f::from_bits(val as u8)
    }
    #[doc = "Rx FIFO 0 full"]
    #[inline(always)]
    pub fn set_f0f(&mut self, val: super::vals::F0f) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Rx FIFO 0 message lost This bit is a copy of interrupt flag IR\\[RF0L\\]. When IR\\[RF0L\\] is reset, this bit is also reset."]
    #[inline(always)]
    pub const fn rf0l(&self) -> super::vals::FdcanRxf0sRf0l {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::FdcanRxf0sRf0l::from_bits(val as u8)
    }
    #[doc = "Rx FIFO 0 message lost This bit is a copy of interrupt flag IR\\[RF0L\\]. When IR\\[RF0L\\] is reset, this bit is also reset."]
    #[inline(always)]
    pub fn set_rf0l(&mut self, val: super::vals::FdcanRxf0sRf0l) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for FdcanRxf0s {
    #[inline(always)]
    fn default() -> FdcanRxf0s {
        FdcanRxf0s(0)
    }
}
#[doc = "FDCAN Rx FIFO 1 acknowledge register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanRxf1a(pub u32);
impl FdcanRxf1a {
    #[doc = "Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFO 1 get index RXF1S\\[F1GI\\] to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\\[F1FL\\]."]
    #[inline(always)]
    pub const fn f1ai(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFO 1 get index RXF1S\\[F1GI\\] to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\\[F1FL\\]."]
    #[inline(always)]
    pub fn set_f1ai(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for FdcanRxf1a {
    #[inline(always)]
    fn default() -> FdcanRxf1a {
        FdcanRxf1a(0)
    }
}
#[doc = "FDCAN Rx FIFO 1 status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanRxf1s(pub u32);
impl FdcanRxf1s {
    #[doc = "Rx FIFO 1 fill level Number of elements stored in Rx FIFO 1, range 0 to 3."]
    #[inline(always)]
    pub const fn f1fl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Rx FIFO 1 fill level Number of elements stored in Rx FIFO 1, range 0 to 3."]
    #[inline(always)]
    pub fn set_f1fl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Rx FIFO 1 get index Rx FIFO 1 read index pointer, range 0 to 2."]
    #[inline(always)]
    pub const fn f1gi(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Rx FIFO 1 get index Rx FIFO 1 read index pointer, range 0 to 2."]
    #[inline(always)]
    pub fn set_f1gi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Rx FIFO 1 put index Rx FIFO 1 write index pointer, range 0 to 2."]
    #[inline(always)]
    pub const fn f1pi(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Rx FIFO 1 put index Rx FIFO 1 write index pointer, range 0 to 2."]
    #[inline(always)]
    pub fn set_f1pi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Rx FIFO 1 full"]
    #[inline(always)]
    pub const fn f1f(&self) -> super::vals::F1f {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::F1f::from_bits(val as u8)
    }
    #[doc = "Rx FIFO 1 full"]
    #[inline(always)]
    pub fn set_f1f(&mut self, val: super::vals::F1f) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Rx FIFO 1 message lost This bit is a copy of interrupt flag IR\\[RF1L\\]. When IR\\[RF1L\\] is reset, this bit is also reset."]
    #[inline(always)]
    pub const fn rf1l(&self) -> super::vals::FdcanRxf1sRf1l {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::FdcanRxf1sRf1l::from_bits(val as u8)
    }
    #[doc = "Rx FIFO 1 message lost This bit is a copy of interrupt flag IR\\[RF1L\\]. When IR\\[RF1L\\] is reset, this bit is also reset."]
    #[inline(always)]
    pub fn set_rf1l(&mut self, val: super::vals::FdcanRxf1sRf1l) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for FdcanRxf1s {
    #[inline(always)]
    fn default() -> FdcanRxf1s {
        FdcanRxf1s(0)
    }
}
#[doc = "FDCAN global filter configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanRxgfc(pub u32);
impl FdcanRxgfc {
    #[doc = "Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub const fn rrfe(&self) -> super::vals::Rrfe {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Rrfe::from_bits(val as u8)
    }
    #[doc = "Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn set_rrfe(&mut self, val: super::vals::Rrfe) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub const fn rrfs(&self) -> super::vals::Rrfs {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Rrfs::from_bits(val as u8)
    }
    #[doc = "Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn set_rrfs(&mut self, val: super::vals::Rrfs) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub const fn anfe(&self) -> super::vals::Anfe {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Anfe::from_bits(val as u8)
    }
    #[doc = "Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn set_anfe(&mut self, val: super::vals::Anfe) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub const fn anfs(&self) -> super::vals::Anfs {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Anfs::from_bits(val as u8)
    }
    #[doc = "Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn set_anfs(&mut self, val: super::vals::Anfs) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub const fn f1om(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn set_f1om(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub const fn f0om(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn set_f0om(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "List size standard 1 to 28: Number of standard message ID filter elements >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub const fn lss(&self) -> super::vals::Lss {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Lss::from_bits(val as u8)
    }
    #[doc = "List size standard 1 to 28: Number of standard message ID filter elements >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn set_lss(&mut self, val: super::vals::Lss) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "List size extended 1 to 8: Number of extended message ID filter elements >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub const fn lse(&self) -> super::vals::Lse {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Lse::from_bits(val as u8)
    }
    #[doc = "List size extended 1 to 8: Number of extended message ID filter elements >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn set_lse(&mut self, val: super::vals::Lse) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for FdcanRxgfc {
    #[inline(always)]
    fn default() -> FdcanRxgfc {
        FdcanRxgfc(0)
    }
}
#[doc = "FDCAN transmitter delay compensation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanTdcr(pub u32);
impl FdcanTdcr {
    #[doc = "Transmitter delay compensation filter window length Defines the minimum value for the SSP position, dominant edges on FDCAN_RX that would result in an earlier SSP position are ignored for transmitter delay measurements. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub const fn tdcf(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmitter delay compensation filter window length Defines the minimum value for the SSP position, dominant edges on FDCAN_RX that would result in an earlier SSP position are ignored for transmitter delay measurements. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn set_tdcf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Transmitter delay compensation offset Offset value defining the distance between the measured delay from FDCAN_TX to FDCAN_RX and the secondary sample point. Valid values are 0 to 127 mtq. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub const fn tdco(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmitter delay compensation offset Offset value defining the distance between the measured delay from FDCAN_TX to FDCAN_RX and the secondary sample point. Valid values are 0 to 127 mtq. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn set_tdco(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
}
impl Default for FdcanTdcr {
    #[inline(always)]
    fn default() -> FdcanTdcr {
        FdcanTdcr(0)
    }
}
#[doc = "FDCAN test register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanTest(pub u32);
impl FdcanTest {
    #[doc = "Loop back mode"]
    #[inline(always)]
    pub const fn lbck(&self) -> super::vals::Lbck {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Lbck::from_bits(val as u8)
    }
    #[doc = "Loop back mode"]
    #[inline(always)]
    pub fn set_lbck(&mut self, val: super::vals::Lbck) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Control of transmit pin"]
    #[inline(always)]
    pub const fn tx(&self) -> super::vals::Tx {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Tx::from_bits(val as u8)
    }
    #[doc = "Control of transmit pin"]
    #[inline(always)]
    pub fn set_tx(&mut self, val: super::vals::Tx) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Receive pin Monitors the actual value of pin FDCANx_RX"]
    #[inline(always)]
    pub const fn rx(&self) -> super::vals::Rx {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Rx::from_bits(val as u8)
    }
    #[doc = "Receive pin Monitors the actual value of pin FDCANx_RX"]
    #[inline(always)]
    pub fn set_rx(&mut self, val: super::vals::Rx) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for FdcanTest {
    #[inline(always)]
    fn default() -> FdcanTest {
        FdcanTest(0)
    }
}
#[doc = "FDCAN timeout counter configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanTocc(pub u32);
impl FdcanTocc {
    #[doc = "Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub const fn etoc(&self) -> super::vals::Etoc {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Etoc::from_bits(val as u8)
    }
    #[doc = "Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn set_etoc(&mut self, val: super::vals::Etoc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\] and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub const fn tos(&self) -> super::vals::Tos {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Tos::from_bits(val as u8)
    }
    #[doc = "Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\] and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn set_tos(&mut self, val: super::vals::Tos) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "Timeout period Start value of the timeout counter (down-counter). Configures the timeout period."]
    #[inline(always)]
    pub const fn top(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Timeout period Start value of the timeout counter (down-counter). Configures the timeout period."]
    #[inline(always)]
    pub fn set_top(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FdcanTocc {
    #[inline(always)]
    fn default() -> FdcanTocc {
        FdcanTocc(0)
    }
}
#[doc = "FDCAN timeout counter value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanTocv(pub u32);
impl FdcanTocv {
    #[doc = "Timeout counter"]
    #[inline(always)]
    pub const fn toc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timeout counter"]
    #[inline(always)]
    pub fn set_toc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for FdcanTocv {
    #[inline(always)]
    fn default() -> FdcanTocv {
        FdcanTocv(0)
    }
}
#[doc = "FDCAN timestamp counter configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanTscc(pub u32);
impl FdcanTscc {
    #[doc = "Timestamp select These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub const fn tss(&self) -> super::vals::Tss {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Tss::from_bits(val as u8)
    }
    #[doc = "Timestamp select These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn set_tss(&mut self, val: super::vals::Tss) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Timestamp counter prescaler"]
    #[inline(always)]
    pub const fn tcp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Timestamp counter prescaler"]
    #[inline(always)]
    pub fn set_tcp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for FdcanTscc {
    #[inline(always)]
    fn default() -> FdcanTscc {
        FdcanTscc(0)
    }
}
#[doc = "FDCAN timestamp counter value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanTscv(pub u32);
impl FdcanTscv {
    #[doc = "Timestamp counter"]
    #[inline(always)]
    pub const fn tsc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timestamp counter"]
    #[inline(always)]
    pub fn set_tsc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for FdcanTscv {
    #[inline(always)]
    fn default() -> FdcanTscv {
        FdcanTscv(0)
    }
}
#[doc = "FDCAN Tx buffer add request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanTxbar(pub u32);
impl FdcanTxbar {
    #[doc = "Add request Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed."]
    #[inline(always)]
    pub const fn ar(&self) -> super::vals::Ar {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ar::from_bits(val as u8)
    }
    #[doc = "Add request Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed."]
    #[inline(always)]
    pub fn set_ar(&mut self, val: super::vals::Ar) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for FdcanTxbar {
    #[inline(always)]
    fn default() -> FdcanTxbar {
        FdcanTxbar(0)
    }
}
#[doc = "FDCAN Tx buffer configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanTxbc(pub u32);
impl FdcanTxbc {
    #[doc = "Tx FIFO/queue mode This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub const fn tfqm(&self) -> super::vals::Tfqm {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Tfqm::from_bits(val as u8)
    }
    #[doc = "Tx FIFO/queue mode This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn set_tfqm(&mut self, val: super::vals::Tfqm) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for FdcanTxbc {
    #[inline(always)]
    fn default() -> FdcanTxbc {
        FdcanTxbc(0)
    }
}
#[doc = "FDCAN Tx buffer cancellation finished register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanTxbcf(pub u32);
impl FdcanTxbcf {
    #[doc = "Cancellation finished Each Tx buffer has its own CF bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
    #[inline(always)]
    pub const fn cf(&self) -> super::vals::Cf {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Cf::from_bits(val as u8)
    }
    #[doc = "Cancellation finished Each Tx buffer has its own CF bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
    #[inline(always)]
    pub fn set_cf(&mut self, val: super::vals::Cf) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for FdcanTxbcf {
    #[inline(always)]
    fn default() -> FdcanTxbcf {
        FdcanTxbcf(0)
    }
}
#[doc = "FDCAN Tx buffer cancellation finished interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanTxbcie(pub u32);
impl FdcanTxbcie {
    #[doc = "Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit."]
    #[inline(always)]
    pub const fn cfie(&self) -> super::vals::Cfie {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Cfie::from_bits(val as u8)
    }
    #[doc = "Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit."]
    #[inline(always)]
    pub fn set_cfie(&mut self, val: super::vals::Cfie) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for FdcanTxbcie {
    #[inline(always)]
    fn default() -> FdcanTxbcie {
        FdcanTxbcie(0)
    }
}
#[doc = "FDCAN Tx buffer cancellation request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanTxbcr(pub u32);
impl FdcanTxbcr {
    #[doc = "Cancellation request Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset."]
    #[inline(always)]
    pub const fn cr(&self) -> super::vals::Cr {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Cr::from_bits(val as u8)
    }
    #[doc = "Cancellation request Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset."]
    #[inline(always)]
    pub fn set_cr(&mut self, val: super::vals::Cr) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for FdcanTxbcr {
    #[inline(always)]
    fn default() -> FdcanTxbcr {
        FdcanTxbcr(0)
    }
}
#[doc = "FDCAN Tx buffer request pending register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanTxbrp(pub u32);
impl FdcanTxbrp {
    #[doc = "Transmission request pending Each Tx buffer has its own transmission request pending bit. The bits are set via register TXBAR. The bits are reset after a requested transmission has completed or has been canceled via register TXBCR. After a TXBRP bit has been set, a Tx scan is started to check for the pending Tx request with the highest priority (Tx buffer with lowest Message ID). A cancellation request resets the corresponding transmission request pending bit of register TXBRP. In case a transmission has already been started when a cancellation is requested, this is done at the end of the transmission, regardless whether the transmission was successful or not. The cancellation request bits are reset directly after the corresponding TXBRP bit has been reset. After a cancellation has been requested, a finished cancellation is signaled via TXBCF after successful transmission together with the corresponding TXBTO bit when the transmission has not yet been started at the point of cancellation when the transmission has been aborted due to lost arbitration when an error occurred during frame transmission In DAR mode all transmissions are automatically canceled if they are not successful. The corresponding TXBCF bit is set for all unsuccessful transmissions."]
    #[inline(always)]
    pub const fn trp(&self) -> super::vals::Trp {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Trp::from_bits(val as u8)
    }
    #[doc = "Transmission request pending Each Tx buffer has its own transmission request pending bit. The bits are set via register TXBAR. The bits are reset after a requested transmission has completed or has been canceled via register TXBCR. After a TXBRP bit has been set, a Tx scan is started to check for the pending Tx request with the highest priority (Tx buffer with lowest Message ID). A cancellation request resets the corresponding transmission request pending bit of register TXBRP. In case a transmission has already been started when a cancellation is requested, this is done at the end of the transmission, regardless whether the transmission was successful or not. The cancellation request bits are reset directly after the corresponding TXBRP bit has been reset. After a cancellation has been requested, a finished cancellation is signaled via TXBCF after successful transmission together with the corresponding TXBTO bit when the transmission has not yet been started at the point of cancellation when the transmission has been aborted due to lost arbitration when an error occurred during frame transmission In DAR mode all transmissions are automatically canceled if they are not successful. The corresponding TXBCF bit is set for all unsuccessful transmissions."]
    #[inline(always)]
    pub fn set_trp(&mut self, val: super::vals::Trp) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for FdcanTxbrp {
    #[inline(always)]
    fn default() -> FdcanTxbrp {
        FdcanTxbrp(0)
    }
}
#[doc = "FDCAN Tx buffer transmission interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanTxbtie(pub u32);
impl FdcanTxbtie {
    #[doc = "Transmission interrupt enable Each Tx buffer has its own TIE bit."]
    #[inline(always)]
    pub const fn tie(&self) -> super::vals::Tie {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Tie::from_bits(val as u8)
    }
    #[doc = "Transmission interrupt enable Each Tx buffer has its own TIE bit."]
    #[inline(always)]
    pub fn set_tie(&mut self, val: super::vals::Tie) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for FdcanTxbtie {
    #[inline(always)]
    fn default() -> FdcanTxbtie {
        FdcanTxbtie(0)
    }
}
#[doc = "FDCAN Tx buffer transmission occurred register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanTxbto(pub u32);
impl FdcanTxbto {
    #[doc = "Transmission occurred. Each Tx buffer has its own TO bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
    #[inline(always)]
    pub const fn to(&self) -> super::vals::To {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::To::from_bits(val as u8)
    }
    #[doc = "Transmission occurred. Each Tx buffer has its own TO bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
    #[inline(always)]
    pub fn set_to(&mut self, val: super::vals::To) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for FdcanTxbto {
    #[inline(always)]
    fn default() -> FdcanTxbto {
        FdcanTxbto(0)
    }
}
#[doc = "FDCAN Tx event FIFO acknowledge register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanTxefa(pub u32);
impl FdcanTxefa {
    #[doc = "Event FIFO acknowledge index After the Host has read an element or a sequence of elements from the Tx event FIFO, it has to write the index of the last element read from Tx event FIFO to EFAI. This sets the Tx event FIFO get index TXEFS\\[EFGI\\] to EFAI + 1 and updates the FIFO 0 fill level TXEFS\\[EFFL\\]."]
    #[inline(always)]
    pub const fn efai(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Event FIFO acknowledge index After the Host has read an element or a sequence of elements from the Tx event FIFO, it has to write the index of the last element read from Tx event FIFO to EFAI. This sets the Tx event FIFO get index TXEFS\\[EFGI\\] to EFAI + 1 and updates the FIFO 0 fill level TXEFS\\[EFFL\\]."]
    #[inline(always)]
    pub fn set_efai(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for FdcanTxefa {
    #[inline(always)]
    fn default() -> FdcanTxefa {
        FdcanTxefa(0)
    }
}
#[doc = "FDCAN Tx event FIFO status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanTxefs(pub u32);
impl FdcanTxefs {
    #[doc = "Event FIFO fill level Number of elements stored in Tx event FIFO, range 0 to 3."]
    #[inline(always)]
    pub const fn effl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Event FIFO fill level Number of elements stored in Tx event FIFO, range 0 to 3."]
    #[inline(always)]
    pub fn set_effl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Event FIFO get index Tx event FIFO read index pointer, range 0 to 3."]
    #[inline(always)]
    pub const fn efgi(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Event FIFO get index Tx event FIFO read index pointer, range 0 to 3."]
    #[inline(always)]
    pub fn set_efgi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Event FIFO put index Tx event FIFO write index pointer, range 0 to 3."]
    #[inline(always)]
    pub const fn efpi(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Event FIFO put index Tx event FIFO write index pointer, range 0 to 3."]
    #[inline(always)]
    pub fn set_efpi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Event FIFO full"]
    #[inline(always)]
    pub const fn eff(&self) -> super::vals::Eff {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Eff::from_bits(val as u8)
    }
    #[doc = "Event FIFO full"]
    #[inline(always)]
    pub fn set_eff(&mut self, val: super::vals::Eff) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Tx event FIFO element lost This bit is a copy of interrupt flag IR\\[TEFL\\]. When IR\\[TEFL\\] is reset, this bit is also reset. 0 No Tx event FIFO element lost 1 Tx event FIFO element lost, also set after write attempt to Tx event FIFO of size 0."]
    #[inline(always)]
    pub const fn tefl(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO element lost This bit is a copy of interrupt flag IR\\[TEFL\\]. When IR\\[TEFL\\] is reset, this bit is also reset. 0 No Tx event FIFO element lost 1 Tx event FIFO element lost, also set after write attempt to Tx event FIFO of size 0."]
    #[inline(always)]
    pub fn set_tefl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for FdcanTxefs {
    #[inline(always)]
    fn default() -> FdcanTxefs {
        FdcanTxefs(0)
    }
}
#[doc = "FDCAN Tx FIFO/queue status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanTxfqs(pub u32);
impl FdcanTxfqs {
    #[doc = "Tx FIFO free level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC\\[TFQM\\] = 1)."]
    #[inline(always)]
    pub const fn tffl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Tx FIFO free level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC\\[TFQM\\] = 1)."]
    #[inline(always)]
    pub fn set_tffl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Tx FIFO get index Tx FIFO read index pointer, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC.TFQM = 1)"]
    #[inline(always)]
    pub const fn tfgi(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Tx FIFO get index Tx FIFO read index pointer, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC.TFQM = 1)"]
    #[inline(always)]
    pub fn set_tfgi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Tx FIFO/queue put index Tx FIFO/queue write index pointer, range 0 to 3"]
    #[inline(always)]
    pub const fn tfqpi(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Tx FIFO/queue put index Tx FIFO/queue write index pointer, range 0 to 3"]
    #[inline(always)]
    pub fn set_tfqpi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Tx FIFO/queue full"]
    #[inline(always)]
    pub const fn tfqf(&self) -> super::vals::Tfqf {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Tfqf::from_bits(val as u8)
    }
    #[doc = "Tx FIFO/queue full"]
    #[inline(always)]
    pub fn set_tfqf(&mut self, val: super::vals::Tfqf) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
}
impl Default for FdcanTxfqs {
    #[inline(always)]
    fn default() -> FdcanTxfqs {
        FdcanTxfqs(0)
    }
}
#[doc = "FDCAN extended ID and mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdcanXidam(pub u32);
impl FdcanXidam {
    #[doc = "Extended ID mask For acceptance filtering of extended frames the Extended ID AND Mask is AND-ed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to 1 the mask is not active. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub const fn eidm(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Extended ID mask For acceptance filtering of extended frames the Extended ID AND Mask is AND-ed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to 1 the mask is not active. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn set_eidm(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
}
impl Default for FdcanXidam {
    #[inline(always)]
    fn default() -> FdcanXidam {
        FdcanXidam(0)
    }
}

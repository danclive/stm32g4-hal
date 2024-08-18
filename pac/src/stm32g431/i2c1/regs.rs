#[doc = "Control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc = "Peripheral enable"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral enable"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TX Interrupt enable"]
    #[inline(always)]
    pub const fn txie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TX Interrupt enable"]
    #[inline(always)]
    pub fn set_txie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RX Interrupt enable"]
    #[inline(always)]
    pub const fn rxie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RX Interrupt enable"]
    #[inline(always)]
    pub fn set_rxie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Address match interrupt enable (slave only)"]
    #[inline(always)]
    pub const fn addrie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Address match interrupt enable (slave only)"]
    #[inline(always)]
    pub fn set_addrie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Not acknowledge received interrupt enable"]
    #[inline(always)]
    pub const fn nackie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Not acknowledge received interrupt enable"]
    #[inline(always)]
    pub fn set_nackie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "STOP detection Interrupt enable"]
    #[inline(always)]
    pub const fn stopie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "STOP detection Interrupt enable"]
    #[inline(always)]
    pub fn set_stopie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transfer Complete interrupt enable"]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Complete interrupt enable"]
    #[inline(always)]
    pub fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Error interrupts enable"]
    #[inline(always)]
    pub const fn errie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Error interrupts enable"]
    #[inline(always)]
    pub fn set_errie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Digital noise filter"]
    #[inline(always)]
    pub const fn dnf(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Digital noise filter"]
    #[inline(always)]
    pub fn set_dnf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Analog noise filter OFF"]
    #[inline(always)]
    pub const fn anfoff(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Analog noise filter OFF"]
    #[inline(always)]
    pub fn set_anfoff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA transmission requests enable"]
    #[inline(always)]
    pub const fn txdmaen(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA transmission requests enable"]
    #[inline(always)]
    pub fn set_txdmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA reception requests enable"]
    #[inline(always)]
    pub const fn rxdmaen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA reception requests enable"]
    #[inline(always)]
    pub fn set_rxdmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Slave byte control"]
    #[inline(always)]
    pub const fn sbc(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Slave byte control"]
    #[inline(always)]
    pub fn set_sbc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Clock stretching disable"]
    #[inline(always)]
    pub const fn nostretch(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Clock stretching disable"]
    #[inline(always)]
    pub fn set_nostretch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Wakeup from STOP enable"]
    #[inline(always)]
    pub const fn wupen(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup from STOP enable"]
    #[inline(always)]
    pub fn set_wupen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "General call enable"]
    #[inline(always)]
    pub const fn gcen(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "General call enable"]
    #[inline(always)]
    pub fn set_gcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SMBus Host address enable"]
    #[inline(always)]
    pub const fn smbhen(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "SMBus Host address enable"]
    #[inline(always)]
    pub fn set_smbhen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "SMBus Device Default address enable"]
    #[inline(always)]
    pub const fn smbden(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "SMBus Device Default address enable"]
    #[inline(always)]
    pub fn set_smbden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "SMBUS alert enable"]
    #[inline(always)]
    pub const fn alerten(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "SMBUS alert enable"]
    #[inline(always)]
    pub fn set_alerten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "PEC enable"]
    #[inline(always)]
    pub const fn pecen(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "PEC enable"]
    #[inline(always)]
    pub fn set_pecen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Cr1 {
    #[inline(always)]
    fn default() -> Cr1 {
        Cr1(0)
    }
}
#[doc = "Control register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc = "Slave address bit (master mode)"]
    #[inline(always)]
    pub const fn sadd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Slave address bit (master mode)"]
    #[inline(always)]
    pub fn set_sadd(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Transfer direction (master mode)"]
    #[inline(always)]
    pub const fn rd_wrn(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer direction (master mode)"]
    #[inline(always)]
    pub fn set_rd_wrn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "10-bit addressing mode (master mode)"]
    #[inline(always)]
    pub const fn add10(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "10-bit addressing mode (master mode)"]
    #[inline(always)]
    pub fn set_add10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "10-bit address header only read direction (master receiver mode)"]
    #[inline(always)]
    pub const fn head10r(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "10-bit address header only read direction (master receiver mode)"]
    #[inline(always)]
    pub fn set_head10r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Start generation"]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Start generation"]
    #[inline(always)]
    pub fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Stop generation (master mode)"]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Stop generation (master mode)"]
    #[inline(always)]
    pub fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "NACK generation (slave mode)"]
    #[inline(always)]
    pub const fn nack(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "NACK generation (slave mode)"]
    #[inline(always)]
    pub fn set_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Number of bytes"]
    #[inline(always)]
    pub const fn nbytes(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Number of bytes"]
    #[inline(always)]
    pub fn set_nbytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "NBYTES reload mode"]
    #[inline(always)]
    pub const fn reload(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "NBYTES reload mode"]
    #[inline(always)]
    pub fn set_reload(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Automatic end mode (master mode)"]
    #[inline(always)]
    pub const fn autoend(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic end mode (master mode)"]
    #[inline(always)]
    pub fn set_autoend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Packet error checking byte"]
    #[inline(always)]
    pub const fn pecbyte(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Packet error checking byte"]
    #[inline(always)]
    pub fn set_pecbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for Cr2 {
    #[inline(always)]
    fn default() -> Cr2 {
        Cr2(0)
    }
}
#[doc = "Interrupt clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "Address Matched flag clear"]
    #[inline(always)]
    pub const fn addrcf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Address Matched flag clear"]
    #[inline(always)]
    pub fn set_addrcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Not Acknowledge flag clear"]
    #[inline(always)]
    pub const fn nackcf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Not Acknowledge flag clear"]
    #[inline(always)]
    pub fn set_nackcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Stop detection flag clear"]
    #[inline(always)]
    pub const fn stopcf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Stop detection flag clear"]
    #[inline(always)]
    pub fn set_stopcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Bus error flag clear"]
    #[inline(always)]
    pub const fn berrcf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Bus error flag clear"]
    #[inline(always)]
    pub fn set_berrcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Arbitration lost flag clear"]
    #[inline(always)]
    pub const fn arlocf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration lost flag clear"]
    #[inline(always)]
    pub fn set_arlocf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Overrun/Underrun flag clear"]
    #[inline(always)]
    pub const fn ovrcf(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun/Underrun flag clear"]
    #[inline(always)]
    pub fn set_ovrcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "PEC Error flag clear"]
    #[inline(always)]
    pub const fn peccf(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PEC Error flag clear"]
    #[inline(always)]
    pub fn set_peccf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Timeout detection flag clear"]
    #[inline(always)]
    pub const fn timoutcf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout detection flag clear"]
    #[inline(always)]
    pub fn set_timoutcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Alert flag clear"]
    #[inline(always)]
    pub const fn alertcf(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Alert flag clear"]
    #[inline(always)]
    pub fn set_alertcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        Icr(0)
    }
}
#[doc = "Interrupt and Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc = "Transmit data register empty (transmitters)"]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit data register empty (transmitters)"]
    #[inline(always)]
    pub fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit interrupt status (transmitters)"]
    #[inline(always)]
    pub const fn txis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit interrupt status (transmitters)"]
    #[inline(always)]
    pub fn set_txis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive data register not empty (receivers)"]
    #[inline(always)]
    pub const fn rxne(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive data register not empty (receivers)"]
    #[inline(always)]
    pub fn set_rxne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Address matched (slave mode)"]
    #[inline(always)]
    pub const fn addr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Address matched (slave mode)"]
    #[inline(always)]
    pub fn set_addr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Not acknowledge received flag"]
    #[inline(always)]
    pub const fn nackf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Not acknowledge received flag"]
    #[inline(always)]
    pub fn set_nackf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Stop detection flag"]
    #[inline(always)]
    pub const fn stopf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Stop detection flag"]
    #[inline(always)]
    pub fn set_stopf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transfer Complete (master mode)"]
    #[inline(always)]
    pub const fn tc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Complete (master mode)"]
    #[inline(always)]
    pub fn set_tc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transfer Complete Reload"]
    #[inline(always)]
    pub const fn tcr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Complete Reload"]
    #[inline(always)]
    pub fn set_tcr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Bus error"]
    #[inline(always)]
    pub const fn berr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Bus error"]
    #[inline(always)]
    pub fn set_berr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Arbitration lost"]
    #[inline(always)]
    pub const fn arlo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration lost"]
    #[inline(always)]
    pub fn set_arlo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Overrun/Underrun (slave mode)"]
    #[inline(always)]
    pub const fn ovr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun/Underrun (slave mode)"]
    #[inline(always)]
    pub fn set_ovr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "PEC Error in reception"]
    #[inline(always)]
    pub const fn pecerr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PEC Error in reception"]
    #[inline(always)]
    pub fn set_pecerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Timeout or t_low detection flag"]
    #[inline(always)]
    pub const fn timeout(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout or t_low detection flag"]
    #[inline(always)]
    pub fn set_timeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SMBus alert"]
    #[inline(always)]
    pub const fn alert(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SMBus alert"]
    #[inline(always)]
    pub fn set_alert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Bus busy"]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Bus busy"]
    #[inline(always)]
    pub fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Transfer direction (Slave mode)"]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer direction (Slave mode)"]
    #[inline(always)]
    pub fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Address match code (Slave mode)"]
    #[inline(always)]
    pub const fn addcode(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x7f;
        val as u8
    }
    #[doc = "Address match code (Slave mode)"]
    #[inline(always)]
    pub fn set_addcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
    }
}
impl Default for Isr {
    #[inline(always)]
    fn default() -> Isr {
        Isr(0)
    }
}
#[doc = "Own address register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oar1(pub u32);
impl Oar1 {
    #[doc = "Interface address"]
    #[inline(always)]
    pub const fn oa1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Interface address"]
    #[inline(always)]
    pub fn set_oa1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Own Address 1 10-bit mode"]
    #[inline(always)]
    pub const fn oa1mode(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Own Address 1 10-bit mode"]
    #[inline(always)]
    pub fn set_oa1mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Own Address 1 enable"]
    #[inline(always)]
    pub const fn oa1en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Own Address 1 enable"]
    #[inline(always)]
    pub fn set_oa1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Oar1 {
    #[inline(always)]
    fn default() -> Oar1 {
        Oar1(0)
    }
}
#[doc = "Own address register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oar2(pub u32);
impl Oar2 {
    #[doc = "Interface address"]
    #[inline(always)]
    pub const fn oa2(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Interface address"]
    #[inline(always)]
    pub fn set_oa2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "Own Address 2 masks"]
    #[inline(always)]
    pub const fn oa2msk(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Own Address 2 masks"]
    #[inline(always)]
    pub fn set_oa2msk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Own Address 2 enable"]
    #[inline(always)]
    pub const fn oa2en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Own Address 2 enable"]
    #[inline(always)]
    pub fn set_oa2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Oar2 {
    #[inline(always)]
    fn default() -> Oar2 {
        Oar2(0)
    }
}
#[doc = "PEC register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pecr(pub u32);
impl Pecr {
    #[doc = "Packet error checking register"]
    #[inline(always)]
    pub const fn pec(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Packet error checking register"]
    #[inline(always)]
    pub fn set_pec(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Pecr {
    #[inline(always)]
    fn default() -> Pecr {
        Pecr(0)
    }
}
#[doc = "Receive data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxdr(pub u32);
impl Rxdr {
    #[doc = "8-bit receive data"]
    #[inline(always)]
    pub const fn rxdata(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit receive data"]
    #[inline(always)]
    pub fn set_rxdata(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rxdr {
    #[inline(always)]
    fn default() -> Rxdr {
        Rxdr(0)
    }
}
#[doc = "Status register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timeoutr(pub u32);
impl Timeoutr {
    #[doc = "Bus timeout A"]
    #[inline(always)]
    pub const fn timeouta(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Bus timeout A"]
    #[inline(always)]
    pub fn set_timeouta(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Idle clock timeout detection"]
    #[inline(always)]
    pub const fn tidle(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Idle clock timeout detection"]
    #[inline(always)]
    pub fn set_tidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Clock timeout enable"]
    #[inline(always)]
    pub const fn timouten(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Clock timeout enable"]
    #[inline(always)]
    pub fn set_timouten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Bus timeout B"]
    #[inline(always)]
    pub const fn timeoutb(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Bus timeout B"]
    #[inline(always)]
    pub fn set_timeoutb(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
    #[doc = "Extended clock timeout enable"]
    #[inline(always)]
    pub const fn texten(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Extended clock timeout enable"]
    #[inline(always)]
    pub fn set_texten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Timeoutr {
    #[inline(always)]
    fn default() -> Timeoutr {
        Timeoutr(0)
    }
}
#[doc = "Timing register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timingr(pub u32);
impl Timingr {
    #[doc = "SCL low period (master mode)"]
    #[inline(always)]
    pub const fn scll(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SCL low period (master mode)"]
    #[inline(always)]
    pub fn set_scll(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "SCL high period (master mode)"]
    #[inline(always)]
    pub const fn sclh(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SCL high period (master mode)"]
    #[inline(always)]
    pub fn set_sclh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data hold time"]
    #[inline(always)]
    pub const fn sdadel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Data hold time"]
    #[inline(always)]
    pub fn set_sdadel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Data setup time"]
    #[inline(always)]
    pub const fn scldel(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Data setup time"]
    #[inline(always)]
    pub fn set_scldel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Timing prescaler"]
    #[inline(always)]
    pub const fn presc(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Timing prescaler"]
    #[inline(always)]
    pub fn set_presc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Timingr {
    #[inline(always)]
    fn default() -> Timingr {
        Timingr(0)
    }
}
#[doc = "Transmit data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdr(pub u32);
impl Txdr {
    #[doc = "8-bit transmit data"]
    #[inline(always)]
    pub const fn txdata(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit transmit data"]
    #[inline(always)]
    pub fn set_txdata(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Txdr {
    #[inline(always)]
    fn default() -> Txdr {
        Txdr(0)
    }
}

#[doc = "control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc = "Clock phase"]
    #[inline(always)]
    pub const fn cpha(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clock phase"]
    #[inline(always)]
    pub fn set_cpha(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clock polarity"]
    #[inline(always)]
    pub const fn cpol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clock polarity"]
    #[inline(always)]
    pub fn set_cpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Master selection"]
    #[inline(always)]
    pub const fn mstr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Master selection"]
    #[inline(always)]
    pub fn set_mstr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Baud rate control"]
    #[inline(always)]
    pub const fn br(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Baud rate control"]
    #[inline(always)]
    pub fn set_br(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "SPI enable"]
    #[inline(always)]
    pub const fn spe(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "SPI enable"]
    #[inline(always)]
    pub fn set_spe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Frame format"]
    #[inline(always)]
    pub const fn lsbfirst(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Frame format"]
    #[inline(always)]
    pub fn set_lsbfirst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Internal slave select"]
    #[inline(always)]
    pub const fn ssi(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Internal slave select"]
    #[inline(always)]
    pub fn set_ssi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Software slave management"]
    #[inline(always)]
    pub const fn ssm(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Software slave management"]
    #[inline(always)]
    pub fn set_ssm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Receive only"]
    #[inline(always)]
    pub const fn rxonly(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Receive only"]
    #[inline(always)]
    pub fn set_rxonly(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Data frame format"]
    #[inline(always)]
    pub const fn dff(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Data frame format"]
    #[inline(always)]
    pub fn set_dff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CRC transfer next"]
    #[inline(always)]
    pub const fn crcnext(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CRC transfer next"]
    #[inline(always)]
    pub fn set_crcnext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Hardware CRC calculation enable"]
    #[inline(always)]
    pub const fn crcen(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware CRC calculation enable"]
    #[inline(always)]
    pub fn set_crcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Output enable in bidirectional mode"]
    #[inline(always)]
    pub const fn bidioe(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Output enable in bidirectional mode"]
    #[inline(always)]
    pub fn set_bidioe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Bidirectional data mode enable"]
    #[inline(always)]
    pub const fn bidimode(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Bidirectional data mode enable"]
    #[inline(always)]
    pub fn set_bidimode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Cr1 {
    #[inline(always)]
    fn default() -> Cr1 {
        Cr1(0)
    }
}
#[doc = "control register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc = "Rx buffer DMA enable"]
    #[inline(always)]
    pub const fn rxdmaen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Rx buffer DMA enable"]
    #[inline(always)]
    pub fn set_rxdmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Tx buffer DMA enable"]
    #[inline(always)]
    pub const fn txdmaen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Tx buffer DMA enable"]
    #[inline(always)]
    pub fn set_txdmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SS output enable"]
    #[inline(always)]
    pub const fn ssoe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SS output enable"]
    #[inline(always)]
    pub fn set_ssoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NSS pulse management"]
    #[inline(always)]
    pub const fn nssp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "NSS pulse management"]
    #[inline(always)]
    pub fn set_nssp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Frame format"]
    #[inline(always)]
    pub const fn frf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Frame format"]
    #[inline(always)]
    pub fn set_frf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Error interrupt enable"]
    #[inline(always)]
    pub const fn errie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Error interrupt enable"]
    #[inline(always)]
    pub fn set_errie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub const fn rxneie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub fn set_rxneie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub const fn txeie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn set_txeie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Data size"]
    #[inline(always)]
    pub const fn ds(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Data size"]
    #[inline(always)]
    pub fn set_ds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "FIFO reception threshold"]
    #[inline(always)]
    pub const fn frxth(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO reception threshold"]
    #[inline(always)]
    pub fn set_frxth(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Last DMA transfer for reception"]
    #[inline(always)]
    pub const fn ldma_rx(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Last DMA transfer for reception"]
    #[inline(always)]
    pub fn set_ldma_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Last DMA transfer for transmission"]
    #[inline(always)]
    pub const fn ldma_tx(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Last DMA transfer for transmission"]
    #[inline(always)]
    pub fn set_ldma_tx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Cr2 {
    #[inline(always)]
    fn default() -> Cr2 {
        Cr2(0)
    }
}
#[doc = "CRC polynomial register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crcpr(pub u32);
impl Crcpr {
    #[doc = "CRC polynomial register"]
    #[inline(always)]
    pub const fn crcpoly(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CRC polynomial register"]
    #[inline(always)]
    pub fn set_crcpoly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Crcpr {
    #[inline(always)]
    fn default() -> Crcpr {
        Crcpr(0)
    }
}
#[doc = "data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc = "Data register"]
    #[inline(always)]
    pub const fn dr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data register"]
    #[inline(always)]
    pub fn set_dr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr {
    #[inline(always)]
    fn default() -> Dr {
        Dr(0)
    }
}
#[doc = "configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2scfgr(pub u32);
impl I2scfgr {
    #[doc = "CHLEN"]
    #[inline(always)]
    pub const fn chlen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CHLEN"]
    #[inline(always)]
    pub fn set_chlen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DATLEN"]
    #[inline(always)]
    pub const fn datlen(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "DATLEN"]
    #[inline(always)]
    pub fn set_datlen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "CKPOL"]
    #[inline(always)]
    pub const fn ckpol(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CKPOL"]
    #[inline(always)]
    pub fn set_ckpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "I2SSTD"]
    #[inline(always)]
    pub const fn i2sstd(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "I2SSTD"]
    #[inline(always)]
    pub fn set_i2sstd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "PCMSYNC"]
    #[inline(always)]
    pub const fn pcmsync(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PCMSYNC"]
    #[inline(always)]
    pub fn set_pcmsync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "I2SCFG"]
    #[inline(always)]
    pub const fn i2scfg(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "I2SCFG"]
    #[inline(always)]
    pub fn set_i2scfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "I2SE"]
    #[inline(always)]
    pub const fn i2se(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "I2SE"]
    #[inline(always)]
    pub fn set_i2se(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "I2SMOD"]
    #[inline(always)]
    pub const fn i2smod(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "I2SMOD"]
    #[inline(always)]
    pub fn set_i2smod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for I2scfgr {
    #[inline(always)]
    fn default() -> I2scfgr {
        I2scfgr(0)
    }
}
#[doc = "prescaler register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2spr(pub u32);
impl I2spr {
    #[doc = "I2SDIV"]
    #[inline(always)]
    pub const fn i2sdiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "I2SDIV"]
    #[inline(always)]
    pub fn set_i2sdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "ODD"]
    #[inline(always)]
    pub const fn odd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "ODD"]
    #[inline(always)]
    pub fn set_odd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "MCKOE"]
    #[inline(always)]
    pub const fn mckoe(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "MCKOE"]
    #[inline(always)]
    pub fn set_mckoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for I2spr {
    #[inline(always)]
    fn default() -> I2spr {
        I2spr(0)
    }
}
#[doc = "RX CRC register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxcrcr(pub u32);
impl Rxcrcr {
    #[doc = "Rx CRC register"]
    #[inline(always)]
    pub const fn rx_crc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Rx CRC register"]
    #[inline(always)]
    pub fn set_rx_crc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Rxcrcr {
    #[inline(always)]
    fn default() -> Rxcrcr {
        Rxcrcr(0)
    }
}
#[doc = "status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Receive buffer not empty"]
    #[inline(always)]
    pub const fn rxne(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive buffer not empty"]
    #[inline(always)]
    pub fn set_rxne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit buffer empty"]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit buffer empty"]
    #[inline(always)]
    pub fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CRC error flag"]
    #[inline(always)]
    pub const fn crcerr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CRC error flag"]
    #[inline(always)]
    pub fn set_crcerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Mode fault"]
    #[inline(always)]
    pub const fn modf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Mode fault"]
    #[inline(always)]
    pub fn set_modf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Overrun flag"]
    #[inline(always)]
    pub const fn ovr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun flag"]
    #[inline(always)]
    pub fn set_ovr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Busy flag"]
    #[inline(always)]
    pub const fn bsy(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Busy flag"]
    #[inline(always)]
    pub fn set_bsy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "TI frame format error"]
    #[inline(always)]
    pub const fn tifrfe(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "TI frame format error"]
    #[inline(always)]
    pub fn set_tifrfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "FIFO reception level"]
    #[inline(always)]
    pub const fn frlvl(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "FIFO reception level"]
    #[inline(always)]
    pub fn set_frlvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "FIFO transmission level"]
    #[inline(always)]
    pub const fn ftlvl(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[doc = "FIFO transmission level"]
    #[inline(always)]
    pub fn set_ftlvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
#[doc = "TX CRC register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txcrcr(pub u32);
impl Txcrcr {
    #[doc = "Tx CRC register"]
    #[inline(always)]
    pub const fn tx_crc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Tx CRC register"]
    #[inline(always)]
    pub fn set_tx_crc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Txcrcr {
    #[inline(always)]
    fn default() -> Txcrcr {
        Txcrcr(0)
    }
}

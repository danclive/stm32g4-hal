#[doc = "UCPD configuration register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg1(pub u32);
impl Cfg1 {
    #[doc = "HBITCLKDIV"]
    #[inline(always)]
    pub const fn hbitclkdiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "HBITCLKDIV"]
    #[inline(always)]
    pub fn set_hbitclkdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "IFRGAP"]
    #[inline(always)]
    pub const fn ifrgap(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x1f;
        val as u8
    }
    #[doc = "IFRGAP"]
    #[inline(always)]
    pub fn set_ifrgap(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
    }
    #[doc = "TRANSWIN"]
    #[inline(always)]
    pub const fn transwin(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "TRANSWIN"]
    #[inline(always)]
    pub fn set_transwin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
    #[doc = "PSC_USBPDCLK"]
    #[inline(always)]
    pub const fn psc_usbpdclk(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x07;
        val as u8
    }
    #[doc = "PSC_USBPDCLK"]
    #[inline(always)]
    pub fn set_psc_usbpdclk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
    }
    #[doc = "RXORDSETEN"]
    #[inline(always)]
    pub const fn rxordseten(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x01ff;
        val as u16
    }
    #[doc = "RXORDSETEN"]
    #[inline(always)]
    pub fn set_rxordseten(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 20usize)) | (((val as u32) & 0x01ff) << 20usize);
    }
    #[doc = "TXDMAEN"]
    #[inline(always)]
    pub const fn txdmaen(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "TXDMAEN"]
    #[inline(always)]
    pub fn set_txdmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "RXDMAEN"]
    #[inline(always)]
    pub const fn rxdmaen(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "RXDMAEN"]
    #[inline(always)]
    pub fn set_rxdmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "UCPDEN"]
    #[inline(always)]
    pub const fn ucpden(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "UCPDEN"]
    #[inline(always)]
    pub fn set_ucpden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cfg1 {
    #[inline(always)]
    fn default() -> Cfg1 {
        Cfg1(0)
    }
}
#[doc = "UCPD configuration register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg2(pub u32);
impl Cfg2 {
    #[doc = "RXFILTDIS"]
    #[inline(always)]
    pub const fn rxfiltdis(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RXFILTDIS"]
    #[inline(always)]
    pub fn set_rxfiltdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RXFILT2N3"]
    #[inline(always)]
    pub const fn rxfilt2n3(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RXFILT2N3"]
    #[inline(always)]
    pub fn set_rxfilt2n3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FORCECLK"]
    #[inline(always)]
    pub const fn forceclk(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FORCECLK"]
    #[inline(always)]
    pub fn set_forceclk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "WUPEN"]
    #[inline(always)]
    pub const fn wupen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "WUPEN"]
    #[inline(always)]
    pub fn set_wupen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Cfg2 {
    #[inline(always)]
    fn default() -> Cfg2 {
        Cfg2(0)
    }
}
#[doc = "UCPD configuration register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "TXMODE"]
    #[inline(always)]
    pub const fn txmode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "TXMODE"]
    #[inline(always)]
    pub fn set_txmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "TXSEND"]
    #[inline(always)]
    pub const fn txsend(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TXSEND"]
    #[inline(always)]
    pub fn set_txsend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TXHRST"]
    #[inline(always)]
    pub const fn txhrst(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TXHRST"]
    #[inline(always)]
    pub fn set_txhrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RXMODE"]
    #[inline(always)]
    pub const fn rxmode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "RXMODE"]
    #[inline(always)]
    pub fn set_rxmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "PHYRXEN"]
    #[inline(always)]
    pub const fn phyrxen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "PHYRXEN"]
    #[inline(always)]
    pub fn set_phyrxen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PHYCCSEL"]
    #[inline(always)]
    pub const fn phyccsel(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PHYCCSEL"]
    #[inline(always)]
    pub fn set_phyccsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "ANASUBMODE"]
    #[inline(always)]
    pub const fn anasubmode(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "ANASUBMODE"]
    #[inline(always)]
    pub fn set_anasubmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[doc = "ANAMODE"]
    #[inline(always)]
    pub const fn anamode(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "ANAMODE"]
    #[inline(always)]
    pub fn set_anamode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CCENABLE"]
    #[inline(always)]
    pub const fn ccenable(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "CCENABLE"]
    #[inline(always)]
    pub fn set_ccenable(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "FRSRXEN"]
    #[inline(always)]
    pub const fn frsrxen(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "FRSRXEN"]
    #[inline(always)]
    pub fn set_frsrxen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "FRSTX"]
    #[inline(always)]
    pub const fn frstx(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "FRSTX"]
    #[inline(always)]
    pub fn set_frstx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "RDCH"]
    #[inline(always)]
    pub const fn rdch(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "RDCH"]
    #[inline(always)]
    pub fn set_rdch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "CC1TCDIS"]
    #[inline(always)]
    pub const fn cc1tcdis(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "CC1TCDIS"]
    #[inline(always)]
    pub fn set_cc1tcdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "CC2TCDIS"]
    #[inline(always)]
    pub const fn cc2tcdis(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "CC2TCDIS"]
    #[inline(always)]
    pub fn set_cc2tcdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
#[doc = "UCPD Interrupt Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "TXMSGDISCCF"]
    #[inline(always)]
    pub const fn txmsgdisccf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TXMSGDISCCF"]
    #[inline(always)]
    pub fn set_txmsgdisccf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TXMSGSENTCF"]
    #[inline(always)]
    pub const fn txmsgsentcf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TXMSGSENTCF"]
    #[inline(always)]
    pub fn set_txmsgsentcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TXMSGABTCF"]
    #[inline(always)]
    pub const fn txmsgabtcf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TXMSGABTCF"]
    #[inline(always)]
    pub fn set_txmsgabtcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "HRSTDISCCF"]
    #[inline(always)]
    pub const fn hrstdisccf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "HRSTDISCCF"]
    #[inline(always)]
    pub fn set_hrstdisccf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "HRSTSENTCF"]
    #[inline(always)]
    pub const fn hrstsentcf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "HRSTSENTCF"]
    #[inline(always)]
    pub fn set_hrstsentcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "TXUNDCF"]
    #[inline(always)]
    pub const fn txundcf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "TXUNDCF"]
    #[inline(always)]
    pub fn set_txundcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "RXORDDETCF"]
    #[inline(always)]
    pub const fn rxorddetcf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "RXORDDETCF"]
    #[inline(always)]
    pub fn set_rxorddetcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "RXHRSTDETCF"]
    #[inline(always)]
    pub const fn rxhrstdetcf(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "RXHRSTDETCF"]
    #[inline(always)]
    pub fn set_rxhrstdetcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RXOVRCF"]
    #[inline(always)]
    pub const fn rxovrcf(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "RXOVRCF"]
    #[inline(always)]
    pub fn set_rxovrcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "RXMSGENDCF"]
    #[inline(always)]
    pub const fn rxmsgendcf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "RXMSGENDCF"]
    #[inline(always)]
    pub fn set_rxmsgendcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "TYPECEVT1CF"]
    #[inline(always)]
    pub const fn typecevt1cf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "TYPECEVT1CF"]
    #[inline(always)]
    pub fn set_typecevt1cf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "TYPECEVT2CF"]
    #[inline(always)]
    pub const fn typecevt2cf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "TYPECEVT2CF"]
    #[inline(always)]
    pub fn set_typecevt2cf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "FRSEVTCF"]
    #[inline(always)]
    pub const fn frsevtcf(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "FRSEVTCF"]
    #[inline(always)]
    pub fn set_frsevtcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        Icr(0)
    }
}
#[doc = "UCPD Interrupt Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imr(pub u32);
impl Imr {
    #[doc = "TXISIE"]
    #[inline(always)]
    pub const fn txisie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TXISIE"]
    #[inline(always)]
    pub fn set_txisie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TXMSGDISCIE"]
    #[inline(always)]
    pub const fn txmsgdiscie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TXMSGDISCIE"]
    #[inline(always)]
    pub fn set_txmsgdiscie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TXMSGSENTIE"]
    #[inline(always)]
    pub const fn txmsgsentie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TXMSGSENTIE"]
    #[inline(always)]
    pub fn set_txmsgsentie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TXMSGABTIE"]
    #[inline(always)]
    pub const fn txmsgabtie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TXMSGABTIE"]
    #[inline(always)]
    pub fn set_txmsgabtie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "HRSTDISCIE"]
    #[inline(always)]
    pub const fn hrstdiscie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "HRSTDISCIE"]
    #[inline(always)]
    pub fn set_hrstdiscie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "HRSTSENTIE"]
    #[inline(always)]
    pub const fn hrstsentie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "HRSTSENTIE"]
    #[inline(always)]
    pub fn set_hrstsentie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "TXUNDIE"]
    #[inline(always)]
    pub const fn txundie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "TXUNDIE"]
    #[inline(always)]
    pub fn set_txundie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "RXNEIE"]
    #[inline(always)]
    pub const fn rxneie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "RXNEIE"]
    #[inline(always)]
    pub fn set_rxneie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "RXORDDETIE"]
    #[inline(always)]
    pub const fn rxorddetie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "RXORDDETIE"]
    #[inline(always)]
    pub fn set_rxorddetie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "RXHRSTDETIE"]
    #[inline(always)]
    pub const fn rxhrstdetie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "RXHRSTDETIE"]
    #[inline(always)]
    pub fn set_rxhrstdetie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RXOVRIE"]
    #[inline(always)]
    pub const fn rxovrie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "RXOVRIE"]
    #[inline(always)]
    pub fn set_rxovrie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "RXMSGENDIE"]
    #[inline(always)]
    pub const fn rxmsgendie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "RXMSGENDIE"]
    #[inline(always)]
    pub fn set_rxmsgendie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "TYPECEVT1IE"]
    #[inline(always)]
    pub const fn typecevt1ie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "TYPECEVT1IE"]
    #[inline(always)]
    pub fn set_typecevt1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "TYPECEVT2IE"]
    #[inline(always)]
    pub const fn typecevt2ie(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "TYPECEVT2IE"]
    #[inline(always)]
    pub fn set_typecevt2ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "FRSEVTIE"]
    #[inline(always)]
    pub const fn frsevtie(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "FRSEVTIE"]
    #[inline(always)]
    pub fn set_frsevtie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Imr {
    #[inline(always)]
    fn default() -> Imr {
        Imr(0)
    }
}
#[doc = "UCPD Rx Ordered Set Extension Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxOrdext1(pub u32);
impl RxOrdext1 {
    #[doc = "RXSOPX1"]
    #[inline(always)]
    pub const fn rxsopx1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "RXSOPX1"]
    #[inline(always)]
    pub fn set_rxsopx1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for RxOrdext1 {
    #[inline(always)]
    fn default() -> RxOrdext1 {
        RxOrdext1(0)
    }
}
#[doc = "UCPD Rx Ordered Set Extension Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxOrdext2(pub u32);
impl RxOrdext2 {
    #[doc = "RXSOPX2"]
    #[inline(always)]
    pub const fn rxsopx2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "RXSOPX2"]
    #[inline(always)]
    pub fn set_rxsopx2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for RxOrdext2 {
    #[inline(always)]
    fn default() -> RxOrdext2 {
        RxOrdext2(0)
    }
}
#[doc = "UCPD Rx Ordered Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxOrdset(pub u32);
impl RxOrdset {
    #[doc = "RXORDSET"]
    #[inline(always)]
    pub const fn rxordset(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "RXORDSET"]
    #[inline(always)]
    pub fn set_rxordset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "RXSOP3OF4"]
    #[inline(always)]
    pub const fn rxsop3of4(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RXSOP3OF4"]
    #[inline(always)]
    pub fn set_rxsop3of4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RXSOPKINVALID"]
    #[inline(always)]
    pub const fn rxsopkinvalid(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "RXSOPKINVALID"]
    #[inline(always)]
    pub fn set_rxsopkinvalid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
}
impl Default for RxOrdset {
    #[inline(always)]
    fn default() -> RxOrdset {
        RxOrdset(0)
    }
}
#[doc = "UCPD Rx Paysize Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxPaysz(pub u32);
impl RxPaysz {
    #[doc = "RXPAYSZ"]
    #[inline(always)]
    pub const fn rxpaysz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "RXPAYSZ"]
    #[inline(always)]
    pub fn set_rxpaysz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for RxPaysz {
    #[inline(always)]
    fn default() -> RxPaysz {
        RxPaysz(0)
    }
}
#[doc = "UCPD Rx Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxdr(pub u32);
impl Rxdr {
    #[doc = "RXDATA"]
    #[inline(always)]
    pub const fn rxdata(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RXDATA"]
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
#[doc = "UCPD Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "TXIS"]
    #[inline(always)]
    pub const fn txis(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TXIS"]
    #[inline(always)]
    pub fn set_txis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TXMSGDISC"]
    #[inline(always)]
    pub const fn txmsgdisc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TXMSGDISC"]
    #[inline(always)]
    pub fn set_txmsgdisc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TXMSGSENT"]
    #[inline(always)]
    pub const fn txmsgsent(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TXMSGSENT"]
    #[inline(always)]
    pub fn set_txmsgsent(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TXMSGABT"]
    #[inline(always)]
    pub const fn txmsgabt(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TXMSGABT"]
    #[inline(always)]
    pub fn set_txmsgabt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "HRSTDISC"]
    #[inline(always)]
    pub const fn hrstdisc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "HRSTDISC"]
    #[inline(always)]
    pub fn set_hrstdisc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "HRSTSENT"]
    #[inline(always)]
    pub const fn hrstsent(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "HRSTSENT"]
    #[inline(always)]
    pub fn set_hrstsent(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "TXUND"]
    #[inline(always)]
    pub const fn txund(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "TXUND"]
    #[inline(always)]
    pub fn set_txund(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "RXNE"]
    #[inline(always)]
    pub const fn rxne(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "RXNE"]
    #[inline(always)]
    pub fn set_rxne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "RXORDDET"]
    #[inline(always)]
    pub const fn rxorddet(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "RXORDDET"]
    #[inline(always)]
    pub fn set_rxorddet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "RXHRSTDET"]
    #[inline(always)]
    pub const fn rxhrstdet(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "RXHRSTDET"]
    #[inline(always)]
    pub fn set_rxhrstdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RXOVR"]
    #[inline(always)]
    pub const fn rxovr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "RXOVR"]
    #[inline(always)]
    pub fn set_rxovr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "RXMSGEND"]
    #[inline(always)]
    pub const fn rxmsgend(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "RXMSGEND"]
    #[inline(always)]
    pub fn set_rxmsgend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "RXERR"]
    #[inline(always)]
    pub const fn rxerr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "RXERR"]
    #[inline(always)]
    pub fn set_rxerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "TYPECEVT1"]
    #[inline(always)]
    pub const fn typecevt1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "TYPECEVT1"]
    #[inline(always)]
    pub fn set_typecevt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "TYPECEVT2"]
    #[inline(always)]
    pub const fn typecevt2(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "TYPECEVT2"]
    #[inline(always)]
    pub fn set_typecevt2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "TYPEC_VSTATE_CC1"]
    #[inline(always)]
    pub const fn typec_vstate_cc1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "TYPEC_VSTATE_CC1"]
    #[inline(always)]
    pub fn set_typec_vstate_cc1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "TYPEC_VSTATE_CC2"]
    #[inline(always)]
    pub const fn typec_vstate_cc2(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "TYPEC_VSTATE_CC2"]
    #[inline(always)]
    pub fn set_typec_vstate_cc2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "FRSEVT"]
    #[inline(always)]
    pub const fn frsevt(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "FRSEVT"]
    #[inline(always)]
    pub fn set_frsevt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
#[doc = "UCPD Tx Ordered Set Type Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxOrdset(pub u32);
impl TxOrdset {
    #[doc = "TXORDSET"]
    #[inline(always)]
    pub const fn txordset(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "TXORDSET"]
    #[inline(always)]
    pub fn set_txordset(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for TxOrdset {
    #[inline(always)]
    fn default() -> TxOrdset {
        TxOrdset(0)
    }
}
#[doc = "UCPD Tx Paysize Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxPaysz(pub u32);
impl TxPaysz {
    #[doc = "TXPAYSZ"]
    #[inline(always)]
    pub const fn txpaysz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "TXPAYSZ"]
    #[inline(always)]
    pub fn set_txpaysz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for TxPaysz {
    #[inline(always)]
    fn default() -> TxPaysz {
        TxPaysz(0)
    }
}
#[doc = "UCPD Tx Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdr(pub u32);
impl Txdr {
    #[doc = "TXDATA"]
    #[inline(always)]
    pub const fn txdata(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "TXDATA"]
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

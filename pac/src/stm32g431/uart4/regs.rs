#[doc = "Baud rate register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brr(pub u32);
impl Brr {
    #[doc = "DIV_Fraction"]
    #[inline(always)]
    pub const fn div_fraction(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "DIV_Fraction"]
    #[inline(always)]
    pub fn set_div_fraction(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "DIV_Mantissa"]
    #[inline(always)]
    pub const fn div_mantissa(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "DIV_Mantissa"]
    #[inline(always)]
    pub fn set_div_mantissa(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
}
impl Default for Brr {
    #[inline(always)]
    fn default() -> Brr {
        Brr(0)
    }
}
#[doc = "Control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc = "USART enable"]
    #[inline(always)]
    pub const fn ue(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USART enable"]
    #[inline(always)]
    pub fn set_ue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "USART enable in Stop mode"]
    #[inline(always)]
    pub const fn uesm(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "USART enable in Stop mode"]
    #[inline(always)]
    pub fn set_uesm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receiver enable"]
    #[inline(always)]
    pub const fn re(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver enable"]
    #[inline(always)]
    pub fn set_re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmitter enable"]
    #[inline(always)]
    pub const fn te(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter enable"]
    #[inline(always)]
    pub fn set_te(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IDLE interrupt enable"]
    #[inline(always)]
    pub const fn idleie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IDLE interrupt enable"]
    #[inline(always)]
    pub fn set_idleie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "RXNE interrupt enable"]
    #[inline(always)]
    pub const fn rxneie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RXNE interrupt enable"]
    #[inline(always)]
    pub fn set_rxneie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmission complete interrupt enable"]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub const fn txeie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn set_txeie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PE interrupt enable"]
    #[inline(always)]
    pub const fn peie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PE interrupt enable"]
    #[inline(always)]
    pub fn set_peie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Parity selection"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Parity selection"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Parity control enable"]
    #[inline(always)]
    pub const fn pce(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Parity control enable"]
    #[inline(always)]
    pub fn set_pce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Receiver wakeup method"]
    #[inline(always)]
    pub const fn wake(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver wakeup method"]
    #[inline(always)]
    pub fn set_wake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Word length"]
    #[inline(always)]
    pub const fn m0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Word length"]
    #[inline(always)]
    pub fn set_m0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Mute mode enable"]
    #[inline(always)]
    pub const fn mme(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Mute mode enable"]
    #[inline(always)]
    pub fn set_mme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Character match interrupt enable"]
    #[inline(always)]
    pub const fn cmie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Character match interrupt enable"]
    #[inline(always)]
    pub fn set_cmie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Oversampling mode"]
    #[inline(always)]
    pub const fn over8(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Oversampling mode"]
    #[inline(always)]
    pub fn set_over8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DEDT0"]
    #[inline(always)]
    pub const fn dedt0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DEDT0"]
    #[inline(always)]
    pub fn set_dedt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DEDT1"]
    #[inline(always)]
    pub const fn dedt1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DEDT1"]
    #[inline(always)]
    pub fn set_dedt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DEDT2"]
    #[inline(always)]
    pub const fn dedt2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DEDT2"]
    #[inline(always)]
    pub fn set_dedt2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DEDT3"]
    #[inline(always)]
    pub const fn dedt3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DEDT3"]
    #[inline(always)]
    pub fn set_dedt3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Driver Enable de-assertion time"]
    #[inline(always)]
    pub const fn dedt4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Driver Enable de-assertion time"]
    #[inline(always)]
    pub fn set_dedt4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DEAT0"]
    #[inline(always)]
    pub const fn deat0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DEAT0"]
    #[inline(always)]
    pub fn set_deat0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DEAT1"]
    #[inline(always)]
    pub const fn deat1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DEAT1"]
    #[inline(always)]
    pub fn set_deat1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DEAT2"]
    #[inline(always)]
    pub const fn deat2(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DEAT2"]
    #[inline(always)]
    pub fn set_deat2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DEAT3"]
    #[inline(always)]
    pub const fn deat3(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DEAT3"]
    #[inline(always)]
    pub fn set_deat3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Driver Enable assertion time"]
    #[inline(always)]
    pub const fn deat4(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Driver Enable assertion time"]
    #[inline(always)]
    pub fn set_deat4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Receiver timeout interrupt enable"]
    #[inline(always)]
    pub const fn rtoie(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver timeout interrupt enable"]
    #[inline(always)]
    pub fn set_rtoie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "End of Block interrupt enable"]
    #[inline(always)]
    pub const fn eobie(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "End of Block interrupt enable"]
    #[inline(always)]
    pub fn set_eobie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "M1"]
    #[inline(always)]
    pub const fn m1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "M1"]
    #[inline(always)]
    pub fn set_m1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "FIFOEN"]
    #[inline(always)]
    pub const fn fifoen(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "FIFOEN"]
    #[inline(always)]
    pub fn set_fifoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "TXFEIE"]
    #[inline(always)]
    pub const fn txfeie(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "TXFEIE"]
    #[inline(always)]
    pub fn set_txfeie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "RXFFIE"]
    #[inline(always)]
    pub const fn rxffie(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "RXFFIE"]
    #[inline(always)]
    pub fn set_rxffie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
    #[doc = "SLVEN"]
    #[inline(always)]
    pub const fn slven(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SLVEN"]
    #[inline(always)]
    pub fn set_slven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DIS_NSS"]
    #[inline(always)]
    pub const fn dis_nss(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DIS_NSS"]
    #[inline(always)]
    pub fn set_dis_nss(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    pub const fn addm7(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    pub fn set_addm7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "LIN break detection length"]
    #[inline(always)]
    pub const fn lbdl(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "LIN break detection length"]
    #[inline(always)]
    pub fn set_lbdl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "LIN break detection interrupt enable"]
    #[inline(always)]
    pub const fn lbdie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn set_lbdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Last bit clock pulse"]
    #[inline(always)]
    pub const fn lbcl(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Last bit clock pulse"]
    #[inline(always)]
    pub fn set_lbcl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Clock phase"]
    #[inline(always)]
    pub const fn cpha(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Clock phase"]
    #[inline(always)]
    pub fn set_cpha(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Clock polarity"]
    #[inline(always)]
    pub const fn cpol(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Clock polarity"]
    #[inline(always)]
    pub fn set_cpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Clock enable"]
    #[inline(always)]
    pub const fn clken(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Clock enable"]
    #[inline(always)]
    pub fn set_clken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "STOP bits"]
    #[inline(always)]
    pub const fn stop(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "STOP bits"]
    #[inline(always)]
    pub fn set_stop(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "LIN mode enable"]
    #[inline(always)]
    pub const fn linen(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "LIN mode enable"]
    #[inline(always)]
    pub fn set_linen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Swap TX/RX pins"]
    #[inline(always)]
    pub const fn swap(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Swap TX/RX pins"]
    #[inline(always)]
    pub fn set_swap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "RX pin active level inversion"]
    #[inline(always)]
    pub const fn rxinv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "RX pin active level inversion"]
    #[inline(always)]
    pub fn set_rxinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TX pin active level inversion"]
    #[inline(always)]
    pub const fn txinv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TX pin active level inversion"]
    #[inline(always)]
    pub fn set_txinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Binary data inversion"]
    #[inline(always)]
    pub const fn tainv(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Binary data inversion"]
    #[inline(always)]
    pub fn set_tainv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Most significant bit first"]
    #[inline(always)]
    pub const fn msbfirst(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Most significant bit first"]
    #[inline(always)]
    pub fn set_msbfirst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Auto baud rate enable"]
    #[inline(always)]
    pub const fn abren(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Auto baud rate enable"]
    #[inline(always)]
    pub fn set_abren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ABRMOD0"]
    #[inline(always)]
    pub const fn abrmod0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ABRMOD0"]
    #[inline(always)]
    pub fn set_abrmod0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Auto baud rate mode"]
    #[inline(always)]
    pub const fn abrmod1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Auto baud rate mode"]
    #[inline(always)]
    pub fn set_abrmod1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Receiver timeout enable"]
    #[inline(always)]
    pub const fn rtoen(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver timeout enable"]
    #[inline(always)]
    pub fn set_rtoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Address of the USART node"]
    #[inline(always)]
    pub const fn add0_3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Address of the USART node"]
    #[inline(always)]
    pub fn set_add0_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Address of the USART node"]
    #[inline(always)]
    pub const fn add4_7(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Address of the USART node"]
    #[inline(always)]
    pub fn set_add4_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Cr2 {
    #[inline(always)]
    fn default() -> Cr2 {
        Cr2(0)
    }
}
#[doc = "Control register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr3(pub u32);
impl Cr3 {
    #[doc = "Error interrupt enable"]
    #[inline(always)]
    pub const fn eie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Error interrupt enable"]
    #[inline(always)]
    pub fn set_eie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Ir mode enable"]
    #[inline(always)]
    pub const fn iren(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Ir mode enable"]
    #[inline(always)]
    pub fn set_iren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Ir low-power"]
    #[inline(always)]
    pub const fn irlp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Ir low-power"]
    #[inline(always)]
    pub fn set_irlp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Half-duplex selection"]
    #[inline(always)]
    pub const fn hdsel(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Half-duplex selection"]
    #[inline(always)]
    pub fn set_hdsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Smartcard NACK enable"]
    #[inline(always)]
    pub const fn nack(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Smartcard NACK enable"]
    #[inline(always)]
    pub fn set_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Smartcard mode enable"]
    #[inline(always)]
    pub const fn scen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Smartcard mode enable"]
    #[inline(always)]
    pub fn set_scen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA enable receiver"]
    #[inline(always)]
    pub const fn dmar(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA enable receiver"]
    #[inline(always)]
    pub fn set_dmar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA enable transmitter"]
    #[inline(always)]
    pub const fn dmat(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA enable transmitter"]
    #[inline(always)]
    pub fn set_dmat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "RTS enable"]
    #[inline(always)]
    pub const fn rtse(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "RTS enable"]
    #[inline(always)]
    pub fn set_rtse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CTS enable"]
    #[inline(always)]
    pub const fn ctse(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CTS enable"]
    #[inline(always)]
    pub fn set_ctse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CTS interrupt enable"]
    #[inline(always)]
    pub const fn ctsie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CTS interrupt enable"]
    #[inline(always)]
    pub fn set_ctsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "One sample bit method enable"]
    #[inline(always)]
    pub const fn onebit(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "One sample bit method enable"]
    #[inline(always)]
    pub fn set_onebit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Overrun Disable"]
    #[inline(always)]
    pub const fn ovrdis(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun Disable"]
    #[inline(always)]
    pub fn set_ovrdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA Disable on Reception Error"]
    #[inline(always)]
    pub const fn ddre(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Disable on Reception Error"]
    #[inline(always)]
    pub fn set_ddre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Driver enable mode"]
    #[inline(always)]
    pub const fn dem(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Driver enable mode"]
    #[inline(always)]
    pub fn set_dem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Driver enable polarity selection"]
    #[inline(always)]
    pub const fn dep(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Driver enable polarity selection"]
    #[inline(always)]
    pub fn set_dep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Smartcard auto-retry count"]
    #[inline(always)]
    pub const fn scarcnt(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x07;
        val as u8
    }
    #[doc = "Smartcard auto-retry count"]
    #[inline(always)]
    pub fn set_scarcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
    }
    #[doc = "Wakeup from Stop mode interrupt flag selection"]
    #[inline(always)]
    pub const fn wus(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Wakeup from Stop mode interrupt flag selection"]
    #[inline(always)]
    pub fn set_wus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Wakeup from Stop mode interrupt enable"]
    #[inline(always)]
    pub const fn wufie(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup from Stop mode interrupt enable"]
    #[inline(always)]
    pub fn set_wufie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "TXFTIE"]
    #[inline(always)]
    pub const fn txftie(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TXFTIE"]
    #[inline(always)]
    pub fn set_txftie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TCBGTIE"]
    #[inline(always)]
    pub const fn tcbgtie(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TCBGTIE"]
    #[inline(always)]
    pub fn set_tcbgtie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "RXFTCFG"]
    #[inline(always)]
    pub const fn rxftcfg(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x07;
        val as u8
    }
    #[doc = "RXFTCFG"]
    #[inline(always)]
    pub fn set_rxftcfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
    }
    #[doc = "RXFTIE"]
    #[inline(always)]
    pub const fn rxftie(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "RXFTIE"]
    #[inline(always)]
    pub fn set_rxftie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "TXFTCFG"]
    #[inline(always)]
    pub const fn txftcfg(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "TXFTCFG"]
    #[inline(always)]
    pub fn set_txftcfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for Cr3 {
    #[inline(always)]
    fn default() -> Cr3 {
        Cr3(0)
    }
}
#[doc = "Guard time and prescaler register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtpr(pub u32);
impl Gtpr {
    #[doc = "Prescaler value"]
    #[inline(always)]
    pub const fn psc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Prescaler value"]
    #[inline(always)]
    pub fn set_psc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Guard time value"]
    #[inline(always)]
    pub const fn gt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Guard time value"]
    #[inline(always)]
    pub fn set_gt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Gtpr {
    #[inline(always)]
    fn default() -> Gtpr {
        Gtpr(0)
    }
}
#[doc = "Interrupt flag clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "Parity error clear flag"]
    #[inline(always)]
    pub const fn pecf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Parity error clear flag"]
    #[inline(always)]
    pub fn set_pecf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Framing error clear flag"]
    #[inline(always)]
    pub const fn fecf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Framing error clear flag"]
    #[inline(always)]
    pub fn set_fecf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Noise detected clear flag"]
    #[inline(always)]
    pub const fn ncf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Noise detected clear flag"]
    #[inline(always)]
    pub fn set_ncf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Overrun error clear flag"]
    #[inline(always)]
    pub const fn orecf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun error clear flag"]
    #[inline(always)]
    pub fn set_orecf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Idle line detected clear flag"]
    #[inline(always)]
    pub const fn idlecf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Idle line detected clear flag"]
    #[inline(always)]
    pub fn set_idlecf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "TXFECF"]
    #[inline(always)]
    pub const fn txfecf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TXFECF"]
    #[inline(always)]
    pub fn set_txfecf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmission complete clear flag"]
    #[inline(always)]
    pub const fn tccf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission complete clear flag"]
    #[inline(always)]
    pub fn set_tccf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "TCBGTCF"]
    #[inline(always)]
    pub const fn tcbgtcf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "TCBGTCF"]
    #[inline(always)]
    pub fn set_tcbgtcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "LIN break detection clear flag"]
    #[inline(always)]
    pub const fn lbdcf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LIN break detection clear flag"]
    #[inline(always)]
    pub fn set_lbdcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CTS clear flag"]
    #[inline(always)]
    pub const fn ctscf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CTS clear flag"]
    #[inline(always)]
    pub fn set_ctscf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Receiver timeout clear flag"]
    #[inline(always)]
    pub const fn rtocf(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver timeout clear flag"]
    #[inline(always)]
    pub fn set_rtocf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "End of block clear flag"]
    #[inline(always)]
    pub const fn eobcf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "End of block clear flag"]
    #[inline(always)]
    pub fn set_eobcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "UDRCF"]
    #[inline(always)]
    pub const fn udrcf(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "UDRCF"]
    #[inline(always)]
    pub fn set_udrcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Character match clear flag"]
    #[inline(always)]
    pub const fn cmcf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Character match clear flag"]
    #[inline(always)]
    pub fn set_cmcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Wakeup from Stop mode clear flag"]
    #[inline(always)]
    pub const fn wucf(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup from Stop mode clear flag"]
    #[inline(always)]
    pub fn set_wucf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        Icr(0)
    }
}
#[doc = "Interrupt & status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc = "PE"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PE"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FE"]
    #[inline(always)]
    pub const fn fe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FE"]
    #[inline(always)]
    pub fn set_fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NF"]
    #[inline(always)]
    pub const fn nf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NF"]
    #[inline(always)]
    pub fn set_nf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ORE"]
    #[inline(always)]
    pub const fn ore(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ORE"]
    #[inline(always)]
    pub fn set_ore(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IDLE"]
    #[inline(always)]
    pub const fn idle(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IDLE"]
    #[inline(always)]
    pub fn set_idle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "RXNE"]
    #[inline(always)]
    pub const fn rxne(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RXNE"]
    #[inline(always)]
    pub fn set_rxne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "TC"]
    #[inline(always)]
    pub const fn tc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "TC"]
    #[inline(always)]
    pub fn set_tc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "TXE"]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "TXE"]
    #[inline(always)]
    pub fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "LBDF"]
    #[inline(always)]
    pub const fn lbdf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LBDF"]
    #[inline(always)]
    pub fn set_lbdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CTSIF"]
    #[inline(always)]
    pub const fn ctsif(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CTSIF"]
    #[inline(always)]
    pub fn set_ctsif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CTS"]
    #[inline(always)]
    pub const fn cts(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CTS"]
    #[inline(always)]
    pub fn set_cts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RTOF"]
    #[inline(always)]
    pub const fn rtof(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "RTOF"]
    #[inline(always)]
    pub fn set_rtof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "EOBF"]
    #[inline(always)]
    pub const fn eobf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "EOBF"]
    #[inline(always)]
    pub fn set_eobf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "UDR"]
    #[inline(always)]
    pub const fn udr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "UDR"]
    #[inline(always)]
    pub fn set_udr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "ABRE"]
    #[inline(always)]
    pub const fn abre(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "ABRE"]
    #[inline(always)]
    pub fn set_abre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ABRF"]
    #[inline(always)]
    pub const fn abrf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ABRF"]
    #[inline(always)]
    pub fn set_abrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "BUSY"]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "BUSY"]
    #[inline(always)]
    pub fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "CMF"]
    #[inline(always)]
    pub const fn cmf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "CMF"]
    #[inline(always)]
    pub fn set_cmf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "SBKF"]
    #[inline(always)]
    pub const fn sbkf(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "SBKF"]
    #[inline(always)]
    pub fn set_sbkf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "RWU"]
    #[inline(always)]
    pub const fn rwu(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "RWU"]
    #[inline(always)]
    pub fn set_rwu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "WUF"]
    #[inline(always)]
    pub const fn wuf(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "WUF"]
    #[inline(always)]
    pub fn set_wuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "TEACK"]
    #[inline(always)]
    pub const fn teack(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "TEACK"]
    #[inline(always)]
    pub fn set_teack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "REACK"]
    #[inline(always)]
    pub const fn reack(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "REACK"]
    #[inline(always)]
    pub fn set_reack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "TXFE"]
    #[inline(always)]
    pub const fn txfe(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TXFE"]
    #[inline(always)]
    pub fn set_txfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "RXFF"]
    #[inline(always)]
    pub const fn rxff(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "RXFF"]
    #[inline(always)]
    pub fn set_rxff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TCBGT"]
    #[inline(always)]
    pub const fn tcbgt(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TCBGT"]
    #[inline(always)]
    pub fn set_tcbgt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "RXFT"]
    #[inline(always)]
    pub const fn rxft(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "RXFT"]
    #[inline(always)]
    pub fn set_rxft(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "TXFT"]
    #[inline(always)]
    pub const fn txft(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "TXFT"]
    #[inline(always)]
    pub fn set_txft(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Isr {
    #[inline(always)]
    fn default() -> Isr {
        Isr(0)
    }
}
#[doc = "USART prescaler register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Presc(pub u32);
impl Presc {
    #[doc = "PRESCALER"]
    #[inline(always)]
    pub const fn prescaler(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PRESCALER"]
    #[inline(always)]
    pub fn set_prescaler(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Presc {
    #[inline(always)]
    fn default() -> Presc {
        Presc(0)
    }
}
#[doc = "Receive data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdr(pub u32);
impl Rdr {
    #[doc = "Receive data value"]
    #[inline(always)]
    pub const fn rdr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Receive data value"]
    #[inline(always)]
    pub fn set_rdr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Rdr {
    #[inline(always)]
    fn default() -> Rdr {
        Rdr(0)
    }
}
#[doc = "Request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rqr(pub u32);
impl Rqr {
    #[doc = "Auto baud rate request"]
    #[inline(always)]
    pub const fn abrrq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Auto baud rate request"]
    #[inline(always)]
    pub fn set_abrrq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Send break request"]
    #[inline(always)]
    pub const fn sbkrq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Send break request"]
    #[inline(always)]
    pub fn set_sbkrq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mute mode request"]
    #[inline(always)]
    pub const fn mmrq(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mute mode request"]
    #[inline(always)]
    pub fn set_mmrq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive data flush request"]
    #[inline(always)]
    pub const fn rxfrq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive data flush request"]
    #[inline(always)]
    pub fn set_rxfrq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit data flush request"]
    #[inline(always)]
    pub const fn txfrq(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit data flush request"]
    #[inline(always)]
    pub fn set_txfrq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Rqr {
    #[inline(always)]
    fn default() -> Rqr {
        Rqr(0)
    }
}
#[doc = "Receiver timeout register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtor(pub u32);
impl Rtor {
    #[doc = "Receiver timeout value"]
    #[inline(always)]
    pub const fn rto(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Receiver timeout value"]
    #[inline(always)]
    pub fn set_rto(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Block Length"]
    #[inline(always)]
    pub const fn blen(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Block Length"]
    #[inline(always)]
    pub fn set_blen(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Rtor {
    #[inline(always)]
    fn default() -> Rtor {
        Rtor(0)
    }
}
#[doc = "Transmit data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr(pub u32);
impl Tdr {
    #[doc = "Transmit data value"]
    #[inline(always)]
    pub const fn tdr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Transmit data value"]
    #[inline(always)]
    pub fn set_tdr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Tdr {
    #[inline(always)]
    fn default() -> Tdr {
        Tdr(0)
    }
}

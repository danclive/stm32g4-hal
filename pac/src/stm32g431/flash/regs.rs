#[doc = "Access control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acr(pub u32);
impl Acr {
    #[doc = "Latency"]
    #[inline(always)]
    pub const fn latency(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Latency"]
    #[inline(always)]
    pub fn set_latency(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Prefetch enable"]
    #[inline(always)]
    pub const fn prften(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Prefetch enable"]
    #[inline(always)]
    pub fn set_prften(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Instruction cache enable"]
    #[inline(always)]
    pub const fn icen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Instruction cache enable"]
    #[inline(always)]
    pub fn set_icen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Data cache enable"]
    #[inline(always)]
    pub const fn dcen(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Data cache enable"]
    #[inline(always)]
    pub fn set_dcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Instruction cache reset"]
    #[inline(always)]
    pub const fn icrst(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Instruction cache reset"]
    #[inline(always)]
    pub fn set_icrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Data cache reset"]
    #[inline(always)]
    pub const fn dcrst(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Data cache reset"]
    #[inline(always)]
    pub fn set_dcrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Flash Power-down mode during Low-power run mode"]
    #[inline(always)]
    pub const fn run_pd(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Flash Power-down mode during Low-power run mode"]
    #[inline(always)]
    pub fn set_run_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Flash Power-down mode during Low-power sleep mode"]
    #[inline(always)]
    pub const fn sleep_pd(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Flash Power-down mode during Low-power sleep mode"]
    #[inline(always)]
    pub fn set_sleep_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Debug software enable"]
    #[inline(always)]
    pub const fn dbg_swen(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Debug software enable"]
    #[inline(always)]
    pub fn set_dbg_swen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Acr {
    #[inline(always)]
    fn default() -> Acr {
        Acr(0)
    }
}
#[doc = "Flash control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Programming"]
    #[inline(always)]
    pub const fn pg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Programming"]
    #[inline(always)]
    pub fn set_pg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Page erase"]
    #[inline(always)]
    pub const fn per(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Page erase"]
    #[inline(always)]
    pub fn set_per(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bank 1 Mass erase"]
    #[inline(always)]
    pub const fn mer1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bank 1 Mass erase"]
    #[inline(always)]
    pub fn set_mer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Page number"]
    #[inline(always)]
    pub const fn pnb(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x7f;
        val as u8
    }
    #[doc = "Page number"]
    #[inline(always)]
    pub fn set_pnb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 3usize)) | (((val as u32) & 0x7f) << 3usize);
    }
    #[doc = "Start"]
    #[inline(always)]
    pub const fn strt(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Start"]
    #[inline(always)]
    pub fn set_strt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Options modification start"]
    #[inline(always)]
    pub const fn optstrt(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Options modification start"]
    #[inline(always)]
    pub fn set_optstrt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Fast programming"]
    #[inline(always)]
    pub const fn fstpg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Fast programming"]
    #[inline(always)]
    pub fn set_fstpg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "End of operation interrupt enable"]
    #[inline(always)]
    pub const fn eopie(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "End of operation interrupt enable"]
    #[inline(always)]
    pub fn set_eopie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Error interrupt enable"]
    #[inline(always)]
    pub const fn errie(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Error interrupt enable"]
    #[inline(always)]
    pub fn set_errie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "PCROP read error interrupt enable"]
    #[inline(always)]
    pub const fn rderrie(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "PCROP read error interrupt enable"]
    #[inline(always)]
    pub fn set_rderrie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Force the option byte loading"]
    #[inline(always)]
    pub const fn obl_launch(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Force the option byte loading"]
    #[inline(always)]
    pub fn set_obl_launch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "SEC_PROT1"]
    #[inline(always)]
    pub const fn sec_prot1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "SEC_PROT1"]
    #[inline(always)]
    pub fn set_sec_prot1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Options Lock"]
    #[inline(always)]
    pub const fn optlock(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Options Lock"]
    #[inline(always)]
    pub fn set_optlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "FLASH_CR Lock"]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FLASH_CR Lock"]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
#[doc = "Flash ECC register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccr(pub u32);
impl Eccr {
    #[doc = "ECC fail address"]
    #[inline(always)]
    pub const fn addr_ecc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "ECC fail address"]
    #[inline(always)]
    pub fn set_addr_ecc(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
    }
    #[doc = "BK_ECC"]
    #[inline(always)]
    pub const fn bk_ecc(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "BK_ECC"]
    #[inline(always)]
    pub fn set_bk_ecc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "SYSF_ECC"]
    #[inline(always)]
    pub const fn sysf_ecc(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "SYSF_ECC"]
    #[inline(always)]
    pub fn set_sysf_ecc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "ECCIE"]
    #[inline(always)]
    pub const fn eccie(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "ECCIE"]
    #[inline(always)]
    pub fn set_eccie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "ECC correction"]
    #[inline(always)]
    pub const fn eccc2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "ECC correction"]
    #[inline(always)]
    pub fn set_eccc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "ECC2 detection"]
    #[inline(always)]
    pub const fn eccd2(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "ECC2 detection"]
    #[inline(always)]
    pub fn set_eccd2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "ECC correction"]
    #[inline(always)]
    pub const fn eccc(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "ECC correction"]
    #[inline(always)]
    pub fn set_eccc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "ECC detection"]
    #[inline(always)]
    pub const fn eccd(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "ECC detection"]
    #[inline(always)]
    pub fn set_eccd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Eccr {
    #[inline(always)]
    fn default() -> Eccr {
        Eccr(0)
    }
}
#[doc = "Flash key register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyr(pub u32);
impl Keyr {
    #[doc = "KEYR"]
    #[inline(always)]
    pub const fn keyr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "KEYR"]
    #[inline(always)]
    pub fn set_keyr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Keyr {
    #[inline(always)]
    fn default() -> Keyr {
        Keyr(0)
    }
}
#[doc = "Option byte key register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Optkeyr(pub u32);
impl Optkeyr {
    #[doc = "Option byte key"]
    #[inline(always)]
    pub const fn optkeyr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Option byte key"]
    #[inline(always)]
    pub fn set_optkeyr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Optkeyr {
    #[inline(always)]
    fn default() -> Optkeyr {
        Optkeyr(0)
    }
}
#[doc = "Flash option register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Optr(pub u32);
impl Optr {
    #[doc = "Read protection level"]
    #[inline(always)]
    pub const fn rdp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Read protection level"]
    #[inline(always)]
    pub fn set_rdp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "BOR reset Level"]
    #[inline(always)]
    pub const fn bor_lev(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "BOR reset Level"]
    #[inline(always)]
    pub fn set_bor_lev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "nRST_STOP"]
    #[inline(always)]
    pub const fn n_rst_stop(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "nRST_STOP"]
    #[inline(always)]
    pub fn set_n_rst_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "nRST_STDBY"]
    #[inline(always)]
    pub const fn n_rst_stdby(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "nRST_STDBY"]
    #[inline(always)]
    pub fn set_n_rst_stdby(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "nRST_SHDW"]
    #[inline(always)]
    pub const fn n_rst_shdw(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "nRST_SHDW"]
    #[inline(always)]
    pub fn set_n_rst_shdw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Independent watchdog selection"]
    #[inline(always)]
    pub const fn idwg_sw(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Independent watchdog selection"]
    #[inline(always)]
    pub fn set_idwg_sw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    pub const fn iwdg_stop(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    pub fn set_iwdg_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Independent watchdog counter freeze in Standby mode"]
    #[inline(always)]
    pub const fn iwdg_stdby(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Independent watchdog counter freeze in Standby mode"]
    #[inline(always)]
    pub fn set_iwdg_stdby(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Window watchdog selection"]
    #[inline(always)]
    pub const fn wwdg_sw(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Window watchdog selection"]
    #[inline(always)]
    pub fn set_wwdg_sw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Boot configuration"]
    #[inline(always)]
    pub const fn n_boot1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Boot configuration"]
    #[inline(always)]
    pub fn set_n_boot1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "SRAM2 parity check enable"]
    #[inline(always)]
    pub const fn sram2_pe(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "SRAM2 parity check enable"]
    #[inline(always)]
    pub fn set_sram2_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SRAM2 Erase when system reset"]
    #[inline(always)]
    pub const fn sram2_rst(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SRAM2 Erase when system reset"]
    #[inline(always)]
    pub fn set_sram2_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "nSWBOOT0"]
    #[inline(always)]
    pub const fn n_swboot0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "nSWBOOT0"]
    #[inline(always)]
    pub fn set_n_swboot0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "nBOOT0"]
    #[inline(always)]
    pub const fn n_boot0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "nBOOT0"]
    #[inline(always)]
    pub fn set_n_boot0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "NRST_MODE"]
    #[inline(always)]
    pub const fn nrst_mode(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "NRST_MODE"]
    #[inline(always)]
    pub fn set_nrst_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "IRHEN"]
    #[inline(always)]
    pub const fn irhen(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "IRHEN"]
    #[inline(always)]
    pub fn set_irhen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Optr {
    #[inline(always)]
    fn default() -> Optr {
        Optr(0)
    }
}
#[doc = "Flash Bank 1 PCROP End address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcrop1er(pub u32);
impl Pcrop1er {
    #[doc = "Bank 1 PCROP area end offset"]
    #[inline(always)]
    pub const fn pcrop1_end(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Bank 1 PCROP area end offset"]
    #[inline(always)]
    pub fn set_pcrop1_end(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "PCROP area preserved when RDP level decreased"]
    #[inline(always)]
    pub const fn pcrop_rdp(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "PCROP area preserved when RDP level decreased"]
    #[inline(always)]
    pub fn set_pcrop_rdp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pcrop1er {
    #[inline(always)]
    fn default() -> Pcrop1er {
        Pcrop1er(0)
    }
}
#[doc = "Flash Bank 1 PCROP Start address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcrop1sr(pub u32);
impl Pcrop1sr {
    #[doc = "Bank 1 PCROP area start offset"]
    #[inline(always)]
    pub const fn pcrop1_strt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Bank 1 PCROP area start offset"]
    #[inline(always)]
    pub fn set_pcrop1_strt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Pcrop1sr {
    #[inline(always)]
    fn default() -> Pcrop1sr {
        Pcrop1sr(0)
    }
}
#[doc = "Power down key register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdkeyr(pub u32);
impl Pdkeyr {
    #[doc = "RUN_PD in FLASH_ACR key"]
    #[inline(always)]
    pub const fn pdkeyr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "RUN_PD in FLASH_ACR key"]
    #[inline(always)]
    pub fn set_pdkeyr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pdkeyr {
    #[inline(always)]
    fn default() -> Pdkeyr {
        Pdkeyr(0)
    }
}
#[doc = "securable area bank1 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sec1r(pub u32);
impl Sec1r {
    #[doc = "SEC_SIZE1"]
    #[inline(always)]
    pub const fn sec_size1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEC_SIZE1"]
    #[inline(always)]
    pub fn set_sec_size1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "BOOT_LOCK"]
    #[inline(always)]
    pub const fn boot_lock(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "BOOT_LOCK"]
    #[inline(always)]
    pub fn set_boot_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Sec1r {
    #[inline(always)]
    fn default() -> Sec1r {
        Sec1r(0)
    }
}
#[doc = "Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "End of operation"]
    #[inline(always)]
    pub const fn eop(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "End of operation"]
    #[inline(always)]
    pub fn set_eop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Operation error"]
    #[inline(always)]
    pub const fn operr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Operation error"]
    #[inline(always)]
    pub fn set_operr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Programming error"]
    #[inline(always)]
    pub const fn progerr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Programming error"]
    #[inline(always)]
    pub fn set_progerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write protected error"]
    #[inline(always)]
    pub const fn wrperr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write protected error"]
    #[inline(always)]
    pub fn set_wrperr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Programming alignment error"]
    #[inline(always)]
    pub const fn pgaerr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Programming alignment error"]
    #[inline(always)]
    pub fn set_pgaerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Size error"]
    #[inline(always)]
    pub const fn sizerr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Size error"]
    #[inline(always)]
    pub fn set_sizerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Programming sequence error"]
    #[inline(always)]
    pub const fn pgserr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Programming sequence error"]
    #[inline(always)]
    pub fn set_pgserr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Fast programming data miss error"]
    #[inline(always)]
    pub const fn miserr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Fast programming data miss error"]
    #[inline(always)]
    pub fn set_miserr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Fast programming error"]
    #[inline(always)]
    pub const fn fasterr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Fast programming error"]
    #[inline(always)]
    pub fn set_fasterr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "PCROP read error"]
    #[inline(always)]
    pub const fn rderr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PCROP read error"]
    #[inline(always)]
    pub fn set_rderr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Option validity error"]
    #[inline(always)]
    pub const fn optverr(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Option validity error"]
    #[inline(always)]
    pub fn set_optverr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Busy"]
    #[inline(always)]
    pub const fn bsy(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Busy"]
    #[inline(always)]
    pub fn set_bsy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
#[doc = "Flash Bank 1 WRP area A address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wrp1ar(pub u32);
impl Wrp1ar {
    #[doc = "Bank 1 WRP first area start offset"]
    #[inline(always)]
    pub const fn wrp1a_strt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Bank 1 WRP first area start offset"]
    #[inline(always)]
    pub fn set_wrp1a_strt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Bank 1 WRP first area A end offset"]
    #[inline(always)]
    pub const fn wrp1a_end(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Bank 1 WRP first area A end offset"]
    #[inline(always)]
    pub fn set_wrp1a_end(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Wrp1ar {
    #[inline(always)]
    fn default() -> Wrp1ar {
        Wrp1ar(0)
    }
}
#[doc = "Flash Bank 1 WRP area B address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wrp1br(pub u32);
impl Wrp1br {
    #[doc = "Bank 1 WRP second area B end offset"]
    #[inline(always)]
    pub const fn wrp1b_strt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Bank 1 WRP second area B end offset"]
    #[inline(always)]
    pub fn set_wrp1b_strt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Bank 1 WRP second area B start offset"]
    #[inline(always)]
    pub const fn wrp1b_end(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Bank 1 WRP second area B start offset"]
    #[inline(always)]
    pub fn set_wrp1b_end(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Wrp1br {
    #[inline(always)]
    fn default() -> Wrp1br {
        Wrp1br(0)
    }
}

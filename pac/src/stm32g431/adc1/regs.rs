#[doc = "Analog Watchdog 2 Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Awd2cr(pub u32);
impl Awd2cr {
    #[doc = "AWD2CH"]
    #[inline(always)]
    pub const fn awd2ch(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "AWD2CH"]
    #[inline(always)]
    pub fn set_awd2ch(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
    }
}
impl Default for Awd2cr {
    #[inline(always)]
    fn default() -> Awd2cr {
        Awd2cr(0)
    }
}
#[doc = "Analog Watchdog 3 Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Awd3cr(pub u32);
impl Awd3cr {
    #[doc = "AWD3CH"]
    #[inline(always)]
    pub const fn awd3ch(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "AWD3CH"]
    #[inline(always)]
    pub fn set_awd3ch(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
    }
}
impl Default for Awd3cr {
    #[inline(always)]
    fn default() -> Awd3cr {
        Awd3cr(0)
    }
}
#[doc = "Calibration Factors"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Calfact(pub u32);
impl Calfact {
    #[doc = "CALFACT_S"]
    #[inline(always)]
    pub const fn calfact_s(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "CALFACT_S"]
    #[inline(always)]
    pub fn set_calfact_s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "CALFACT_D"]
    #[inline(always)]
    pub const fn calfact_d(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "CALFACT_D"]
    #[inline(always)]
    pub fn set_calfact_d(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Calfact {
    #[inline(always)]
    fn default() -> Calfact {
        Calfact(0)
    }
}
#[doc = "configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgr(pub u32);
impl Cfgr {
    #[doc = "DMAEN"]
    #[inline(always)]
    pub const fn dmaen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMAEN"]
    #[inline(always)]
    pub fn set_dmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMACFG"]
    #[inline(always)]
    pub const fn dmacfg(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMACFG"]
    #[inline(always)]
    pub fn set_dmacfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RES"]
    #[inline(always)]
    pub const fn res(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "RES"]
    #[inline(always)]
    pub fn set_res(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "External trigger selection for regular group"]
    #[inline(always)]
    pub const fn extsel(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "External trigger selection for regular group"]
    #[inline(always)]
    pub fn set_extsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "EXTEN"]
    #[inline(always)]
    pub const fn exten(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "EXTEN"]
    #[inline(always)]
    pub fn set_exten(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "OVRMOD"]
    #[inline(always)]
    pub const fn ovrmod(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "OVRMOD"]
    #[inline(always)]
    pub fn set_ovrmod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CONT"]
    #[inline(always)]
    pub const fn cont(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CONT"]
    #[inline(always)]
    pub fn set_cont(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "AUTDLY"]
    #[inline(always)]
    pub const fn autdly(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "AUTDLY"]
    #[inline(always)]
    pub fn set_autdly(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ALIGN"]
    #[inline(always)]
    pub const fn align(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ALIGN"]
    #[inline(always)]
    pub fn set_align(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DISCEN"]
    #[inline(always)]
    pub const fn discen(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DISCEN"]
    #[inline(always)]
    pub fn set_discen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DISCNUM"]
    #[inline(always)]
    pub const fn discnum(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x07;
        val as u8
    }
    #[doc = "DISCNUM"]
    #[inline(always)]
    pub fn set_discnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
    }
    #[doc = "JDISCEN"]
    #[inline(always)]
    pub const fn jdiscen(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "JDISCEN"]
    #[inline(always)]
    pub fn set_jdiscen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "JQM"]
    #[inline(always)]
    pub const fn jqm(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "JQM"]
    #[inline(always)]
    pub fn set_jqm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "AWD1SGL"]
    #[inline(always)]
    pub const fn awd1sgl(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "AWD1SGL"]
    #[inline(always)]
    pub fn set_awd1sgl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "AWD1EN"]
    #[inline(always)]
    pub const fn awd1en(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "AWD1EN"]
    #[inline(always)]
    pub fn set_awd1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "JAWD1EN"]
    #[inline(always)]
    pub const fn jawd1en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "JAWD1EN"]
    #[inline(always)]
    pub fn set_jawd1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "JAUTO"]
    #[inline(always)]
    pub const fn jauto(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "JAUTO"]
    #[inline(always)]
    pub fn set_jauto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Analog watchdog 1 channel selection"]
    #[inline(always)]
    pub const fn awd1ch(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[doc = "Analog watchdog 1 channel selection"]
    #[inline(always)]
    pub fn set_awd1ch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
    #[doc = "Injected Queue disable"]
    #[inline(always)]
    pub const fn jqdis(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Injected Queue disable"]
    #[inline(always)]
    pub fn set_jqdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cfgr {
    #[inline(always)]
    fn default() -> Cfgr {
        Cfgr(0)
    }
}
#[doc = "configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgr2(pub u32);
impl Cfgr2 {
    #[doc = "DMAEN"]
    #[inline(always)]
    pub const fn rovse(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMAEN"]
    #[inline(always)]
    pub fn set_rovse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMACFG"]
    #[inline(always)]
    pub const fn jovse(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMACFG"]
    #[inline(always)]
    pub fn set_jovse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RES"]
    #[inline(always)]
    pub const fn ovsr(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "RES"]
    #[inline(always)]
    pub fn set_ovsr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "ALIGN"]
    #[inline(always)]
    pub const fn ovss(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x0f;
        val as u8
    }
    #[doc = "ALIGN"]
    #[inline(always)]
    pub fn set_ovss(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
    }
    #[doc = "Triggered Regular Oversampling"]
    #[inline(always)]
    pub const fn trovs(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Triggered Regular Oversampling"]
    #[inline(always)]
    pub fn set_trovs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "EXTEN"]
    #[inline(always)]
    pub const fn rovsm(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "EXTEN"]
    #[inline(always)]
    pub fn set_rovsm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "GCOMP"]
    #[inline(always)]
    pub const fn gcomp(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "GCOMP"]
    #[inline(always)]
    pub fn set_gcomp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SWTRIG"]
    #[inline(always)]
    pub const fn swtrig(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SWTRIG"]
    #[inline(always)]
    pub fn set_swtrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "BULB"]
    #[inline(always)]
    pub const fn bulb(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "BULB"]
    #[inline(always)]
    pub fn set_bulb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "SMPTRIG"]
    #[inline(always)]
    pub const fn smptrig(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "SMPTRIG"]
    #[inline(always)]
    pub fn set_smptrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Cfgr2 {
    #[inline(always)]
    fn default() -> Cfgr2 {
        Cfgr2(0)
    }
}
#[doc = "control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "ADEN"]
    #[inline(always)]
    pub const fn aden(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ADEN"]
    #[inline(always)]
    pub fn set_aden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADDIS"]
    #[inline(always)]
    pub const fn addis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADDIS"]
    #[inline(always)]
    pub fn set_addis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADSTART"]
    #[inline(always)]
    pub const fn adstart(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADSTART"]
    #[inline(always)]
    pub fn set_adstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "JADSTART"]
    #[inline(always)]
    pub const fn jadstart(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "JADSTART"]
    #[inline(always)]
    pub fn set_jadstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ADSTP"]
    #[inline(always)]
    pub const fn adstp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ADSTP"]
    #[inline(always)]
    pub fn set_adstp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "JADSTP"]
    #[inline(always)]
    pub const fn jadstp(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "JADSTP"]
    #[inline(always)]
    pub fn set_jadstp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "ADVREGEN"]
    #[inline(always)]
    pub const fn advregen(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "ADVREGEN"]
    #[inline(always)]
    pub fn set_advregen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DEEPPWD"]
    #[inline(always)]
    pub const fn deeppwd(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DEEPPWD"]
    #[inline(always)]
    pub fn set_deeppwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "ADCALDIF"]
    #[inline(always)]
    pub const fn adcaldif(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "ADCALDIF"]
    #[inline(always)]
    pub fn set_adcaldif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "ADCAL"]
    #[inline(always)]
    pub const fn adcal(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "ADCAL"]
    #[inline(always)]
    pub fn set_adcal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
#[doc = "Differential Mode Selection Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Difsel(pub u32);
impl Difsel {
    #[doc = "Differential mode for channels 0"]
    #[inline(always)]
    pub const fn difsel_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Differential mode for channels 0"]
    #[inline(always)]
    pub fn set_difsel_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub const fn difsel_1_18(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn set_difsel_1_18(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 1usize)) | (((val as u32) & 0x0003_ffff) << 1usize);
    }
}
impl Default for Difsel {
    #[inline(always)]
    fn default() -> Difsel {
        Difsel(0)
    }
}
#[doc = "regular Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc = "Regular Data converted"]
    #[inline(always)]
    pub const fn rdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Regular Data converted"]
    #[inline(always)]
    pub fn set_rdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr {
    #[inline(always)]
    fn default() -> Dr {
        Dr(0)
    }
}
#[doc = "Gain compensation Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gcomp(pub u32);
impl Gcomp {
    #[doc = "GCOMPCOEFF"]
    #[inline(always)]
    pub const fn gcompcoeff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "GCOMPCOEFF"]
    #[inline(always)]
    pub fn set_gcompcoeff(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for Gcomp {
    #[inline(always)]
    fn default() -> Gcomp {
        Gcomp(0)
    }
}
#[doc = "interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "ADRDYIE"]
    #[inline(always)]
    pub const fn adrdyie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ADRDYIE"]
    #[inline(always)]
    pub fn set_adrdyie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "EOSMPIE"]
    #[inline(always)]
    pub const fn eosmpie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "EOSMPIE"]
    #[inline(always)]
    pub fn set_eosmpie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "EOCIE"]
    #[inline(always)]
    pub const fn eocie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "EOCIE"]
    #[inline(always)]
    pub fn set_eocie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "EOSIE"]
    #[inline(always)]
    pub const fn eosie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "EOSIE"]
    #[inline(always)]
    pub fn set_eosie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "OVRIE"]
    #[inline(always)]
    pub const fn ovrie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "OVRIE"]
    #[inline(always)]
    pub fn set_ovrie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "JEOCIE"]
    #[inline(always)]
    pub const fn jeocie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "JEOCIE"]
    #[inline(always)]
    pub fn set_jeocie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "JEOSIE"]
    #[inline(always)]
    pub const fn jeosie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "JEOSIE"]
    #[inline(always)]
    pub fn set_jeosie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "AWD1IE"]
    #[inline(always)]
    pub const fn awd1ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "AWD1IE"]
    #[inline(always)]
    pub fn set_awd1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "AWD2IE"]
    #[inline(always)]
    pub const fn awd2ie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "AWD2IE"]
    #[inline(always)]
    pub fn set_awd2ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "AWD3IE"]
    #[inline(always)]
    pub const fn awd3ie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "AWD3IE"]
    #[inline(always)]
    pub fn set_awd3ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "JQOVFIE"]
    #[inline(always)]
    pub const fn jqovfie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "JQOVFIE"]
    #[inline(always)]
    pub fn set_jqovfie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Ier {
    #[inline(always)]
    fn default() -> Ier {
        Ier(0)
    }
}
#[doc = "interrupt and status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc = "ADRDY"]
    #[inline(always)]
    pub const fn adrdy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ADRDY"]
    #[inline(always)]
    pub fn set_adrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "EOSMP"]
    #[inline(always)]
    pub const fn eosmp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "EOSMP"]
    #[inline(always)]
    pub fn set_eosmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "EOC"]
    #[inline(always)]
    pub const fn eoc(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "EOC"]
    #[inline(always)]
    pub fn set_eoc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "EOS"]
    #[inline(always)]
    pub const fn eos(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "EOS"]
    #[inline(always)]
    pub fn set_eos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "OVR"]
    #[inline(always)]
    pub const fn ovr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "OVR"]
    #[inline(always)]
    pub fn set_ovr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "JEOC"]
    #[inline(always)]
    pub const fn jeoc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "JEOC"]
    #[inline(always)]
    pub fn set_jeoc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "JEOS"]
    #[inline(always)]
    pub const fn jeos(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "JEOS"]
    #[inline(always)]
    pub fn set_jeos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "AWD1"]
    #[inline(always)]
    pub const fn awd1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "AWD1"]
    #[inline(always)]
    pub fn set_awd1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "AWD2"]
    #[inline(always)]
    pub const fn awd2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "AWD2"]
    #[inline(always)]
    pub fn set_awd2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "AWD3"]
    #[inline(always)]
    pub const fn awd3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "AWD3"]
    #[inline(always)]
    pub fn set_awd3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "JQOVF"]
    #[inline(always)]
    pub const fn jqovf(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "JQOVF"]
    #[inline(always)]
    pub fn set_jqovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Isr {
    #[inline(always)]
    fn default() -> Isr {
        Isr(0)
    }
}
#[doc = "injected data register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jdr1(pub u32);
impl Jdr1 {
    #[doc = "JDATA1"]
    #[inline(always)]
    pub const fn jdata1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "JDATA1"]
    #[inline(always)]
    pub fn set_jdata1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Jdr1 {
    #[inline(always)]
    fn default() -> Jdr1 {
        Jdr1(0)
    }
}
#[doc = "injected data register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jdr2(pub u32);
impl Jdr2 {
    #[doc = "JDATA2"]
    #[inline(always)]
    pub const fn jdata2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "JDATA2"]
    #[inline(always)]
    pub fn set_jdata2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Jdr2 {
    #[inline(always)]
    fn default() -> Jdr2 {
        Jdr2(0)
    }
}
#[doc = "injected data register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jdr3(pub u32);
impl Jdr3 {
    #[doc = "JDATA3"]
    #[inline(always)]
    pub const fn jdata3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "JDATA3"]
    #[inline(always)]
    pub fn set_jdata3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Jdr3 {
    #[inline(always)]
    fn default() -> Jdr3 {
        Jdr3(0)
    }
}
#[doc = "injected data register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jdr4(pub u32);
impl Jdr4 {
    #[doc = "JDATA4"]
    #[inline(always)]
    pub const fn jdata4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "JDATA4"]
    #[inline(always)]
    pub fn set_jdata4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Jdr4 {
    #[inline(always)]
    fn default() -> Jdr4 {
        Jdr4(0)
    }
}
#[doc = "injected sequence register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jsqr(pub u32);
impl Jsqr {
    #[doc = "JL"]
    #[inline(always)]
    pub const fn jl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "JL"]
    #[inline(always)]
    pub fn set_jl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "JEXTSEL"]
    #[inline(always)]
    pub const fn jextsel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x1f;
        val as u8
    }
    #[doc = "JEXTSEL"]
    #[inline(always)]
    pub fn set_jextsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
    }
    #[doc = "JEXTEN"]
    #[inline(always)]
    pub const fn jexten(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "JEXTEN"]
    #[inline(always)]
    pub fn set_jexten(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[doc = "JSQ1"]
    #[inline(always)]
    pub const fn jsq1(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[doc = "JSQ1"]
    #[inline(always)]
    pub fn set_jsq1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u32) & 0x1f) << 9usize);
    }
    #[doc = "JSQ2"]
    #[inline(always)]
    pub const fn jsq2(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x1f;
        val as u8
    }
    #[doc = "JSQ2"]
    #[inline(always)]
    pub fn set_jsq2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
    }
    #[doc = "JSQ3"]
    #[inline(always)]
    pub const fn jsq3(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x1f;
        val as u8
    }
    #[doc = "JSQ3"]
    #[inline(always)]
    pub fn set_jsq3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 21usize)) | (((val as u32) & 0x1f) << 21usize);
    }
    #[doc = "JSQ4"]
    #[inline(always)]
    pub const fn jsq4(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x1f;
        val as u8
    }
    #[doc = "JSQ4"]
    #[inline(always)]
    pub fn set_jsq4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 27usize)) | (((val as u32) & 0x1f) << 27usize);
    }
}
impl Default for Jsqr {
    #[inline(always)]
    fn default() -> Jsqr {
        Jsqr(0)
    }
}
#[doc = "offset register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ofr1(pub u32);
impl Ofr1 {
    #[doc = "OFFSET1"]
    #[inline(always)]
    pub const fn offset1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "OFFSET1"]
    #[inline(always)]
    pub fn set_offset1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "OFFSETPOS"]
    #[inline(always)]
    pub const fn offsetpos(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "OFFSETPOS"]
    #[inline(always)]
    pub fn set_offsetpos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SATEN"]
    #[inline(always)]
    pub const fn saten(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SATEN"]
    #[inline(always)]
    pub fn set_saten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "OFFSET1_CH"]
    #[inline(always)]
    pub const fn offset1_ch(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[doc = "OFFSET1_CH"]
    #[inline(always)]
    pub fn set_offset1_ch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
    #[doc = "OFFSET1_EN"]
    #[inline(always)]
    pub const fn offset1_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "OFFSET1_EN"]
    #[inline(always)]
    pub fn set_offset1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ofr1 {
    #[inline(always)]
    fn default() -> Ofr1 {
        Ofr1(0)
    }
}
#[doc = "offset register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ofr2(pub u32);
impl Ofr2 {
    #[doc = "OFFSET1"]
    #[inline(always)]
    pub const fn offset1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "OFFSET1"]
    #[inline(always)]
    pub fn set_offset1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "OFFSETPOS"]
    #[inline(always)]
    pub const fn offsetpos(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "OFFSETPOS"]
    #[inline(always)]
    pub fn set_offsetpos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SATEN"]
    #[inline(always)]
    pub const fn saten(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SATEN"]
    #[inline(always)]
    pub fn set_saten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "OFFSET1_CH"]
    #[inline(always)]
    pub const fn offset1_ch(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[doc = "OFFSET1_CH"]
    #[inline(always)]
    pub fn set_offset1_ch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
    #[doc = "OFFSET1_EN"]
    #[inline(always)]
    pub const fn offset1_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "OFFSET1_EN"]
    #[inline(always)]
    pub fn set_offset1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ofr2 {
    #[inline(always)]
    fn default() -> Ofr2 {
        Ofr2(0)
    }
}
#[doc = "offset register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ofr3(pub u32);
impl Ofr3 {
    #[doc = "OFFSET1"]
    #[inline(always)]
    pub const fn offset1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "OFFSET1"]
    #[inline(always)]
    pub fn set_offset1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "OFFSETPOS"]
    #[inline(always)]
    pub const fn offsetpos(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "OFFSETPOS"]
    #[inline(always)]
    pub fn set_offsetpos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SATEN"]
    #[inline(always)]
    pub const fn saten(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SATEN"]
    #[inline(always)]
    pub fn set_saten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "OFFSET1_CH"]
    #[inline(always)]
    pub const fn offset1_ch(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[doc = "OFFSET1_CH"]
    #[inline(always)]
    pub fn set_offset1_ch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
    #[doc = "OFFSET1_EN"]
    #[inline(always)]
    pub const fn offset1_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "OFFSET1_EN"]
    #[inline(always)]
    pub fn set_offset1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ofr3 {
    #[inline(always)]
    fn default() -> Ofr3 {
        Ofr3(0)
    }
}
#[doc = "offset register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ofr4(pub u32);
impl Ofr4 {
    #[doc = "OFFSET1"]
    #[inline(always)]
    pub const fn offset1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "OFFSET1"]
    #[inline(always)]
    pub fn set_offset1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "OFFSETPOS"]
    #[inline(always)]
    pub const fn offsetpos(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "OFFSETPOS"]
    #[inline(always)]
    pub fn set_offsetpos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SATEN"]
    #[inline(always)]
    pub const fn saten(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SATEN"]
    #[inline(always)]
    pub fn set_saten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "OFFSET1_CH"]
    #[inline(always)]
    pub const fn offset1_ch(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[doc = "OFFSET1_CH"]
    #[inline(always)]
    pub fn set_offset1_ch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
    #[doc = "OFFSET1_EN"]
    #[inline(always)]
    pub const fn offset1_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "OFFSET1_EN"]
    #[inline(always)]
    pub fn set_offset1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ofr4 {
    #[inline(always)]
    fn default() -> Ofr4 {
        Ofr4(0)
    }
}
#[doc = "sample time register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpr1(pub u32);
impl Smpr1 {
    #[doc = "SMP0"]
    #[inline(always)]
    pub const fn smp0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "SMP0"]
    #[inline(always)]
    pub fn set_smp0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "SMP1"]
    #[inline(always)]
    pub const fn smp1(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "SMP1"]
    #[inline(always)]
    pub fn set_smp1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "SMP2"]
    #[inline(always)]
    pub const fn smp2(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "SMP2"]
    #[inline(always)]
    pub fn set_smp2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "SMP3"]
    #[inline(always)]
    pub const fn smp3(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "SMP3"]
    #[inline(always)]
    pub fn set_smp3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "SMP4"]
    #[inline(always)]
    pub const fn smp4(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "SMP4"]
    #[inline(always)]
    pub fn set_smp4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "SMP5"]
    #[inline(always)]
    pub const fn smp5(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "SMP5"]
    #[inline(always)]
    pub fn set_smp5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "SMP6"]
    #[inline(always)]
    pub const fn smp6(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "SMP6"]
    #[inline(always)]
    pub fn set_smp6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "SMP7"]
    #[inline(always)]
    pub const fn smp7(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "SMP7"]
    #[inline(always)]
    pub fn set_smp7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "SMP8"]
    #[inline(always)]
    pub const fn smp8(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "SMP8"]
    #[inline(always)]
    pub fn set_smp8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "SMP9"]
    #[inline(always)]
    pub const fn smp9(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x07;
        val as u8
    }
    #[doc = "SMP9"]
    #[inline(always)]
    pub fn set_smp9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val as u32) & 0x07) << 27usize);
    }
    #[doc = "Addition of one clock cycle to the sampling time"]
    #[inline(always)]
    pub const fn smpplus(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Addition of one clock cycle to the sampling time"]
    #[inline(always)]
    pub fn set_smpplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Smpr1 {
    #[inline(always)]
    fn default() -> Smpr1 {
        Smpr1(0)
    }
}
#[doc = "sample time register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpr2(pub u32);
impl Smpr2 {
    #[doc = "SMP10"]
    #[inline(always)]
    pub const fn smp10(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "SMP10"]
    #[inline(always)]
    pub fn set_smp10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "SMP11"]
    #[inline(always)]
    pub const fn smp11(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "SMP11"]
    #[inline(always)]
    pub fn set_smp11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "SMP12"]
    #[inline(always)]
    pub const fn smp12(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "SMP12"]
    #[inline(always)]
    pub fn set_smp12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "SMP13"]
    #[inline(always)]
    pub const fn smp13(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "SMP13"]
    #[inline(always)]
    pub fn set_smp13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "SMP14"]
    #[inline(always)]
    pub const fn smp14(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "SMP14"]
    #[inline(always)]
    pub fn set_smp14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "SMP15"]
    #[inline(always)]
    pub const fn smp15(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "SMP15"]
    #[inline(always)]
    pub fn set_smp15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "SMP16"]
    #[inline(always)]
    pub const fn smp16(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "SMP16"]
    #[inline(always)]
    pub fn set_smp16(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "SMP17"]
    #[inline(always)]
    pub const fn smp17(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "SMP17"]
    #[inline(always)]
    pub fn set_smp17(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "SMP18"]
    #[inline(always)]
    pub const fn smp18(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "SMP18"]
    #[inline(always)]
    pub fn set_smp18(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
}
impl Default for Smpr2 {
    #[inline(always)]
    fn default() -> Smpr2 {
        Smpr2(0)
    }
}
#[doc = "regular sequence register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqr1(pub u32);
impl Sqr1 {
    #[doc = "Regular channel sequence length"]
    #[inline(always)]
    pub const fn l(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Regular channel sequence length"]
    #[inline(always)]
    pub fn set_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "SQ1"]
    #[inline(always)]
    pub const fn sq1(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x1f;
        val as u8
    }
    #[doc = "SQ1"]
    #[inline(always)]
    pub fn set_sq1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
    }
    #[doc = "SQ2"]
    #[inline(always)]
    pub const fn sq2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[doc = "SQ2"]
    #[inline(always)]
    pub fn set_sq2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[doc = "SQ3"]
    #[inline(always)]
    pub const fn sq3(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x1f;
        val as u8
    }
    #[doc = "SQ3"]
    #[inline(always)]
    pub fn set_sq3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
    }
    #[doc = "SQ4"]
    #[inline(always)]
    pub const fn sq4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "SQ4"]
    #[inline(always)]
    pub fn set_sq4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Sqr1 {
    #[inline(always)]
    fn default() -> Sqr1 {
        Sqr1(0)
    }
}
#[doc = "regular sequence register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqr2(pub u32);
impl Sqr2 {
    #[doc = "SQ5"]
    #[inline(always)]
    pub const fn sq5(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "SQ5"]
    #[inline(always)]
    pub fn set_sq5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "SQ6"]
    #[inline(always)]
    pub const fn sq6(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x1f;
        val as u8
    }
    #[doc = "SQ6"]
    #[inline(always)]
    pub fn set_sq6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
    }
    #[doc = "SQ7"]
    #[inline(always)]
    pub const fn sq7(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[doc = "SQ7"]
    #[inline(always)]
    pub fn set_sq7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[doc = "SQ8"]
    #[inline(always)]
    pub const fn sq8(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x1f;
        val as u8
    }
    #[doc = "SQ8"]
    #[inline(always)]
    pub fn set_sq8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
    }
    #[doc = "SQ9"]
    #[inline(always)]
    pub const fn sq9(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "SQ9"]
    #[inline(always)]
    pub fn set_sq9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Sqr2 {
    #[inline(always)]
    fn default() -> Sqr2 {
        Sqr2(0)
    }
}
#[doc = "regular sequence register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqr3(pub u32);
impl Sqr3 {
    #[doc = "SQ10"]
    #[inline(always)]
    pub const fn sq10(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "SQ10"]
    #[inline(always)]
    pub fn set_sq10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "SQ11"]
    #[inline(always)]
    pub const fn sq11(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x1f;
        val as u8
    }
    #[doc = "SQ11"]
    #[inline(always)]
    pub fn set_sq11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
    }
    #[doc = "SQ12"]
    #[inline(always)]
    pub const fn sq12(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[doc = "SQ12"]
    #[inline(always)]
    pub fn set_sq12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[doc = "SQ13"]
    #[inline(always)]
    pub const fn sq13(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x1f;
        val as u8
    }
    #[doc = "SQ13"]
    #[inline(always)]
    pub fn set_sq13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
    }
    #[doc = "SQ14"]
    #[inline(always)]
    pub const fn sq14(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "SQ14"]
    #[inline(always)]
    pub fn set_sq14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Sqr3 {
    #[inline(always)]
    fn default() -> Sqr3 {
        Sqr3(0)
    }
}
#[doc = "regular sequence register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqr4(pub u32);
impl Sqr4 {
    #[doc = "SQ15"]
    #[inline(always)]
    pub const fn sq15(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "SQ15"]
    #[inline(always)]
    pub fn set_sq15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "SQ16"]
    #[inline(always)]
    pub const fn sq16(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x1f;
        val as u8
    }
    #[doc = "SQ16"]
    #[inline(always)]
    pub fn set_sq16(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
    }
}
impl Default for Sqr4 {
    #[inline(always)]
    fn default() -> Sqr4 {
        Sqr4(0)
    }
}
#[doc = "watchdog threshold register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tr1(pub u32);
impl Tr1 {
    #[doc = "LT1"]
    #[inline(always)]
    pub const fn lt1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "LT1"]
    #[inline(always)]
    pub fn set_lt1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "AWDFILT"]
    #[inline(always)]
    pub const fn awdfilt(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "AWDFILT"]
    #[inline(always)]
    pub fn set_awdfilt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "HT1"]
    #[inline(always)]
    pub const fn ht1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "HT1"]
    #[inline(always)]
    pub fn set_ht1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Tr1 {
    #[inline(always)]
    fn default() -> Tr1 {
        Tr1(0)
    }
}
#[doc = "watchdog threshold register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tr2(pub u32);
impl Tr2 {
    #[doc = "LT2"]
    #[inline(always)]
    pub const fn lt2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "LT2"]
    #[inline(always)]
    pub fn set_lt2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "HT2"]
    #[inline(always)]
    pub const fn ht2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "HT2"]
    #[inline(always)]
    pub fn set_ht2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Tr2 {
    #[inline(always)]
    fn default() -> Tr2 {
        Tr2(0)
    }
}
#[doc = "watchdog threshold register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tr3(pub u32);
impl Tr3 {
    #[doc = "LT3"]
    #[inline(always)]
    pub const fn lt3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "LT3"]
    #[inline(always)]
    pub fn set_lt3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "HT3"]
    #[inline(always)]
    pub const fn ht3(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "HT3"]
    #[inline(always)]
    pub fn set_ht3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Tr3 {
    #[inline(always)]
    fn default() -> Tr3 {
        Tr3(0)
    }
}

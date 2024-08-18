#[doc = "ADC common control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc = "Dual ADC mode selection"]
    #[inline(always)]
    pub const fn dual(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Dual ADC mode selection"]
    #[inline(always)]
    pub fn set_dual(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Delay between 2 sampling phases"]
    #[inline(always)]
    pub const fn delay(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay between 2 sampling phases"]
    #[inline(always)]
    pub fn set_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "DMA configuration (for multi-ADC mode)"]
    #[inline(always)]
    pub const fn dmacfg(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA configuration (for multi-ADC mode)"]
    #[inline(always)]
    pub fn set_dmacfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Direct memory access mode for multi ADC mode"]
    #[inline(always)]
    pub const fn mdma(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Direct memory access mode for multi ADC mode"]
    #[inline(always)]
    pub fn set_mdma(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "ADC clock mode"]
    #[inline(always)]
    pub const fn ckmode(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "ADC clock mode"]
    #[inline(always)]
    pub fn set_ckmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "ADC prescaler"]
    #[inline(always)]
    pub const fn presc(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[doc = "ADC prescaler"]
    #[inline(always)]
    pub fn set_presc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
    #[doc = "VREFINT enable"]
    #[inline(always)]
    pub const fn vrefen(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "VREFINT enable"]
    #[inline(always)]
    pub fn set_vrefen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "VTS selection"]
    #[inline(always)]
    pub const fn vsensesel(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "VTS selection"]
    #[inline(always)]
    pub fn set_vsensesel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "VBAT selection"]
    #[inline(always)]
    pub const fn vbatsel(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "VBAT selection"]
    #[inline(always)]
    pub fn set_vbatsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Ccr {
    #[inline(always)]
    fn default() -> Ccr {
        Ccr(0)
    }
}
#[doc = "ADC common regular data register for dual and triple modes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdr(pub u32);
impl Cdr {
    #[doc = "Regular data of the master ADC"]
    #[inline(always)]
    pub const fn rdata_mst(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Regular data of the master ADC"]
    #[inline(always)]
    pub fn set_rdata_mst(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Regular data of the slave ADC"]
    #[inline(always)]
    pub const fn rdata_slv(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Regular data of the slave ADC"]
    #[inline(always)]
    pub fn set_rdata_slv(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cdr {
    #[inline(always)]
    fn default() -> Cdr {
        Cdr(0)
    }
}
#[doc = "ADC Common status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "ADDRDY_MST"]
    #[inline(always)]
    pub const fn addrdy_mst(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ADDRDY_MST"]
    #[inline(always)]
    pub fn set_addrdy_mst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "EOSMP_MST"]
    #[inline(always)]
    pub const fn eosmp_mst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "EOSMP_MST"]
    #[inline(always)]
    pub fn set_eosmp_mst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "EOC_MST"]
    #[inline(always)]
    pub const fn eoc_mst(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "EOC_MST"]
    #[inline(always)]
    pub fn set_eoc_mst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "EOS_MST"]
    #[inline(always)]
    pub const fn eos_mst(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "EOS_MST"]
    #[inline(always)]
    pub fn set_eos_mst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "OVR_MST"]
    #[inline(always)]
    pub const fn ovr_mst(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "OVR_MST"]
    #[inline(always)]
    pub fn set_ovr_mst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "JEOC_MST"]
    #[inline(always)]
    pub const fn jeoc_mst(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "JEOC_MST"]
    #[inline(always)]
    pub fn set_jeoc_mst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "JEOS_MST"]
    #[inline(always)]
    pub const fn jeos_mst(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "JEOS_MST"]
    #[inline(always)]
    pub fn set_jeos_mst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "AWD1_MST"]
    #[inline(always)]
    pub const fn awd1_mst(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "AWD1_MST"]
    #[inline(always)]
    pub fn set_awd1_mst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "AWD2_MST"]
    #[inline(always)]
    pub const fn awd2_mst(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "AWD2_MST"]
    #[inline(always)]
    pub fn set_awd2_mst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "AWD3_MST"]
    #[inline(always)]
    pub const fn awd3_mst(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "AWD3_MST"]
    #[inline(always)]
    pub fn set_awd3_mst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "JQOVF_MST"]
    #[inline(always)]
    pub const fn jqovf_mst(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "JQOVF_MST"]
    #[inline(always)]
    pub fn set_jqovf_mst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "ADRDY_SLV"]
    #[inline(always)]
    pub const fn adrdy_slv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ADRDY_SLV"]
    #[inline(always)]
    pub fn set_adrdy_slv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "EOSMP_SLV"]
    #[inline(always)]
    pub const fn eosmp_slv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "EOSMP_SLV"]
    #[inline(always)]
    pub fn set_eosmp_slv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "End of regular conversion of the slave ADC"]
    #[inline(always)]
    pub const fn eoc_slv(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "End of regular conversion of the slave ADC"]
    #[inline(always)]
    pub fn set_eoc_slv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "End of regular sequence flag of the slave ADC"]
    #[inline(always)]
    pub const fn eos_slv(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "End of regular sequence flag of the slave ADC"]
    #[inline(always)]
    pub fn set_eos_slv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Overrun flag of the slave ADC"]
    #[inline(always)]
    pub const fn ovr_slv(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun flag of the slave ADC"]
    #[inline(always)]
    pub fn set_ovr_slv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "End of injected conversion flag of the slave ADC"]
    #[inline(always)]
    pub const fn jeoc_slv(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "End of injected conversion flag of the slave ADC"]
    #[inline(always)]
    pub fn set_jeoc_slv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "End of injected sequence flag of the slave ADC"]
    #[inline(always)]
    pub const fn jeos_slv(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "End of injected sequence flag of the slave ADC"]
    #[inline(always)]
    pub fn set_jeos_slv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Analog watchdog 1 flag of the slave ADC"]
    #[inline(always)]
    pub const fn awd1_slv(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Analog watchdog 1 flag of the slave ADC"]
    #[inline(always)]
    pub fn set_awd1_slv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Analog watchdog 2 flag of the slave ADC"]
    #[inline(always)]
    pub const fn awd2_slv(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Analog watchdog 2 flag of the slave ADC"]
    #[inline(always)]
    pub fn set_awd2_slv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Analog watchdog 3 flag of the slave ADC"]
    #[inline(always)]
    pub const fn awd3_slv(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Analog watchdog 3 flag of the slave ADC"]
    #[inline(always)]
    pub fn set_awd3_slv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Injected Context Queue Overflow flag of the slave ADC"]
    #[inline(always)]
    pub const fn jqovf_slv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Injected Context Queue Overflow flag of the slave ADC"]
    #[inline(always)]
    pub fn set_jqovf_slv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for Csr {
    #[inline(always)]
    fn default() -> Csr {
        Csr(0)
    }
}

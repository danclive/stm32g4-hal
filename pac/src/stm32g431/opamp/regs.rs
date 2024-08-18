#[doc = "OPAMP1 control/status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opamp1Csr(pub u32);
impl Opamp1Csr {
    #[doc = "Operational amplifier Enable"]
    #[inline(always)]
    pub const fn opaen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Operational amplifier Enable"]
    #[inline(always)]
    pub fn set_opaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FORCE_VP"]
    #[inline(always)]
    pub const fn force_vp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FORCE_VP"]
    #[inline(always)]
    pub fn set_force_vp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "VP_SEL"]
    #[inline(always)]
    pub const fn vp_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "VP_SEL"]
    #[inline(always)]
    pub fn set_vp_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "USERTRIM"]
    #[inline(always)]
    pub const fn usertrim(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "USERTRIM"]
    #[inline(always)]
    pub fn set_usertrim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "VM_SEL"]
    #[inline(always)]
    pub const fn vm_sel(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "VM_SEL"]
    #[inline(always)]
    pub fn set_vm_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "OPAHSM"]
    #[inline(always)]
    pub const fn opahsm(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "OPAHSM"]
    #[inline(always)]
    pub fn set_opahsm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "OPAINTOEN"]
    #[inline(always)]
    pub const fn opaintoen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OPAINTOEN"]
    #[inline(always)]
    pub fn set_opaintoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CALON"]
    #[inline(always)]
    pub const fn calon(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CALON"]
    #[inline(always)]
    pub fn set_calon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CALSEL"]
    #[inline(always)]
    pub const fn calsel(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "CALSEL"]
    #[inline(always)]
    pub fn set_calsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "PGA_GAIN"]
    #[inline(always)]
    pub const fn pga_gain(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x1f;
        val as u8
    }
    #[doc = "PGA_GAIN"]
    #[inline(always)]
    pub fn set_pga_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 14usize)) | (((val as u32) & 0x1f) << 14usize);
    }
    #[doc = "TRIMOFFSETP"]
    #[inline(always)]
    pub const fn trimoffsetp(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "TRIMOFFSETP"]
    #[inline(always)]
    pub fn set_trimoffsetp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "TRIMOFFSETN"]
    #[inline(always)]
    pub const fn trimoffsetn(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "TRIMOFFSETN"]
    #[inline(always)]
    pub fn set_trimoffsetn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "CALOUT"]
    #[inline(always)]
    pub const fn calout(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "CALOUT"]
    #[inline(always)]
    pub fn set_calout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Opamp1Csr {
    #[inline(always)]
    fn default() -> Opamp1Csr {
        Opamp1Csr(0)
    }
}
#[doc = "OPAMP1 control/status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opamp1Tcmr(pub u32);
impl Opamp1Tcmr {
    #[doc = "VMS_SEL"]
    #[inline(always)]
    pub const fn vms_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VMS_SEL"]
    #[inline(always)]
    pub fn set_vms_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "VPS_SEL"]
    #[inline(always)]
    pub const fn vps_sel(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "VPS_SEL"]
    #[inline(always)]
    pub fn set_vps_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "T1CM_EN"]
    #[inline(always)]
    pub const fn t1cm_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "T1CM_EN"]
    #[inline(always)]
    pub fn set_t1cm_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "T8CM_EN"]
    #[inline(always)]
    pub const fn t8cm_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "T8CM_EN"]
    #[inline(always)]
    pub fn set_t8cm_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "T20CM_EN"]
    #[inline(always)]
    pub const fn t20cm_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "T20CM_EN"]
    #[inline(always)]
    pub fn set_t20cm_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Opamp1Tcmr {
    #[inline(always)]
    fn default() -> Opamp1Tcmr {
        Opamp1Tcmr(0)
    }
}
#[doc = "OPAMP2 control/status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opamp2Csr(pub u32);
impl Opamp2Csr {
    #[doc = "Operational amplifier Enable"]
    #[inline(always)]
    pub const fn opaen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Operational amplifier Enable"]
    #[inline(always)]
    pub fn set_opaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FORCE_VP"]
    #[inline(always)]
    pub const fn force_vp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FORCE_VP"]
    #[inline(always)]
    pub fn set_force_vp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "VP_SEL"]
    #[inline(always)]
    pub const fn vp_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "VP_SEL"]
    #[inline(always)]
    pub fn set_vp_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "USERTRIM"]
    #[inline(always)]
    pub const fn usertrim(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "USERTRIM"]
    #[inline(always)]
    pub fn set_usertrim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "VM_SEL"]
    #[inline(always)]
    pub const fn vm_sel(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "VM_SEL"]
    #[inline(always)]
    pub fn set_vm_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "OPAHSM"]
    #[inline(always)]
    pub const fn opahsm(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "OPAHSM"]
    #[inline(always)]
    pub fn set_opahsm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "OPAINTOEN"]
    #[inline(always)]
    pub const fn opaintoen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OPAINTOEN"]
    #[inline(always)]
    pub fn set_opaintoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CALON"]
    #[inline(always)]
    pub const fn calon(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CALON"]
    #[inline(always)]
    pub fn set_calon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CALSEL"]
    #[inline(always)]
    pub const fn calsel(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "CALSEL"]
    #[inline(always)]
    pub fn set_calsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "PGA_GAIN"]
    #[inline(always)]
    pub const fn pga_gain(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x1f;
        val as u8
    }
    #[doc = "PGA_GAIN"]
    #[inline(always)]
    pub fn set_pga_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 14usize)) | (((val as u32) & 0x1f) << 14usize);
    }
    #[doc = "TRIMOFFSETP"]
    #[inline(always)]
    pub const fn trimoffsetp(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "TRIMOFFSETP"]
    #[inline(always)]
    pub fn set_trimoffsetp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "TRIMOFFSETN"]
    #[inline(always)]
    pub const fn trimoffsetn(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "TRIMOFFSETN"]
    #[inline(always)]
    pub fn set_trimoffsetn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "CALOUT"]
    #[inline(always)]
    pub const fn calout(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "CALOUT"]
    #[inline(always)]
    pub fn set_calout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Opamp2Csr {
    #[inline(always)]
    fn default() -> Opamp2Csr {
        Opamp2Csr(0)
    }
}
#[doc = "OPAMP2 control/status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opamp2Tcmr(pub u32);
impl Opamp2Tcmr {
    #[doc = "VMS_SEL"]
    #[inline(always)]
    pub const fn vms_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VMS_SEL"]
    #[inline(always)]
    pub fn set_vms_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "VPS_SEL"]
    #[inline(always)]
    pub const fn vps_sel(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "VPS_SEL"]
    #[inline(always)]
    pub fn set_vps_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "T1CM_EN"]
    #[inline(always)]
    pub const fn t1cm_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "T1CM_EN"]
    #[inline(always)]
    pub fn set_t1cm_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "T8CM_EN"]
    #[inline(always)]
    pub const fn t8cm_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "T8CM_EN"]
    #[inline(always)]
    pub fn set_t8cm_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "T20CM_EN"]
    #[inline(always)]
    pub const fn t20cm_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "T20CM_EN"]
    #[inline(always)]
    pub fn set_t20cm_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Opamp2Tcmr {
    #[inline(always)]
    fn default() -> Opamp2Tcmr {
        Opamp2Tcmr(0)
    }
}
#[doc = "OPAMP3 control/status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opamp3Csr(pub u32);
impl Opamp3Csr {
    #[doc = "Operational amplifier Enable"]
    #[inline(always)]
    pub const fn opaen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Operational amplifier Enable"]
    #[inline(always)]
    pub fn set_opaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FORCE_VP"]
    #[inline(always)]
    pub const fn force_vp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FORCE_VP"]
    #[inline(always)]
    pub fn set_force_vp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "VP_SEL"]
    #[inline(always)]
    pub const fn vp_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "VP_SEL"]
    #[inline(always)]
    pub fn set_vp_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "USERTRIM"]
    #[inline(always)]
    pub const fn usertrim(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "USERTRIM"]
    #[inline(always)]
    pub fn set_usertrim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "VM_SEL"]
    #[inline(always)]
    pub const fn vm_sel(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "VM_SEL"]
    #[inline(always)]
    pub fn set_vm_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "OPAHSM"]
    #[inline(always)]
    pub const fn opahsm(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "OPAHSM"]
    #[inline(always)]
    pub fn set_opahsm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "OPAINTOEN"]
    #[inline(always)]
    pub const fn opaintoen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OPAINTOEN"]
    #[inline(always)]
    pub fn set_opaintoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CALON"]
    #[inline(always)]
    pub const fn calon(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CALON"]
    #[inline(always)]
    pub fn set_calon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CALSEL"]
    #[inline(always)]
    pub const fn calsel(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "CALSEL"]
    #[inline(always)]
    pub fn set_calsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "PGA_GAIN"]
    #[inline(always)]
    pub const fn pga_gain(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x1f;
        val as u8
    }
    #[doc = "PGA_GAIN"]
    #[inline(always)]
    pub fn set_pga_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 14usize)) | (((val as u32) & 0x1f) << 14usize);
    }
    #[doc = "TRIMOFFSETP"]
    #[inline(always)]
    pub const fn trimoffsetp(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "TRIMOFFSETP"]
    #[inline(always)]
    pub fn set_trimoffsetp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "TRIMOFFSETN"]
    #[inline(always)]
    pub const fn trimoffsetn(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "TRIMOFFSETN"]
    #[inline(always)]
    pub fn set_trimoffsetn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "CALOUT"]
    #[inline(always)]
    pub const fn calout(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "CALOUT"]
    #[inline(always)]
    pub fn set_calout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Opamp3Csr {
    #[inline(always)]
    fn default() -> Opamp3Csr {
        Opamp3Csr(0)
    }
}
#[doc = "OPAMP3 control/status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opamp3Tcmr(pub u32);
impl Opamp3Tcmr {
    #[doc = "VMS_SEL"]
    #[inline(always)]
    pub const fn vms_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VMS_SEL"]
    #[inline(always)]
    pub fn set_vms_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "VPS_SEL"]
    #[inline(always)]
    pub const fn vps_sel(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "VPS_SEL"]
    #[inline(always)]
    pub fn set_vps_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "T1CM_EN"]
    #[inline(always)]
    pub const fn t1cm_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "T1CM_EN"]
    #[inline(always)]
    pub fn set_t1cm_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "T8CM_EN"]
    #[inline(always)]
    pub const fn t8cm_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "T8CM_EN"]
    #[inline(always)]
    pub fn set_t8cm_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "T20CM_EN"]
    #[inline(always)]
    pub const fn t20cm_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "T20CM_EN"]
    #[inline(always)]
    pub fn set_t20cm_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Opamp3Tcmr {
    #[inline(always)]
    fn default() -> Opamp3Tcmr {
        Opamp3Tcmr(0)
    }
}

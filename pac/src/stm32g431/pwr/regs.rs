#[doc = "Power control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrCr1(pub u32);
impl PwrCr1 {
    #[doc = "Low-power mode selection These bits select the low-power mode entered when CPU enters the deepsleep mode. 1xx: Shutdown mode Note: In Standby mode, SRAM2 can be preserved or not, depending on RRS bit configuration in PWR_CR3."]
    #[inline(always)]
    pub const fn lpms(&self) -> super::vals::Lpms {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Lpms::from_bits(val as u8)
    }
    #[doc = "Low-power mode selection These bits select the low-power mode entered when CPU enters the deepsleep mode. 1xx: Shutdown mode Note: In Standby mode, SRAM2 can be preserved or not, depending on RRS bit configuration in PWR_CR3."]
    #[inline(always)]
    pub fn set_lpms(&mut self, val: super::vals::Lpms) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "FPD_STOP"]
    #[inline(always)]
    pub const fn fpd_stop(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FPD_STOP"]
    #[inline(always)]
    pub fn set_fpd_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Disable backup domain write protection In reset state, the RTC and backup registers are protected against parasitic write access. This bit must be set to enable write access to these registers."]
    #[inline(always)]
    pub const fn dbp(&self) -> super::vals::Dbp {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Dbp::from_bits(val as u8)
    }
    #[doc = "Disable backup domain write protection In reset state, the RTC and backup registers are protected against parasitic write access. This bit must be set to enable write access to these registers."]
    #[inline(always)]
    pub fn set_dbp(&mut self, val: super::vals::Dbp) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Voltage scaling range selection"]
    #[inline(always)]
    pub const fn vos(&self) -> super::vals::Vos {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::Vos::from_bits(val as u8)
    }
    #[doc = "Voltage scaling range selection"]
    #[inline(always)]
    pub fn set_vos(&mut self, val: super::vals::Vos) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "Low-power run When this bit is set, the regulator is switched from main mode (MR) to low-power mode (LPR)."]
    #[inline(always)]
    pub const fn lpr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Low-power run When this bit is set, the regulator is switched from main mode (MR) to low-power mode (LPR)."]
    #[inline(always)]
    pub fn set_lpr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for PwrCr1 {
    #[inline(always)]
    fn default() -> PwrCr1 {
        PwrCr1(0)
    }
}
#[doc = "Power control register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrCr2(pub u32);
impl PwrCr2 {
    #[doc = "Programmable voltage detector enable Note: This bit is write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset."]
    #[inline(always)]
    pub const fn pvde(&self) -> super::vals::Pvde {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pvde::from_bits(val as u8)
    }
    #[doc = "Programmable voltage detector enable Note: This bit is write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset."]
    #[inline(always)]
    pub fn set_pvde(&mut self, val: super::vals::Pvde) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Programmable voltage detector level selection. These bits select the PVD falling threshold: Note: These bits are write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset."]
    #[inline(always)]
    pub const fn pvdls(&self) -> super::vals::Pvdls {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::Pvdls::from_bits(val as u8)
    }
    #[doc = "Programmable voltage detector level selection. These bits select the PVD falling threshold: Note: These bits are write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset."]
    #[inline(always)]
    pub fn set_pvdls(&mut self, val: super::vals::Pvdls) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
    #[doc = "Peripheral voltage monitoring 3 enable: V<sub>DDA</sub> vs. ADC/COMP min voltage 1.62V"]
    #[inline(always)]
    pub const fn pvmen1(&self) -> super::vals::Pvmen1 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pvmen1::from_bits(val as u8)
    }
    #[doc = "Peripheral voltage monitoring 3 enable: V<sub>DDA</sub> vs. ADC/COMP min voltage 1.62V"]
    #[inline(always)]
    pub fn set_pvmen1(&mut self, val: super::vals::Pvmen1) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Peripheral voltage monitoring 4 enable: V<sub>DDA</sub> vs. DAC 1MSPS /DAC 15MSPS min voltage."]
    #[inline(always)]
    pub const fn pvmen2(&self) -> super::vals::Pvmen2 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pvmen2::from_bits(val as u8)
    }
    #[doc = "Peripheral voltage monitoring 4 enable: V<sub>DDA</sub> vs. DAC 1MSPS /DAC 15MSPS min voltage."]
    #[inline(always)]
    pub fn set_pvmen2(&mut self, val: super::vals::Pvmen2) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for PwrCr2 {
    #[inline(always)]
    fn default() -> PwrCr2 {
        PwrCr2(0)
    }
}
#[doc = "Power control register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrCr3(pub u32);
impl PwrCr3 {
    #[doc = "Enable Wakeup pin WKUP1 When this bit is set, the external wakeup pin WKUP1 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub const fn ewup1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Wakeup pin WKUP1 When this bit is set, the external wakeup pin WKUP1 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn set_ewup1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Wakeup pin WKUP2 When this bit is set, the external wakeup pin WKUP2 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub const fn ewup2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Wakeup pin WKUP2 When this bit is set, the external wakeup pin WKUP2 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn set_ewup2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Wakeup pin WKUP3 When this bit is set, the external wakeup pin WKUP3 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub const fn ewup3(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Wakeup pin WKUP3 When this bit is set, the external wakeup pin WKUP3 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn set_ewup3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable Wakeup pin WKUP4 When this bit is set, the external wakeup pin WKUP4 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub const fn ewup4(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Wakeup pin WKUP4 When this bit is set, the external wakeup pin WKUP4 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn set_ewup4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Wakeup pin WKUP5 When this bit is set, the external wakeup pin WKUP5 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub const fn ewup5(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Wakeup pin WKUP5 When this bit is set, the external wakeup pin WKUP5 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn set_ewup5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SRAM2 retention in Standby mode"]
    #[inline(always)]
    pub const fn rrs(&self) -> super::vals::Rrs {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Rrs::from_bits(val as u8)
    }
    #[doc = "SRAM2 retention in Standby mode"]
    #[inline(always)]
    pub fn set_rrs(&mut self, val: super::vals::Rrs) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os."]
    #[inline(always)]
    pub const fn apc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os."]
    #[inline(always)]
    pub fn set_apc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "UCPD1_STDBY USB Type-C and Power Delivery standby mode."]
    #[inline(always)]
    pub const fn ucpd1_stdby(&self) -> super::vals::Ucpd1Stdby {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Ucpd1Stdby::from_bits(val as u8)
    }
    #[doc = "UCPD1_STDBY USB Type-C and Power Delivery standby mode."]
    #[inline(always)]
    pub fn set_ucpd1_stdby(&mut self, val: super::vals::Ucpd1Stdby) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "USB Type-C and Power Delivery Dead Battery disable. After exiting reset, the USB Type-C dead battery behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to hand over control to the UCPD1 (which should therefore be initialized before doing the disable)."]
    #[inline(always)]
    pub const fn ucpd1_dbdis(&self) -> super::vals::Ucpd1Dbdis {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Ucpd1Dbdis::from_bits(val as u8)
    }
    #[doc = "USB Type-C and Power Delivery Dead Battery disable. After exiting reset, the USB Type-C dead battery behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to hand over control to the UCPD1 (which should therefore be initialized before doing the disable)."]
    #[inline(always)]
    pub fn set_ucpd1_dbdis(&mut self, val: super::vals::Ucpd1Dbdis) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable internal wakeup line"]
    #[inline(always)]
    pub const fn eiwul(&self) -> super::vals::Eiwul {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Eiwul::from_bits(val as u8)
    }
    #[doc = "Enable internal wakeup line"]
    #[inline(always)]
    pub fn set_eiwul(&mut self, val: super::vals::Eiwul) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for PwrCr3 {
    #[inline(always)]
    fn default() -> PwrCr3 {
        PwrCr3(0)
    }
}
#[doc = "Power control register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrCr4(pub u32);
impl PwrCr4 {
    #[doc = "Wakeup pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1"]
    #[inline(always)]
    pub const fn wp1(&self) -> super::vals::Wp1 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Wp1::from_bits(val as u8)
    }
    #[doc = "Wakeup pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1"]
    #[inline(always)]
    pub fn set_wp1(&mut self, val: super::vals::Wp1) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Wakeup pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2"]
    #[inline(always)]
    pub const fn wp2(&self) -> super::vals::Wp2 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Wp2::from_bits(val as u8)
    }
    #[doc = "Wakeup pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2"]
    #[inline(always)]
    pub fn set_wp2(&mut self, val: super::vals::Wp2) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Wakeup pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3"]
    #[inline(always)]
    pub const fn wp3(&self) -> super::vals::Wp3 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Wp3::from_bits(val as u8)
    }
    #[doc = "Wakeup pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3"]
    #[inline(always)]
    pub fn set_wp3(&mut self, val: super::vals::Wp3) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Wakeup pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4"]
    #[inline(always)]
    pub const fn wp4(&self) -> super::vals::Wp4 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Wp4::from_bits(val as u8)
    }
    #[doc = "Wakeup pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4"]
    #[inline(always)]
    pub fn set_wp4(&mut self, val: super::vals::Wp4) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Wakeup pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5"]
    #[inline(always)]
    pub const fn wp5(&self) -> super::vals::Wp5 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Wp5::from_bits(val as u8)
    }
    #[doc = "Wakeup pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5"]
    #[inline(always)]
    pub fn set_wp5(&mut self, val: super::vals::Wp5) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "V<sub>BAT</sub> battery charging enable"]
    #[inline(always)]
    pub const fn vbe(&self) -> super::vals::Vbe {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Vbe::from_bits(val as u8)
    }
    #[doc = "V<sub>BAT</sub> battery charging enable"]
    #[inline(always)]
    pub fn set_vbe(&mut self, val: super::vals::Vbe) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "V<sub>BAT</sub> battery charging resistor selection"]
    #[inline(always)]
    pub const fn vbrs(&self) -> super::vals::Vbrs {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Vbrs::from_bits(val as u8)
    }
    #[doc = "V<sub>BAT</sub> battery charging resistor selection"]
    #[inline(always)]
    pub fn set_vbrs(&mut self, val: super::vals::Vbrs) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PwrCr4 {
    #[inline(always)]
    fn default() -> PwrCr4 {
        PwrCr4(0)
    }
}
#[doc = "Power control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrCr5(pub u32);
impl PwrCr5 {
    #[doc = "Main regular range 1 mode This bit is only valid for the main regulator in range 1 and has no effect on range 2. It is recommended to reset this bit when the system frequency is greater than 150 MHz. Refer to"]
    #[inline(always)]
    pub const fn r1mode(&self) -> super::vals::R1mode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::R1mode::from_bits(val as u8)
    }
    #[doc = "Main regular range 1 mode This bit is only valid for the main regulator in range 1 and has no effect on range 2. It is recommended to reset this bit when the system frequency is greater than 150 MHz. Refer to"]
    #[inline(always)]
    pub fn set_r1mode(&mut self, val: super::vals::R1mode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for PwrCr5 {
    #[inline(always)]
    fn default() -> PwrCr5 {
        PwrCr5(0)
    }
}
#[doc = "Power Port A pull-down control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrPdcra(pub u32);
impl PwrPdcra {
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-down bit y (y=0..12) When set, this bit activates the pull-down on PA\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port A pull-down bit 14 When set, this bit activates the pull-down on PA\\[14\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-down bit 14 When set, this bit activates the pull-down on PA\\[14\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for PwrPdcra {
    #[inline(always)]
    fn default() -> PwrPdcra {
        PwrPdcra(0)
    }
}
#[doc = "Power Port B pull-down control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrPdcrb(pub u32);
impl PwrPdcrb {
    #[doc = "Port B pull-down bit y (y=0..3) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-down bit y (y=0..3) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port B pull-down bit y (y=0..3) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-down bit y (y=0..3) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port B pull-down bit y (y=0..3) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-down bit y (y=0..3) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port B pull-down bit y (y=0..3) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-down bit y (y=0..3) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-down bit y (y=5..15) When set, this bit activates the pull-down on PB\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for PwrPdcrb {
    #[inline(always)]
    fn default() -> PwrPdcrb {
        PwrPdcrb(0)
    }
}
#[doc = "Power Port C pull-down control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrPdcrc(pub u32);
impl PwrPdcrc {
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-down bit y (y=0..15) When set, this bit activates the pull-down on PC\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for PwrPdcrc {
    #[inline(always)]
    fn default() -> PwrPdcrc {
        PwrPdcrc(0)
    }
}
#[doc = "Power Port D pull-down control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrPdcrd(pub u32);
impl PwrPdcrd {
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-down bit y (y=0..15) When set, this bit activates the pull-down on PD\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for PwrPdcrd {
    #[inline(always)]
    fn default() -> PwrPdcrd {
        PwrPdcrd(0)
    }
}
#[doc = "Power Port E pull-down control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrPdcre(pub u32);
impl PwrPdcre {
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-down bit y (y=0..15) When set, this bit activates the pull-down on PE\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for PwrPdcre {
    #[inline(always)]
    fn default() -> PwrPdcre {
        PwrPdcre(0)
    }
}
#[doc = "Power Port F pull-down control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrPdcrf(pub u32);
impl PwrPdcrf {
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-down bit y (y=0..15) When set, this bit activates the pull-down on PF\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for PwrPdcrf {
    #[inline(always)]
    fn default() -> PwrPdcrf {
        PwrPdcrf(0)
    }
}
#[doc = "Power Port G pull-down control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrPdcrg(pub u32);
impl PwrPdcrg {
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub const fn pd10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn set_pd10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for PwrPdcrg {
    #[inline(always)]
    fn default() -> PwrPdcrg {
        PwrPdcrg(0)
    }
}
#[doc = "Power Port A pull-up control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrPucra(pub u32);
impl PwrPucra {
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-up bit y (y=0..13) When set, this bit activates the pull-up on PA\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port A pull-up bit 15 When set, this bit activates the pull-up on PA\\[15\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PD15 bit is also set."]
    #[inline(always)]
    pub const fn pu15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port A pull-up bit 15 When set, this bit activates the pull-up on PA\\[15\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PD15 bit is also set."]
    #[inline(always)]
    pub fn set_pu15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for PwrPucra {
    #[inline(always)]
    fn default() -> PwrPucra {
        PwrPucra(0)
    }
}
#[doc = "Power Port B pull-up control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrPucrb(pub u32);
impl PwrPucrb {
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port B pull-up bit y (y=0..15) When set, this bit activates the pull-up on PB\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for PwrPucrb {
    #[inline(always)]
    fn default() -> PwrPucrb {
        PwrPucrb(0)
    }
}
#[doc = "Power Port C pull-up control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrPucrc(pub u32);
impl PwrPucrc {
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port C pull-up bit y (y=0..15) When set, this bit activates the pull-up on PC\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for PwrPucrc {
    #[inline(always)]
    fn default() -> PwrPucrc {
        PwrPucrc(0)
    }
}
#[doc = "Power Port D pull-up control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrPucrd(pub u32);
impl PwrPucrd {
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port D pull-up bit y (y=0..15) When set, this bit activates the pull-up on PD\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for PwrPucrd {
    #[inline(always)]
    fn default() -> PwrPucrd {
        PwrPucrd(0)
    }
}
#[doc = "Power Port E pull-up control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrPucre(pub u32);
impl PwrPucre {
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port E pull-up bit y (y=0..15) When set, this bit activates the pull-up on PE\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for PwrPucre {
    #[inline(always)]
    fn default() -> PwrPucre {
        PwrPucre(0)
    }
}
#[doc = "Power Port F pull-up control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrPucrf(pub u32);
impl PwrPucrf {
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port F pull-up bit y (y=0..15) When set, this bit activates the pull-up on PF\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for PwrPucrf {
    #[inline(always)]
    fn default() -> PwrPucrf {
        PwrPucrf(0)
    }
}
#[doc = "Power Port G pull-up control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrPucrg(pub u32);
impl PwrPucrg {
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub const fn pu10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port G pull-up bit y (y=0..10) When set, this bit activates the pull-up on PG\\[y\\] when APC bit is set in PWR_CR3 register. The pull-up is not activated if the corresponding PDy bit is also set."]
    #[inline(always)]
    pub fn set_pu10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for PwrPucrg {
    #[inline(always)]
    fn default() -> PwrPucrg {
        PwrPucrg(0)
    }
}
#[doc = "Power status clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrScr(pub u32);
impl PwrScr {
    #[doc = "Clear wakeup flag 1 Setting this bit clears the WUF1 flag in the PWR_SR1 register."]
    #[inline(always)]
    pub const fn cwuf1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear wakeup flag 1 Setting this bit clears the WUF1 flag in the PWR_SR1 register."]
    #[inline(always)]
    pub fn set_cwuf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clear wakeup flag 2 Setting this bit clears the WUF2 flag in the PWR_SR1 register."]
    #[inline(always)]
    pub const fn cwuf2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clear wakeup flag 2 Setting this bit clears the WUF2 flag in the PWR_SR1 register."]
    #[inline(always)]
    pub fn set_cwuf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Clear wakeup flag 3 Setting this bit clears the WUF3 flag in the PWR_SR1 register."]
    #[inline(always)]
    pub const fn cwuf3(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Clear wakeup flag 3 Setting this bit clears the WUF3 flag in the PWR_SR1 register."]
    #[inline(always)]
    pub fn set_cwuf3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear wakeup flag 4 Setting this bit clears the WUF4 flag in the PWR_SR1 register."]
    #[inline(always)]
    pub const fn cwuf4(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clear wakeup flag 4 Setting this bit clears the WUF4 flag in the PWR_SR1 register."]
    #[inline(always)]
    pub fn set_cwuf4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Clear wakeup flag 5 Setting this bit clears the WUF5 flag in the PWR_SR1 register."]
    #[inline(always)]
    pub const fn cwuf5(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Clear wakeup flag 5 Setting this bit clears the WUF5 flag in the PWR_SR1 register."]
    #[inline(always)]
    pub fn set_cwuf5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Clear standby flag Setting this bit clears the SBF flag in the PWR_SR1 register."]
    #[inline(always)]
    pub const fn csbf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Clear standby flag Setting this bit clears the SBF flag in the PWR_SR1 register."]
    #[inline(always)]
    pub fn set_csbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for PwrScr {
    #[inline(always)]
    fn default() -> PwrScr {
        PwrScr(0)
    }
}
#[doc = "Power status register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrSr1(pub u32);
impl PwrSr1 {
    #[doc = "Wakeup flag 1 This bit is set when a wakeup event is detected on wakeup pin, WKUP1. It is cleared by writing 1 in the CWUF1 bit of the PWR_SCR register."]
    #[inline(always)]
    pub const fn wuf1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup flag 1 This bit is set when a wakeup event is detected on wakeup pin, WKUP1. It is cleared by writing 1 in the CWUF1 bit of the PWR_SCR register."]
    #[inline(always)]
    pub fn set_wuf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Wakeup flag 2 This bit is set when a wakeup event is detected on wakeup pin, WKUP2. It is cleared by writing 1 in the CWUF2 bit of the PWR_SCR register."]
    #[inline(always)]
    pub const fn wuf2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup flag 2 This bit is set when a wakeup event is detected on wakeup pin, WKUP2. It is cleared by writing 1 in the CWUF2 bit of the PWR_SCR register."]
    #[inline(always)]
    pub fn set_wuf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Wakeup flag 3 This bit is set when a wakeup event is detected on wakeup pin, WKUP3. It is cleared by writing 1 in the CWUF3 bit of the PWR_SCR register."]
    #[inline(always)]
    pub const fn wuf3(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup flag 3 This bit is set when a wakeup event is detected on wakeup pin, WKUP3. It is cleared by writing 1 in the CWUF3 bit of the PWR_SCR register."]
    #[inline(always)]
    pub fn set_wuf3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Wakeup flag 4 This bit is set when a wakeup event is detected on wakeup pin,WKUP4. It is cleared by writing 1 in the CWUF4 bit of the PWR_SCR register."]
    #[inline(always)]
    pub const fn wuf4(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup flag 4 This bit is set when a wakeup event is detected on wakeup pin,WKUP4. It is cleared by writing 1 in the CWUF4 bit of the PWR_SCR register."]
    #[inline(always)]
    pub fn set_wuf4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Wakeup flag 5 This bit is set when a wakeup event is detected on wakeup pin, WKUP5. It is cleared by writing 1 in the CWUF5 bit of the PWR_SCR register."]
    #[inline(always)]
    pub const fn wuf5(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup flag 5 This bit is set when a wakeup event is detected on wakeup pin, WKUP5. It is cleared by writing 1 in the CWUF5 bit of the PWR_SCR register."]
    #[inline(always)]
    pub fn set_wuf5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Standby flag This bit is set by hardware when the device enters the Standby mode and is cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset."]
    #[inline(always)]
    pub const fn sbf(&self) -> super::vals::Sbf {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Sbf::from_bits(val as u8)
    }
    #[doc = "Standby flag This bit is set by hardware when the device enters the Standby mode and is cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset."]
    #[inline(always)]
    pub fn set_sbf(&mut self, val: super::vals::Sbf) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Wakeup flag internal This bit is set when a wakeup is detected on the internal wakeup line. It is cleared when all internal wakeup sources are cleared."]
    #[inline(always)]
    pub const fn wufi(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup flag internal This bit is set when a wakeup is detected on the internal wakeup line. It is cleared when all internal wakeup sources are cleared."]
    #[inline(always)]
    pub fn set_wufi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for PwrSr1 {
    #[inline(always)]
    fn default() -> PwrSr1 {
        PwrSr1(0)
    }
}
#[doc = "Power status register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrSr2(pub u32);
impl PwrSr2 {
    #[doc = "Low-power regulator started This bit provides the information whether the low-power regulator is ready after a power-on reset or a Standby/Shutdown. If the Standby mode is entered while REGLPS bit is still cleared, the wakeup from Standby mode time may be increased."]
    #[inline(always)]
    pub const fn reglps(&self) -> super::vals::Reglps {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Reglps::from_bits(val as u8)
    }
    #[doc = "Low-power regulator started This bit provides the information whether the low-power regulator is ready after a power-on reset or a Standby/Shutdown. If the Standby mode is entered while REGLPS bit is still cleared, the wakeup from Standby mode time may be increased."]
    #[inline(always)]
    pub fn set_reglps(&mut self, val: super::vals::Reglps) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Low-power regulator flag This bit is set by hardware when the MCU is in Low-power run mode. When the MCU exits the Low-power run mode, this bit remains at 1 until the regulator is ready in main mode. A polling on this bit must be done before increasing the product frequency. This bit is cleared by hardware when the regulator is ready."]
    #[inline(always)]
    pub const fn reglpf(&self) -> super::vals::Reglpf {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Reglpf::from_bits(val as u8)
    }
    #[doc = "Low-power regulator flag This bit is set by hardware when the MCU is in Low-power run mode. When the MCU exits the Low-power run mode, this bit remains at 1 until the regulator is ready in main mode. A polling on this bit must be done before increasing the product frequency. This bit is cleared by hardware when the regulator is ready."]
    #[inline(always)]
    pub fn set_reglpf(&mut self, val: super::vals::Reglpf) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Voltage scaling flag A delay is required for the internal regulator to be ready after the voltage scaling has been changed. VOSF indicates that the regulator reached the voltage level defined with VOS bits of the PWR_CR1 register."]
    #[inline(always)]
    pub const fn vosf(&self) -> super::vals::Vosf {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Vosf::from_bits(val as u8)
    }
    #[doc = "Voltage scaling flag A delay is required for the internal regulator to be ready after the voltage scaling has been changed. VOSF indicates that the regulator reached the voltage level defined with VOS bits of the PWR_CR1 register."]
    #[inline(always)]
    pub fn set_vosf(&mut self, val: super::vals::Vosf) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Programmable voltage detector output"]
    #[inline(always)]
    pub const fn pvdo(&self) -> super::vals::Pvdo {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pvdo::from_bits(val as u8)
    }
    #[doc = "Programmable voltage detector output"]
    #[inline(always)]
    pub fn set_pvdo(&mut self, val: super::vals::Pvdo) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Peripheral voltage monitoring output: V<sub>DDA</sub> vs. 1.62 V Note: PVMO1 is cleared when PVM1 is disabled (PVME = 0). After enabling PVM1, the PVM1 output is valid after the PVM1 wakeup time."]
    #[inline(always)]
    pub const fn pvmo1(&self) -> super::vals::Pvmo1 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pvmo1::from_bits(val as u8)
    }
    #[doc = "Peripheral voltage monitoring output: V<sub>DDA</sub> vs. 1.62 V Note: PVMO1 is cleared when PVM1 is disabled (PVME = 0). After enabling PVM1, the PVM1 output is valid after the PVM1 wakeup time."]
    #[inline(always)]
    pub fn set_pvmo1(&mut self, val: super::vals::Pvmo1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Peripheral voltage monitoring output: V<sub>DDA</sub> vs. 1.8 V Note: PVMO2 is cleared when PVM2 is disabled (PVME = 0). After enabling PVM2, the PVM2 output is valid after the PVM2 wakeup time."]
    #[inline(always)]
    pub const fn pvmo2(&self) -> super::vals::Pvmo2 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pvmo2::from_bits(val as u8)
    }
    #[doc = "Peripheral voltage monitoring output: V<sub>DDA</sub> vs. 1.8 V Note: PVMO2 is cleared when PVM2 is disabled (PVME = 0). After enabling PVM2, the PVM2 output is valid after the PVM2 wakeup time."]
    #[inline(always)]
    pub fn set_pvmo2(&mut self, val: super::vals::Pvmo2) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for PwrSr2 {
    #[inline(always)]
    fn default() -> PwrSr2 {
        PwrSr2(0)
    }
}

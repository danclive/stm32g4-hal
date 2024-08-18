#[doc = "peripheral mode configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgr1(pub u32);
impl Cfgr1 {
    #[doc = "BOOSTEN"]
    #[inline(always)]
    pub const fn boosten(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "BOOSTEN"]
    #[inline(always)]
    pub fn set_boosten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "GPIO analog switch control voltage selection"]
    #[inline(always)]
    pub const fn anaswvdd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO analog switch control voltage selection"]
    #[inline(always)]
    pub fn set_anaswvdd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "FM+ drive capability on PB6"]
    #[inline(always)]
    pub const fn i2c_pb6_fmp(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "FM+ drive capability on PB6"]
    #[inline(always)]
    pub fn set_i2c_pb6_fmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "FM+ drive capability on PB6"]
    #[inline(always)]
    pub const fn i2c_pb7_fmp(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "FM+ drive capability on PB6"]
    #[inline(always)]
    pub fn set_i2c_pb7_fmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FM+ drive capability on PB6"]
    #[inline(always)]
    pub const fn i2c_pb8_fmp(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FM+ drive capability on PB6"]
    #[inline(always)]
    pub fn set_i2c_pb8_fmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "FM+ drive capability on PB6"]
    #[inline(always)]
    pub const fn i2c_pb9_fmp(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "FM+ drive capability on PB6"]
    #[inline(always)]
    pub fn set_i2c_pb9_fmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "I2C1 FM+ drive capability enable"]
    #[inline(always)]
    pub const fn i2c1_fmp(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "I2C1 FM+ drive capability enable"]
    #[inline(always)]
    pub fn set_i2c1_fmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "I2C1 FM+ drive capability enable"]
    #[inline(always)]
    pub const fn i2c2_fmp(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "I2C1 FM+ drive capability enable"]
    #[inline(always)]
    pub fn set_i2c2_fmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "I2C1 FM+ drive capability enable"]
    #[inline(always)]
    pub const fn i2c3_fmp(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "I2C1 FM+ drive capability enable"]
    #[inline(always)]
    pub fn set_i2c3_fmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "I2C1 FM+ drive capability enable"]
    #[inline(always)]
    pub const fn i2c4_fmp(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "I2C1 FM+ drive capability enable"]
    #[inline(always)]
    pub fn set_i2c4_fmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "FPU Interrupts Enable"]
    #[inline(always)]
    pub const fn fpu_ie(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x3f;
        val as u8
    }
    #[doc = "FPU Interrupts Enable"]
    #[inline(always)]
    pub fn set_fpu_ie(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
    }
}
impl Default for Cfgr1 {
    #[inline(always)]
    fn default() -> Cfgr1 {
        Cfgr1(0)
    }
}
#[doc = "configuration register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgr2(pub u32);
impl Cfgr2 {
    #[doc = "Core Lockup Lock"]
    #[inline(always)]
    pub const fn cll(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Core Lockup Lock"]
    #[inline(always)]
    pub fn set_cll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SRAM Parity Lock"]
    #[inline(always)]
    pub const fn spl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SRAM Parity Lock"]
    #[inline(always)]
    pub fn set_spl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "PVD Lock"]
    #[inline(always)]
    pub const fn pvdl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "PVD Lock"]
    #[inline(always)]
    pub fn set_pvdl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ECC Lock"]
    #[inline(always)]
    pub const fn eccl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ECC Lock"]
    #[inline(always)]
    pub fn set_eccl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SRAM Parity Flag"]
    #[inline(always)]
    pub const fn spf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SRAM Parity Flag"]
    #[inline(always)]
    pub fn set_spf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Cfgr2 {
    #[inline(always)]
    fn default() -> Cfgr2 {
        Cfgr2(0)
    }
}
#[doc = "external interrupt configuration register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exticr1(pub u32);
impl Exticr1 {
    #[doc = "EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub const fn exti0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub fn set_exti0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub const fn exti1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub fn set_exti1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub const fn exti2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub fn set_exti2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub const fn exti3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI x configuration (x = 0 to 3)"]
    #[inline(always)]
    pub fn set_exti3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for Exticr1 {
    #[inline(always)]
    fn default() -> Exticr1 {
        Exticr1(0)
    }
}
#[doc = "external interrupt configuration register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exticr2(pub u32);
impl Exticr2 {
    #[doc = "EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    pub const fn exti4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    pub fn set_exti4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    pub const fn exti5(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    pub fn set_exti5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    pub const fn exti6(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    pub fn set_exti6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    pub const fn exti7(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    pub fn set_exti7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for Exticr2 {
    #[inline(always)]
    fn default() -> Exticr2 {
        Exticr2(0)
    }
}
#[doc = "external interrupt configuration register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exticr3(pub u32);
impl Exticr3 {
    #[doc = "EXTI x configuration (x = 8 to 11)"]
    #[inline(always)]
    pub const fn exti8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI x configuration (x = 8 to 11)"]
    #[inline(always)]
    pub fn set_exti8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "EXTI x configuration (x = 8 to 11)"]
    #[inline(always)]
    pub const fn exti9(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI x configuration (x = 8 to 11)"]
    #[inline(always)]
    pub fn set_exti9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "EXTI10"]
    #[inline(always)]
    pub const fn exti10(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI10"]
    #[inline(always)]
    pub fn set_exti10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "EXTI x configuration (x = 8 to 11)"]
    #[inline(always)]
    pub const fn exti11(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI x configuration (x = 8 to 11)"]
    #[inline(always)]
    pub fn set_exti11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for Exticr3 {
    #[inline(always)]
    fn default() -> Exticr3 {
        Exticr3(0)
    }
}
#[doc = "external interrupt configuration register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exticr4(pub u32);
impl Exticr4 {
    #[doc = "EXTI x configuration (x = 12 to 15)"]
    #[inline(always)]
    pub const fn exti12(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI x configuration (x = 12 to 15)"]
    #[inline(always)]
    pub fn set_exti12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "EXTI x configuration (x = 12 to 15)"]
    #[inline(always)]
    pub const fn exti13(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI x configuration (x = 12 to 15)"]
    #[inline(always)]
    pub fn set_exti13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "EXTI x configuration (x = 12 to 15)"]
    #[inline(always)]
    pub const fn exti14(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI x configuration (x = 12 to 15)"]
    #[inline(always)]
    pub fn set_exti14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "EXTI x configuration (x = 12 to 15)"]
    #[inline(always)]
    pub const fn exti15(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI x configuration (x = 12 to 15)"]
    #[inline(always)]
    pub fn set_exti15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for Exticr4 {
    #[inline(always)]
    fn default() -> Exticr4 {
        Exticr4(0)
    }
}
#[doc = "Remap Memory register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memrmp(pub u32);
impl Memrmp {
    #[doc = "Memory mapping selection"]
    #[inline(always)]
    pub const fn mem_mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Memory mapping selection"]
    #[inline(always)]
    pub fn set_mem_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "User Flash Bank mode"]
    #[inline(always)]
    pub const fn fb_mode(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "User Flash Bank mode"]
    #[inline(always)]
    pub fn set_fb_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Memrmp {
    #[inline(always)]
    fn default() -> Memrmp {
        Memrmp(0)
    }
}
#[doc = "CCM SRAM control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scsr(pub u32);
impl Scsr {
    #[doc = "CCM SRAM Erase"]
    #[inline(always)]
    pub const fn ccmer(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CCM SRAM Erase"]
    #[inline(always)]
    pub fn set_ccmer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CCM SRAM busy by erase operation"]
    #[inline(always)]
    pub const fn ccmbsy(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CCM SRAM busy by erase operation"]
    #[inline(always)]
    pub fn set_ccmbsy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Scsr {
    #[inline(always)]
    fn default() -> Scsr {
        Scsr(0)
    }
}
#[doc = "SRAM2 Key Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Skr(pub u32);
impl Skr {
    #[doc = "SRAM2 Key for software erase"]
    #[inline(always)]
    pub const fn key(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SRAM2 Key for software erase"]
    #[inline(always)]
    pub fn set_key(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Skr {
    #[inline(always)]
    fn default() -> Skr {
        Skr(0)
    }
}
#[doc = "SRAM Write protection register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swpr(pub u32);
impl Swpr {
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page0_wp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page0_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page1_wp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page1_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page2_wp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page2_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page3_wp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page3_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page4_wp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page4_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page5_wp(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page5_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page6_wp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page6_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page7_wp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page7_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page8_wp(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page8_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page9_wp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page9_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page10_wp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page10_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page11_wp(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page11_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page12_wp(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page12_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page13_wp(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page13_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page14_wp(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page14_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page15_wp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page15_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page16_wp(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page16_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page17_wp(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page17_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page18_wp(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page18_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page19_wp(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page19_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page20_wp(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page20_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page21_wp(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page21_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page22_wp(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page22_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page23_wp(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page23_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page24_wp(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page24_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page25_wp(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page25_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page26_wp(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page26_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page27_wp(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page27_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page28_wp(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page28_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page29_wp(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page29_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page30_wp(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page30_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub const fn page31_wp(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn set_page31_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Swpr {
    #[inline(always)]
    fn default() -> Swpr {
        Swpr(0)
    }
}

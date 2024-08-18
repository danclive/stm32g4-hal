#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Adc12en {
    #[doc = "ADC12 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "ADC12 clock enabled"]
    B_0X1 = 0x01,
}
impl Adc12en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc12en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc12en {
    #[inline(always)]
    fn from(val: u8) -> Adc12en {
        Adc12en::from_bits(val)
    }
}
impl From<Adc12en> for u8 {
    #[inline(always)]
    fn from(val: Adc12en) -> u8 {
        Adc12en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Adc12rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset ADC12 interface"]
    B_0X1 = 0x01,
}
impl Adc12rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc12rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc12rst {
    #[inline(always)]
    fn from(val: u8) -> Adc12rst {
        Adc12rst::from_bits(val)
    }
}
impl From<Adc12rst> for u8 {
    #[inline(always)]
    fn from(val: Adc12rst) -> u8 {
        Adc12rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Adc12sel {
    #[doc = "No clock selected"]
    B_0X0 = 0x0,
    #[doc = "PLL P clock selected as ADC1/2 clock"]
    B_0X1 = 0x01,
    #[doc = "System clock selected as ADC1/2 clock"]
    B_0X2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Adc12sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc12sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc12sel {
    #[inline(always)]
    fn from(val: u8) -> Adc12sel {
        Adc12sel::from_bits(val)
    }
}
impl From<Adc12sel> for u8 {
    #[inline(always)]
    fn from(val: Adc12sel) -> u8 {
        Adc12sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Adc12smen {
    #[doc = "ADC12 clocks disabled by the clock gating during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "ADC12 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Adc12smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc12smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc12smen {
    #[inline(always)]
    fn from(val: u8) -> Adc12smen {
        Adc12smen::from_bits(val)
    }
}
impl From<Adc12smen> for u8 {
    #[inline(always)]
    fn from(val: Adc12smen) -> u8 {
        Adc12smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Adc345en {
    #[doc = "ADC345 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "ADC345 clock enabled"]
    B_0X1 = 0x01,
}
impl Adc345en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc345en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc345en {
    #[inline(always)]
    fn from(val: u8) -> Adc345en {
        Adc345en::from_bits(val)
    }
}
impl From<Adc345en> for u8 {
    #[inline(always)]
    fn from(val: Adc345en) -> u8 {
        Adc345en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Adc345rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset ADC345"]
    B_0X1 = 0x01,
}
impl Adc345rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc345rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc345rst {
    #[inline(always)]
    fn from(val: u8) -> Adc345rst {
        Adc345rst::from_bits(val)
    }
}
impl From<Adc345rst> for u8 {
    #[inline(always)]
    fn from(val: Adc345rst) -> u8 {
        Adc345rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Adc345sel {
    #[doc = "No clock selected"]
    B_0X0 = 0x0,
    #[doc = "PLL P clock selected as ADC345 clock"]
    B_0X1 = 0x01,
    #[doc = "System clock selected as ADC3/4/5 clock"]
    B_0X2 = 0x02,
    #[doc = "Reserved."]
    B_0X3 = 0x03,
}
impl Adc345sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc345sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc345sel {
    #[inline(always)]
    fn from(val: u8) -> Adc345sel {
        Adc345sel::from_bits(val)
    }
}
impl From<Adc345sel> for u8 {
    #[inline(always)]
    fn from(val: Adc345sel) -> u8 {
        Adc345sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Adc345smen {
    #[doc = "ADC345 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "ADC345 clock enabled"]
    B_0X1 = 0x01,
}
impl Adc345smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc345smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc345smen {
    #[inline(always)]
    fn from(val: u8) -> Adc345smen {
        Adc345smen::from_bits(val)
    }
}
impl From<Adc345smen> for u8 {
    #[inline(always)]
    fn from(val: Adc345smen) -> u8 {
        Adc345smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Aesen {
    #[doc = "AES clock disabled"]
    B_0X0 = 0x0,
    #[doc = "AES clock enabled"]
    B_0X1 = 0x01,
}
impl Aesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aesen {
    #[inline(always)]
    fn from(val: u8) -> Aesen {
        Aesen::from_bits(val)
    }
}
impl From<Aesen> for u8 {
    #[inline(always)]
    fn from(val: Aesen) -> u8 {
        Aesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Aesrst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset AES"]
    B_0X1 = 0x01,
}
impl Aesrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aesrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aesrst {
    #[inline(always)]
    fn from(val: u8) -> Aesrst {
        Aesrst::from_bits(val)
    }
}
impl From<Aesrst> for u8 {
    #[inline(always)]
    fn from(val: Aesrst) -> u8 {
        Aesrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Aessmen {
    #[doc = "AESM clocks disabled"]
    B_0X0 = 0x0,
    #[doc = "AESM clocks enabled"]
    B_0X1 = 0x01,
}
impl Aessmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aessmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aessmen {
    #[inline(always)]
    fn from(val: u8) -> Aessmen {
        Aessmen::from_bits(val)
    }
}
impl From<Aessmen> for u8 {
    #[inline(always)]
    fn from(val: Aessmen) -> u8 {
        Aessmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Bdrst {
    #[doc = "Reset not activated"]
    B_0X0 = 0x0,
    #[doc = "Reset the entire RTC domain"]
    B_0X1 = 0x01,
}
impl Bdrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bdrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bdrst {
    #[inline(always)]
    fn from(val: u8) -> Bdrst {
        Bdrst::from_bits(val)
    }
}
impl From<Bdrst> for u8 {
    #[inline(always)]
    fn from(val: Bdrst) -> u8 {
        Bdrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Borrstf {
    #[doc = "No BOR occurred"]
    B_0X0 = 0x0,
    #[doc = "BOR occurred"]
    B_0X1 = 0x01,
}
impl Borrstf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Borrstf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Borrstf {
    #[inline(always)]
    fn from(val: u8) -> Borrstf {
        Borrstf::from_bits(val)
    }
}
impl From<Borrstf> for u8 {
    #[inline(always)]
    fn from(val: Borrstf) -> u8 {
        Borrstf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ccmsramsmen {
    #[doc = "CCM SRAM interface clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "CCM SRAM interface clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Ccmsramsmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccmsramsmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccmsramsmen {
    #[inline(always)]
    fn from(val: u8) -> Ccmsramsmen {
        Ccmsramsmen::from_bits(val)
    }
}
impl From<Ccmsramsmen> for u8 {
    #[inline(always)]
    fn from(val: Ccmsramsmen) -> u8 {
        Ccmsramsmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Clk48sel {
    #[doc = "HSI48 clock selected as 48 MHz clock"]
    B_0X0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "PLL Q clock (PLL48M1CLK) selected as 48 MHz clock"]
    B_0X2 = 0x02,
    #[doc = "Reserved, must be kept at reset value"]
    B_0X3 = 0x03,
}
impl Clk48sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clk48sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clk48sel {
    #[inline(always)]
    fn from(val: u8) -> Clk48sel {
        Clk48sel::from_bits(val)
    }
}
impl From<Clk48sel> for u8 {
    #[inline(always)]
    fn from(val: Clk48sel) -> u8 {
        Clk48sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cordicen {
    #[doc = "CORDIC clock disabled"]
    B_0X0 = 0x0,
    #[doc = "CORDIC clock enabled"]
    B_0X1 = 0x01,
}
impl Cordicen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cordicen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cordicen {
    #[inline(always)]
    fn from(val: u8) -> Cordicen {
        Cordicen::from_bits(val)
    }
}
impl From<Cordicen> for u8 {
    #[inline(always)]
    fn from(val: Cordicen) -> u8 {
        Cordicen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cordicrst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset CORDIC"]
    B_0X1 = 0x01,
}
impl Cordicrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cordicrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cordicrst {
    #[inline(always)]
    fn from(val: u8) -> Cordicrst {
        Cordicrst::from_bits(val)
    }
}
impl From<Cordicrst> for u8 {
    #[inline(always)]
    fn from(val: Cordicrst) -> u8 {
        Cordicrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cordicsmen {
    #[doc = "CORDICSM clocks disabled."]
    B_0X0 = 0x0,
    #[doc = "CORDICSM clocks enabled."]
    B_0X1 = 0x01,
}
impl Cordicsmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cordicsmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cordicsmen {
    #[inline(always)]
    fn from(val: u8) -> Cordicsmen {
        Cordicsmen::from_bits(val)
    }
}
impl From<Cordicsmen> for u8 {
    #[inline(always)]
    fn from(val: Cordicsmen) -> u8 {
        Cordicsmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Crcen {
    #[doc = "CRC clock disable"]
    B_0X0 = 0x0,
    #[doc = "CRC clock enable"]
    B_0X1 = 0x01,
}
impl Crcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crcen {
    #[inline(always)]
    fn from(val: u8) -> Crcen {
        Crcen::from_bits(val)
    }
}
impl From<Crcen> for u8 {
    #[inline(always)]
    fn from(val: Crcen) -> u8 {
        Crcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Crcrst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset CRC"]
    B_0X1 = 0x01,
}
impl Crcrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crcrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crcrst {
    #[inline(always)]
    fn from(val: u8) -> Crcrst {
        Crcrst::from_bits(val)
    }
}
impl From<Crcrst> for u8 {
    #[inline(always)]
    fn from(val: Crcrst) -> u8 {
        Crcrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Crcsmen {
    #[doc = "CRC clocks disabled by the clock gating during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "CRC clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Crcsmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crcsmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crcsmen {
    #[inline(always)]
    fn from(val: u8) -> Crcsmen {
        Crcsmen::from_bits(val)
    }
}
impl From<Crcsmen> for u8 {
    #[inline(always)]
    fn from(val: Crcsmen) -> u8 {
        Crcsmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Crsen {
    #[doc = "CRS clock disabled"]
    B_0X0 = 0x0,
    #[doc = "CRS clock enabled"]
    B_0X1 = 0x01,
}
impl Crsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crsen {
    #[inline(always)]
    fn from(val: u8) -> Crsen {
        Crsen::from_bits(val)
    }
}
impl From<Crsen> for u8 {
    #[inline(always)]
    fn from(val: Crsen) -> u8 {
        Crsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Crsrst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset CRS"]
    B_0X1 = 0x01,
}
impl Crsrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crsrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crsrst {
    #[inline(always)]
    fn from(val: u8) -> Crsrst {
        Crsrst::from_bits(val)
    }
}
impl From<Crsrst> for u8 {
    #[inline(always)]
    fn from(val: Crsrst) -> u8 {
        Crsrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Crssmen {
    #[doc = "CRS clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "CRS clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Crssmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crssmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crssmen {
    #[inline(always)]
    fn from(val: u8) -> Crssmen {
        Crssmen::from_bits(val)
    }
}
impl From<Crssmen> for u8 {
    #[inline(always)]
    fn from(val: Crssmen) -> u8 {
        Crssmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cssc {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Clear CSSF flag"]
    B_0X1 = 0x01,
}
impl Cssc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cssc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cssc {
    #[inline(always)]
    fn from(val: u8) -> Cssc {
        Cssc::from_bits(val)
    }
}
impl From<Cssc> for u8 {
    #[inline(always)]
    fn from(val: Cssc) -> u8 {
        Cssc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cssf {
    #[doc = "No clock security interrupt caused by HSE clock failure"]
    B_0X0 = 0x0,
    #[doc = "Clock security interrupt caused by HSE clock failure"]
    B_0X1 = 0x01,
}
impl Cssf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cssf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cssf {
    #[inline(always)]
    fn from(val: u8) -> Cssf {
        Cssf::from_bits(val)
    }
}
impl From<Cssf> for u8 {
    #[inline(always)]
    fn from(val: Cssf) -> u8 {
        Cssf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Csson {
    #[doc = "Clock security system OFF (clock detector OFF)"]
    B_0X0 = 0x0,
    #[doc = "Clock security system ON (Clock detector ON if the HSE oscillator is stable, OFF if not)."]
    B_0X1 = 0x01,
}
impl Csson {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csson {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csson {
    #[inline(always)]
    fn from(val: u8) -> Csson {
        Csson::from_bits(val)
    }
}
impl From<Csson> for u8 {
    #[inline(always)]
    fn from(val: Csson) -> u8 {
        Csson::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dac1en {
    #[doc = "DAC1 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "DAC1 clock enabled"]
    B_0X1 = 0x01,
}
impl Dac1en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac1en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac1en {
    #[inline(always)]
    fn from(val: u8) -> Dac1en {
        Dac1en::from_bits(val)
    }
}
impl From<Dac1en> for u8 {
    #[inline(always)]
    fn from(val: Dac1en) -> u8 {
        Dac1en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dac1rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset DAC1"]
    B_0X1 = 0x01,
}
impl Dac1rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac1rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac1rst {
    #[inline(always)]
    fn from(val: u8) -> Dac1rst {
        Dac1rst::from_bits(val)
    }
}
impl From<Dac1rst> for u8 {
    #[inline(always)]
    fn from(val: Dac1rst) -> u8 {
        Dac1rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dac1smen {
    #[doc = "DAC1 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "DAC1 clock enabled during sleep and stop modes"]
    B_0X1 = 0x01,
}
impl Dac1smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac1smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac1smen {
    #[inline(always)]
    fn from(val: u8) -> Dac1smen {
        Dac1smen::from_bits(val)
    }
}
impl From<Dac1smen> for u8 {
    #[inline(always)]
    fn from(val: Dac1smen) -> u8 {
        Dac1smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dac2en {
    #[doc = "DAC2 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "DAC2 clock enabled"]
    B_0X1 = 0x01,
}
impl Dac2en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac2en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac2en {
    #[inline(always)]
    fn from(val: u8) -> Dac2en {
        Dac2en::from_bits(val)
    }
}
impl From<Dac2en> for u8 {
    #[inline(always)]
    fn from(val: Dac2en) -> u8 {
        Dac2en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dac2rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset DAC2"]
    B_0X1 = 0x01,
}
impl Dac2rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac2rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac2rst {
    #[inline(always)]
    fn from(val: u8) -> Dac2rst {
        Dac2rst::from_bits(val)
    }
}
impl From<Dac2rst> for u8 {
    #[inline(always)]
    fn from(val: Dac2rst) -> u8 {
        Dac2rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dac2smen {
    #[doc = "DAC2 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "DAC2 clock enabled during sleep and stop modes"]
    B_0X1 = 0x01,
}
impl Dac2smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac2smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac2smen {
    #[inline(always)]
    fn from(val: u8) -> Dac2smen {
        Dac2smen::from_bits(val)
    }
}
impl From<Dac2smen> for u8 {
    #[inline(always)]
    fn from(val: Dac2smen) -> u8 {
        Dac2smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dac3en {
    #[doc = "DAC3 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "DAC3 clock enabled"]
    B_0X1 = 0x01,
}
impl Dac3en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac3en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac3en {
    #[inline(always)]
    fn from(val: u8) -> Dac3en {
        Dac3en::from_bits(val)
    }
}
impl From<Dac3en> for u8 {
    #[inline(always)]
    fn from(val: Dac3en) -> u8 {
        Dac3en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dac3rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset DAC3"]
    B_0X1 = 0x01,
}
impl Dac3rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac3rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac3rst {
    #[inline(always)]
    fn from(val: u8) -> Dac3rst {
        Dac3rst::from_bits(val)
    }
}
impl From<Dac3rst> for u8 {
    #[inline(always)]
    fn from(val: Dac3rst) -> u8 {
        Dac3rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dac3smen {
    #[doc = "DAC3 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "DAC3 clock enabled during sleep and stop modes"]
    B_0X1 = 0x01,
}
impl Dac3smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac3smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac3smen {
    #[inline(always)]
    fn from(val: u8) -> Dac3smen {
        Dac3smen::from_bits(val)
    }
}
impl From<Dac3smen> for u8 {
    #[inline(always)]
    fn from(val: Dac3smen) -> u8 {
        Dac3smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dac4en {
    #[doc = "DAC4 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "DAC4 clock enabled"]
    B_0X1 = 0x01,
}
impl Dac4en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac4en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac4en {
    #[inline(always)]
    fn from(val: u8) -> Dac4en {
        Dac4en::from_bits(val)
    }
}
impl From<Dac4en> for u8 {
    #[inline(always)]
    fn from(val: Dac4en) -> u8 {
        Dac4en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dac4rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset DAC4"]
    B_0X1 = 0x01,
}
impl Dac4rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac4rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac4rst {
    #[inline(always)]
    fn from(val: u8) -> Dac4rst {
        Dac4rst::from_bits(val)
    }
}
impl From<Dac4rst> for u8 {
    #[inline(always)]
    fn from(val: Dac4rst) -> u8 {
        Dac4rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dac4smen {
    #[doc = "DAC4 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "DAC4 clock enabled during sleep and stop modes"]
    B_0X1 = 0x01,
}
impl Dac4smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac4smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac4smen {
    #[inline(always)]
    fn from(val: u8) -> Dac4smen {
        Dac4smen::from_bits(val)
    }
}
impl From<Dac4smen> for u8 {
    #[inline(always)]
    fn from(val: Dac4smen) -> u8 {
        Dac4smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dma1en {
    #[doc = "DMA1 clock disable"]
    B_0X0 = 0x0,
    #[doc = "DMA1 clock enable"]
    B_0X1 = 0x01,
}
impl Dma1en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1en {
    #[inline(always)]
    fn from(val: u8) -> Dma1en {
        Dma1en::from_bits(val)
    }
}
impl From<Dma1en> for u8 {
    #[inline(always)]
    fn from(val: Dma1en) -> u8 {
        Dma1en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dma1rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset DMA1"]
    B_0X1 = 0x01,
}
impl Dma1rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1rst {
    #[inline(always)]
    fn from(val: u8) -> Dma1rst {
        Dma1rst::from_bits(val)
    }
}
impl From<Dma1rst> for u8 {
    #[inline(always)]
    fn from(val: Dma1rst) -> u8 {
        Dma1rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dma1smen {
    #[doc = "DMA1 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "DMA1 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Dma1smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1smen {
    #[inline(always)]
    fn from(val: u8) -> Dma1smen {
        Dma1smen::from_bits(val)
    }
}
impl From<Dma1smen> for u8 {
    #[inline(always)]
    fn from(val: Dma1smen) -> u8 {
        Dma1smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dma2en {
    #[doc = "DMA2 clock disable"]
    B_0X0 = 0x0,
    #[doc = "DMA2 clock enable"]
    B_0X1 = 0x01,
}
impl Dma2en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma2en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma2en {
    #[inline(always)]
    fn from(val: u8) -> Dma2en {
        Dma2en::from_bits(val)
    }
}
impl From<Dma2en> for u8 {
    #[inline(always)]
    fn from(val: Dma2en) -> u8 {
        Dma2en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dma2rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset DMA2"]
    B_0X1 = 0x01,
}
impl Dma2rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma2rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma2rst {
    #[inline(always)]
    fn from(val: u8) -> Dma2rst {
        Dma2rst::from_bits(val)
    }
}
impl From<Dma2rst> for u8 {
    #[inline(always)]
    fn from(val: Dma2rst) -> u8 {
        Dma2rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dma2smen {
    #[doc = "DMA2 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "DMA2 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Dma2smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma2smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma2smen {
    #[inline(always)]
    fn from(val: u8) -> Dma2smen {
        Dma2smen::from_bits(val)
    }
}
impl From<Dma2smen> for u8 {
    #[inline(always)]
    fn from(val: Dma2smen) -> u8 {
        Dma2smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dmamux1en {
    #[doc = "DMAMUX1 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "DMAMUX1 clock enabled"]
    B_0X1 = 0x01,
}
impl Dmamux1en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmamux1en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmamux1en {
    #[inline(always)]
    fn from(val: u8) -> Dmamux1en {
        Dmamux1en::from_bits(val)
    }
}
impl From<Dmamux1en> for u8 {
    #[inline(always)]
    fn from(val: Dmamux1en) -> u8 {
        Dmamux1en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dmamux1rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset DMAMUX1"]
    B_0X1 = 0x01,
}
impl Dmamux1rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmamux1rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmamux1rst {
    #[inline(always)]
    fn from(val: u8) -> Dmamux1rst {
        Dmamux1rst::from_bits(val)
    }
}
impl From<Dmamux1rst> for u8 {
    #[inline(always)]
    fn from(val: Dmamux1rst) -> u8 {
        Dmamux1rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dmamux1smen {
    #[doc = "DMAMUX1 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "DMAMUX1 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Dmamux1smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmamux1smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmamux1smen {
    #[inline(always)]
    fn from(val: u8) -> Dmamux1smen {
        Dmamux1smen::from_bits(val)
    }
}
impl From<Dmamux1smen> for u8 {
    #[inline(always)]
    fn from(val: Dmamux1smen) -> u8 {
        Dmamux1smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Fdcanen {
    #[doc = "FDCAN clock disabled"]
    B_0X0 = 0x0,
    #[doc = "FDCAN clock enabled"]
    B_0X1 = 0x01,
}
impl Fdcanen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fdcanen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fdcanen {
    #[inline(always)]
    fn from(val: u8) -> Fdcanen {
        Fdcanen::from_bits(val)
    }
}
impl From<Fdcanen> for u8 {
    #[inline(always)]
    fn from(val: Fdcanen) -> u8 {
        Fdcanen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Fdcanrst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset the FDCAN"]
    B_0X1 = 0x01,
}
impl Fdcanrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fdcanrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fdcanrst {
    #[inline(always)]
    fn from(val: u8) -> Fdcanrst {
        Fdcanrst::from_bits(val)
    }
}
impl From<Fdcanrst> for u8 {
    #[inline(always)]
    fn from(val: Fdcanrst) -> u8 {
        Fdcanrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Fdcansmen {
    #[doc = "FDCAN clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "FDCAN clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Fdcansmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fdcansmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fdcansmen {
    #[inline(always)]
    fn from(val: u8) -> Fdcansmen {
        Fdcansmen::from_bits(val)
    }
}
impl From<Fdcansmen> for u8 {
    #[inline(always)]
    fn from(val: Fdcansmen) -> u8 {
        Fdcansmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Flashen {
    #[doc = "Flash memory interface clock disable"]
    B_0X0 = 0x0,
    #[doc = "Flash memory interface clock enable"]
    B_0X1 = 0x01,
}
impl Flashen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flashen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flashen {
    #[inline(always)]
    fn from(val: u8) -> Flashen {
        Flashen::from_bits(val)
    }
}
impl From<Flashen> for u8 {
    #[inline(always)]
    fn from(val: Flashen) -> u8 {
        Flashen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Flashrst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset Flash memory interface"]
    B_0X1 = 0x01,
}
impl Flashrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flashrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flashrst {
    #[inline(always)]
    fn from(val: u8) -> Flashrst {
        Flashrst::from_bits(val)
    }
}
impl From<Flashrst> for u8 {
    #[inline(always)]
    fn from(val: Flashrst) -> u8 {
        Flashrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Flashsmen {
    #[doc = "Flash memory interface clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "Flash memory interface clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Flashsmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flashsmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flashsmen {
    #[inline(always)]
    fn from(val: u8) -> Flashsmen {
        Flashsmen::from_bits(val)
    }
}
impl From<Flashsmen> for u8 {
    #[inline(always)]
    fn from(val: Flashsmen) -> u8 {
        Flashsmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Fmacen {
    #[doc = "FMAC clock disabled"]
    B_0X0 = 0x0,
    #[doc = "FMAC clock enabled"]
    B_0X1 = 0x01,
}
impl Fmacen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fmacen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fmacen {
    #[inline(always)]
    fn from(val: u8) -> Fmacen {
        Fmacen::from_bits(val)
    }
}
impl From<Fmacen> for u8 {
    #[inline(always)]
    fn from(val: Fmacen) -> u8 {
        Fmacen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Fmacrst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset FMAC"]
    B_0X1 = 0x01,
}
impl Fmacrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fmacrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fmacrst {
    #[inline(always)]
    fn from(val: u8) -> Fmacrst {
        Fmacrst::from_bits(val)
    }
}
impl From<Fmacrst> for u8 {
    #[inline(always)]
    fn from(val: Fmacrst) -> u8 {
        Fmacrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Fmacsmen {
    #[doc = "FMACSM clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "FMACSM clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Fmacsmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fmacsmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fmacsmen {
    #[inline(always)]
    fn from(val: u8) -> Fmacsmen {
        Fmacsmen::from_bits(val)
    }
}
impl From<Fmacsmen> for u8 {
    #[inline(always)]
    fn from(val: Fmacsmen) -> u8 {
        Fmacsmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Fmcen {
    #[doc = "FSMC clock disable"]
    B_0X0 = 0x0,
    #[doc = "FSMC clock enable"]
    B_0X1 = 0x01,
}
impl Fmcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fmcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fmcen {
    #[inline(always)]
    fn from(val: u8) -> Fmcen {
        Fmcen::from_bits(val)
    }
}
impl From<Fmcen> for u8 {
    #[inline(always)]
    fn from(val: Fmcen) -> u8 {
        Fmcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Fmcrst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset FSMC"]
    B_0X1 = 0x01,
}
impl Fmcrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fmcrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fmcrst {
    #[inline(always)]
    fn from(val: u8) -> Fmcrst {
        Fmcrst::from_bits(val)
    }
}
impl From<Fmcrst> for u8 {
    #[inline(always)]
    fn from(val: Fmcrst) -> u8 {
        Fmcrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Fmcsmen {
    #[doc = "FSMC clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "FSMC clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Fmcsmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fmcsmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fmcsmen {
    #[inline(always)]
    fn from(val: u8) -> Fmcsmen {
        Fmcsmen::from_bits(val)
    }
}
impl From<Fmcsmen> for u8 {
    #[inline(always)]
    fn from(val: Fmcsmen) -> u8 {
        Fmcsmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpioaen {
    #[doc = "IO port A clock disabled"]
    B_0X0 = 0x0,
    #[doc = "IO port A clock enabled"]
    B_0X1 = 0x01,
}
impl Gpioaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpioaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpioaen {
    #[inline(always)]
    fn from(val: u8) -> Gpioaen {
        Gpioaen::from_bits(val)
    }
}
impl From<Gpioaen> for u8 {
    #[inline(always)]
    fn from(val: Gpioaen) -> u8 {
        Gpioaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpioarst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset IO port A"]
    B_0X1 = 0x01,
}
impl Gpioarst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpioarst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpioarst {
    #[inline(always)]
    fn from(val: u8) -> Gpioarst {
        Gpioarst::from_bits(val)
    }
}
impl From<Gpioarst> for u8 {
    #[inline(always)]
    fn from(val: Gpioarst) -> u8 {
        Gpioarst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpioasmen {
    #[doc = "IO port A clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "IO port A clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Gpioasmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpioasmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpioasmen {
    #[inline(always)]
    fn from(val: u8) -> Gpioasmen {
        Gpioasmen::from_bits(val)
    }
}
impl From<Gpioasmen> for u8 {
    #[inline(always)]
    fn from(val: Gpioasmen) -> u8 {
        Gpioasmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpioben {
    #[doc = "IO port B clock disabled"]
    B_0X0 = 0x0,
    #[doc = "IO port B clock enabled"]
    B_0X1 = 0x01,
}
impl Gpioben {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpioben {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpioben {
    #[inline(always)]
    fn from(val: u8) -> Gpioben {
        Gpioben::from_bits(val)
    }
}
impl From<Gpioben> for u8 {
    #[inline(always)]
    fn from(val: Gpioben) -> u8 {
        Gpioben::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpiobrst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset IO port B"]
    B_0X1 = 0x01,
}
impl Gpiobrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpiobrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpiobrst {
    #[inline(always)]
    fn from(val: u8) -> Gpiobrst {
        Gpiobrst::from_bits(val)
    }
}
impl From<Gpiobrst> for u8 {
    #[inline(always)]
    fn from(val: Gpiobrst) -> u8 {
        Gpiobrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpiobsmen {
    #[doc = "IO port B clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "IO port B clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Gpiobsmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpiobsmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpiobsmen {
    #[inline(always)]
    fn from(val: u8) -> Gpiobsmen {
        Gpiobsmen::from_bits(val)
    }
}
impl From<Gpiobsmen> for u8 {
    #[inline(always)]
    fn from(val: Gpiobsmen) -> u8 {
        Gpiobsmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpiocen {
    #[doc = "IO port C clock disabled"]
    B_0X0 = 0x0,
    #[doc = "IO port C clock enabled"]
    B_0X1 = 0x01,
}
impl Gpiocen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpiocen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpiocen {
    #[inline(always)]
    fn from(val: u8) -> Gpiocen {
        Gpiocen::from_bits(val)
    }
}
impl From<Gpiocen> for u8 {
    #[inline(always)]
    fn from(val: Gpiocen) -> u8 {
        Gpiocen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpiocrst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset IO port C"]
    B_0X1 = 0x01,
}
impl Gpiocrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpiocrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpiocrst {
    #[inline(always)]
    fn from(val: u8) -> Gpiocrst {
        Gpiocrst::from_bits(val)
    }
}
impl From<Gpiocrst> for u8 {
    #[inline(always)]
    fn from(val: Gpiocrst) -> u8 {
        Gpiocrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpiocsmen {
    #[doc = "IO port C clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "IO port C clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Gpiocsmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpiocsmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpiocsmen {
    #[inline(always)]
    fn from(val: u8) -> Gpiocsmen {
        Gpiocsmen::from_bits(val)
    }
}
impl From<Gpiocsmen> for u8 {
    #[inline(always)]
    fn from(val: Gpiocsmen) -> u8 {
        Gpiocsmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpioden {
    #[doc = "IO port D clock disabled"]
    B_0X0 = 0x0,
    #[doc = "IO port D clock enabled"]
    B_0X1 = 0x01,
}
impl Gpioden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpioden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpioden {
    #[inline(always)]
    fn from(val: u8) -> Gpioden {
        Gpioden::from_bits(val)
    }
}
impl From<Gpioden> for u8 {
    #[inline(always)]
    fn from(val: Gpioden) -> u8 {
        Gpioden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpiodrst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset IO port D"]
    B_0X1 = 0x01,
}
impl Gpiodrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpiodrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpiodrst {
    #[inline(always)]
    fn from(val: u8) -> Gpiodrst {
        Gpiodrst::from_bits(val)
    }
}
impl From<Gpiodrst> for u8 {
    #[inline(always)]
    fn from(val: Gpiodrst) -> u8 {
        Gpiodrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpiodsmen {
    #[doc = "IO port D clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "IO port D clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Gpiodsmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpiodsmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpiodsmen {
    #[inline(always)]
    fn from(val: u8) -> Gpiodsmen {
        Gpiodsmen::from_bits(val)
    }
}
impl From<Gpiodsmen> for u8 {
    #[inline(always)]
    fn from(val: Gpiodsmen) -> u8 {
        Gpiodsmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpioeen {
    #[doc = "IO port E clock disabled"]
    B_0X0 = 0x0,
    #[doc = "IO port E clock enabled"]
    B_0X1 = 0x01,
}
impl Gpioeen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpioeen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpioeen {
    #[inline(always)]
    fn from(val: u8) -> Gpioeen {
        Gpioeen::from_bits(val)
    }
}
impl From<Gpioeen> for u8 {
    #[inline(always)]
    fn from(val: Gpioeen) -> u8 {
        Gpioeen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpioerst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset IO port E"]
    B_0X1 = 0x01,
}
impl Gpioerst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpioerst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpioerst {
    #[inline(always)]
    fn from(val: u8) -> Gpioerst {
        Gpioerst::from_bits(val)
    }
}
impl From<Gpioerst> for u8 {
    #[inline(always)]
    fn from(val: Gpioerst) -> u8 {
        Gpioerst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpioesmen {
    #[doc = "IO port E clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "IO port E clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Gpioesmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpioesmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpioesmen {
    #[inline(always)]
    fn from(val: u8) -> Gpioesmen {
        Gpioesmen::from_bits(val)
    }
}
impl From<Gpioesmen> for u8 {
    #[inline(always)]
    fn from(val: Gpioesmen) -> u8 {
        Gpioesmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpiofen {
    #[doc = "IO port F clock disabled"]
    B_0X0 = 0x0,
    #[doc = "IO port F clock enabled"]
    B_0X1 = 0x01,
}
impl Gpiofen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpiofen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpiofen {
    #[inline(always)]
    fn from(val: u8) -> Gpiofen {
        Gpiofen::from_bits(val)
    }
}
impl From<Gpiofen> for u8 {
    #[inline(always)]
    fn from(val: Gpiofen) -> u8 {
        Gpiofen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpiofrst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset IO port F"]
    B_0X1 = 0x01,
}
impl Gpiofrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpiofrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpiofrst {
    #[inline(always)]
    fn from(val: u8) -> Gpiofrst {
        Gpiofrst::from_bits(val)
    }
}
impl From<Gpiofrst> for u8 {
    #[inline(always)]
    fn from(val: Gpiofrst) -> u8 {
        Gpiofrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpiofsmen {
    #[doc = "IO port F clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "IO port F clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Gpiofsmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpiofsmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpiofsmen {
    #[inline(always)]
    fn from(val: u8) -> Gpiofsmen {
        Gpiofsmen::from_bits(val)
    }
}
impl From<Gpiofsmen> for u8 {
    #[inline(always)]
    fn from(val: Gpiofsmen) -> u8 {
        Gpiofsmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpiogen {
    #[doc = "IO port G clock disabled"]
    B_0X0 = 0x0,
    #[doc = "IO port G clock enabled"]
    B_0X1 = 0x01,
}
impl Gpiogen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpiogen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpiogen {
    #[inline(always)]
    fn from(val: u8) -> Gpiogen {
        Gpiogen::from_bits(val)
    }
}
impl From<Gpiogen> for u8 {
    #[inline(always)]
    fn from(val: Gpiogen) -> u8 {
        Gpiogen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpiogrst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset IO port G"]
    B_0X1 = 0x01,
}
impl Gpiogrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpiogrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpiogrst {
    #[inline(always)]
    fn from(val: u8) -> Gpiogrst {
        Gpiogrst::from_bits(val)
    }
}
impl From<Gpiogrst> for u8 {
    #[inline(always)]
    fn from(val: Gpiogrst) -> u8 {
        Gpiogrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpiogsmen {
    #[doc = "IO port G clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "IO port G clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Gpiogsmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpiogsmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpiogsmen {
    #[inline(always)]
    fn from(val: u8) -> Gpiogsmen {
        Gpiogsmen::from_bits(val)
    }
}
impl From<Gpiogsmen> for u8 {
    #[inline(always)]
    fn from(val: Gpiogsmen) -> u8 {
        Gpiogsmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hpre {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "SYSCLK divided by 2"]
    B_0X8 = 0x08,
    #[doc = "SYSCLK divided by 4"]
    B_0X9 = 0x09,
    #[doc = "SYSCLK divided by 8"]
    B_0XA = 0x0a,
    #[doc = "SYSCLK divided by 16"]
    B_0XB = 0x0b,
    #[doc = "SYSCLK divided by 64"]
    B_0XC = 0x0c,
    #[doc = "SYSCLK divided by 128"]
    B_0XD = 0x0d,
    #[doc = "SYSCLK divided by 256"]
    B_0XE = 0x0e,
    #[doc = "SYSCLK divided by 512"]
    B_0XF = 0x0f,
}
impl Hpre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hpre {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hpre {
    #[inline(always)]
    fn from(val: u8) -> Hpre {
        Hpre::from_bits(val)
    }
}
impl From<Hpre> for u8 {
    #[inline(always)]
    fn from(val: Hpre) -> u8 {
        Hpre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hrtim1en {
    #[doc = "HRTIM1 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "HRTIM1 clock enable"]
    B_0X1 = 0x01,
}
impl Hrtim1en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hrtim1en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hrtim1en {
    #[inline(always)]
    fn from(val: u8) -> Hrtim1en {
        Hrtim1en::from_bits(val)
    }
}
impl From<Hrtim1en> for u8 {
    #[inline(always)]
    fn from(val: Hrtim1en) -> u8 {
        Hrtim1en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hrtim1rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset HRTIM1"]
    B_0X1 = 0x01,
}
impl Hrtim1rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hrtim1rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hrtim1rst {
    #[inline(always)]
    fn from(val: u8) -> Hrtim1rst {
        Hrtim1rst::from_bits(val)
    }
}
impl From<Hrtim1rst> for u8 {
    #[inline(always)]
    fn from(val: Hrtim1rst) -> u8 {
        Hrtim1rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hrtim1smen {
    #[doc = "HRTIM1 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "HRTIM1 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Hrtim1smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hrtim1smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hrtim1smen {
    #[inline(always)]
    fn from(val: u8) -> Hrtim1smen {
        Hrtim1smen::from_bits(val)
    }
}
impl From<Hrtim1smen> for u8 {
    #[inline(always)]
    fn from(val: Hrtim1smen) -> u8 {
        Hrtim1smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hsebyp {
    #[doc = "HSE crystal oscillator not bypassed"]
    B_0X0 = 0x0,
    #[doc = "HSE crystal oscillator bypassed with external clock"]
    B_0X1 = 0x01,
}
impl Hsebyp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsebyp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsebyp {
    #[inline(always)]
    fn from(val: u8) -> Hsebyp {
        Hsebyp::from_bits(val)
    }
}
impl From<Hsebyp> for u8 {
    #[inline(always)]
    fn from(val: Hsebyp) -> u8 {
        Hsebyp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hseon {
    #[doc = "HSE oscillator OFF"]
    B_0X0 = 0x0,
    #[doc = "HSE oscillator ON"]
    B_0X1 = 0x01,
}
impl Hseon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hseon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hseon {
    #[inline(always)]
    fn from(val: u8) -> Hseon {
        Hseon::from_bits(val)
    }
}
impl From<Hseon> for u8 {
    #[inline(always)]
    fn from(val: Hseon) -> u8 {
        Hseon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hserdy {
    #[doc = "HSE oscillator not ready"]
    B_0X0 = 0x0,
    #[doc = "HSE oscillator ready"]
    B_0X1 = 0x01,
}
impl Hserdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hserdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hserdy {
    #[inline(always)]
    fn from(val: u8) -> Hserdy {
        Hserdy::from_bits(val)
    }
}
impl From<Hserdy> for u8 {
    #[inline(always)]
    fn from(val: Hserdy) -> u8 {
        Hserdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hserdyc {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Clear HSERDYF flag"]
    B_0X1 = 0x01,
}
impl Hserdyc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hserdyc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hserdyc {
    #[inline(always)]
    fn from(val: u8) -> Hserdyc {
        Hserdyc::from_bits(val)
    }
}
impl From<Hserdyc> for u8 {
    #[inline(always)]
    fn from(val: Hserdyc) -> u8 {
        Hserdyc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hserdyf {
    #[doc = "No clock ready interrupt caused by the HSE oscillator"]
    B_0X0 = 0x0,
    #[doc = "Clock ready interrupt caused by the HSE oscillator"]
    B_0X1 = 0x01,
}
impl Hserdyf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hserdyf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hserdyf {
    #[inline(always)]
    fn from(val: u8) -> Hserdyf {
        Hserdyf::from_bits(val)
    }
}
impl From<Hserdyf> for u8 {
    #[inline(always)]
    fn from(val: Hserdyf) -> u8 {
        Hserdyf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hserdyie {
    #[doc = "HSE ready interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "HSE ready interrupt enabled"]
    B_0X1 = 0x01,
}
impl Hserdyie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hserdyie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hserdyie {
    #[inline(always)]
    fn from(val: u8) -> Hserdyie {
        Hserdyie::from_bits(val)
    }
}
impl From<Hserdyie> for u8 {
    #[inline(always)]
    fn from(val: Hserdyie) -> u8 {
        Hserdyie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hsi48on {
    #[doc = "HSI48 oscillator OFF"]
    B_0X0 = 0x0,
    #[doc = "HSI48 oscillator ON"]
    B_0X1 = 0x01,
}
impl Hsi48on {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsi48on {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsi48on {
    #[inline(always)]
    fn from(val: u8) -> Hsi48on {
        Hsi48on::from_bits(val)
    }
}
impl From<Hsi48on> for u8 {
    #[inline(always)]
    fn from(val: Hsi48on) -> u8 {
        Hsi48on::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hsi48rdy {
    #[doc = "HSI48 oscillator not ready"]
    B_0X0 = 0x0,
    #[doc = "HSI48 oscillator ready"]
    B_0X1 = 0x01,
}
impl Hsi48rdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsi48rdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsi48rdy {
    #[inline(always)]
    fn from(val: u8) -> Hsi48rdy {
        Hsi48rdy::from_bits(val)
    }
}
impl From<Hsi48rdy> for u8 {
    #[inline(always)]
    fn from(val: Hsi48rdy) -> u8 {
        Hsi48rdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hsi48rdyc {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Clear the HSI48RDYC flag"]
    B_0X1 = 0x01,
}
impl Hsi48rdyc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsi48rdyc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsi48rdyc {
    #[inline(always)]
    fn from(val: u8) -> Hsi48rdyc {
        Hsi48rdyc::from_bits(val)
    }
}
impl From<Hsi48rdyc> for u8 {
    #[inline(always)]
    fn from(val: Hsi48rdyc) -> u8 {
        Hsi48rdyc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hsi48rdyf {
    #[doc = "No clock ready interrupt caused by the HSI48 oscillator"]
    B_0X0 = 0x0,
    #[doc = "Clock ready interrupt caused by the HSI48 oscillator"]
    B_0X1 = 0x01,
}
impl Hsi48rdyf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsi48rdyf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsi48rdyf {
    #[inline(always)]
    fn from(val: u8) -> Hsi48rdyf {
        Hsi48rdyf::from_bits(val)
    }
}
impl From<Hsi48rdyf> for u8 {
    #[inline(always)]
    fn from(val: Hsi48rdyf) -> u8 {
        Hsi48rdyf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hsi48rdyie {
    #[doc = "HSI48 ready interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "HSI48 ready interrupt enabled"]
    B_0X1 = 0x01,
}
impl Hsi48rdyie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsi48rdyie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsi48rdyie {
    #[inline(always)]
    fn from(val: u8) -> Hsi48rdyie {
        Hsi48rdyie::from_bits(val)
    }
}
impl From<Hsi48rdyie> for u8 {
    #[inline(always)]
    fn from(val: Hsi48rdyie) -> u8 {
        Hsi48rdyie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hsikeron {
    #[doc = "No effect on HSI16 oscillator."]
    B_0X0 = 0x0,
    #[doc = "HSI16 oscillator is forced ON even in Stop mode."]
    B_0X1 = 0x01,
}
impl Hsikeron {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsikeron {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsikeron {
    #[inline(always)]
    fn from(val: u8) -> Hsikeron {
        Hsikeron::from_bits(val)
    }
}
impl From<Hsikeron> for u8 {
    #[inline(always)]
    fn from(val: Hsikeron) -> u8 {
        Hsikeron::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hsion {
    #[doc = "HSI16 oscillator OFF"]
    B_0X0 = 0x0,
    #[doc = "HSI16 oscillator ON"]
    B_0X1 = 0x01,
}
impl Hsion {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsion {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsion {
    #[inline(always)]
    fn from(val: u8) -> Hsion {
        Hsion::from_bits(val)
    }
}
impl From<Hsion> for u8 {
    #[inline(always)]
    fn from(val: Hsion) -> u8 {
        Hsion::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hsirdy {
    #[doc = "HSI16 oscillator not ready"]
    B_0X0 = 0x0,
    #[doc = "HSI16 oscillator ready"]
    B_0X1 = 0x01,
}
impl Hsirdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsirdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsirdy {
    #[inline(always)]
    fn from(val: u8) -> Hsirdy {
        Hsirdy::from_bits(val)
    }
}
impl From<Hsirdy> for u8 {
    #[inline(always)]
    fn from(val: Hsirdy) -> u8 {
        Hsirdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hsirdyc {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Clear HSIRDYF flag"]
    B_0X1 = 0x01,
}
impl Hsirdyc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsirdyc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsirdyc {
    #[inline(always)]
    fn from(val: u8) -> Hsirdyc {
        Hsirdyc::from_bits(val)
    }
}
impl From<Hsirdyc> for u8 {
    #[inline(always)]
    fn from(val: Hsirdyc) -> u8 {
        Hsirdyc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hsirdyf {
    #[doc = "No clock ready interrupt caused by the HSI16 oscillator"]
    B_0X0 = 0x0,
    #[doc = "Clock ready interrupt caused by the HSI16 oscillator"]
    B_0X1 = 0x01,
}
impl Hsirdyf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsirdyf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsirdyf {
    #[inline(always)]
    fn from(val: u8) -> Hsirdyf {
        Hsirdyf::from_bits(val)
    }
}
impl From<Hsirdyf> for u8 {
    #[inline(always)]
    fn from(val: Hsirdyf) -> u8 {
        Hsirdyf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hsirdyie {
    #[doc = "HSI16 ready interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "HSI16 ready interrupt enabled"]
    B_0X1 = 0x01,
}
impl Hsirdyie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsirdyie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsirdyie {
    #[inline(always)]
    fn from(val: u8) -> Hsirdyie {
        Hsirdyie::from_bits(val)
    }
}
impl From<Hsirdyie> for u8 {
    #[inline(always)]
    fn from(val: Hsirdyie) -> u8 {
        Hsirdyie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum I2c1en {
    #[doc = "I2C1 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "I2C1 clock enabled"]
    B_0X1 = 0x01,
}
impl I2c1en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2c1en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2c1en {
    #[inline(always)]
    fn from(val: u8) -> I2c1en {
        I2c1en::from_bits(val)
    }
}
impl From<I2c1en> for u8 {
    #[inline(always)]
    fn from(val: I2c1en) -> u8 {
        I2c1en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum I2c1rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset I2C1"]
    B_0X1 = 0x01,
}
impl I2c1rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2c1rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2c1rst {
    #[inline(always)]
    fn from(val: u8) -> I2c1rst {
        I2c1rst::from_bits(val)
    }
}
impl From<I2c1rst> for u8 {
    #[inline(always)]
    fn from(val: I2c1rst) -> u8 {
        I2c1rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum I2c1sel {
    #[doc = "PCLK selected as I2C1 clock"]
    B_0X0 = 0x0,
    #[doc = "System clock (SYSCLK) selected as I2C1 clock"]
    B_0X1 = 0x01,
    #[doc = "HSI16 clock selected as I2C1 clock"]
    B_0X2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl I2c1sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2c1sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2c1sel {
    #[inline(always)]
    fn from(val: u8) -> I2c1sel {
        I2c1sel::from_bits(val)
    }
}
impl From<I2c1sel> for u8 {
    #[inline(always)]
    fn from(val: I2c1sel) -> u8 {
        I2c1sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum I2c1smen {
    #[doc = "I2C1 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "I2C1 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl I2c1smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2c1smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2c1smen {
    #[inline(always)]
    fn from(val: u8) -> I2c1smen {
        I2c1smen::from_bits(val)
    }
}
impl From<I2c1smen> for u8 {
    #[inline(always)]
    fn from(val: I2c1smen) -> u8 {
        I2c1smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum I2c2en {
    #[doc = "I2C2 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "I2C2 clock enabled"]
    B_0X1 = 0x01,
}
impl I2c2en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2c2en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2c2en {
    #[inline(always)]
    fn from(val: u8) -> I2c2en {
        I2c2en::from_bits(val)
    }
}
impl From<I2c2en> for u8 {
    #[inline(always)]
    fn from(val: I2c2en) -> u8 {
        I2c2en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum I2c2rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset I2C2"]
    B_0X1 = 0x01,
}
impl I2c2rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2c2rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2c2rst {
    #[inline(always)]
    fn from(val: u8) -> I2c2rst {
        I2c2rst::from_bits(val)
    }
}
impl From<I2c2rst> for u8 {
    #[inline(always)]
    fn from(val: I2c2rst) -> u8 {
        I2c2rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum I2c2sel {
    #[doc = "PCLK selected as I2C2 clock"]
    B_0X0 = 0x0,
    #[doc = "System clock (SYSCLK) selected as I2C2 clock"]
    B_0X1 = 0x01,
    #[doc = "HSI16 clock selected as I2C2 clock"]
    B_0X2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl I2c2sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2c2sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2c2sel {
    #[inline(always)]
    fn from(val: u8) -> I2c2sel {
        I2c2sel::from_bits(val)
    }
}
impl From<I2c2sel> for u8 {
    #[inline(always)]
    fn from(val: I2c2sel) -> u8 {
        I2c2sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum I2c2smen {
    #[doc = "I2C2 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "I2C2 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl I2c2smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2c2smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2c2smen {
    #[inline(always)]
    fn from(val: u8) -> I2c2smen {
        I2c2smen::from_bits(val)
    }
}
impl From<I2c2smen> for u8 {
    #[inline(always)]
    fn from(val: I2c2smen) -> u8 {
        I2c2smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum I2c3en {
    #[doc = "I2C3 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "I2C3 clock enabled"]
    B_0X1 = 0x01,
}
impl I2c3en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2c3en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2c3en {
    #[inline(always)]
    fn from(val: u8) -> I2c3en {
        I2c3en::from_bits(val)
    }
}
impl From<I2c3en> for u8 {
    #[inline(always)]
    fn from(val: I2c3en) -> u8 {
        I2c3en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum I2c3rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset I2C3 interface"]
    B_0X1 = 0x01,
}
impl I2c3rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2c3rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2c3rst {
    #[inline(always)]
    fn from(val: u8) -> I2c3rst {
        I2c3rst::from_bits(val)
    }
}
impl From<I2c3rst> for u8 {
    #[inline(always)]
    fn from(val: I2c3rst) -> u8 {
        I2c3rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum I2c3sel {
    #[doc = "PCLK selected as I2C3 clock"]
    B_0X0 = 0x0,
    #[doc = "System clock (SYSCLK) selected as I2C3 clock"]
    B_0X1 = 0x01,
    #[doc = "HSI16 clock selected as I2C3 clock"]
    B_0X2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl I2c3sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2c3sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2c3sel {
    #[inline(always)]
    fn from(val: u8) -> I2c3sel {
        I2c3sel::from_bits(val)
    }
}
impl From<I2c3sel> for u8 {
    #[inline(always)]
    fn from(val: I2c3sel) -> u8 {
        I2c3sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum I2c3smen {
    #[doc = "I2C3 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "I2C3 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl I2c3smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2c3smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2c3smen {
    #[inline(always)]
    fn from(val: u8) -> I2c3smen {
        I2c3smen::from_bits(val)
    }
}
impl From<I2c3smen> for u8 {
    #[inline(always)]
    fn from(val: I2c3smen) -> u8 {
        I2c3smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum I2c4en {
    #[doc = "I2C4 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "I2C4 clock enabled"]
    B_0X1 = 0x01,
}
impl I2c4en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2c4en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2c4en {
    #[inline(always)]
    fn from(val: u8) -> I2c4en {
        I2c4en::from_bits(val)
    }
}
impl From<I2c4en> for u8 {
    #[inline(always)]
    fn from(val: I2c4en) -> u8 {
        I2c4en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum I2c4rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset I2C4"]
    B_0X1 = 0x01,
}
impl I2c4rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2c4rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2c4rst {
    #[inline(always)]
    fn from(val: u8) -> I2c4rst {
        I2c4rst::from_bits(val)
    }
}
impl From<I2c4rst> for u8 {
    #[inline(always)]
    fn from(val: I2c4rst) -> u8 {
        I2c4rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum I2c4sel {
    #[doc = "PCLK selected as I2C4 clock"]
    B_0X0 = 0x0,
    #[doc = "System clock (SYSCLK) selected as I2C4 clock"]
    B_0X1 = 0x01,
    #[doc = "HSI16 clock selected as I2C4 clock"]
    B_0X2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl I2c4sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2c4sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2c4sel {
    #[inline(always)]
    fn from(val: u8) -> I2c4sel {
        I2c4sel::from_bits(val)
    }
}
impl From<I2c4sel> for u8 {
    #[inline(always)]
    fn from(val: I2c4sel) -> u8 {
        I2c4sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum I2c4smen {
    #[doc = "I2C4 clocks disabled by the clock gating during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "I2C4 clock enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl I2c4smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2c4smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2c4smen {
    #[inline(always)]
    fn from(val: u8) -> I2c4smen {
        I2c4smen::from_bits(val)
    }
}
impl From<I2c4smen> for u8 {
    #[inline(always)]
    fn from(val: I2c4smen) -> u8 {
        I2c4smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum I2s23sel {
    #[doc = "System clock selected as I2S23 clock"]
    B_0X0 = 0x0,
    #[doc = "PLL Q clock selected as I2S23 clock"]
    B_0X1 = 0x01,
    #[doc = "Clock provided on I2S_CKIN pin is selected as I2S23 clock"]
    B_0X2 = 0x02,
    #[doc = "HSI16 clock selected as I2S23 clock."]
    B_0X3 = 0x03,
}
impl I2s23sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2s23sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2s23sel {
    #[inline(always)]
    fn from(val: u8) -> I2s23sel {
        I2s23sel::from_bits(val)
    }
}
impl From<I2s23sel> for u8 {
    #[inline(always)]
    fn from(val: I2s23sel) -> u8 {
        I2s23sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Iwdgrstf {
    #[doc = "No independent watchdog reset occurred"]
    B_0X0 = 0x0,
    #[doc = "Independent watchdog reset occurred"]
    B_0X1 = 0x01,
}
impl Iwdgrstf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iwdgrstf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iwdgrstf {
    #[inline(always)]
    fn from(val: u8) -> Iwdgrstf {
        Iwdgrstf::from_bits(val)
    }
}
impl From<Iwdgrstf> for u8 {
    #[inline(always)]
    fn from(val: Iwdgrstf) -> u8 {
        Iwdgrstf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lptim1en {
    #[doc = "LPTIM1 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "LPTIM1 clock enabled"]
    B_0X1 = 0x01,
}
impl Lptim1en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lptim1en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lptim1en {
    #[inline(always)]
    fn from(val: u8) -> Lptim1en {
        Lptim1en::from_bits(val)
    }
}
impl From<Lptim1en> for u8 {
    #[inline(always)]
    fn from(val: Lptim1en) -> u8 {
        Lptim1en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lptim1rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset LPTIM1"]
    B_0X1 = 0x01,
}
impl Lptim1rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lptim1rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lptim1rst {
    #[inline(always)]
    fn from(val: u8) -> Lptim1rst {
        Lptim1rst::from_bits(val)
    }
}
impl From<Lptim1rst> for u8 {
    #[inline(always)]
    fn from(val: Lptim1rst) -> u8 {
        Lptim1rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lptim1sel {
    #[doc = "PCLK selected as LPTIM1 clock"]
    B_0X0 = 0x0,
    #[doc = "LSI clock selected as LPTIM1 clock"]
    B_0X1 = 0x01,
    #[doc = "HSI16 clock selected as LPTIM1 clock"]
    B_0X2 = 0x02,
    #[doc = "LSE clock selected as LPTIM1 clock"]
    B_0X3 = 0x03,
}
impl Lptim1sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lptim1sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lptim1sel {
    #[inline(always)]
    fn from(val: u8) -> Lptim1sel {
        Lptim1sel::from_bits(val)
    }
}
impl From<Lptim1sel> for u8 {
    #[inline(always)]
    fn from(val: Lptim1sel) -> u8 {
        Lptim1sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lptim1smen {
    #[doc = "LPTIM1 clocks disabled by the clock gating during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "LPTIM1 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Lptim1smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lptim1smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lptim1smen {
    #[inline(always)]
    fn from(val: u8) -> Lptim1smen {
        Lptim1smen::from_bits(val)
    }
}
impl From<Lptim1smen> for u8 {
    #[inline(always)]
    fn from(val: Lptim1smen) -> u8 {
        Lptim1smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lpuart1en {
    #[doc = "LPUART1 clock disable"]
    B_0X0 = 0x0,
    #[doc = "LPUART1 clock enable"]
    B_0X1 = 0x01,
}
impl Lpuart1en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart1en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart1en {
    #[inline(always)]
    fn from(val: u8) -> Lpuart1en {
        Lpuart1en::from_bits(val)
    }
}
impl From<Lpuart1en> for u8 {
    #[inline(always)]
    fn from(val: Lpuart1en) -> u8 {
        Lpuart1en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lpuart1rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset LPUART1"]
    B_0X1 = 0x01,
}
impl Lpuart1rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart1rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart1rst {
    #[inline(always)]
    fn from(val: u8) -> Lpuart1rst {
        Lpuart1rst::from_bits(val)
    }
}
impl From<Lpuart1rst> for u8 {
    #[inline(always)]
    fn from(val: Lpuart1rst) -> u8 {
        Lpuart1rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lpuart1sel {
    #[doc = "PCLK selected as LPUART1 clock"]
    B_0X0 = 0x0,
    #[doc = "System clock (SYSCLK) selected as LPUART1 clock"]
    B_0X1 = 0x01,
    #[doc = "HSI16 clock selected as LPUART1 clock"]
    B_0X2 = 0x02,
    #[doc = "LSE clock selected as LPUART1 clock"]
    B_0X3 = 0x03,
}
impl Lpuart1sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart1sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart1sel {
    #[inline(always)]
    fn from(val: u8) -> Lpuart1sel {
        Lpuart1sel::from_bits(val)
    }
}
impl From<Lpuart1sel> for u8 {
    #[inline(always)]
    fn from(val: Lpuart1sel) -> u8 {
        Lpuart1sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lpuart1smen {
    #[doc = "LPUART1 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "LPUART1 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Lpuart1smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart1smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart1smen {
    #[inline(always)]
    fn from(val: u8) -> Lpuart1smen {
        Lpuart1smen::from_bits(val)
    }
}
impl From<Lpuart1smen> for u8 {
    #[inline(always)]
    fn from(val: Lpuart1smen) -> u8 {
        Lpuart1smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lpwrrstf {
    #[doc = "No illegal mode reset occurred"]
    B_0X0 = 0x0,
    #[doc = "Illegal mode reset occurred"]
    B_0X1 = 0x01,
}
impl Lpwrrstf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpwrrstf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpwrrstf {
    #[inline(always)]
    fn from(val: u8) -> Lpwrrstf {
        Lpwrrstf::from_bits(val)
    }
}
impl From<Lpwrrstf> for u8 {
    #[inline(always)]
    fn from(val: Lpwrrstf) -> u8 {
        Lpwrrstf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lscoen {
    #[doc = "Low speed clock output (LSCO) disable"]
    B_0X0 = 0x0,
    #[doc = "Low speed clock output (LSCO) enable"]
    B_0X1 = 0x01,
}
impl Lscoen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lscoen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lscoen {
    #[inline(always)]
    fn from(val: u8) -> Lscoen {
        Lscoen::from_bits(val)
    }
}
impl From<Lscoen> for u8 {
    #[inline(always)]
    fn from(val: Lscoen) -> u8 {
        Lscoen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lscosel {
    #[doc = "LSI clock selected"]
    B_0X0 = 0x0,
    #[doc = "LSE clock selected"]
    B_0X1 = 0x01,
}
impl Lscosel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lscosel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lscosel {
    #[inline(always)]
    fn from(val: u8) -> Lscosel {
        Lscosel::from_bits(val)
    }
}
impl From<Lscosel> for u8 {
    #[inline(always)]
    fn from(val: Lscosel) -> u8 {
        Lscosel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lsebyp {
    #[doc = "LSE oscillator not bypassed"]
    B_0X0 = 0x0,
    #[doc = "LSE oscillator bypassed"]
    B_0X1 = 0x01,
}
impl Lsebyp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsebyp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsebyp {
    #[inline(always)]
    fn from(val: u8) -> Lsebyp {
        Lsebyp::from_bits(val)
    }
}
impl From<Lsebyp> for u8 {
    #[inline(always)]
    fn from(val: Lsebyp) -> u8 {
        Lsebyp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lsecssc {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Clear LSECSSF flag"]
    B_0X1 = 0x01,
}
impl Lsecssc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsecssc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsecssc {
    #[inline(always)]
    fn from(val: u8) -> Lsecssc {
        Lsecssc::from_bits(val)
    }
}
impl From<Lsecssc> for u8 {
    #[inline(always)]
    fn from(val: Lsecssc) -> u8 {
        Lsecssc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lsecssd {
    #[doc = "No failure detected on LSE (32 kHz oscillator)"]
    B_0X0 = 0x0,
    #[doc = "Failure detected on LSE (32 kHz oscillator)"]
    B_0X1 = 0x01,
}
impl Lsecssd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsecssd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsecssd {
    #[inline(always)]
    fn from(val: u8) -> Lsecssd {
        Lsecssd::from_bits(val)
    }
}
impl From<Lsecssd> for u8 {
    #[inline(always)]
    fn from(val: Lsecssd) -> u8 {
        Lsecssd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lsecssf {
    #[doc = "No clock security interrupt caused by LSE clock failure"]
    B_0X0 = 0x0,
    #[doc = "Clock security interrupt caused by LSE clock failure"]
    B_0X1 = 0x01,
}
impl Lsecssf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsecssf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsecssf {
    #[inline(always)]
    fn from(val: u8) -> Lsecssf {
        Lsecssf::from_bits(val)
    }
}
impl From<Lsecssf> for u8 {
    #[inline(always)]
    fn from(val: Lsecssf) -> u8 {
        Lsecssf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lsecssie {
    #[doc = "Clock security interrupt caused by LSE clock failure disabled"]
    B_0X0 = 0x0,
    #[doc = "Clock security interrupt caused by LSE clock failure enabled"]
    B_0X1 = 0x01,
}
impl Lsecssie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsecssie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsecssie {
    #[inline(always)]
    fn from(val: u8) -> Lsecssie {
        Lsecssie::from_bits(val)
    }
}
impl From<Lsecssie> for u8 {
    #[inline(always)]
    fn from(val: Lsecssie) -> u8 {
        Lsecssie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lsecsson {
    #[doc = "CSS on LSE (32 kHz external oscillator) OFF"]
    B_0X0 = 0x0,
    #[doc = "CSS on LSE (32 kHz external oscillator) ON"]
    B_0X1 = 0x01,
}
impl Lsecsson {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsecsson {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsecsson {
    #[inline(always)]
    fn from(val: u8) -> Lsecsson {
        Lsecsson::from_bits(val)
    }
}
impl From<Lsecsson> for u8 {
    #[inline(always)]
    fn from(val: Lsecsson) -> u8 {
        Lsecsson::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lsedrv {
    #[doc = "Xtal mode lower driving capability"]
    B_0X0 = 0x0,
    #[doc = "Xtal mode medium low driving capability"]
    B_0X1 = 0x01,
    #[doc = "Xtal mode medium high driving capability"]
    B_0X2 = 0x02,
    #[doc = "Xtal mode higher driving capability"]
    B_0X3 = 0x03,
}
impl Lsedrv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsedrv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsedrv {
    #[inline(always)]
    fn from(val: u8) -> Lsedrv {
        Lsedrv::from_bits(val)
    }
}
impl From<Lsedrv> for u8 {
    #[inline(always)]
    fn from(val: Lsedrv) -> u8 {
        Lsedrv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lseon {
    #[doc = "LSE oscillator OFF"]
    B_0X0 = 0x0,
    #[doc = "LSE oscillator ON"]
    B_0X1 = 0x01,
}
impl Lseon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lseon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lseon {
    #[inline(always)]
    fn from(val: u8) -> Lseon {
        Lseon::from_bits(val)
    }
}
impl From<Lseon> for u8 {
    #[inline(always)]
    fn from(val: Lseon) -> u8 {
        Lseon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lserdy {
    #[doc = "LSE oscillator not ready"]
    B_0X0 = 0x0,
    #[doc = "LSE oscillator ready"]
    B_0X1 = 0x01,
}
impl Lserdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lserdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lserdy {
    #[inline(always)]
    fn from(val: u8) -> Lserdy {
        Lserdy::from_bits(val)
    }
}
impl From<Lserdy> for u8 {
    #[inline(always)]
    fn from(val: Lserdy) -> u8 {
        Lserdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lserdyc {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "LSERDYF cleared"]
    B_0X1 = 0x01,
}
impl Lserdyc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lserdyc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lserdyc {
    #[inline(always)]
    fn from(val: u8) -> Lserdyc {
        Lserdyc::from_bits(val)
    }
}
impl From<Lserdyc> for u8 {
    #[inline(always)]
    fn from(val: Lserdyc) -> u8 {
        Lserdyc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lserdyf {
    #[doc = "No clock ready interrupt caused by the LSE oscillator"]
    B_0X0 = 0x0,
    #[doc = "Clock ready interrupt caused by the LSE oscillator"]
    B_0X1 = 0x01,
}
impl Lserdyf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lserdyf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lserdyf {
    #[inline(always)]
    fn from(val: u8) -> Lserdyf {
        Lserdyf::from_bits(val)
    }
}
impl From<Lserdyf> for u8 {
    #[inline(always)]
    fn from(val: Lserdyf) -> u8 {
        Lserdyf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lserdyie {
    #[doc = "LSE ready interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "LSE ready interrupt enabled"]
    B_0X1 = 0x01,
}
impl Lserdyie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lserdyie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lserdyie {
    #[inline(always)]
    fn from(val: u8) -> Lserdyie {
        Lserdyie::from_bits(val)
    }
}
impl From<Lserdyie> for u8 {
    #[inline(always)]
    fn from(val: Lserdyie) -> u8 {
        Lserdyie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lsion {
    #[doc = "LSI oscillator OFF"]
    B_0X0 = 0x0,
    #[doc = "LSI oscillator ON"]
    B_0X1 = 0x01,
}
impl Lsion {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsion {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsion {
    #[inline(always)]
    fn from(val: u8) -> Lsion {
        Lsion::from_bits(val)
    }
}
impl From<Lsion> for u8 {
    #[inline(always)]
    fn from(val: Lsion) -> u8 {
        Lsion::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lsirdy {
    #[doc = "LSI oscillator not ready"]
    B_0X0 = 0x0,
    #[doc = "LSI oscillator ready"]
    B_0X1 = 0x01,
}
impl Lsirdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsirdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsirdy {
    #[inline(always)]
    fn from(val: u8) -> Lsirdy {
        Lsirdy::from_bits(val)
    }
}
impl From<Lsirdy> for u8 {
    #[inline(always)]
    fn from(val: Lsirdy) -> u8 {
        Lsirdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lsirdyc {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "LSIRDYF cleared"]
    B_0X1 = 0x01,
}
impl Lsirdyc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsirdyc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsirdyc {
    #[inline(always)]
    fn from(val: u8) -> Lsirdyc {
        Lsirdyc::from_bits(val)
    }
}
impl From<Lsirdyc> for u8 {
    #[inline(always)]
    fn from(val: Lsirdyc) -> u8 {
        Lsirdyc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lsirdyf {
    #[doc = "No clock ready interrupt caused by the LSI oscillator"]
    B_0X0 = 0x0,
    #[doc = "Clock ready interrupt caused by the LSI oscillator"]
    B_0X1 = 0x01,
}
impl Lsirdyf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsirdyf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsirdyf {
    #[inline(always)]
    fn from(val: u8) -> Lsirdyf {
        Lsirdyf::from_bits(val)
    }
}
impl From<Lsirdyf> for u8 {
    #[inline(always)]
    fn from(val: Lsirdyf) -> u8 {
        Lsirdyf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lsirdyie {
    #[doc = "LSI ready interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "LSI ready interrupt enabled"]
    B_0X1 = 0x01,
}
impl Lsirdyie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsirdyie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsirdyie {
    #[inline(always)]
    fn from(val: u8) -> Lsirdyie {
        Lsirdyie::from_bits(val)
    }
}
impl From<Lsirdyie> for u8 {
    #[inline(always)]
    fn from(val: Lsirdyie) -> u8 {
        Lsirdyie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mcopre {
    #[doc = "MCO is divided by 1"]
    B_0X0 = 0x0,
    #[doc = "MCO is divided by 2"]
    B_0X1 = 0x01,
    #[doc = "MCO is divided by 4"]
    B_0X2 = 0x02,
    #[doc = "MCO is divided by 8"]
    B_0X3 = 0x03,
    #[doc = "MCO is divided by 16"]
    B_0X4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Mcopre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mcopre {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mcopre {
    #[inline(always)]
    fn from(val: u8) -> Mcopre {
        Mcopre::from_bits(val)
    }
}
impl From<Mcopre> for u8 {
    #[inline(always)]
    fn from(val: Mcopre) -> u8 {
        Mcopre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mcosel {
    #[doc = "MCO output disabled, no clock on MCO"]
    B_0X0 = 0x0,
    #[doc = "SYSCLK system clock selected"]
    B_0X1 = 0x01,
    #[doc = "Reserved, must be kept at reset value"]
    B_0X2 = 0x02,
    #[doc = "HSI16 clock selected"]
    B_0X3 = 0x03,
    #[doc = "HSE clock selected"]
    B_0X4 = 0x04,
    #[doc = "Main PLL clock selected"]
    B_0X5 = 0x05,
    #[doc = "LSI clock selected"]
    B_0X6 = 0x06,
    #[doc = "LSE clock selected"]
    B_0X7 = 0x07,
    #[doc = "Internal HSI48 clock selected"]
    B_0X8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Mcosel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mcosel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mcosel {
    #[inline(always)]
    fn from(val: u8) -> Mcosel {
        Mcosel::from_bits(val)
    }
}
impl From<Mcosel> for u8 {
    #[inline(always)]
    fn from(val: Mcosel) -> u8 {
        Mcosel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Oblrstf {
    #[doc = "No reset from Option Byte loading occurred"]
    B_0X0 = 0x0,
    #[doc = "Reset from Option Byte loading occurred"]
    B_0X1 = 0x01,
}
impl Oblrstf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oblrstf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oblrstf {
    #[inline(always)]
    fn from(val: u8) -> Oblrstf {
        Oblrstf::from_bits(val)
    }
}
impl From<Oblrstf> for u8 {
    #[inline(always)]
    fn from(val: Oblrstf) -> u8 {
        Oblrstf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pinrstf {
    #[doc = "No reset from NRST pin occurred"]
    B_0X0 = 0x0,
    #[doc = "Reset from NRST pin occurred"]
    B_0X1 = 0x01,
}
impl Pinrstf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pinrstf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pinrstf {
    #[inline(always)]
    fn from(val: u8) -> Pinrstf {
        Pinrstf::from_bits(val)
    }
}
impl From<Pinrstf> for u8 {
    #[inline(always)]
    fn from(val: Pinrstf) -> u8 {
        Pinrstf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pllm {
    #[doc = "PLLM = 1"]
    B_0X0 = 0x0,
    #[doc = "PLLM = 2"]
    B_0X1 = 0x01,
    #[doc = "PLLM = 3"]
    B_0X2 = 0x02,
    #[doc = "PLLM = 4"]
    B_0X3 = 0x03,
    #[doc = "PLLM = 5"]
    B_0X4 = 0x04,
    #[doc = "PLLM = 6"]
    B_0X5 = 0x05,
    #[doc = "PLLM = 7"]
    B_0X6 = 0x06,
    #[doc = "PLLM = 8"]
    B_0X7 = 0x07,
    #[doc = "PLLSYSM = 9"]
    B_0X8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "PLLSYSM= 16"]
    B_0XF = 0x0f,
}
impl Pllm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pllm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pllm {
    #[inline(always)]
    fn from(val: u8) -> Pllm {
        Pllm::from_bits(val)
    }
}
impl From<Pllm> for u8 {
    #[inline(always)]
    fn from(val: Pllm) -> u8 {
        Pllm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Plln {
    #[doc = "PLLN = 0 wrong configuration"]
    B_0X0 = 0x0,
    #[doc = "PLLN = 1 wrong configuration"]
    B_0X1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "PLLN = 7 wrong configuration"]
    B_0X7 = 0x07,
    #[doc = "PLLN = 8"]
    B_0X8 = 0x08,
    #[doc = "PLLN = 9"]
    B_0X9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
    _RESERVED_40 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    #[doc = "PLLN = 127"]
    B_0X7F = 0x7f,
}
impl Plln {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Plln {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Plln {
    #[inline(always)]
    fn from(val: u8) -> Plln {
        Plln::from_bits(val)
    }
}
impl From<Plln> for u8 {
    #[inline(always)]
    fn from(val: Plln) -> u8 {
        Plln::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pllon {
    #[doc = "PLL OFF"]
    B_0X0 = 0x0,
    #[doc = "PLL ON"]
    B_0X1 = 0x01,
}
impl Pllon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pllon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pllon {
    #[inline(always)]
    fn from(val: u8) -> Pllon {
        Pllon::from_bits(val)
    }
}
impl From<Pllon> for u8 {
    #[inline(always)]
    fn from(val: Pllon) -> u8 {
        Pllon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pllp {
    #[doc = "PLLP = 7"]
    B_0X0 = 0x0,
    #[doc = "PLLP = 17"]
    B_0X1 = 0x01,
}
impl Pllp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pllp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pllp {
    #[inline(always)]
    fn from(val: u8) -> Pllp {
        Pllp::from_bits(val)
    }
}
impl From<Pllp> for u8 {
    #[inline(always)]
    fn from(val: Pllp) -> u8 {
        Pllp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pllpdiv(pub u8);
impl Pllpdiv {
    #[doc = "PLL P clock is controlled by the bit PLLP"]
    pub const B_0X0: Self = Self(0x0);
    #[doc = "Reserved."]
    pub const B_0X1: Self = Self(0x01);
    #[doc = "PLL P clock = VCO / 2"]
    pub const B_0X2: Self = Self(0x02);
    #[doc = "PLL P clock = VCO / 31"]
    pub const B_0X1F: Self = Self(0x1f);
}
impl Pllpdiv {
    pub const fn from_bits(val: u8) -> Pllpdiv {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Pllpdiv {
    #[inline(always)]
    fn from(val: u8) -> Pllpdiv {
        Pllpdiv::from_bits(val)
    }
}
impl From<Pllpdiv> for u8 {
    #[inline(always)]
    fn from(val: Pllpdiv) -> u8 {
        Pllpdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pllpen {
    #[doc = "PLL P clock output disable"]
    B_0X0 = 0x0,
    #[doc = "PLL P clock output enable"]
    B_0X1 = 0x01,
}
impl Pllpen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pllpen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pllpen {
    #[inline(always)]
    fn from(val: u8) -> Pllpen {
        Pllpen::from_bits(val)
    }
}
impl From<Pllpen> for u8 {
    #[inline(always)]
    fn from(val: Pllpen) -> u8 {
        Pllpen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pllq {
    #[doc = "PLLQ = 2"]
    B_0X0 = 0x0,
    #[doc = "PLLQ = 4"]
    B_0X1 = 0x01,
    #[doc = "PLLQ = 6"]
    B_0X2 = 0x02,
    #[doc = "PLLQ = 8"]
    B_0X3 = 0x03,
}
impl Pllq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pllq {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pllq {
    #[inline(always)]
    fn from(val: u8) -> Pllq {
        Pllq::from_bits(val)
    }
}
impl From<Pllq> for u8 {
    #[inline(always)]
    fn from(val: Pllq) -> u8 {
        Pllq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pllqen {
    #[doc = "PLL Q clock output disable"]
    B_0X0 = 0x0,
    #[doc = "PLL Q clock output enable"]
    B_0X1 = 0x01,
}
impl Pllqen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pllqen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pllqen {
    #[inline(always)]
    fn from(val: u8) -> Pllqen {
        Pllqen::from_bits(val)
    }
}
impl From<Pllqen> for u8 {
    #[inline(always)]
    fn from(val: Pllqen) -> u8 {
        Pllqen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pllr {
    #[doc = "PLLR = 2"]
    B_0X0 = 0x0,
    #[doc = "PLLR = 4"]
    B_0X1 = 0x01,
    #[doc = "PLLR = 6"]
    B_0X2 = 0x02,
    #[doc = "PLLR = 8"]
    B_0X3 = 0x03,
}
impl Pllr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pllr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pllr {
    #[inline(always)]
    fn from(val: u8) -> Pllr {
        Pllr::from_bits(val)
    }
}
impl From<Pllr> for u8 {
    #[inline(always)]
    fn from(val: Pllr) -> u8 {
        Pllr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pllrdy {
    #[doc = "PLL unlocked"]
    B_0X0 = 0x0,
    #[doc = "PLL locked"]
    B_0X1 = 0x01,
}
impl Pllrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pllrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pllrdy {
    #[inline(always)]
    fn from(val: u8) -> Pllrdy {
        Pllrdy::from_bits(val)
    }
}
impl From<Pllrdy> for u8 {
    #[inline(always)]
    fn from(val: Pllrdy) -> u8 {
        Pllrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pllrdyc {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Clear PLLRDYF flag"]
    B_0X1 = 0x01,
}
impl Pllrdyc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pllrdyc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pllrdyc {
    #[inline(always)]
    fn from(val: u8) -> Pllrdyc {
        Pllrdyc::from_bits(val)
    }
}
impl From<Pllrdyc> for u8 {
    #[inline(always)]
    fn from(val: Pllrdyc) -> u8 {
        Pllrdyc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pllrdyf {
    #[doc = "No clock ready interrupt caused by PLL lock"]
    B_0X0 = 0x0,
    #[doc = "Clock ready interrupt caused by PLL lock"]
    B_0X1 = 0x01,
}
impl Pllrdyf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pllrdyf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pllrdyf {
    #[inline(always)]
    fn from(val: u8) -> Pllrdyf {
        Pllrdyf::from_bits(val)
    }
}
impl From<Pllrdyf> for u8 {
    #[inline(always)]
    fn from(val: Pllrdyf) -> u8 {
        Pllrdyf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pllrdyie {
    #[doc = "PLL lock interrupt disabled"]
    B_0X0 = 0x0,
    #[doc = "PLL lock interrupt enabled"]
    B_0X1 = 0x01,
}
impl Pllrdyie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pllrdyie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pllrdyie {
    #[inline(always)]
    fn from(val: u8) -> Pllrdyie {
        Pllrdyie::from_bits(val)
    }
}
impl From<Pllrdyie> for u8 {
    #[inline(always)]
    fn from(val: Pllrdyie) -> u8 {
        Pllrdyie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pllren {
    #[doc = "PLL R clock output disable"]
    B_0X0 = 0x0,
    #[doc = "PLL R clock output enable"]
    B_0X1 = 0x01,
}
impl Pllren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pllren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pllren {
    #[inline(always)]
    fn from(val: u8) -> Pllren {
        Pllren::from_bits(val)
    }
}
impl From<Pllren> for u8 {
    #[inline(always)]
    fn from(val: Pllren) -> u8 {
        Pllren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pllsrc {
    #[doc = "No clock sent to PLL"]
    B_0X0 = 0x0,
    #[doc = "No clock sent to PLL"]
    B_0X1 = 0x01,
    #[doc = "HSI16 clock selected as PLL clock entry"]
    B_0X2 = 0x02,
    #[doc = "HSE clock selected as PLL clock entry"]
    B_0X3 = 0x03,
}
impl Pllsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pllsrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pllsrc {
    #[inline(always)]
    fn from(val: u8) -> Pllsrc {
        Pllsrc::from_bits(val)
    }
}
impl From<Pllsrc> for u8 {
    #[inline(always)]
    fn from(val: Pllsrc) -> u8 {
        Pllsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ppre1 {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "HCLK divided by 2"]
    B_0X4 = 0x04,
    #[doc = "HCLK divided by 4"]
    B_0X5 = 0x05,
    #[doc = "HCLK divided by 8"]
    B_0X6 = 0x06,
    #[doc = "HCLK divided by 16"]
    B_0X7 = 0x07,
}
impl Ppre1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ppre1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ppre1 {
    #[inline(always)]
    fn from(val: u8) -> Ppre1 {
        Ppre1::from_bits(val)
    }
}
impl From<Ppre1> for u8 {
    #[inline(always)]
    fn from(val: Ppre1) -> u8 {
        Ppre1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ppre2 {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "HCLK divided by 2"]
    B_0X4 = 0x04,
    #[doc = "HCLK divided by 4"]
    B_0X5 = 0x05,
    #[doc = "HCLK divided by 8"]
    B_0X6 = 0x06,
    #[doc = "HCLK divided by 16"]
    B_0X7 = 0x07,
}
impl Ppre2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ppre2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ppre2 {
    #[inline(always)]
    fn from(val: u8) -> Ppre2 {
        Ppre2::from_bits(val)
    }
}
impl From<Ppre2> for u8 {
    #[inline(always)]
    fn from(val: Ppre2) -> u8 {
        Ppre2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pwren {
    #[doc = "Power interface clock disabled"]
    B_0X0 = 0x0,
    #[doc = "Power interface clock enabled"]
    B_0X1 = 0x01,
}
impl Pwren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwren {
    #[inline(always)]
    fn from(val: u8) -> Pwren {
        Pwren::from_bits(val)
    }
}
impl From<Pwren> for u8 {
    #[inline(always)]
    fn from(val: Pwren) -> u8 {
        Pwren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pwrrst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset PWR"]
    B_0X1 = 0x01,
}
impl Pwrrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwrrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwrrst {
    #[inline(always)]
    fn from(val: u8) -> Pwrrst {
        Pwrrst::from_bits(val)
    }
}
impl From<Pwrrst> for u8 {
    #[inline(always)]
    fn from(val: Pwrrst) -> u8 {
        Pwrrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pwrsmen {
    #[doc = "Power interface clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "Power interface clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Pwrsmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwrsmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwrsmen {
    #[inline(always)]
    fn from(val: u8) -> Pwrsmen {
        Pwrsmen::from_bits(val)
    }
}
impl From<Pwrsmen> for u8 {
    #[inline(always)]
    fn from(val: Pwrsmen) -> u8 {
        Pwrsmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Qspien {
    #[doc = "QUADSPI clock disable"]
    B_0X0 = 0x0,
    #[doc = "QUADSPI clock enable"]
    B_0X1 = 0x01,
}
impl Qspien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qspien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qspien {
    #[inline(always)]
    fn from(val: u8) -> Qspien {
        Qspien::from_bits(val)
    }
}
impl From<Qspien> for u8 {
    #[inline(always)]
    fn from(val: Qspien) -> u8 {
        Qspien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Qspirst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset QUADSPI"]
    B_0X1 = 0x01,
}
impl Qspirst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qspirst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qspirst {
    #[inline(always)]
    fn from(val: u8) -> Qspirst {
        Qspirst::from_bits(val)
    }
}
impl From<Qspirst> for u8 {
    #[inline(always)]
    fn from(val: Qspirst) -> u8 {
        Qspirst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Qspisel {
    #[doc = "system clock selected as QUADSPI kernel clock"]
    B_0X0 = 0x0,
    #[doc = "HSI16 clock selected as QUADSPI kernel clock"]
    B_0X1 = 0x01,
    #[doc = "PLL Q clock selected as QUADSPI kernel clock"]
    B_0X2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Qspisel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qspisel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qspisel {
    #[inline(always)]
    fn from(val: u8) -> Qspisel {
        Qspisel::from_bits(val)
    }
}
impl From<Qspisel> for u8 {
    #[inline(always)]
    fn from(val: Qspisel) -> u8 {
        Qspisel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Qspismen {
    #[doc = "QUADSPI clock disabled by the clock gating during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "QUADSPI clock enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Qspismen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qspismen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qspismen {
    #[inline(always)]
    fn from(val: u8) -> Qspismen {
        Qspismen::from_bits(val)
    }
}
impl From<Qspismen> for u8 {
    #[inline(always)]
    fn from(val: Qspismen) -> u8 {
        Qspismen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RccAhb2enrRngen {
    #[doc = "RNG disabled"]
    B_0X0 = 0x0,
    #[doc = "RNG enabled"]
    B_0X1 = 0x01,
}
impl RccAhb2enrRngen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RccAhb2enrRngen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RccAhb2enrRngen {
    #[inline(always)]
    fn from(val: u8) -> RccAhb2enrRngen {
        RccAhb2enrRngen::from_bits(val)
    }
}
impl From<RccAhb2enrRngen> for u8 {
    #[inline(always)]
    fn from(val: RccAhb2enrRngen) -> u8 {
        RccAhb2enrRngen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RccAhb2smenrRngen {
    #[doc = "RNG disabled"]
    B_0X0 = 0x0,
    #[doc = "RNG enabled"]
    B_0X1 = 0x01,
}
impl RccAhb2smenrRngen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RccAhb2smenrRngen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RccAhb2smenrRngen {
    #[inline(always)]
    fn from(val: u8) -> RccAhb2smenrRngen {
        RccAhb2smenrRngen::from_bits(val)
    }
}
impl From<RccAhb2smenrRngen> for u8 {
    #[inline(always)]
    fn from(val: RccAhb2smenrRngen) -> u8 {
        RccAhb2smenrRngen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rmvf {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Clear the reset flags"]
    B_0X1 = 0x01,
}
impl Rmvf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rmvf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rmvf {
    #[inline(always)]
    fn from(val: u8) -> Rmvf {
        Rmvf::from_bits(val)
    }
}
impl From<Rmvf> for u8 {
    #[inline(always)]
    fn from(val: Rmvf) -> u8 {
        Rmvf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rngrst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset RNG"]
    B_0X1 = 0x01,
}
impl Rngrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rngrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rngrst {
    #[inline(always)]
    fn from(val: u8) -> Rngrst {
        Rngrst::from_bits(val)
    }
}
impl From<Rngrst> for u8 {
    #[inline(always)]
    fn from(val: Rngrst) -> u8 {
        Rngrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rtcapben {
    #[doc = "RTC APB clock disabled"]
    B_0X0 = 0x0,
    #[doc = "RTC APB clock enabled"]
    B_0X1 = 0x01,
}
impl Rtcapben {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtcapben {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtcapben {
    #[inline(always)]
    fn from(val: u8) -> Rtcapben {
        Rtcapben::from_bits(val)
    }
}
impl From<Rtcapben> for u8 {
    #[inline(always)]
    fn from(val: Rtcapben) -> u8 {
        Rtcapben::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rtcapbsmen {
    #[doc = "RTC APB clock disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "RTC APB clock enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Rtcapbsmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtcapbsmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtcapbsmen {
    #[inline(always)]
    fn from(val: u8) -> Rtcapbsmen {
        Rtcapbsmen::from_bits(val)
    }
}
impl From<Rtcapbsmen> for u8 {
    #[inline(always)]
    fn from(val: Rtcapbsmen) -> u8 {
        Rtcapbsmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rtcen {
    #[doc = "RTC clock disabled"]
    B_0X0 = 0x0,
    #[doc = "RTC clock enabled"]
    B_0X1 = 0x01,
}
impl Rtcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtcen {
    #[inline(always)]
    fn from(val: u8) -> Rtcen {
        Rtcen::from_bits(val)
    }
}
impl From<Rtcen> for u8 {
    #[inline(always)]
    fn from(val: Rtcen) -> u8 {
        Rtcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rtcsel {
    #[doc = "No clock"]
    B_0X0 = 0x0,
    #[doc = "LSE oscillator clock used as RTC clock"]
    B_0X1 = 0x01,
    #[doc = "LSI oscillator clock used as RTC clock"]
    B_0X2 = 0x02,
    #[doc = "HSE oscillator clock divided by 32 used as RTC clock"]
    B_0X3 = 0x03,
}
impl Rtcsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtcsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtcsel {
    #[inline(always)]
    fn from(val: u8) -> Rtcsel {
        Rtcsel::from_bits(val)
    }
}
impl From<Rtcsel> for u8 {
    #[inline(always)]
    fn from(val: Rtcsel) -> u8 {
        Rtcsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sai1en {
    #[doc = "SAI1 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "SAI1 clock enabled"]
    B_0X1 = 0x01,
}
impl Sai1en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1en {
    #[inline(always)]
    fn from(val: u8) -> Sai1en {
        Sai1en::from_bits(val)
    }
}
impl From<Sai1en> for u8 {
    #[inline(always)]
    fn from(val: Sai1en) -> u8 {
        Sai1en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sai1rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset SAI1"]
    B_0X1 = 0x01,
}
impl Sai1rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1rst {
    #[inline(always)]
    fn from(val: u8) -> Sai1rst {
        Sai1rst::from_bits(val)
    }
}
impl From<Sai1rst> for u8 {
    #[inline(always)]
    fn from(val: Sai1rst) -> u8 {
        Sai1rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sai1sel {
    #[doc = "System clock selected as SAI clock"]
    B_0X0 = 0x0,
    #[doc = "PLL Q clock selected as SAI clock"]
    B_0X1 = 0x01,
    #[doc = "Clock provided on I2S_CKIN pin selected as SAI clock"]
    B_0X2 = 0x02,
    #[doc = "HSI16 clock selected as SAI clock"]
    B_0X3 = 0x03,
}
impl Sai1sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1sel {
    #[inline(always)]
    fn from(val: u8) -> Sai1sel {
        Sai1sel::from_bits(val)
    }
}
impl From<Sai1sel> for u8 {
    #[inline(always)]
    fn from(val: Sai1sel) -> u8 {
        Sai1sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sai1smen {
    #[doc = "SAI1 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "SAI1 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Sai1smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1smen {
    #[inline(always)]
    fn from(val: u8) -> Sai1smen {
        Sai1smen::from_bits(val)
    }
}
impl From<Sai1smen> for u8 {
    #[inline(always)]
    fn from(val: Sai1smen) -> u8 {
        Sai1smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sftrstf {
    #[doc = "No software reset occurred"]
    B_0X0 = 0x0,
    #[doc = "Software reset occurred"]
    B_0X1 = 0x01,
}
impl Sftrstf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sftrstf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sftrstf {
    #[inline(always)]
    fn from(val: u8) -> Sftrstf {
        Sftrstf::from_bits(val)
    }
}
impl From<Sftrstf> for u8 {
    #[inline(always)]
    fn from(val: Sftrstf) -> u8 {
        Sftrstf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Spi1en {
    #[doc = "SPI1 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "SPI1 clock enabled"]
    B_0X1 = 0x01,
}
impl Spi1en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spi1en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spi1en {
    #[inline(always)]
    fn from(val: u8) -> Spi1en {
        Spi1en::from_bits(val)
    }
}
impl From<Spi1en> for u8 {
    #[inline(always)]
    fn from(val: Spi1en) -> u8 {
        Spi1en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Spi1rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset SPI1"]
    B_0X1 = 0x01,
}
impl Spi1rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spi1rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spi1rst {
    #[inline(always)]
    fn from(val: u8) -> Spi1rst {
        Spi1rst::from_bits(val)
    }
}
impl From<Spi1rst> for u8 {
    #[inline(always)]
    fn from(val: Spi1rst) -> u8 {
        Spi1rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Spi1smen {
    #[doc = "SPI1 clocks disabled by the clock gating during<sup>(1)</sup> Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "SPI1 clocks enabled by the clock gating during<sup>(1)</sup> Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Spi1smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spi1smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spi1smen {
    #[inline(always)]
    fn from(val: u8) -> Spi1smen {
        Spi1smen::from_bits(val)
    }
}
impl From<Spi1smen> for u8 {
    #[inline(always)]
    fn from(val: Spi1smen) -> u8 {
        Spi1smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Spi2en {
    #[doc = "SPI2 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "SPI2 clock enabled"]
    B_0X1 = 0x01,
}
impl Spi2en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spi2en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spi2en {
    #[inline(always)]
    fn from(val: u8) -> Spi2en {
        Spi2en::from_bits(val)
    }
}
impl From<Spi2en> for u8 {
    #[inline(always)]
    fn from(val: Spi2en) -> u8 {
        Spi2en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Spi2rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset SPI2"]
    B_0X1 = 0x01,
}
impl Spi2rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spi2rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spi2rst {
    #[inline(always)]
    fn from(val: u8) -> Spi2rst {
        Spi2rst::from_bits(val)
    }
}
impl From<Spi2rst> for u8 {
    #[inline(always)]
    fn from(val: Spi2rst) -> u8 {
        Spi2rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Spi2smen {
    #[doc = "SPI2 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "SPI2 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Spi2smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spi2smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spi2smen {
    #[inline(always)]
    fn from(val: u8) -> Spi2smen {
        Spi2smen::from_bits(val)
    }
}
impl From<Spi2smen> for u8 {
    #[inline(always)]
    fn from(val: Spi2smen) -> u8 {
        Spi2smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Spi3en {
    #[doc = "SPI3 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "SPI3 clock enabled"]
    B_0X1 = 0x01,
}
impl Spi3en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spi3en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spi3en {
    #[inline(always)]
    fn from(val: u8) -> Spi3en {
        Spi3en::from_bits(val)
    }
}
impl From<Spi3en> for u8 {
    #[inline(always)]
    fn from(val: Spi3en) -> u8 {
        Spi3en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Spi3rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset SPI3"]
    B_0X1 = 0x01,
}
impl Spi3rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spi3rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spi3rst {
    #[inline(always)]
    fn from(val: u8) -> Spi3rst {
        Spi3rst::from_bits(val)
    }
}
impl From<Spi3rst> for u8 {
    #[inline(always)]
    fn from(val: Spi3rst) -> u8 {
        Spi3rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Spi3smen {
    #[doc = "SPI3 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "SPI3 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Spi3smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spi3smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spi3smen {
    #[inline(always)]
    fn from(val: u8) -> Spi3smen {
        Spi3smen::from_bits(val)
    }
}
impl From<Spi3smen> for u8 {
    #[inline(always)]
    fn from(val: Spi3smen) -> u8 {
        Spi3smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Spi4en {
    #[doc = "SPI4 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "SPI4 clock enabled"]
    B_0X1 = 0x01,
}
impl Spi4en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spi4en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spi4en {
    #[inline(always)]
    fn from(val: u8) -> Spi4en {
        Spi4en::from_bits(val)
    }
}
impl From<Spi4en> for u8 {
    #[inline(always)]
    fn from(val: Spi4en) -> u8 {
        Spi4en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Spi4rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset SPI4"]
    B_0X1 = 0x01,
}
impl Spi4rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spi4rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spi4rst {
    #[inline(always)]
    fn from(val: u8) -> Spi4rst {
        Spi4rst::from_bits(val)
    }
}
impl From<Spi4rst> for u8 {
    #[inline(always)]
    fn from(val: Spi4rst) -> u8 {
        Spi4rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Spi4smen {
    #[doc = "SPI4 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "SPI4 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop mode"]
    B_0X1 = 0x01,
}
impl Spi4smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spi4smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spi4smen {
    #[inline(always)]
    fn from(val: u8) -> Spi4smen {
        Spi4smen::from_bits(val)
    }
}
impl From<Spi4smen> for u8 {
    #[inline(always)]
    fn from(val: Spi4smen) -> u8 {
        Spi4smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sram1smen {
    #[doc = "SRAM1 interface clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "SRAM1 interface clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Sram1smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sram1smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sram1smen {
    #[inline(always)]
    fn from(val: u8) -> Sram1smen {
        Sram1smen::from_bits(val)
    }
}
impl From<Sram1smen> for u8 {
    #[inline(always)]
    fn from(val: Sram1smen) -> u8 {
        Sram1smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sram2smen {
    #[doc = "SRAM2 interface clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "SRAM2 interface clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Sram2smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sram2smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sram2smen {
    #[inline(always)]
    fn from(val: u8) -> Sram2smen {
        Sram2smen::from_bits(val)
    }
}
impl From<Sram2smen> for u8 {
    #[inline(always)]
    fn from(val: Sram2smen) -> u8 {
        Sram2smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sw {
    #[doc = "Reserved, must be kept at reset value"]
    B_0X0 = 0x0,
    #[doc = "HSI16 selected as system clock"]
    B_0X1 = 0x01,
    #[doc = "HSE selected as system clock"]
    B_0X2 = 0x02,
    #[doc = "PLL selected as system clock"]
    B_0X3 = 0x03,
}
impl Sw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sw {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sw {
    #[inline(always)]
    fn from(val: u8) -> Sw {
        Sw::from_bits(val)
    }
}
impl From<Sw> for u8 {
    #[inline(always)]
    fn from(val: Sw) -> u8 {
        Sw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sws {
    #[doc = "Reserved, must be kept at reset value"]
    B_0X0 = 0x0,
    #[doc = "HSI16 oscillator used as system clock"]
    B_0X1 = 0x01,
    #[doc = "HSE used as system clock"]
    B_0X2 = 0x02,
    #[doc = "PLL used as system clock"]
    B_0X3 = 0x03,
}
impl Sws {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sws {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sws {
    #[inline(always)]
    fn from(val: u8) -> Sws {
        Sws::from_bits(val)
    }
}
impl From<Sws> for u8 {
    #[inline(always)]
    fn from(val: Sws) -> u8 {
        Sws::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Syscfgen {
    #[doc = "SYSCFG + COMP + VREFBUF + OPAMP clock disabled"]
    B_0X0 = 0x0,
    #[doc = "SYSCFG + COMP + VREFBUF + OPAMP clock enabled"]
    B_0X1 = 0x01,
}
impl Syscfgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Syscfgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Syscfgen {
    #[inline(always)]
    fn from(val: u8) -> Syscfgen {
        Syscfgen::from_bits(val)
    }
}
impl From<Syscfgen> for u8 {
    #[inline(always)]
    fn from(val: Syscfgen) -> u8 {
        Syscfgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Syscfgrst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset SYSCFG + COMP + OPAMP + VREFBUF"]
    B_0X1 = 0x01,
}
impl Syscfgrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Syscfgrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Syscfgrst {
    #[inline(always)]
    fn from(val: u8) -> Syscfgrst {
        Syscfgrst::from_bits(val)
    }
}
impl From<Syscfgrst> for u8 {
    #[inline(always)]
    fn from(val: Syscfgrst) -> u8 {
        Syscfgrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Syscfgsmen {
    #[doc = "SYSCFG + COMP + VREFBUF + OPAMP clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "SYSCFG + COMP + VREFBUF + OPAMP clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Syscfgsmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Syscfgsmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Syscfgsmen {
    #[inline(always)]
    fn from(val: u8) -> Syscfgsmen {
        Syscfgsmen::from_bits(val)
    }
}
impl From<Syscfgsmen> for u8 {
    #[inline(always)]
    fn from(val: Syscfgsmen) -> u8 {
        Syscfgsmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim15en {
    #[doc = "TIM15 timer clock disabled"]
    B_0X0 = 0x0,
    #[doc = "TIM15 timer clock enabled"]
    B_0X1 = 0x01,
}
impl Tim15en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim15en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim15en {
    #[inline(always)]
    fn from(val: u8) -> Tim15en {
        Tim15en::from_bits(val)
    }
}
impl From<Tim15en> for u8 {
    #[inline(always)]
    fn from(val: Tim15en) -> u8 {
        Tim15en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim15rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset TIM15 timer"]
    B_0X1 = 0x01,
}
impl Tim15rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim15rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim15rst {
    #[inline(always)]
    fn from(val: u8) -> Tim15rst {
        Tim15rst::from_bits(val)
    }
}
impl From<Tim15rst> for u8 {
    #[inline(always)]
    fn from(val: Tim15rst) -> u8 {
        Tim15rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim15smen {
    #[doc = "TIM15 timer clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "TIM15 timer clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop mode"]
    B_0X1 = 0x01,
}
impl Tim15smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim15smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim15smen {
    #[inline(always)]
    fn from(val: u8) -> Tim15smen {
        Tim15smen::from_bits(val)
    }
}
impl From<Tim15smen> for u8 {
    #[inline(always)]
    fn from(val: Tim15smen) -> u8 {
        Tim15smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim16en {
    #[doc = "TIM16 timer clock disabled"]
    B_0X0 = 0x0,
    #[doc = "TIM16 timer clock enabled"]
    B_0X1 = 0x01,
}
impl Tim16en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim16en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim16en {
    #[inline(always)]
    fn from(val: u8) -> Tim16en {
        Tim16en::from_bits(val)
    }
}
impl From<Tim16en> for u8 {
    #[inline(always)]
    fn from(val: Tim16en) -> u8 {
        Tim16en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim16rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset TIM16 timer"]
    B_0X1 = 0x01,
}
impl Tim16rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim16rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim16rst {
    #[inline(always)]
    fn from(val: u8) -> Tim16rst {
        Tim16rst::from_bits(val)
    }
}
impl From<Tim16rst> for u8 {
    #[inline(always)]
    fn from(val: Tim16rst) -> u8 {
        Tim16rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim16smen {
    #[doc = "TIM16 timer clocks disabled by the clock gating during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "TIM16 timer clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Tim16smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim16smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim16smen {
    #[inline(always)]
    fn from(val: u8) -> Tim16smen {
        Tim16smen::from_bits(val)
    }
}
impl From<Tim16smen> for u8 {
    #[inline(always)]
    fn from(val: Tim16smen) -> u8 {
        Tim16smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim17en {
    #[doc = "TIM17 timer clock disabled"]
    B_0X0 = 0x0,
    #[doc = "TIM17 timer clock enabled"]
    B_0X1 = 0x01,
}
impl Tim17en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim17en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim17en {
    #[inline(always)]
    fn from(val: u8) -> Tim17en {
        Tim17en::from_bits(val)
    }
}
impl From<Tim17en> for u8 {
    #[inline(always)]
    fn from(val: Tim17en) -> u8 {
        Tim17en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim17rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset TIM17 timer"]
    B_0X1 = 0x01,
}
impl Tim17rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim17rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim17rst {
    #[inline(always)]
    fn from(val: u8) -> Tim17rst {
        Tim17rst::from_bits(val)
    }
}
impl From<Tim17rst> for u8 {
    #[inline(always)]
    fn from(val: Tim17rst) -> u8 {
        Tim17rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim17smen {
    #[doc = "TIM17 timer clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "TIM17 timer clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Tim17smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim17smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim17smen {
    #[inline(always)]
    fn from(val: u8) -> Tim17smen {
        Tim17smen::from_bits(val)
    }
}
impl From<Tim17smen> for u8 {
    #[inline(always)]
    fn from(val: Tim17smen) -> u8 {
        Tim17smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim1en {
    #[doc = "TIM1 timer clock disabled"]
    B_0X0 = 0x0,
    #[doc = "TIM1P timer clock enabled"]
    B_0X1 = 0x01,
}
impl Tim1en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim1en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim1en {
    #[inline(always)]
    fn from(val: u8) -> Tim1en {
        Tim1en::from_bits(val)
    }
}
impl From<Tim1en> for u8 {
    #[inline(always)]
    fn from(val: Tim1en) -> u8 {
        Tim1en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim1rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset TIM1 timer"]
    B_0X1 = 0x01,
}
impl Tim1rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim1rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim1rst {
    #[inline(always)]
    fn from(val: u8) -> Tim1rst {
        Tim1rst::from_bits(val)
    }
}
impl From<Tim1rst> for u8 {
    #[inline(always)]
    fn from(val: Tim1rst) -> u8 {
        Tim1rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim1smen {
    #[doc = "TIM1 timer clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "TIM1P timer clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Tim1smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim1smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim1smen {
    #[inline(always)]
    fn from(val: u8) -> Tim1smen {
        Tim1smen::from_bits(val)
    }
}
impl From<Tim1smen> for u8 {
    #[inline(always)]
    fn from(val: Tim1smen) -> u8 {
        Tim1smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim20en {
    #[doc = "TIM20 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "TIM20 clock enabled"]
    B_0X1 = 0x01,
}
impl Tim20en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim20en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim20en {
    #[inline(always)]
    fn from(val: u8) -> Tim20en {
        Tim20en::from_bits(val)
    }
}
impl From<Tim20en> for u8 {
    #[inline(always)]
    fn from(val: Tim20en) -> u8 {
        Tim20en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim20rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset TIM20"]
    B_0X1 = 0x01,
}
impl Tim20rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim20rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim20rst {
    #[inline(always)]
    fn from(val: u8) -> Tim20rst {
        Tim20rst::from_bits(val)
    }
}
impl From<Tim20rst> for u8 {
    #[inline(always)]
    fn from(val: Tim20rst) -> u8 {
        Tim20rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim20smen {
    #[doc = "TIM20 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "TIM20 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Tim20smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim20smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim20smen {
    #[inline(always)]
    fn from(val: u8) -> Tim20smen {
        Tim20smen::from_bits(val)
    }
}
impl From<Tim20smen> for u8 {
    #[inline(always)]
    fn from(val: Tim20smen) -> u8 {
        Tim20smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim2en {
    #[doc = "TIM2 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "TIM2 clock enabled"]
    B_0X1 = 0x01,
}
impl Tim2en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim2en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim2en {
    #[inline(always)]
    fn from(val: u8) -> Tim2en {
        Tim2en::from_bits(val)
    }
}
impl From<Tim2en> for u8 {
    #[inline(always)]
    fn from(val: Tim2en) -> u8 {
        Tim2en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim2rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset TIM2"]
    B_0X1 = 0x01,
}
impl Tim2rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim2rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim2rst {
    #[inline(always)]
    fn from(val: u8) -> Tim2rst {
        Tim2rst::from_bits(val)
    }
}
impl From<Tim2rst> for u8 {
    #[inline(always)]
    fn from(val: Tim2rst) -> u8 {
        Tim2rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim2smen {
    #[doc = "TIM2 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "TIM2 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Tim2smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim2smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim2smen {
    #[inline(always)]
    fn from(val: u8) -> Tim2smen {
        Tim2smen::from_bits(val)
    }
}
impl From<Tim2smen> for u8 {
    #[inline(always)]
    fn from(val: Tim2smen) -> u8 {
        Tim2smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim3en {
    #[doc = "TIM3 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "TIM3 clock enabled"]
    B_0X1 = 0x01,
}
impl Tim3en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim3en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim3en {
    #[inline(always)]
    fn from(val: u8) -> Tim3en {
        Tim3en::from_bits(val)
    }
}
impl From<Tim3en> for u8 {
    #[inline(always)]
    fn from(val: Tim3en) -> u8 {
        Tim3en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim3rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset TIM3"]
    B_0X1 = 0x01,
}
impl Tim3rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim3rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim3rst {
    #[inline(always)]
    fn from(val: u8) -> Tim3rst {
        Tim3rst::from_bits(val)
    }
}
impl From<Tim3rst> for u8 {
    #[inline(always)]
    fn from(val: Tim3rst) -> u8 {
        Tim3rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim3smen {
    #[doc = "TIM3 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "TIM3 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Tim3smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim3smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim3smen {
    #[inline(always)]
    fn from(val: u8) -> Tim3smen {
        Tim3smen::from_bits(val)
    }
}
impl From<Tim3smen> for u8 {
    #[inline(always)]
    fn from(val: Tim3smen) -> u8 {
        Tim3smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim4en {
    #[doc = "TIM4 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "TIM4 clock enabled"]
    B_0X1 = 0x01,
}
impl Tim4en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim4en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim4en {
    #[inline(always)]
    fn from(val: u8) -> Tim4en {
        Tim4en::from_bits(val)
    }
}
impl From<Tim4en> for u8 {
    #[inline(always)]
    fn from(val: Tim4en) -> u8 {
        Tim4en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim4rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset TIM3"]
    B_0X1 = 0x01,
}
impl Tim4rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim4rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim4rst {
    #[inline(always)]
    fn from(val: u8) -> Tim4rst {
        Tim4rst::from_bits(val)
    }
}
impl From<Tim4rst> for u8 {
    #[inline(always)]
    fn from(val: Tim4rst) -> u8 {
        Tim4rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim4smen {
    #[doc = "TIM4 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "TIM4 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Tim4smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim4smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim4smen {
    #[inline(always)]
    fn from(val: u8) -> Tim4smen {
        Tim4smen::from_bits(val)
    }
}
impl From<Tim4smen> for u8 {
    #[inline(always)]
    fn from(val: Tim4smen) -> u8 {
        Tim4smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim5en {
    #[doc = "TIM5 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "TIM5 clock enabled"]
    B_0X1 = 0x01,
}
impl Tim5en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim5en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim5en {
    #[inline(always)]
    fn from(val: u8) -> Tim5en {
        Tim5en::from_bits(val)
    }
}
impl From<Tim5en> for u8 {
    #[inline(always)]
    fn from(val: Tim5en) -> u8 {
        Tim5en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim5rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset TIM5"]
    B_0X1 = 0x01,
}
impl Tim5rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim5rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim5rst {
    #[inline(always)]
    fn from(val: u8) -> Tim5rst {
        Tim5rst::from_bits(val)
    }
}
impl From<Tim5rst> for u8 {
    #[inline(always)]
    fn from(val: Tim5rst) -> u8 {
        Tim5rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim5smen {
    #[doc = "TIM5 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "TIM5 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Tim5smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim5smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim5smen {
    #[inline(always)]
    fn from(val: u8) -> Tim5smen {
        Tim5smen::from_bits(val)
    }
}
impl From<Tim5smen> for u8 {
    #[inline(always)]
    fn from(val: Tim5smen) -> u8 {
        Tim5smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim6en {
    #[doc = "TIM6 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "TIM6 clock enabled"]
    B_0X1 = 0x01,
}
impl Tim6en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim6en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim6en {
    #[inline(always)]
    fn from(val: u8) -> Tim6en {
        Tim6en::from_bits(val)
    }
}
impl From<Tim6en> for u8 {
    #[inline(always)]
    fn from(val: Tim6en) -> u8 {
        Tim6en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim6rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset TIM7"]
    B_0X1 = 0x01,
}
impl Tim6rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim6rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim6rst {
    #[inline(always)]
    fn from(val: u8) -> Tim6rst {
        Tim6rst::from_bits(val)
    }
}
impl From<Tim6rst> for u8 {
    #[inline(always)]
    fn from(val: Tim6rst) -> u8 {
        Tim6rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim6smen {
    #[doc = "TIM6 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "TIM6 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Tim6smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim6smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim6smen {
    #[inline(always)]
    fn from(val: u8) -> Tim6smen {
        Tim6smen::from_bits(val)
    }
}
impl From<Tim6smen> for u8 {
    #[inline(always)]
    fn from(val: Tim6smen) -> u8 {
        Tim6smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim7en {
    #[doc = "TIM7 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "TIM7 clock enabled"]
    B_0X1 = 0x01,
}
impl Tim7en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim7en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim7en {
    #[inline(always)]
    fn from(val: u8) -> Tim7en {
        Tim7en::from_bits(val)
    }
}
impl From<Tim7en> for u8 {
    #[inline(always)]
    fn from(val: Tim7en) -> u8 {
        Tim7en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim7rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset TIM7"]
    B_0X1 = 0x01,
}
impl Tim7rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim7rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim7rst {
    #[inline(always)]
    fn from(val: u8) -> Tim7rst {
        Tim7rst::from_bits(val)
    }
}
impl From<Tim7rst> for u8 {
    #[inline(always)]
    fn from(val: Tim7rst) -> u8 {
        Tim7rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim7smen {
    #[doc = "TIM7 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "TIM7 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Tim7smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim7smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim7smen {
    #[inline(always)]
    fn from(val: u8) -> Tim7smen {
        Tim7smen::from_bits(val)
    }
}
impl From<Tim7smen> for u8 {
    #[inline(always)]
    fn from(val: Tim7smen) -> u8 {
        Tim7smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim8en {
    #[doc = "TIM8 timer clock disabled"]
    B_0X0 = 0x0,
    #[doc = "TIM8 timer clock enabled"]
    B_0X1 = 0x01,
}
impl Tim8en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim8en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim8en {
    #[inline(always)]
    fn from(val: u8) -> Tim8en {
        Tim8en::from_bits(val)
    }
}
impl From<Tim8en> for u8 {
    #[inline(always)]
    fn from(val: Tim8en) -> u8 {
        Tim8en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim8rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset TIM8 timer"]
    B_0X1 = 0x01,
}
impl Tim8rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim8rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim8rst {
    #[inline(always)]
    fn from(val: u8) -> Tim8rst {
        Tim8rst::from_bits(val)
    }
}
impl From<Tim8rst> for u8 {
    #[inline(always)]
    fn from(val: Tim8rst) -> u8 {
        Tim8rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tim8smen {
    #[doc = "TIM8 timer clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "TIM8 timer clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Tim8smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tim8smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tim8smen {
    #[inline(always)]
    fn from(val: u8) -> Tim8smen {
        Tim8smen::from_bits(val)
    }
}
impl From<Tim8smen> for u8 {
    #[inline(always)]
    fn from(val: Tim8smen) -> u8 {
        Tim8smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Uart4en {
    #[doc = "UART4 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "UART4 clock enabled"]
    B_0X1 = 0x01,
}
impl Uart4en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uart4en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uart4en {
    #[inline(always)]
    fn from(val: u8) -> Uart4en {
        Uart4en::from_bits(val)
    }
}
impl From<Uart4en> for u8 {
    #[inline(always)]
    fn from(val: Uart4en) -> u8 {
        Uart4en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Uart4rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset UART4"]
    B_0X1 = 0x01,
}
impl Uart4rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uart4rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uart4rst {
    #[inline(always)]
    fn from(val: u8) -> Uart4rst {
        Uart4rst::from_bits(val)
    }
}
impl From<Uart4rst> for u8 {
    #[inline(always)]
    fn from(val: Uart4rst) -> u8 {
        Uart4rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Uart4sel {
    #[doc = "PCLK selected as UART4 clock"]
    B_0X0 = 0x0,
    #[doc = "System clock (SYSCLK) selected as UART4 clock"]
    B_0X1 = 0x01,
    #[doc = "HSI16 clock selected as UART4 clock"]
    B_0X2 = 0x02,
    #[doc = "LSE clock selected as UART4 clock"]
    B_0X3 = 0x03,
}
impl Uart4sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uart4sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uart4sel {
    #[inline(always)]
    fn from(val: u8) -> Uart4sel {
        Uart4sel::from_bits(val)
    }
}
impl From<Uart4sel> for u8 {
    #[inline(always)]
    fn from(val: Uart4sel) -> u8 {
        Uart4sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Uart4smen {
    #[doc = "UART4 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "UART4 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Uart4smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uart4smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uart4smen {
    #[inline(always)]
    fn from(val: u8) -> Uart4smen {
        Uart4smen::from_bits(val)
    }
}
impl From<Uart4smen> for u8 {
    #[inline(always)]
    fn from(val: Uart4smen) -> u8 {
        Uart4smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Uart5en {
    #[doc = "UART5 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "UART5 clock enabled"]
    B_0X1 = 0x01,
}
impl Uart5en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uart5en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uart5en {
    #[inline(always)]
    fn from(val: u8) -> Uart5en {
        Uart5en::from_bits(val)
    }
}
impl From<Uart5en> for u8 {
    #[inline(always)]
    fn from(val: Uart5en) -> u8 {
        Uart5en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Uart5rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset UART5"]
    B_0X1 = 0x01,
}
impl Uart5rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uart5rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uart5rst {
    #[inline(always)]
    fn from(val: u8) -> Uart5rst {
        Uart5rst::from_bits(val)
    }
}
impl From<Uart5rst> for u8 {
    #[inline(always)]
    fn from(val: Uart5rst) -> u8 {
        Uart5rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Uart5sel {
    #[doc = "PCLK selected as UART5 clock"]
    B_0X0 = 0x0,
    #[doc = "System clock (SYSCLK) selected as UART5 clock"]
    B_0X1 = 0x01,
    #[doc = "HSI16 clock selected as UART5 clock"]
    B_0X2 = 0x02,
    #[doc = "LSE clock selected as UART5 clock"]
    B_0X3 = 0x03,
}
impl Uart5sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uart5sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uart5sel {
    #[inline(always)]
    fn from(val: u8) -> Uart5sel {
        Uart5sel::from_bits(val)
    }
}
impl From<Uart5sel> for u8 {
    #[inline(always)]
    fn from(val: Uart5sel) -> u8 {
        Uart5sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Uart5smen {
    #[doc = "UART5 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "UART5 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Uart5smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uart5smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uart5smen {
    #[inline(always)]
    fn from(val: u8) -> Uart5smen {
        Uart5smen::from_bits(val)
    }
}
impl From<Uart5smen> for u8 {
    #[inline(always)]
    fn from(val: Uart5smen) -> u8 {
        Uart5smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ucpd1en {
    #[doc = "UCPD1 clock disable"]
    B_0X0 = 0x0,
    #[doc = "UCPD1 clock enable"]
    B_0X1 = 0x01,
}
impl Ucpd1en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ucpd1en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ucpd1en {
    #[inline(always)]
    fn from(val: u8) -> Ucpd1en {
        Ucpd1en::from_bits(val)
    }
}
impl From<Ucpd1en> for u8 {
    #[inline(always)]
    fn from(val: Ucpd1en) -> u8 {
        Ucpd1en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ucpd1rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset UCPD1"]
    B_0X1 = 0x01,
}
impl Ucpd1rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ucpd1rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ucpd1rst {
    #[inline(always)]
    fn from(val: u8) -> Ucpd1rst {
        Ucpd1rst::from_bits(val)
    }
}
impl From<Ucpd1rst> for u8 {
    #[inline(always)]
    fn from(val: Ucpd1rst) -> u8 {
        Ucpd1rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ucpd1smen {
    #[doc = "UCPD1 clocks disabled by the clock gating during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "UCPD1 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Ucpd1smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ucpd1smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ucpd1smen {
    #[inline(always)]
    fn from(val: u8) -> Ucpd1smen {
        Ucpd1smen::from_bits(val)
    }
}
impl From<Ucpd1smen> for u8 {
    #[inline(always)]
    fn from(val: Ucpd1smen) -> u8 {
        Ucpd1smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Usart1en {
    #[doc = "USART1clock disabled"]
    B_0X0 = 0x0,
    #[doc = "USART1clock enabled"]
    B_0X1 = 0x01,
}
impl Usart1en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usart1en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usart1en {
    #[inline(always)]
    fn from(val: u8) -> Usart1en {
        Usart1en::from_bits(val)
    }
}
impl From<Usart1en> for u8 {
    #[inline(always)]
    fn from(val: Usart1en) -> u8 {
        Usart1en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Usart1rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset USART1"]
    B_0X1 = 0x01,
}
impl Usart1rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usart1rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usart1rst {
    #[inline(always)]
    fn from(val: u8) -> Usart1rst {
        Usart1rst::from_bits(val)
    }
}
impl From<Usart1rst> for u8 {
    #[inline(always)]
    fn from(val: Usart1rst) -> u8 {
        Usart1rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Usart1sel {
    #[doc = "PCLK selected as USART1 clock"]
    B_0X0 = 0x0,
    #[doc = "System clock (SYSCLK) selected as USART1 clock"]
    B_0X1 = 0x01,
    #[doc = "HSI16 clock selected as USART1 clock"]
    B_0X2 = 0x02,
    #[doc = "LSE clock selected as USART1 clock"]
    B_0X3 = 0x03,
}
impl Usart1sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usart1sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usart1sel {
    #[inline(always)]
    fn from(val: u8) -> Usart1sel {
        Usart1sel::from_bits(val)
    }
}
impl From<Usart1sel> for u8 {
    #[inline(always)]
    fn from(val: Usart1sel) -> u8 {
        Usart1sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Usart1smen {
    #[doc = "USART1clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "USART1clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Usart1smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usart1smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usart1smen {
    #[inline(always)]
    fn from(val: u8) -> Usart1smen {
        Usart1smen::from_bits(val)
    }
}
impl From<Usart1smen> for u8 {
    #[inline(always)]
    fn from(val: Usart1smen) -> u8 {
        Usart1smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Usart2en {
    #[doc = "USART2 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "USART2 clock enabled"]
    B_0X1 = 0x01,
}
impl Usart2en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usart2en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usart2en {
    #[inline(always)]
    fn from(val: u8) -> Usart2en {
        Usart2en::from_bits(val)
    }
}
impl From<Usart2en> for u8 {
    #[inline(always)]
    fn from(val: Usart2en) -> u8 {
        Usart2en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Usart2rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset USART2"]
    B_0X1 = 0x01,
}
impl Usart2rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usart2rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usart2rst {
    #[inline(always)]
    fn from(val: u8) -> Usart2rst {
        Usart2rst::from_bits(val)
    }
}
impl From<Usart2rst> for u8 {
    #[inline(always)]
    fn from(val: Usart2rst) -> u8 {
        Usart2rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Usart2sel {
    #[doc = "PCLK selected as USART2 clock"]
    B_0X0 = 0x0,
    #[doc = "System clock (SYSCLK) selected as USART2 clock"]
    B_0X1 = 0x01,
    #[doc = "HSI16 clock selected as USART2 clock"]
    B_0X2 = 0x02,
    #[doc = "LSE clock selected as USART2 clock"]
    B_0X3 = 0x03,
}
impl Usart2sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usart2sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usart2sel {
    #[inline(always)]
    fn from(val: u8) -> Usart2sel {
        Usart2sel::from_bits(val)
    }
}
impl From<Usart2sel> for u8 {
    #[inline(always)]
    fn from(val: Usart2sel) -> u8 {
        Usart2sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Usart2smen {
    #[doc = "USART2 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "USART2 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Usart2smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usart2smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usart2smen {
    #[inline(always)]
    fn from(val: u8) -> Usart2smen {
        Usart2smen::from_bits(val)
    }
}
impl From<Usart2smen> for u8 {
    #[inline(always)]
    fn from(val: Usart2smen) -> u8 {
        Usart2smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Usart3en {
    #[doc = "USART3 clock disabled"]
    B_0X0 = 0x0,
    #[doc = "USART3 clock enabled"]
    B_0X1 = 0x01,
}
impl Usart3en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usart3en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usart3en {
    #[inline(always)]
    fn from(val: u8) -> Usart3en {
        Usart3en::from_bits(val)
    }
}
impl From<Usart3en> for u8 {
    #[inline(always)]
    fn from(val: Usart3en) -> u8 {
        Usart3en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Usart3rst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset USART3"]
    B_0X1 = 0x01,
}
impl Usart3rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usart3rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usart3rst {
    #[inline(always)]
    fn from(val: u8) -> Usart3rst {
        Usart3rst::from_bits(val)
    }
}
impl From<Usart3rst> for u8 {
    #[inline(always)]
    fn from(val: Usart3rst) -> u8 {
        Usart3rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Usart3sel {
    #[doc = "PCLK selected as USART3 clock"]
    B_0X0 = 0x0,
    #[doc = "System clock (SYSCLK) selected as USART3 clock"]
    B_0X1 = 0x01,
    #[doc = "HSI16 clock selected as USART3 clock"]
    B_0X2 = 0x02,
    #[doc = "LSE clock selected as USART3 clock"]
    B_0X3 = 0x03,
}
impl Usart3sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usart3sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usart3sel {
    #[inline(always)]
    fn from(val: u8) -> Usart3sel {
        Usart3sel::from_bits(val)
    }
}
impl From<Usart3sel> for u8 {
    #[inline(always)]
    fn from(val: Usart3sel) -> u8 {
        Usart3sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Usart3smen {
    #[doc = "USART3 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "USART3 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Usart3smen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usart3smen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usart3smen {
    #[inline(always)]
    fn from(val: u8) -> Usart3smen {
        Usart3smen::from_bits(val)
    }
}
impl From<Usart3smen> for u8 {
    #[inline(always)]
    fn from(val: Usart3smen) -> u8 {
        Usart3smen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Usben {
    #[doc = "USB device clock disabled"]
    B_0X0 = 0x0,
    #[doc = "USB device clock enabled"]
    B_0X1 = 0x01,
}
impl Usben {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usben {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usben {
    #[inline(always)]
    fn from(val: u8) -> Usben {
        Usben::from_bits(val)
    }
}
impl From<Usben> for u8 {
    #[inline(always)]
    fn from(val: Usben) -> u8 {
        Usben::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Usbrst {
    #[doc = "No effect"]
    B_0X0 = 0x0,
    #[doc = "Reset USB device"]
    B_0X1 = 0x01,
}
impl Usbrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbrst {
    #[inline(always)]
    fn from(val: u8) -> Usbrst {
        Usbrst::from_bits(val)
    }
}
impl From<Usbrst> for u8 {
    #[inline(always)]
    fn from(val: Usbrst) -> u8 {
        Usbrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Usbsmen {
    #[doc = "USB device clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "USB device clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Usbsmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbsmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbsmen {
    #[inline(always)]
    fn from(val: u8) -> Usbsmen {
        Usbsmen::from_bits(val)
    }
}
impl From<Usbsmen> for u8 {
    #[inline(always)]
    fn from(val: Usbsmen) -> u8 {
        Usbsmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Wwdgen {
    #[doc = "Window watchdog clock disabled"]
    B_0X0 = 0x0,
    #[doc = "Window watchdog clock enabled"]
    B_0X1 = 0x01,
}
impl Wwdgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wwdgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wwdgen {
    #[inline(always)]
    fn from(val: u8) -> Wwdgen {
        Wwdgen::from_bits(val)
    }
}
impl From<Wwdgen> for u8 {
    #[inline(always)]
    fn from(val: Wwdgen) -> u8 {
        Wwdgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Wwdgrstf {
    #[doc = "No window watchdog reset occurred"]
    B_0X0 = 0x0,
    #[doc = "Window watchdog reset occurred"]
    B_0X1 = 0x01,
}
impl Wwdgrstf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wwdgrstf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wwdgrstf {
    #[inline(always)]
    fn from(val: u8) -> Wwdgrstf {
        Wwdgrstf::from_bits(val)
    }
}
impl From<Wwdgrstf> for u8 {
    #[inline(always)]
    fn from(val: Wwdgrstf) -> u8 {
        Wwdgrstf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Wwdgsmen {
    #[doc = "Window watchdog clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X0 = 0x0,
    #[doc = "Window watchdog clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B_0X1 = 0x01,
}
impl Wwdgsmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wwdgsmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wwdgsmen {
    #[inline(always)]
    fn from(val: u8) -> Wwdgsmen {
        Wwdgsmen::from_bits(val)
    }
}
impl From<Wwdgsmen> for u8 {
    #[inline(always)]
    fn from(val: Wwdgsmen) -> u8 {
        Wwdgsmen::to_bits(val)
    }
}

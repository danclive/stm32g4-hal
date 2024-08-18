#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dbp {
    #[doc = "Access to RTC and Backup registers disabled"]
    B_0X0 = 0x0,
    #[doc = "Access to RTC and Backup registers enabled"]
    B_0X1 = 0x01,
}
impl Dbp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbp {
    #[inline(always)]
    fn from(val: u8) -> Dbp {
        Dbp::from_bits(val)
    }
}
impl From<Dbp> for u8 {
    #[inline(always)]
    fn from(val: Dbp) -> u8 {
        Dbp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Eiwul {
    #[doc = "Internal wakeup line disable."]
    B_0X0 = 0x0,
    #[doc = "Internal wakeup line enable."]
    B_0X1 = 0x01,
}
impl Eiwul {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eiwul {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eiwul {
    #[inline(always)]
    fn from(val: u8) -> Eiwul {
        Eiwul::from_bits(val)
    }
}
impl From<Eiwul> for u8 {
    #[inline(always)]
    fn from(val: Eiwul) -> u8 {
        Eiwul::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lpms {
    #[doc = "Stop 0 mode"]
    B_0X0 = 0x0,
    #[doc = "Stop 1 mode"]
    B_0X1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Standby mode"]
    B_0X3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Lpms {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpms {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpms {
    #[inline(always)]
    fn from(val: u8) -> Lpms {
        Lpms::from_bits(val)
    }
}
impl From<Lpms> for u8 {
    #[inline(always)]
    fn from(val: Lpms) -> u8 {
        Lpms::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pvde {
    #[doc = "Programmable voltage detector disable."]
    B_0X0 = 0x0,
    #[doc = "Programmable voltage detector enable."]
    B_0X1 = 0x01,
}
impl Pvde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pvde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pvde {
    #[inline(always)]
    fn from(val: u8) -> Pvde {
        Pvde::from_bits(val)
    }
}
impl From<Pvde> for u8 {
    #[inline(always)]
    fn from(val: Pvde) -> u8 {
        Pvde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pvdls {
    #[doc = "V<sub>PVD0</sub> PVD threshold 0"]
    B_0X0 = 0x0,
    #[doc = "V<sub>PVD1</sub> PVD threshold 1"]
    B_0X1 = 0x01,
    #[doc = "V<sub>PVD2</sub> PVD threshold 2"]
    B_0X2 = 0x02,
    #[doc = "V<sub>PVD3</sub> PVD threshold 3"]
    B_0X3 = 0x03,
    #[doc = "V<sub>PVD4</sub> PVD threshold 4"]
    B_0X4 = 0x04,
    #[doc = "V<sub>PVD5</sub> PVD threshold 5"]
    B_0X5 = 0x05,
    #[doc = "V<sub>PVD6</sub> PVD threshold 6"]
    B_0X6 = 0x06,
    #[doc = "External input analog voltage PVD_IN (compared internally to V<sub>REFINT</sub>)"]
    B_0X7 = 0x07,
}
impl Pvdls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pvdls {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pvdls {
    #[inline(always)]
    fn from(val: u8) -> Pvdls {
        Pvdls::from_bits(val)
    }
}
impl From<Pvdls> for u8 {
    #[inline(always)]
    fn from(val: Pvdls) -> u8 {
        Pvdls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pvdo {
    #[doc = "V<sub>DD</sub> is above the selected PVD threshold"]
    B_0X0 = 0x0,
    #[doc = "V<sub>DD</sub> is below the selected PVD threshold"]
    B_0X1 = 0x01,
}
impl Pvdo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pvdo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pvdo {
    #[inline(always)]
    fn from(val: u8) -> Pvdo {
        Pvdo::from_bits(val)
    }
}
impl From<Pvdo> for u8 {
    #[inline(always)]
    fn from(val: Pvdo) -> u8 {
        Pvdo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pvmen1 {
    #[doc = "PVM1 (V<sub>DDA</sub> monitoring vs. 1.62V threshold) disable."]
    B_0X0 = 0x0,
    #[doc = "PVM1 (V<sub>DDA</sub> monitoring vs. 1.62V threshold) enable."]
    B_0X1 = 0x01,
}
impl Pvmen1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pvmen1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pvmen1 {
    #[inline(always)]
    fn from(val: u8) -> Pvmen1 {
        Pvmen1::from_bits(val)
    }
}
impl From<Pvmen1> for u8 {
    #[inline(always)]
    fn from(val: Pvmen1) -> u8 {
        Pvmen1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pvmen2 {
    #[doc = "PVM2 (V<sub>DDA</sub> monitoring vs. 1.8 V threshold) disable."]
    B_0X0 = 0x0,
    #[doc = "PVM2 (V<sub>DDA</sub> monitoring vs. 1.8 V threshold) enable."]
    B_0X1 = 0x01,
}
impl Pvmen2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pvmen2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pvmen2 {
    #[inline(always)]
    fn from(val: u8) -> Pvmen2 {
        Pvmen2::from_bits(val)
    }
}
impl From<Pvmen2> for u8 {
    #[inline(always)]
    fn from(val: Pvmen2) -> u8 {
        Pvmen2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pvmo1 {
    #[doc = "V<sub>DDA</sub> voltage is above PVM1 threshold (around 1.62 V)."]
    B_0X0 = 0x0,
    #[doc = "V<sub>DDA</sub> voltage is below PVM1 threshold (around 1.62 V)."]
    B_0X1 = 0x01,
}
impl Pvmo1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pvmo1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pvmo1 {
    #[inline(always)]
    fn from(val: u8) -> Pvmo1 {
        Pvmo1::from_bits(val)
    }
}
impl From<Pvmo1> for u8 {
    #[inline(always)]
    fn from(val: Pvmo1) -> u8 {
        Pvmo1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pvmo2 {
    #[doc = "V<sub>DDA</sub> voltage is above PVM2 threshold (around 1.8 V)."]
    B_0X0 = 0x0,
    #[doc = "V<sub>DDA</sub> voltage is below PVM2 threshold (around 1.8 V)."]
    B_0X1 = 0x01,
}
impl Pvmo2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pvmo2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pvmo2 {
    #[inline(always)]
    fn from(val: u8) -> Pvmo2 {
        Pvmo2::from_bits(val)
    }
}
impl From<Pvmo2> for u8 {
    #[inline(always)]
    fn from(val: Pvmo2) -> u8 {
        Pvmo2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum R1mode {
    #[doc = "Main regulator in range 1 boost mode."]
    B_0X0 = 0x0,
    #[doc = "Main regulator in range 1 normal mode."]
    B_0X1 = 0x01,
}
impl R1mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> R1mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for R1mode {
    #[inline(always)]
    fn from(val: u8) -> R1mode {
        R1mode::from_bits(val)
    }
}
impl From<R1mode> for u8 {
    #[inline(always)]
    fn from(val: R1mode) -> u8 {
        R1mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Reglpf {
    #[doc = "The regulator is ready in main mode (MR)"]
    B_0X0 = 0x0,
    #[doc = "The regulator is in low-power mode (LPR)"]
    B_0X1 = 0x01,
}
impl Reglpf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reglpf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reglpf {
    #[inline(always)]
    fn from(val: u8) -> Reglpf {
        Reglpf::from_bits(val)
    }
}
impl From<Reglpf> for u8 {
    #[inline(always)]
    fn from(val: Reglpf) -> u8 {
        Reglpf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Reglps {
    #[doc = "The low-power regulator is not ready"]
    B_0X0 = 0x0,
    #[doc = "The low-power regulator is ready"]
    B_0X1 = 0x01,
}
impl Reglps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reglps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reglps {
    #[inline(always)]
    fn from(val: u8) -> Reglps {
        Reglps::from_bits(val)
    }
}
impl From<Reglps> for u8 {
    #[inline(always)]
    fn from(val: Reglps) -> u8 {
        Reglps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rrs {
    #[doc = "SRAM2 is powered off in Standby mode (SRAM2 content is lost)."]
    B_0X0 = 0x0,
    #[doc = "SRAM2 is powered by the low-power regulator in Standby mode (SRAM2 content is kept)."]
    B_0X1 = 0x01,
}
impl Rrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rrs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rrs {
    #[inline(always)]
    fn from(val: u8) -> Rrs {
        Rrs::from_bits(val)
    }
}
impl From<Rrs> for u8 {
    #[inline(always)]
    fn from(val: Rrs) -> u8 {
        Rrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sbf {
    #[doc = "The device did not enter the Standby mode"]
    B_0X0 = 0x0,
    #[doc = "The device entered the Standby mode"]
    B_0X1 = 0x01,
}
impl Sbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbf {
    #[inline(always)]
    fn from(val: u8) -> Sbf {
        Sbf::from_bits(val)
    }
}
impl From<Sbf> for u8 {
    #[inline(always)]
    fn from(val: Sbf) -> u8 {
        Sbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ucpd1Dbdis {
    #[doc = "Enable USB Type-C dead battery pull-down behavior on UCPD1_CC1 and UCPD1_CC2 pins."]
    B_0X0 = 0x0,
    #[doc = "Disable USB Type-C dead battery pull-down behavior on UCPD1_CC1 and UCPD1_CC2 pins."]
    B_0X1 = 0x01,
}
impl Ucpd1Dbdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ucpd1Dbdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ucpd1Dbdis {
    #[inline(always)]
    fn from(val: u8) -> Ucpd1Dbdis {
        Ucpd1Dbdis::from_bits(val)
    }
}
impl From<Ucpd1Dbdis> for u8 {
    #[inline(always)]
    fn from(val: Ucpd1Dbdis) -> u8 {
        Ucpd1Dbdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ucpd1Stdby {
    #[doc = "Write 0 immediately after standby exit when using UCPD1, (and before writing any UCPD1 registers)."]
    B_0X0 = 0x0,
    #[doc = "Write 1 just before entering standby when using UCPD1."]
    B_0X1 = 0x01,
}
impl Ucpd1Stdby {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ucpd1Stdby {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ucpd1Stdby {
    #[inline(always)]
    fn from(val: u8) -> Ucpd1Stdby {
        Ucpd1Stdby::from_bits(val)
    }
}
impl From<Ucpd1Stdby> for u8 {
    #[inline(always)]
    fn from(val: Ucpd1Stdby) -> u8 {
        Ucpd1Stdby::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Vbe {
    #[doc = "V<sub>BAT</sub> battery charging disable"]
    B_0X0 = 0x0,
    #[doc = "V<sub>BAT</sub> battery charging enable"]
    B_0X1 = 0x01,
}
impl Vbe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbe {
    #[inline(always)]
    fn from(val: u8) -> Vbe {
        Vbe::from_bits(val)
    }
}
impl From<Vbe> for u8 {
    #[inline(always)]
    fn from(val: Vbe) -> u8 {
        Vbe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Vbrs {
    #[doc = "Charge V<sub>BAT</sub> through a 5 kOhms resistor"]
    B_0X0 = 0x0,
    #[doc = "Charge V<sub>BAT</sub> through a 1.5 kOhms resistor"]
    B_0X1 = 0x01,
}
impl Vbrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbrs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbrs {
    #[inline(always)]
    fn from(val: u8) -> Vbrs {
        Vbrs::from_bits(val)
    }
}
impl From<Vbrs> for u8 {
    #[inline(always)]
    fn from(val: Vbrs) -> u8 {
        Vbrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Vos {
    #[doc = "Cannot be written (forbidden by hardware)"]
    B_0X0 = 0x0,
    #[doc = "Range 1"]
    B_0X1 = 0x01,
    #[doc = "Range 2"]
    B_0X2 = 0x02,
    #[doc = "Cannot be written (forbidden by hardware)"]
    B_0X3 = 0x03,
}
impl Vos {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vos {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vos {
    #[inline(always)]
    fn from(val: u8) -> Vos {
        Vos::from_bits(val)
    }
}
impl From<Vos> for u8 {
    #[inline(always)]
    fn from(val: Vos) -> u8 {
        Vos::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Vosf {
    #[doc = "The regulator is ready in the selected voltage range"]
    B_0X0 = 0x0,
    #[doc = "The regulator output voltage is changing to the required voltage level"]
    B_0X1 = 0x01,
}
impl Vosf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vosf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vosf {
    #[inline(always)]
    fn from(val: u8) -> Vosf {
        Vosf::from_bits(val)
    }
}
impl From<Vosf> for u8 {
    #[inline(always)]
    fn from(val: Vosf) -> u8 {
        Vosf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Wp1 {
    #[doc = "Detection on high level (rising edge)"]
    B_0X0 = 0x0,
    #[doc = "Detection on low level (falling edge)"]
    B_0X1 = 0x01,
}
impl Wp1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wp1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wp1 {
    #[inline(always)]
    fn from(val: u8) -> Wp1 {
        Wp1::from_bits(val)
    }
}
impl From<Wp1> for u8 {
    #[inline(always)]
    fn from(val: Wp1) -> u8 {
        Wp1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Wp2 {
    #[doc = "Detection on high level (rising edge)"]
    B_0X0 = 0x0,
    #[doc = "Detection on low level (falling edge)"]
    B_0X1 = 0x01,
}
impl Wp2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wp2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wp2 {
    #[inline(always)]
    fn from(val: u8) -> Wp2 {
        Wp2::from_bits(val)
    }
}
impl From<Wp2> for u8 {
    #[inline(always)]
    fn from(val: Wp2) -> u8 {
        Wp2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Wp3 {
    #[doc = "Detection on high level (rising edge)"]
    B_0X0 = 0x0,
    #[doc = "Detection on low level (falling edge)"]
    B_0X1 = 0x01,
}
impl Wp3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wp3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wp3 {
    #[inline(always)]
    fn from(val: u8) -> Wp3 {
        Wp3::from_bits(val)
    }
}
impl From<Wp3> for u8 {
    #[inline(always)]
    fn from(val: Wp3) -> u8 {
        Wp3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Wp4 {
    #[doc = "Detection on high level (rising edge)"]
    B_0X0 = 0x0,
    #[doc = "Detection on low level (falling edge)"]
    B_0X1 = 0x01,
}
impl Wp4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wp4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wp4 {
    #[inline(always)]
    fn from(val: u8) -> Wp4 {
        Wp4::from_bits(val)
    }
}
impl From<Wp4> for u8 {
    #[inline(always)]
    fn from(val: Wp4) -> u8 {
        Wp4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Wp5 {
    #[doc = "Detection on high level (rising edge)"]
    B_0X0 = 0x0,
    #[doc = "Detection on low level (falling edge)"]
    B_0X1 = 0x01,
}
impl Wp5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wp5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wp5 {
    #[inline(always)]
    fn from(val: u8) -> Wp5 {
        Wp5::from_bits(val)
    }
}
impl From<Wp5> for u8 {
    #[inline(always)]
    fn from(val: Wp5) -> u8 {
        Wp5::to_bits(val)
    }
}

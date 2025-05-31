#[doc = "Register `RCC_BDCR` reader"]
pub type R = crate::R<RccBdcrSpec>;
#[doc = "Register `RCC_BDCR` writer"]
pub type W = crate::W<RccBdcrSpec>;
#[doc = "LSE oscillator enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lseon {
    #[doc = "0: LSE oscillator OFF"]
    B0x0 = 0,
    #[doc = "1: LSE oscillator ON"]
    B0x1 = 1,
}
impl From<Lseon> for bool {
    #[inline(always)]
    fn from(variant: Lseon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSEON` reader - LSE oscillator enable Set and cleared by software."]
pub type LseonR = crate::BitReader<Lseon>;
impl LseonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lseon {
        match self.bits {
            false => Lseon::B0x0,
            true => Lseon::B0x1,
        }
    }
    #[doc = "LSE oscillator OFF"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lseon::B0x0
    }
    #[doc = "LSE oscillator ON"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lseon::B0x1
    }
}
#[doc = "Field `LSEON` writer - LSE oscillator enable Set and cleared by software."]
pub type LseonW<'a, REG> = crate::BitWriter<'a, REG, Lseon>;
impl<'a, REG> LseonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSE oscillator OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lseon::B0x0)
    }
    #[doc = "LSE oscillator ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lseon::B0x1)
    }
}
#[doc = "LSE oscillator ready Set and cleared by hardware to indicate when the external 32 kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after 6 external low-speed oscillator clock cycles.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lserdy {
    #[doc = "0: LSE oscillator not ready"]
    B0x0 = 0,
    #[doc = "1: LSE oscillator ready"]
    B0x1 = 1,
}
impl From<Lserdy> for bool {
    #[inline(always)]
    fn from(variant: Lserdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSERDY` reader - LSE oscillator ready Set and cleared by hardware to indicate when the external 32 kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after 6 external low-speed oscillator clock cycles."]
pub type LserdyR = crate::BitReader<Lserdy>;
impl LserdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lserdy {
        match self.bits {
            false => Lserdy::B0x0,
            true => Lserdy::B0x1,
        }
    }
    #[doc = "LSE oscillator not ready"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lserdy::B0x0
    }
    #[doc = "LSE oscillator ready"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lserdy::B0x1
    }
}
#[doc = "LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32 kHz oscillator is disabled (LSEON=0 and LSERDY=0).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsebyp {
    #[doc = "0: LSE oscillator not bypassed"]
    B0x0 = 0,
    #[doc = "1: LSE oscillator bypassed"]
    B0x1 = 1,
}
impl From<Lsebyp> for bool {
    #[inline(always)]
    fn from(variant: Lsebyp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSEBYP` reader - LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32 kHz oscillator is disabled (LSEON=0 and LSERDY=0)."]
pub type LsebypR = crate::BitReader<Lsebyp>;
impl LsebypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsebyp {
        match self.bits {
            false => Lsebyp::B0x0,
            true => Lsebyp::B0x1,
        }
    }
    #[doc = "LSE oscillator not bypassed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lsebyp::B0x0
    }
    #[doc = "LSE oscillator bypassed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lsebyp::B0x1
    }
}
#[doc = "Field `LSEBYP` writer - LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32 kHz oscillator is disabled (LSEON=0 and LSERDY=0)."]
pub type LsebypW<'a, REG> = crate::BitWriter<'a, REG, Lsebyp>;
impl<'a, REG> LsebypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSE oscillator not bypassed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lsebyp::B0x0)
    }
    #[doc = "LSE oscillator bypassed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lsebyp::B0x1)
    }
}
#[doc = "LSE oscillator drive capability Set by software to modulate the LSE oscillators drive capability. The oscillator is in Xtal mode when it is not in bypass mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lsedrv {
    #[doc = "0: Xtal mode lower driving capability"]
    B0x0 = 0,
    #[doc = "1: Xtal mode medium low driving capability"]
    B0x1 = 1,
    #[doc = "2: Xtal mode medium high driving capability"]
    B0x2 = 2,
    #[doc = "3: Xtal mode higher driving capability"]
    B0x3 = 3,
}
impl From<Lsedrv> for u8 {
    #[inline(always)]
    fn from(variant: Lsedrv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lsedrv {
    type Ux = u8;
}
impl crate::IsEnum for Lsedrv {}
#[doc = "Field `LSEDRV` reader - LSE oscillator drive capability Set by software to modulate the LSE oscillators drive capability. The oscillator is in Xtal mode when it is not in bypass mode."]
pub type LsedrvR = crate::FieldReader<Lsedrv>;
impl LsedrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsedrv {
        match self.bits {
            0 => Lsedrv::B0x0,
            1 => Lsedrv::B0x1,
            2 => Lsedrv::B0x2,
            3 => Lsedrv::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Xtal mode lower driving capability"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lsedrv::B0x0
    }
    #[doc = "Xtal mode medium low driving capability"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lsedrv::B0x1
    }
    #[doc = "Xtal mode medium high driving capability"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Lsedrv::B0x2
    }
    #[doc = "Xtal mode higher driving capability"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Lsedrv::B0x3
    }
}
#[doc = "Field `LSEDRV` writer - LSE oscillator drive capability Set by software to modulate the LSE oscillators drive capability. The oscillator is in Xtal mode when it is not in bypass mode."]
pub type LsedrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lsedrv, crate::Safe>;
impl<'a, REG> LsedrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Xtal mode lower driving capability"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lsedrv::B0x0)
    }
    #[doc = "Xtal mode medium low driving capability"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lsedrv::B0x1)
    }
    #[doc = "Xtal mode medium high driving capability"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Lsedrv::B0x2)
    }
    #[doc = "Xtal mode higher driving capability"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Lsedrv::B0x3)
    }
}
#[doc = "CSS on LSE enable Set by software to enable the Clock Security System on LSE (32 kHz oscillator). LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software MUST disable the LSECSSON bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsecsson {
    #[doc = "0: CSS on LSE (32 kHz external oscillator) OFF"]
    B0x0 = 0,
    #[doc = "1: CSS on LSE (32 kHz external oscillator) ON"]
    B0x1 = 1,
}
impl From<Lsecsson> for bool {
    #[inline(always)]
    fn from(variant: Lsecsson) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSON` reader - CSS on LSE enable Set by software to enable the Clock Security System on LSE (32 kHz oscillator). LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software MUST disable the LSECSSON bit."]
pub type LsecssonR = crate::BitReader<Lsecsson>;
impl LsecssonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsecsson {
        match self.bits {
            false => Lsecsson::B0x0,
            true => Lsecsson::B0x1,
        }
    }
    #[doc = "CSS on LSE (32 kHz external oscillator) OFF"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lsecsson::B0x0
    }
    #[doc = "CSS on LSE (32 kHz external oscillator) ON"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lsecsson::B0x1
    }
}
#[doc = "Field `LSECSSON` writer - CSS on LSE enable Set by software to enable the Clock Security System on LSE (32 kHz oscillator). LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software MUST disable the LSECSSON bit."]
pub type LsecssonW<'a, REG> = crate::BitWriter<'a, REG, Lsecsson>;
impl<'a, REG> LsecssonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CSS on LSE (32 kHz external oscillator) OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lsecsson::B0x0)
    }
    #[doc = "CSS on LSE (32 kHz external oscillator) ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lsecsson::B0x1)
    }
}
#[doc = "CSS on LSE failure Detection Set by hardware to indicate when a failure has been detected by the Clock Security System on the external 32 kHz oscillator (LSE).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsecssd {
    #[doc = "0: No failure detected on LSE (32 kHz oscillator)"]
    B0x0 = 0,
    #[doc = "1: Failure detected on LSE (32 kHz oscillator)"]
    B0x1 = 1,
}
impl From<Lsecssd> for bool {
    #[inline(always)]
    fn from(variant: Lsecssd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSD` reader - CSS on LSE failure Detection Set by hardware to indicate when a failure has been detected by the Clock Security System on the external 32 kHz oscillator (LSE)."]
pub type LsecssdR = crate::BitReader<Lsecssd>;
impl LsecssdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsecssd {
        match self.bits {
            false => Lsecssd::B0x0,
            true => Lsecssd::B0x1,
        }
    }
    #[doc = "No failure detected on LSE (32 kHz oscillator)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lsecssd::B0x0
    }
    #[doc = "Failure detected on LSE (32 kHz oscillator)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lsecssd::B0x1
    }
}
#[doc = "RTC clock source selection Set by software to select the clock source for the RTC. Once the RTC clock source has been selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset them.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtcsel {
    #[doc = "0: No clock"]
    B0x0 = 0,
    #[doc = "1: LSE oscillator clock used as RTC clock"]
    B0x1 = 1,
    #[doc = "2: LSI oscillator clock used as RTC clock"]
    B0x2 = 2,
    #[doc = "3: HSE oscillator clock divided by 32 used as RTC clock"]
    B0x3 = 3,
}
impl From<Rtcsel> for u8 {
    #[inline(always)]
    fn from(variant: Rtcsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtcsel {
    type Ux = u8;
}
impl crate::IsEnum for Rtcsel {}
#[doc = "Field `RTCSEL` reader - RTC clock source selection Set by software to select the clock source for the RTC. Once the RTC clock source has been selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset them."]
pub type RtcselR = crate::FieldReader<Rtcsel>;
impl RtcselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcsel {
        match self.bits {
            0 => Rtcsel::B0x0,
            1 => Rtcsel::B0x1,
            2 => Rtcsel::B0x2,
            3 => Rtcsel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rtcsel::B0x0
    }
    #[doc = "LSE oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rtcsel::B0x1
    }
    #[doc = "LSI oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Rtcsel::B0x2
    }
    #[doc = "HSE oscillator clock divided by 32 used as RTC clock"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Rtcsel::B0x3
    }
}
#[doc = "Field `RTCSEL` writer - RTC clock source selection Set by software to select the clock source for the RTC. Once the RTC clock source has been selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset them."]
pub type RtcselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rtcsel, crate::Safe>;
impl<'a, REG> RtcselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcsel::B0x0)
    }
    #[doc = "LSE oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcsel::B0x1)
    }
    #[doc = "LSI oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcsel::B0x2)
    }
    #[doc = "HSE oscillator clock divided by 32 used as RTC clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcsel::B0x3)
    }
}
#[doc = "RTC clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcen {
    #[doc = "0: RTC clock disabled"]
    B0x0 = 0,
    #[doc = "1: RTC clock enabled"]
    B0x1 = 1,
}
impl From<Rtcen> for bool {
    #[inline(always)]
    fn from(variant: Rtcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEN` reader - RTC clock enable Set and cleared by software."]
pub type RtcenR = crate::BitReader<Rtcen>;
impl RtcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcen {
        match self.bits {
            false => Rtcen::B0x0,
            true => Rtcen::B0x1,
        }
    }
    #[doc = "RTC clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rtcen::B0x0
    }
    #[doc = "RTC clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rtcen::B0x1
    }
}
#[doc = "Field `RTCEN` writer - RTC clock enable Set and cleared by software."]
pub type RtcenW<'a, REG> = crate::BitWriter<'a, REG, Rtcen>;
impl<'a, REG> RtcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcen::B0x0)
    }
    #[doc = "RTC clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcen::B0x1)
    }
}
#[doc = "RTC domain software reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bdrst {
    #[doc = "0: Reset not activated"]
    B0x0 = 0,
    #[doc = "1: Reset the entire RTC domain"]
    B0x1 = 1,
}
impl From<Bdrst> for bool {
    #[inline(always)]
    fn from(variant: Bdrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BDRST` reader - RTC domain software reset Set and cleared by software."]
pub type BdrstR = crate::BitReader<Bdrst>;
impl BdrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bdrst {
        match self.bits {
            false => Bdrst::B0x0,
            true => Bdrst::B0x1,
        }
    }
    #[doc = "Reset not activated"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bdrst::B0x0
    }
    #[doc = "Reset the entire RTC domain"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bdrst::B0x1
    }
}
#[doc = "Field `BDRST` writer - RTC domain software reset Set and cleared by software."]
pub type BdrstW<'a, REG> = crate::BitWriter<'a, REG, Bdrst>;
impl<'a, REG> BdrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset not activated"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bdrst::B0x0)
    }
    #[doc = "Reset the entire RTC domain"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bdrst::B0x1)
    }
}
#[doc = "Low speed clock output enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lscoen {
    #[doc = "0: Low speed clock output (LSCO) disable"]
    B0x0 = 0,
    #[doc = "1: Low speed clock output (LSCO) enable"]
    B0x1 = 1,
}
impl From<Lscoen> for bool {
    #[inline(always)]
    fn from(variant: Lscoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSCOEN` reader - Low speed clock output enable Set and cleared by software."]
pub type LscoenR = crate::BitReader<Lscoen>;
impl LscoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lscoen {
        match self.bits {
            false => Lscoen::B0x0,
            true => Lscoen::B0x1,
        }
    }
    #[doc = "Low speed clock output (LSCO) disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lscoen::B0x0
    }
    #[doc = "Low speed clock output (LSCO) enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lscoen::B0x1
    }
}
#[doc = "Field `LSCOEN` writer - Low speed clock output enable Set and cleared by software."]
pub type LscoenW<'a, REG> = crate::BitWriter<'a, REG, Lscoen>;
impl<'a, REG> LscoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low speed clock output (LSCO) disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lscoen::B0x0)
    }
    #[doc = "Low speed clock output (LSCO) enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lscoen::B0x1)
    }
}
#[doc = "Low speed clock output selection Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lscosel {
    #[doc = "0: LSI clock selected"]
    B0x0 = 0,
    #[doc = "1: LSE clock selected"]
    B0x1 = 1,
}
impl From<Lscosel> for bool {
    #[inline(always)]
    fn from(variant: Lscosel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSCOSEL` reader - Low speed clock output selection Set and cleared by software."]
pub type LscoselR = crate::BitReader<Lscosel>;
impl LscoselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lscosel {
        match self.bits {
            false => Lscosel::B0x0,
            true => Lscosel::B0x1,
        }
    }
    #[doc = "LSI clock selected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lscosel::B0x0
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lscosel::B0x1
    }
}
#[doc = "Field `LSCOSEL` writer - Low speed clock output selection Set and cleared by software."]
pub type LscoselW<'a, REG> = crate::BitWriter<'a, REG, Lscosel>;
impl<'a, REG> LscoselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSI clock selected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lscosel::B0x0)
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lscosel::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - LSE oscillator enable Set and cleared by software."]
    #[inline(always)]
    pub fn lseon(&self) -> LseonR {
        LseonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE oscillator ready Set and cleared by hardware to indicate when the external 32 kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after 6 external low-speed oscillator clock cycles."]
    #[inline(always)]
    pub fn lserdy(&self) -> LserdyR {
        LserdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32 kHz oscillator is disabled (LSEON=0 and LSERDY=0)."]
    #[inline(always)]
    pub fn lsebyp(&self) -> LsebypR {
        LsebypR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability Set by software to modulate the LSE oscillators drive capability. The oscillator is in Xtal mode when it is not in bypass mode."]
    #[inline(always)]
    pub fn lsedrv(&self) -> LsedrvR {
        LsedrvR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - CSS on LSE enable Set by software to enable the Clock Security System on LSE (32 kHz oscillator). LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software MUST disable the LSECSSON bit."]
    #[inline(always)]
    pub fn lsecsson(&self) -> LsecssonR {
        LsecssonR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CSS on LSE failure Detection Set by hardware to indicate when a failure has been detected by the Clock Security System on the external 32 kHz oscillator (LSE)."]
    #[inline(always)]
    pub fn lsecssd(&self) -> LsecssdR {
        LsecssdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC clock source selection Set by software to select the clock source for the RTC. Once the RTC clock source has been selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset them."]
    #[inline(always)]
    pub fn rtcsel(&self) -> RtcselR {
        RtcselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn rtcen(&self) -> RtcenR {
        RtcenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC domain software reset Set and cleared by software."]
    #[inline(always)]
    pub fn bdrst(&self) -> BdrstR {
        BdrstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Low speed clock output enable Set and cleared by software."]
    #[inline(always)]
    pub fn lscoen(&self) -> LscoenR {
        LscoenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Low speed clock output selection Set and cleared by software."]
    #[inline(always)]
    pub fn lscosel(&self) -> LscoselR {
        LscoselR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_BDCR")
            .field("lseon", &self.lseon())
            .field("lserdy", &self.lserdy())
            .field("lsebyp", &self.lsebyp())
            .field("lsedrv", &self.lsedrv())
            .field("lsecsson", &self.lsecsson())
            .field("lsecssd", &self.lsecssd())
            .field("rtcsel", &self.rtcsel())
            .field("rtcen", &self.rtcen())
            .field("bdrst", &self.bdrst())
            .field("lscoen", &self.lscoen())
            .field("lscosel", &self.lscosel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - LSE oscillator enable Set and cleared by software."]
    #[inline(always)]
    pub fn lseon(&mut self) -> LseonW<RccBdcrSpec> {
        LseonW::new(self, 0)
    }
    #[doc = "Bit 2 - LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32 kHz oscillator is disabled (LSEON=0 and LSERDY=0)."]
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LsebypW<RccBdcrSpec> {
        LsebypW::new(self, 2)
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability Set by software to modulate the LSE oscillators drive capability. The oscillator is in Xtal mode when it is not in bypass mode."]
    #[inline(always)]
    pub fn lsedrv(&mut self) -> LsedrvW<RccBdcrSpec> {
        LsedrvW::new(self, 3)
    }
    #[doc = "Bit 5 - CSS on LSE enable Set by software to enable the Clock Security System on LSE (32 kHz oscillator). LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software MUST disable the LSECSSON bit."]
    #[inline(always)]
    pub fn lsecsson(&mut self) -> LsecssonW<RccBdcrSpec> {
        LsecssonW::new(self, 5)
    }
    #[doc = "Bits 8:9 - RTC clock source selection Set by software to select the clock source for the RTC. Once the RTC clock source has been selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset them."]
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RtcselW<RccBdcrSpec> {
        RtcselW::new(self, 8)
    }
    #[doc = "Bit 15 - RTC clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RtcenW<RccBdcrSpec> {
        RtcenW::new(self, 15)
    }
    #[doc = "Bit 16 - RTC domain software reset Set and cleared by software."]
    #[inline(always)]
    pub fn bdrst(&mut self) -> BdrstW<RccBdcrSpec> {
        BdrstW::new(self, 16)
    }
    #[doc = "Bit 24 - Low speed clock output enable Set and cleared by software."]
    #[inline(always)]
    pub fn lscoen(&mut self) -> LscoenW<RccBdcrSpec> {
        LscoenW::new(self, 24)
    }
    #[doc = "Bit 25 - Low speed clock output selection Set and cleared by software."]
    #[inline(always)]
    pub fn lscosel(&mut self) -> LscoselW<RccBdcrSpec> {
        LscoselW::new(self, 25)
    }
}
#[doc = "RTC domain control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_bdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_bdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccBdcrSpec;
impl crate::RegisterSpec for RccBdcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_bdcr::R`](R) reader structure"]
impl crate::Readable for RccBdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_bdcr::W`](W) writer structure"]
impl crate::Writable for RccBdcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCC_BDCR to value 0"]
impl crate::Resettable for RccBdcrSpec {}

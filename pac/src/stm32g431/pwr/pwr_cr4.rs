#[doc = "Register `PWR_CR4` reader"]
pub type R = crate::R<PwrCr4Spec>;
#[doc = "Register `PWR_CR4` writer"]
pub type W = crate::W<PwrCr4Spec>;
#[doc = "Wakeup pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp1 {
    #[doc = "0: Detection on high level (rising edge)"]
    B0x0 = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    B0x1 = 1,
}
impl From<Wp1> for bool {
    #[inline(always)]
    fn from(variant: Wp1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP1` reader - Wakeup pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1"]
pub type Wp1R = crate::BitReader<Wp1>;
impl Wp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp1 {
        match self.bits {
            false => Wp1::B0x0,
            true => Wp1::B0x1,
        }
    }
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wp1::B0x0
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wp1::B0x1
    }
}
#[doc = "Field `WP1` writer - Wakeup pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1"]
pub type Wp1W<'a, REG> = crate::BitWriter<'a, REG, Wp1>;
impl<'a, REG> Wp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wp1::B0x0)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wp1::B0x1)
    }
}
#[doc = "Wakeup pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp2 {
    #[doc = "0: Detection on high level (rising edge)"]
    B0x0 = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    B0x1 = 1,
}
impl From<Wp2> for bool {
    #[inline(always)]
    fn from(variant: Wp2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP2` reader - Wakeup pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2"]
pub type Wp2R = crate::BitReader<Wp2>;
impl Wp2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp2 {
        match self.bits {
            false => Wp2::B0x0,
            true => Wp2::B0x1,
        }
    }
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wp2::B0x0
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wp2::B0x1
    }
}
#[doc = "Field `WP2` writer - Wakeup pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2"]
pub type Wp2W<'a, REG> = crate::BitWriter<'a, REG, Wp2>;
impl<'a, REG> Wp2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wp2::B0x0)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wp2::B0x1)
    }
}
#[doc = "Wakeup pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp3 {
    #[doc = "0: Detection on high level (rising edge)"]
    B0x0 = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    B0x1 = 1,
}
impl From<Wp3> for bool {
    #[inline(always)]
    fn from(variant: Wp3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP3` reader - Wakeup pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3"]
pub type Wp3R = crate::BitReader<Wp3>;
impl Wp3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp3 {
        match self.bits {
            false => Wp3::B0x0,
            true => Wp3::B0x1,
        }
    }
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wp3::B0x0
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wp3::B0x1
    }
}
#[doc = "Field `WP3` writer - Wakeup pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3"]
pub type Wp3W<'a, REG> = crate::BitWriter<'a, REG, Wp3>;
impl<'a, REG> Wp3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wp3::B0x0)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wp3::B0x1)
    }
}
#[doc = "Wakeup pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp4 {
    #[doc = "0: Detection on high level (rising edge)"]
    B0x0 = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    B0x1 = 1,
}
impl From<Wp4> for bool {
    #[inline(always)]
    fn from(variant: Wp4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP4` reader - Wakeup pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4"]
pub type Wp4R = crate::BitReader<Wp4>;
impl Wp4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp4 {
        match self.bits {
            false => Wp4::B0x0,
            true => Wp4::B0x1,
        }
    }
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wp4::B0x0
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wp4::B0x1
    }
}
#[doc = "Field `WP4` writer - Wakeup pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4"]
pub type Wp4W<'a, REG> = crate::BitWriter<'a, REG, Wp4>;
impl<'a, REG> Wp4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wp4::B0x0)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wp4::B0x1)
    }
}
#[doc = "Wakeup pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp5 {
    #[doc = "0: Detection on high level (rising edge)"]
    B0x0 = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    B0x1 = 1,
}
impl From<Wp5> for bool {
    #[inline(always)]
    fn from(variant: Wp5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP5` reader - Wakeup pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5"]
pub type Wp5R = crate::BitReader<Wp5>;
impl Wp5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp5 {
        match self.bits {
            false => Wp5::B0x0,
            true => Wp5::B0x1,
        }
    }
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wp5::B0x0
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wp5::B0x1
    }
}
#[doc = "Field `WP5` writer - Wakeup pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5"]
pub type Wp5W<'a, REG> = crate::BitWriter<'a, REG, Wp5>;
impl<'a, REG> Wp5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wp5::B0x0)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wp5::B0x1)
    }
}
#[doc = "V<sub>BAT</sub> battery charging enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbe {
    #[doc = "0: V<sub>BAT</sub> battery charging disable"]
    B0x0 = 0,
    #[doc = "1: V<sub>BAT</sub> battery charging enable"]
    B0x1 = 1,
}
impl From<Vbe> for bool {
    #[inline(always)]
    fn from(variant: Vbe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBE` reader - V<sub>BAT</sub> battery charging enable"]
pub type VbeR = crate::BitReader<Vbe>;
impl VbeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbe {
        match self.bits {
            false => Vbe::B0x0,
            true => Vbe::B0x1,
        }
    }
    #[doc = "V<sub>BAT</sub> battery charging disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Vbe::B0x0
    }
    #[doc = "V<sub>BAT</sub> battery charging enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Vbe::B0x1
    }
}
#[doc = "Field `VBE` writer - V<sub>BAT</sub> battery charging enable"]
pub type VbeW<'a, REG> = crate::BitWriter<'a, REG, Vbe>;
impl<'a, REG> VbeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "V<sub>BAT</sub> battery charging disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Vbe::B0x0)
    }
    #[doc = "V<sub>BAT</sub> battery charging enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbe::B0x1)
    }
}
#[doc = "V<sub>BAT</sub> battery charging resistor selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbrs {
    #[doc = "0: Charge V<sub>BAT</sub> through a 5 kOhms resistor"]
    B0x0 = 0,
    #[doc = "1: Charge V<sub>BAT</sub> through a 1.5 kOhms resistor"]
    B0x1 = 1,
}
impl From<Vbrs> for bool {
    #[inline(always)]
    fn from(variant: Vbrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBRS` reader - V<sub>BAT</sub> battery charging resistor selection"]
pub type VbrsR = crate::BitReader<Vbrs>;
impl VbrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbrs {
        match self.bits {
            false => Vbrs::B0x0,
            true => Vbrs::B0x1,
        }
    }
    #[doc = "Charge V<sub>BAT</sub> through a 5 kOhms resistor"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Vbrs::B0x0
    }
    #[doc = "Charge V<sub>BAT</sub> through a 1.5 kOhms resistor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Vbrs::B0x1
    }
}
#[doc = "Field `VBRS` writer - V<sub>BAT</sub> battery charging resistor selection"]
pub type VbrsW<'a, REG> = crate::BitWriter<'a, REG, Vbrs>;
impl<'a, REG> VbrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Charge V<sub>BAT</sub> through a 5 kOhms resistor"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Vbrs::B0x0)
    }
    #[doc = "Charge V<sub>BAT</sub> through a 1.5 kOhms resistor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbrs::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1"]
    #[inline(always)]
    pub fn wp1(&self) -> Wp1R {
        Wp1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2"]
    #[inline(always)]
    pub fn wp2(&self) -> Wp2R {
        Wp2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3"]
    #[inline(always)]
    pub fn wp3(&self) -> Wp3R {
        Wp3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4"]
    #[inline(always)]
    pub fn wp4(&self) -> Wp4R {
        Wp4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5"]
    #[inline(always)]
    pub fn wp5(&self) -> Wp5R {
        Wp5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - V<sub>BAT</sub> battery charging enable"]
    #[inline(always)]
    pub fn vbe(&self) -> VbeR {
        VbeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - V<sub>BAT</sub> battery charging resistor selection"]
    #[inline(always)]
    pub fn vbrs(&self) -> VbrsR {
        VbrsR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_CR4")
            .field("wp1", &self.wp1())
            .field("wp2", &self.wp2())
            .field("wp3", &self.wp3())
            .field("wp4", &self.wp4())
            .field("wp5", &self.wp5())
            .field("vbe", &self.vbe())
            .field("vbrs", &self.vbrs())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1"]
    #[inline(always)]
    pub fn wp1(&mut self) -> Wp1W<PwrCr4Spec> {
        Wp1W::new(self, 0)
    }
    #[doc = "Bit 1 - Wakeup pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2"]
    #[inline(always)]
    pub fn wp2(&mut self) -> Wp2W<PwrCr4Spec> {
        Wp2W::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3"]
    #[inline(always)]
    pub fn wp3(&mut self) -> Wp3W<PwrCr4Spec> {
        Wp3W::new(self, 2)
    }
    #[doc = "Bit 3 - Wakeup pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4"]
    #[inline(always)]
    pub fn wp4(&mut self) -> Wp4W<PwrCr4Spec> {
        Wp4W::new(self, 3)
    }
    #[doc = "Bit 4 - Wakeup pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5"]
    #[inline(always)]
    pub fn wp5(&mut self) -> Wp5W<PwrCr4Spec> {
        Wp5W::new(self, 4)
    }
    #[doc = "Bit 8 - V<sub>BAT</sub> battery charging enable"]
    #[inline(always)]
    pub fn vbe(&mut self) -> VbeW<PwrCr4Spec> {
        VbeW::new(self, 8)
    }
    #[doc = "Bit 9 - V<sub>BAT</sub> battery charging resistor selection"]
    #[inline(always)]
    pub fn vbrs(&mut self) -> VbrsW<PwrCr4Spec> {
        VbrsW::new(self, 9)
    }
}
#[doc = "Power control register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_cr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrCr4Spec;
impl crate::RegisterSpec for PwrCr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_cr4::R`](R) reader structure"]
impl crate::Readable for PwrCr4Spec {}
#[doc = "`write(|w| ..)` method takes [`pwr_cr4::W`](W) writer structure"]
impl crate::Writable for PwrCr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_CR4 to value 0"]
impl crate::Resettable for PwrCr4Spec {
    const RESET_VALUE: u32 = 0;
}

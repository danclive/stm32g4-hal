#[doc = "Register `RCC_CIER` reader"]
pub type R = crate::R<RccCierSpec>;
#[doc = "Register `RCC_CIER` writer"]
pub type W = crate::W<RccCierSpec>;
#[doc = "LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsirdyie {
    #[doc = "0: LSI ready interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: LSI ready interrupt enabled"]
    B0x1 = 1,
}
impl From<Lsirdyie> for bool {
    #[inline(always)]
    fn from(variant: Lsirdyie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
pub type LsirdyieR = crate::BitReader<Lsirdyie>;
impl LsirdyieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsirdyie {
        match self.bits {
            false => Lsirdyie::B0x0,
            true => Lsirdyie::B0x1,
        }
    }
    #[doc = "LSI ready interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lsirdyie::B0x0
    }
    #[doc = "LSI ready interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lsirdyie::B0x1
    }
}
#[doc = "Field `LSIRDYIE` writer - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
pub type LsirdyieW<'a, REG> = crate::BitWriter<'a, REG, Lsirdyie>;
impl<'a, REG> LsirdyieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSI ready interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lsirdyie::B0x0)
    }
    #[doc = "LSI ready interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lsirdyie::B0x1)
    }
}
#[doc = "LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lserdyie {
    #[doc = "0: LSE ready interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: LSE ready interrupt enabled"]
    B0x1 = 1,
}
impl From<Lserdyie> for bool {
    #[inline(always)]
    fn from(variant: Lserdyie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSERDYIE` reader - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
pub type LserdyieR = crate::BitReader<Lserdyie>;
impl LserdyieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lserdyie {
        match self.bits {
            false => Lserdyie::B0x0,
            true => Lserdyie::B0x1,
        }
    }
    #[doc = "LSE ready interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lserdyie::B0x0
    }
    #[doc = "LSE ready interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lserdyie::B0x1
    }
}
#[doc = "Field `LSERDYIE` writer - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
pub type LserdyieW<'a, REG> = crate::BitWriter<'a, REG, Lserdyie>;
impl<'a, REG> LserdyieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSE ready interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lserdyie::B0x0)
    }
    #[doc = "LSE ready interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lserdyie::B0x1)
    }
}
#[doc = "HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsirdyie {
    #[doc = "0: HSI16 ready interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: HSI16 ready interrupt enabled"]
    B0x1 = 1,
}
impl From<Hsirdyie> for bool {
    #[inline(always)]
    fn from(variant: Hsirdyie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDYIE` reader - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization."]
pub type HsirdyieR = crate::BitReader<Hsirdyie>;
impl HsirdyieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsirdyie {
        match self.bits {
            false => Hsirdyie::B0x0,
            true => Hsirdyie::B0x1,
        }
    }
    #[doc = "HSI16 ready interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hsirdyie::B0x0
    }
    #[doc = "HSI16 ready interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hsirdyie::B0x1
    }
}
#[doc = "Field `HSIRDYIE` writer - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization."]
pub type HsirdyieW<'a, REG> = crate::BitWriter<'a, REG, Hsirdyie>;
impl<'a, REG> HsirdyieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI16 ready interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hsirdyie::B0x0)
    }
    #[doc = "HSI16 ready interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hsirdyie::B0x1)
    }
}
#[doc = "HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hserdyie {
    #[doc = "0: HSE ready interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: HSE ready interrupt enabled"]
    B0x1 = 1,
}
impl From<Hserdyie> for bool {
    #[inline(always)]
    fn from(variant: Hserdyie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
pub type HserdyieR = crate::BitReader<Hserdyie>;
impl HserdyieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hserdyie {
        match self.bits {
            false => Hserdyie::B0x0,
            true => Hserdyie::B0x1,
        }
    }
    #[doc = "HSE ready interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hserdyie::B0x0
    }
    #[doc = "HSE ready interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hserdyie::B0x1
    }
}
#[doc = "Field `HSERDYIE` writer - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
pub type HserdyieW<'a, REG> = crate::BitWriter<'a, REG, Hserdyie>;
impl<'a, REG> HserdyieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSE ready interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hserdyie::B0x0)
    }
    #[doc = "HSE ready interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hserdyie::B0x1)
    }
}
#[doc = "PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL lock.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllrdyie {
    #[doc = "0: PLL lock interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: PLL lock interrupt enabled"]
    B0x1 = 1,
}
impl From<Pllrdyie> for bool {
    #[inline(always)]
    fn from(variant: Pllrdyie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLRDYIE` reader - PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL lock."]
pub type PllrdyieR = crate::BitReader<Pllrdyie>;
impl PllrdyieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllrdyie {
        match self.bits {
            false => Pllrdyie::B0x0,
            true => Pllrdyie::B0x1,
        }
    }
    #[doc = "PLL lock interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pllrdyie::B0x0
    }
    #[doc = "PLL lock interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllrdyie::B0x1
    }
}
#[doc = "Field `PLLRDYIE` writer - PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL lock."]
pub type PllrdyieW<'a, REG> = crate::BitWriter<'a, REG, Pllrdyie>;
impl<'a, REG> PllrdyieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL lock interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllrdyie::B0x0)
    }
    #[doc = "PLL lock interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllrdyie::B0x1)
    }
}
#[doc = "LSE clock security system interrupt enable Set and cleared by software to enable/disable interrupt caused by the clock security system on LSE.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsecssie {
    #[doc = "0: Clock security interrupt caused by LSE clock failure disabled"]
    B0x0 = 0,
    #[doc = "1: Clock security interrupt caused by LSE clock failure enabled"]
    B0x1 = 1,
}
impl From<Lsecssie> for bool {
    #[inline(always)]
    fn from(variant: Lsecssie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSIE` reader - LSE clock security system interrupt enable Set and cleared by software to enable/disable interrupt caused by the clock security system on LSE."]
pub type LsecssieR = crate::BitReader<Lsecssie>;
impl LsecssieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsecssie {
        match self.bits {
            false => Lsecssie::B0x0,
            true => Lsecssie::B0x1,
        }
    }
    #[doc = "Clock security interrupt caused by LSE clock failure disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lsecssie::B0x0
    }
    #[doc = "Clock security interrupt caused by LSE clock failure enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lsecssie::B0x1
    }
}
#[doc = "Field `LSECSSIE` writer - LSE clock security system interrupt enable Set and cleared by software to enable/disable interrupt caused by the clock security system on LSE."]
pub type LsecssieW<'a, REG> = crate::BitWriter<'a, REG, Lsecssie>;
impl<'a, REG> LsecssieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock security interrupt caused by LSE clock failure disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lsecssie::B0x0)
    }
    #[doc = "Clock security interrupt caused by LSE clock failure enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lsecssie::B0x1)
    }
}
#[doc = "HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the internal HSI48 oscillator.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsi48rdyie {
    #[doc = "0: HSI48 ready interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: HSI48 ready interrupt enabled"]
    B0x1 = 1,
}
impl From<Hsi48rdyie> for bool {
    #[inline(always)]
    fn from(variant: Hsi48rdyie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48RDYIE` reader - HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the internal HSI48 oscillator."]
pub type Hsi48rdyieR = crate::BitReader<Hsi48rdyie>;
impl Hsi48rdyieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsi48rdyie {
        match self.bits {
            false => Hsi48rdyie::B0x0,
            true => Hsi48rdyie::B0x1,
        }
    }
    #[doc = "HSI48 ready interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hsi48rdyie::B0x0
    }
    #[doc = "HSI48 ready interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hsi48rdyie::B0x1
    }
}
#[doc = "Field `HSI48RDYIE` writer - HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the internal HSI48 oscillator."]
pub type Hsi48rdyieW<'a, REG> = crate::BitWriter<'a, REG, Hsi48rdyie>;
impl<'a, REG> Hsi48rdyieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI48 ready interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hsi48rdyie::B0x0)
    }
    #[doc = "HSI48 ready interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hsi48rdyie::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LsirdyieR {
        LsirdyieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
    #[inline(always)]
    pub fn lserdyie(&self) -> LserdyieR {
        LserdyieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization."]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HsirdyieR {
        HsirdyieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
    #[inline(always)]
    pub fn hserdyie(&self) -> HserdyieR {
        HserdyieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL lock."]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PllrdyieR {
        PllrdyieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system interrupt enable Set and cleared by software to enable/disable interrupt caused by the clock security system on LSE."]
    #[inline(always)]
    pub fn lsecssie(&self) -> LsecssieR {
        LsecssieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the internal HSI48 oscillator."]
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> Hsi48rdyieR {
        Hsi48rdyieR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CIER")
            .field("lsirdyie", &self.lsirdyie())
            .field("lserdyie", &self.lserdyie())
            .field("hsirdyie", &self.hsirdyie())
            .field("hserdyie", &self.hserdyie())
            .field("pllrdyie", &self.pllrdyie())
            .field("lsecssie", &self.lsecssie())
            .field("hsi48rdyie", &self.hsi48rdyie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LsirdyieW<RccCierSpec> {
        LsirdyieW::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LserdyieW<RccCierSpec> {
        LserdyieW::new(self, 1)
    }
    #[doc = "Bit 3 - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization."]
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HsirdyieW<RccCierSpec> {
        HsirdyieW::new(self, 3)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HserdyieW<RccCierSpec> {
        HserdyieW::new(self, 4)
    }
    #[doc = "Bit 5 - PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL lock."]
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PllrdyieW<RccCierSpec> {
        PllrdyieW::new(self, 5)
    }
    #[doc = "Bit 9 - LSE clock security system interrupt enable Set and cleared by software to enable/disable interrupt caused by the clock security system on LSE."]
    #[inline(always)]
    pub fn lsecssie(&mut self) -> LsecssieW<RccCierSpec> {
        LsecssieW::new(self, 9)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the internal HSI48 oscillator."]
    #[inline(always)]
    pub fn hsi48rdyie(&mut self) -> Hsi48rdyieW<RccCierSpec> {
        Hsi48rdyieW::new(self, 10)
    }
}
#[doc = "Clock interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_cier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccCierSpec;
impl crate::RegisterSpec for RccCierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_cier::R`](R) reader structure"]
impl crate::Readable for RccCierSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_cier::W`](W) writer structure"]
impl crate::Writable for RccCierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCC_CIER to value 0"]
impl crate::Resettable for RccCierSpec {}

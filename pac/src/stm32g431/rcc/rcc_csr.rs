#[doc = "Register `RCC_CSR` reader"]
pub type R = crate::R<RccCsrSpec>;
#[doc = "Register `RCC_CSR` writer"]
pub type W = crate::W<RccCsrSpec>;
#[doc = "LSI oscillator enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsion {
    #[doc = "0: LSI oscillator OFF"]
    B0x0 = 0,
    #[doc = "1: LSI oscillator ON"]
    B0x1 = 1,
}
impl From<Lsion> for bool {
    #[inline(always)]
    fn from(variant: Lsion) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSION` reader - LSI oscillator enable Set and cleared by software."]
pub type LsionR = crate::BitReader<Lsion>;
impl LsionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsion {
        match self.bits {
            false => Lsion::B0x0,
            true => Lsion::B0x1,
        }
    }
    #[doc = "LSI oscillator OFF"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lsion::B0x0
    }
    #[doc = "LSI oscillator ON"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lsion::B0x1
    }
}
#[doc = "Field `LSION` writer - LSI oscillator enable Set and cleared by software."]
pub type LsionW<'a, REG> = crate::BitWriter<'a, REG, Lsion>;
impl<'a, REG> LsionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSI oscillator OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lsion::B0x0)
    }
    #[doc = "LSI oscillator ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lsion::B0x1)
    }
}
#[doc = "LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after 3 LSI oscillator clock cycles. This bit can be set even if LSION = 0 if the LSI is requested by the Clock Security System on LSE, by the Independent Watchdog or by the RTC.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsirdy {
    #[doc = "0: LSI oscillator not ready"]
    B0x0 = 0,
    #[doc = "1: LSI oscillator ready"]
    B0x1 = 1,
}
impl From<Lsirdy> for bool {
    #[inline(always)]
    fn from(variant: Lsirdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDY` reader - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after 3 LSI oscillator clock cycles. This bit can be set even if LSION = 0 if the LSI is requested by the Clock Security System on LSE, by the Independent Watchdog or by the RTC."]
pub type LsirdyR = crate::BitReader<Lsirdy>;
impl LsirdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsirdy {
        match self.bits {
            false => Lsirdy::B0x0,
            true => Lsirdy::B0x1,
        }
    }
    #[doc = "LSI oscillator not ready"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lsirdy::B0x0
    }
    #[doc = "LSI oscillator ready"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lsirdy::B0x1
    }
}
#[doc = "Remove reset flag Set by software to clear the reset flags.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rmvf {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Clear the reset flags"]
    B0x1 = 1,
}
impl From<Rmvf> for bool {
    #[inline(always)]
    fn from(variant: Rmvf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMVF` reader - Remove reset flag Set by software to clear the reset flags."]
pub type RmvfR = crate::BitReader<Rmvf>;
impl RmvfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rmvf {
        match self.bits {
            false => Rmvf::B0x0,
            true => Rmvf::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rmvf::B0x0
    }
    #[doc = "Clear the reset flags"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rmvf::B0x1
    }
}
#[doc = "Field `RMVF` writer - Remove reset flag Set by software to clear the reset flags."]
pub type RmvfW<'a, REG> = crate::BitWriter<'a, REG, Rmvf>;
impl<'a, REG> RmvfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rmvf::B0x0)
    }
    #[doc = "Clear the reset flags"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rmvf::B0x1)
    }
}
#[doc = "Option byte loader reset flag Set by hardware when a reset from the Option Byte loading occurs. Cleared by writing to the RMVF bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oblrstf {
    #[doc = "0: No reset from Option Byte loading occurred"]
    B0x0 = 0,
    #[doc = "1: Reset from Option Byte loading occurred"]
    B0x1 = 1,
}
impl From<Oblrstf> for bool {
    #[inline(always)]
    fn from(variant: Oblrstf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBLRSTF` reader - Option byte loader reset flag Set by hardware when a reset from the Option Byte loading occurs. Cleared by writing to the RMVF bit."]
pub type OblrstfR = crate::BitReader<Oblrstf>;
impl OblrstfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oblrstf {
        match self.bits {
            false => Oblrstf::B0x0,
            true => Oblrstf::B0x1,
        }
    }
    #[doc = "No reset from Option Byte loading occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Oblrstf::B0x0
    }
    #[doc = "Reset from Option Byte loading occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Oblrstf::B0x1
    }
}
#[doc = "Pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by writing to the RMVF bit.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pinrstf {
    #[doc = "0: No reset from NRST pin occurred"]
    B0x0 = 0,
    #[doc = "1: Reset from NRST pin occurred"]
    B0x1 = 1,
}
impl From<Pinrstf> for bool {
    #[inline(always)]
    fn from(variant: Pinrstf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINRSTF` reader - Pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by writing to the RMVF bit."]
pub type PinrstfR = crate::BitReader<Pinrstf>;
impl PinrstfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pinrstf {
        match self.bits {
            false => Pinrstf::B0x0,
            true => Pinrstf::B0x1,
        }
    }
    #[doc = "No reset from NRST pin occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pinrstf::B0x0
    }
    #[doc = "Reset from NRST pin occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pinrstf::B0x1
    }
}
#[doc = "BOR flag Set by hardware when a BOR occurs. Cleared by writing to the RMVF bit.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Borrstf {
    #[doc = "0: No BOR occurred"]
    B0x0 = 0,
    #[doc = "1: BOR occurred"]
    B0x1 = 1,
}
impl From<Borrstf> for bool {
    #[inline(always)]
    fn from(variant: Borrstf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BORRSTF` reader - BOR flag Set by hardware when a BOR occurs. Cleared by writing to the RMVF bit."]
pub type BorrstfR = crate::BitReader<Borrstf>;
impl BorrstfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Borrstf {
        match self.bits {
            false => Borrstf::B0x0,
            true => Borrstf::B0x1,
        }
    }
    #[doc = "No BOR occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Borrstf::B0x0
    }
    #[doc = "BOR occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Borrstf::B0x1
    }
}
#[doc = "Software reset flag Set by hardware when a software reset occurs. Cleared by writing to the RMVF bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sftrstf {
    #[doc = "0: No software reset occurred"]
    B0x0 = 0,
    #[doc = "1: Software reset occurred"]
    B0x1 = 1,
}
impl From<Sftrstf> for bool {
    #[inline(always)]
    fn from(variant: Sftrstf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFTRSTF` reader - Software reset flag Set by hardware when a software reset occurs. Cleared by writing to the RMVF bit."]
pub type SftrstfR = crate::BitReader<Sftrstf>;
impl SftrstfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sftrstf {
        match self.bits {
            false => Sftrstf::B0x0,
            true => Sftrstf::B0x1,
        }
    }
    #[doc = "No software reset occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sftrstf::B0x0
    }
    #[doc = "Software reset occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sftrstf::B0x1
    }
}
#[doc = "Independent window watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by writing to the RMVF bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iwdgrstf {
    #[doc = "0: No independent watchdog reset occurred"]
    B0x0 = 0,
    #[doc = "1: Independent watchdog reset occurred"]
    B0x1 = 1,
}
impl From<Iwdgrstf> for bool {
    #[inline(always)]
    fn from(variant: Iwdgrstf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDGRSTF` reader - Independent window watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by writing to the RMVF bit."]
pub type IwdgrstfR = crate::BitReader<Iwdgrstf>;
impl IwdgrstfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iwdgrstf {
        match self.bits {
            false => Iwdgrstf::B0x0,
            true => Iwdgrstf::B0x1,
        }
    }
    #[doc = "No independent watchdog reset occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Iwdgrstf::B0x0
    }
    #[doc = "Independent watchdog reset occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Iwdgrstf::B0x1
    }
}
#[doc = "Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by writing to the RMVF bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdgrstf {
    #[doc = "0: No window watchdog reset occurred"]
    B0x0 = 0,
    #[doc = "1: Window watchdog reset occurred"]
    B0x1 = 1,
}
impl From<Wwdgrstf> for bool {
    #[inline(always)]
    fn from(variant: Wwdgrstf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by writing to the RMVF bit."]
pub type WwdgrstfR = crate::BitReader<Wwdgrstf>;
impl WwdgrstfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wwdgrstf {
        match self.bits {
            false => Wwdgrstf::B0x0,
            true => Wwdgrstf::B0x1,
        }
    }
    #[doc = "No window watchdog reset occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wwdgrstf::B0x0
    }
    #[doc = "Window watchdog reset occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wwdgrstf::B0x1
    }
}
#[doc = "Low-power reset flag Set by hardware when a reset occurs due to illegal Stop, Standby or Shutdown mode entry. Cleared by writing to the RMVF bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpwrrstf {
    #[doc = "0: No illegal mode reset occurred"]
    B0x0 = 0,
    #[doc = "1: Illegal mode reset occurred"]
    B0x1 = 1,
}
impl From<Lpwrrstf> for bool {
    #[inline(always)]
    fn from(variant: Lpwrrstf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPWRRSTF` reader - Low-power reset flag Set by hardware when a reset occurs due to illegal Stop, Standby or Shutdown mode entry. Cleared by writing to the RMVF bit."]
pub type LpwrrstfR = crate::BitReader<Lpwrrstf>;
impl LpwrrstfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpwrrstf {
        match self.bits {
            false => Lpwrrstf::B0x0,
            true => Lpwrrstf::B0x1,
        }
    }
    #[doc = "No illegal mode reset occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lpwrrstf::B0x0
    }
    #[doc = "Illegal mode reset occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lpwrrstf::B0x1
    }
}
impl R {
    #[doc = "Bit 0 - LSI oscillator enable Set and cleared by software."]
    #[inline(always)]
    pub fn lsion(&self) -> LsionR {
        LsionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after 3 LSI oscillator clock cycles. This bit can be set even if LSION = 0 if the LSI is requested by the Clock Security System on LSE, by the Independent Watchdog or by the RTC."]
    #[inline(always)]
    pub fn lsirdy(&self) -> LsirdyR {
        LsirdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 23 - Remove reset flag Set by software to clear the reset flags."]
    #[inline(always)]
    pub fn rmvf(&self) -> RmvfR {
        RmvfR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Option byte loader reset flag Set by hardware when a reset from the Option Byte loading occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn oblrstf(&self) -> OblrstfR {
        OblrstfR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn pinrstf(&self) -> PinrstfR {
        PinrstfR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - BOR flag Set by hardware when a BOR occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn borrstf(&self) -> BorrstfR {
        BorrstfR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag Set by hardware when a software reset occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn sftrstf(&self) -> SftrstfR {
        SftrstfR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Independent window watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IwdgrstfR {
        IwdgrstfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WwdgrstfR {
        WwdgrstfR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag Set by hardware when a reset occurs due to illegal Stop, Standby or Shutdown mode entry. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LpwrrstfR {
        LpwrrstfR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CSR")
            .field("lsion", &self.lsion())
            .field("lsirdy", &self.lsirdy())
            .field("rmvf", &self.rmvf())
            .field("oblrstf", &self.oblrstf())
            .field("pinrstf", &self.pinrstf())
            .field("borrstf", &self.borrstf())
            .field("sftrstf", &self.sftrstf())
            .field("iwdgrstf", &self.iwdgrstf())
            .field("wwdgrstf", &self.wwdgrstf())
            .field("lpwrrstf", &self.lpwrrstf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - LSI oscillator enable Set and cleared by software."]
    #[inline(always)]
    pub fn lsion(&mut self) -> LsionW<RccCsrSpec> {
        LsionW::new(self, 0)
    }
    #[doc = "Bit 23 - Remove reset flag Set by software to clear the reset flags."]
    #[inline(always)]
    pub fn rmvf(&mut self) -> RmvfW<RccCsrSpec> {
        RmvfW::new(self, 23)
    }
}
#[doc = "Control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccCsrSpec;
impl crate::RegisterSpec for RccCsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_csr::R`](R) reader structure"]
impl crate::Readable for RccCsrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_csr::W`](W) writer structure"]
impl crate::Writable for RccCsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_CSR to value 0x0c00_0000"]
impl crate::Resettable for RccCsrSpec {
    const RESET_VALUE: u32 = 0x0c00_0000;
}

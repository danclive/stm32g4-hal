#[doc = "Register `PWR_CR1` reader"]
pub type R = crate::R<PwrCr1Spec>;
#[doc = "Register `PWR_CR1` writer"]
pub type W = crate::W<PwrCr1Spec>;
#[doc = "Low-power mode selection These bits select the low-power mode entered when CPU enters the deepsleep mode. 1xx: Shutdown mode Note: In Standby mode, SRAM2 can be preserved or not, depending on RRS bit configuration in PWR_CR3.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpms {
    #[doc = "0: Stop 0 mode"]
    B0x0 = 0,
    #[doc = "1: Stop 1 mode"]
    B0x1 = 1,
    #[doc = "3: Standby mode"]
    B0x3 = 3,
}
impl From<Lpms> for u8 {
    #[inline(always)]
    fn from(variant: Lpms) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpms {
    type Ux = u8;
}
impl crate::IsEnum for Lpms {}
#[doc = "Field `LPMS` reader - Low-power mode selection These bits select the low-power mode entered when CPU enters the deepsleep mode. 1xx: Shutdown mode Note: In Standby mode, SRAM2 can be preserved or not, depending on RRS bit configuration in PWR_CR3."]
pub type LpmsR = crate::FieldReader<Lpms>;
impl LpmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lpms> {
        match self.bits {
            0 => Some(Lpms::B0x0),
            1 => Some(Lpms::B0x1),
            3 => Some(Lpms::B0x3),
            _ => None,
        }
    }
    #[doc = "Stop 0 mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lpms::B0x0
    }
    #[doc = "Stop 1 mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lpms::B0x1
    }
    #[doc = "Standby mode"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Lpms::B0x3
    }
}
#[doc = "Field `LPMS` writer - Low-power mode selection These bits select the low-power mode entered when CPU enters the deepsleep mode. 1xx: Shutdown mode Note: In Standby mode, SRAM2 can be preserved or not, depending on RRS bit configuration in PWR_CR3."]
pub type LpmsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lpms>;
impl<'a, REG> LpmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Stop 0 mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpms::B0x0)
    }
    #[doc = "Stop 1 mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpms::B0x1)
    }
    #[doc = "Standby mode"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Lpms::B0x3)
    }
}
#[doc = "Field `FPD_STOP` reader - FPD_STOP"]
pub type FpdStopR = crate::BitReader;
#[doc = "Field `FPD_STOP` writer - FPD_STOP"]
pub type FpdStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Disable backup domain write protection In reset state, the RTC and backup registers are protected against parasitic write access. This bit must be set to enable write access to these registers.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbp {
    #[doc = "0: Access to RTC and Backup registers disabled"]
    B0x0 = 0,
    #[doc = "1: Access to RTC and Backup registers enabled"]
    B0x1 = 1,
}
impl From<Dbp> for bool {
    #[inline(always)]
    fn from(variant: Dbp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBP` reader - Disable backup domain write protection In reset state, the RTC and backup registers are protected against parasitic write access. This bit must be set to enable write access to these registers."]
pub type DbpR = crate::BitReader<Dbp>;
impl DbpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbp {
        match self.bits {
            false => Dbp::B0x0,
            true => Dbp::B0x1,
        }
    }
    #[doc = "Access to RTC and Backup registers disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dbp::B0x0
    }
    #[doc = "Access to RTC and Backup registers enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dbp::B0x1
    }
}
#[doc = "Field `DBP` writer - Disable backup domain write protection In reset state, the RTC and backup registers are protected against parasitic write access. This bit must be set to enable write access to these registers."]
pub type DbpW<'a, REG> = crate::BitWriter<'a, REG, Dbp>;
impl<'a, REG> DbpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Access to RTC and Backup registers disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dbp::B0x0)
    }
    #[doc = "Access to RTC and Backup registers enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dbp::B0x1)
    }
}
#[doc = "Voltage scaling range selection\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vos {
    #[doc = "0: Cannot be written (forbidden by hardware)"]
    B0x0 = 0,
    #[doc = "1: Range 1"]
    B0x1 = 1,
    #[doc = "2: Range 2"]
    B0x2 = 2,
    #[doc = "3: Cannot be written (forbidden by hardware)"]
    B0x3 = 3,
}
impl From<Vos> for u8 {
    #[inline(always)]
    fn from(variant: Vos) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vos {
    type Ux = u8;
}
impl crate::IsEnum for Vos {}
#[doc = "Field `VOS` reader - Voltage scaling range selection"]
pub type VosR = crate::FieldReader<Vos>;
impl VosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vos {
        match self.bits {
            0 => Vos::B0x0,
            1 => Vos::B0x1,
            2 => Vos::B0x2,
            3 => Vos::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Cannot be written (forbidden by hardware)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Vos::B0x0
    }
    #[doc = "Range 1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Vos::B0x1
    }
    #[doc = "Range 2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Vos::B0x2
    }
    #[doc = "Cannot be written (forbidden by hardware)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Vos::B0x3
    }
}
#[doc = "Field `VOS` writer - Voltage scaling range selection"]
pub type VosW<'a, REG> = crate::FieldWriter<'a, REG, 2, Vos, crate::Safe>;
impl<'a, REG> VosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Cannot be written (forbidden by hardware)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Vos::B0x0)
    }
    #[doc = "Range 1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Vos::B0x1)
    }
    #[doc = "Range 2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Vos::B0x2)
    }
    #[doc = "Cannot be written (forbidden by hardware)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Vos::B0x3)
    }
}
#[doc = "Field `LPR` reader - Low-power run When this bit is set, the regulator is switched from main mode (MR) to low-power mode (LPR)."]
pub type LprR = crate::BitReader;
#[doc = "Field `LPR` writer - Low-power run When this bit is set, the regulator is switched from main mode (MR) to low-power mode (LPR)."]
pub type LprW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Low-power mode selection These bits select the low-power mode entered when CPU enters the deepsleep mode. 1xx: Shutdown mode Note: In Standby mode, SRAM2 can be preserved or not, depending on RRS bit configuration in PWR_CR3."]
    #[inline(always)]
    pub fn lpms(&self) -> LpmsR {
        LpmsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - FPD_STOP"]
    #[inline(always)]
    pub fn fpd_stop(&self) -> FpdStopR {
        FpdStopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable backup domain write protection In reset state, the RTC and backup registers are protected against parasitic write access. This bit must be set to enable write access to these registers."]
    #[inline(always)]
    pub fn dbp(&self) -> DbpR {
        DbpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Voltage scaling range selection"]
    #[inline(always)]
    pub fn vos(&self) -> VosR {
        VosR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 14 - Low-power run When this bit is set, the regulator is switched from main mode (MR) to low-power mode (LPR)."]
    #[inline(always)]
    pub fn lpr(&self) -> LprR {
        LprR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_CR1")
            .field("lpms", &self.lpms())
            .field("fpd_stop", &self.fpd_stop())
            .field("dbp", &self.dbp())
            .field("vos", &self.vos())
            .field("lpr", &self.lpr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Low-power mode selection These bits select the low-power mode entered when CPU enters the deepsleep mode. 1xx: Shutdown mode Note: In Standby mode, SRAM2 can be preserved or not, depending on RRS bit configuration in PWR_CR3."]
    #[inline(always)]
    pub fn lpms(&mut self) -> LpmsW<'_, PwrCr1Spec> {
        LpmsW::new(self, 0)
    }
    #[doc = "Bit 3 - FPD_STOP"]
    #[inline(always)]
    pub fn fpd_stop(&mut self) -> FpdStopW<'_, PwrCr1Spec> {
        FpdStopW::new(self, 3)
    }
    #[doc = "Bit 8 - Disable backup domain write protection In reset state, the RTC and backup registers are protected against parasitic write access. This bit must be set to enable write access to these registers."]
    #[inline(always)]
    pub fn dbp(&mut self) -> DbpW<'_, PwrCr1Spec> {
        DbpW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Voltage scaling range selection"]
    #[inline(always)]
    pub fn vos(&mut self) -> VosW<'_, PwrCr1Spec> {
        VosW::new(self, 9)
    }
    #[doc = "Bit 14 - Low-power run When this bit is set, the regulator is switched from main mode (MR) to low-power mode (LPR)."]
    #[inline(always)]
    pub fn lpr(&mut self) -> LprW<'_, PwrCr1Spec> {
        LprW::new(self, 14)
    }
}
#[doc = "Power control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrCr1Spec;
impl crate::RegisterSpec for PwrCr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_cr1::R`](R) reader structure"]
impl crate::Readable for PwrCr1Spec {}
#[doc = "`write(|w| ..)` method takes [`pwr_cr1::W`](W) writer structure"]
impl crate::Writable for PwrCr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWR_CR1 to value 0x0200"]
impl crate::Resettable for PwrCr1Spec {
    const RESET_VALUE: u32 = 0x0200;
}

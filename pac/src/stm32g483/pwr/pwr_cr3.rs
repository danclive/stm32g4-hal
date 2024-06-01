#[doc = "Register `PWR_CR3` reader"]
pub type R = crate::R<PwrCr3Spec>;
#[doc = "Register `PWR_CR3` writer"]
pub type W = crate::W<PwrCr3Spec>;
#[doc = "Field `EWUP1` reader - Enable Wakeup pin WKUP1 When this bit is set, the external wakeup pin WKUP1 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register."]
pub type Ewup1R = crate::BitReader;
#[doc = "Field `EWUP1` writer - Enable Wakeup pin WKUP1 When this bit is set, the external wakeup pin WKUP1 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register."]
pub type Ewup1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP2` reader - Enable Wakeup pin WKUP2 When this bit is set, the external wakeup pin WKUP2 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register."]
pub type Ewup2R = crate::BitReader;
#[doc = "Field `EWUP2` writer - Enable Wakeup pin WKUP2 When this bit is set, the external wakeup pin WKUP2 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register."]
pub type Ewup2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP3` reader - Enable Wakeup pin WKUP3 When this bit is set, the external wakeup pin WKUP3 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register."]
pub type Ewup3R = crate::BitReader;
#[doc = "Field `EWUP3` writer - Enable Wakeup pin WKUP3 When this bit is set, the external wakeup pin WKUP3 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register."]
pub type Ewup3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP4` reader - Enable Wakeup pin WKUP4 When this bit is set, the external wakeup pin WKUP4 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register."]
pub type Ewup4R = crate::BitReader;
#[doc = "Field `EWUP4` writer - Enable Wakeup pin WKUP4 When this bit is set, the external wakeup pin WKUP4 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register."]
pub type Ewup4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP5` reader - Enable Wakeup pin WKUP5 When this bit is set, the external wakeup pin WKUP5 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register."]
pub type Ewup5R = crate::BitReader;
#[doc = "Field `EWUP5` writer - Enable Wakeup pin WKUP5 When this bit is set, the external wakeup pin WKUP5 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register."]
pub type Ewup5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SRAM2 retention in Standby mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rrs {
    #[doc = "0: SRAM2 is powered off in Standby mode (SRAM2 content is lost)."]
    B0x0 = 0,
    #[doc = "1: SRAM2 is powered by the low-power regulator in Standby mode (SRAM2 content is kept)."]
    B0x1 = 1,
}
impl From<Rrs> for bool {
    #[inline(always)]
    fn from(variant: Rrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRS` reader - SRAM2 retention in Standby mode"]
pub type RrsR = crate::BitReader<Rrs>;
impl RrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rrs {
        match self.bits {
            false => Rrs::B0x0,
            true => Rrs::B0x1,
        }
    }
    #[doc = "SRAM2 is powered off in Standby mode (SRAM2 content is lost)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rrs::B0x0
    }
    #[doc = "SRAM2 is powered by the low-power regulator in Standby mode (SRAM2 content is kept)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rrs::B0x1
    }
}
#[doc = "Field `RRS` writer - SRAM2 retention in Standby mode"]
pub type RrsW<'a, REG> = crate::BitWriter<'a, REG, Rrs>;
impl<'a, REG> RrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM2 is powered off in Standby mode (SRAM2 content is lost)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rrs::B0x0)
    }
    #[doc = "SRAM2 is powered by the low-power regulator in Standby mode (SRAM2 content is kept)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rrs::B0x1)
    }
}
#[doc = "Field `APC` reader - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os."]
pub type ApcR = crate::BitReader;
#[doc = "Field `APC` writer - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os."]
pub type ApcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "UCPD1_STDBY USB Type-C and Power Delivery standby mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucpd1Stdby {
    #[doc = "0: Write 0 immediately after standby exit when using UCPD1, (and before writing any UCPD1 registers)."]
    B0x0 = 0,
    #[doc = "1: Write 1 just before entering standby when using UCPD1."]
    B0x1 = 1,
}
impl From<Ucpd1Stdby> for bool {
    #[inline(always)]
    fn from(variant: Ucpd1Stdby) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCPD1_STDBY` reader - UCPD1_STDBY USB Type-C and Power Delivery standby mode."]
pub type Ucpd1StdbyR = crate::BitReader<Ucpd1Stdby>;
impl Ucpd1StdbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucpd1Stdby {
        match self.bits {
            false => Ucpd1Stdby::B0x0,
            true => Ucpd1Stdby::B0x1,
        }
    }
    #[doc = "Write 0 immediately after standby exit when using UCPD1, (and before writing any UCPD1 registers)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ucpd1Stdby::B0x0
    }
    #[doc = "Write 1 just before entering standby when using UCPD1."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ucpd1Stdby::B0x1
    }
}
#[doc = "Field `UCPD1_STDBY` writer - UCPD1_STDBY USB Type-C and Power Delivery standby mode."]
pub type Ucpd1StdbyW<'a, REG> = crate::BitWriter<'a, REG, Ucpd1Stdby>;
impl<'a, REG> Ucpd1StdbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write 0 immediately after standby exit when using UCPD1, (and before writing any UCPD1 registers)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucpd1Stdby::B0x0)
    }
    #[doc = "Write 1 just before entering standby when using UCPD1."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucpd1Stdby::B0x1)
    }
}
#[doc = "USB Type-C and Power Delivery Dead Battery disable. After exiting reset, the USB Type-C dead battery behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to hand over control to the UCPD1 (which should therefore be initialized before doing the disable).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucpd1Dbdis {
    #[doc = "0: Enable USB Type-C dead battery pull-down behavior on UCPD1_CC1 and UCPD1_CC2 pins."]
    B0x0 = 0,
    #[doc = "1: Disable USB Type-C dead battery pull-down behavior on UCPD1_CC1 and UCPD1_CC2 pins."]
    B0x1 = 1,
}
impl From<Ucpd1Dbdis> for bool {
    #[inline(always)]
    fn from(variant: Ucpd1Dbdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCPD1_DBDIS` reader - USB Type-C and Power Delivery Dead Battery disable. After exiting reset, the USB Type-C dead battery behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to hand over control to the UCPD1 (which should therefore be initialized before doing the disable)."]
pub type Ucpd1DbdisR = crate::BitReader<Ucpd1Dbdis>;
impl Ucpd1DbdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucpd1Dbdis {
        match self.bits {
            false => Ucpd1Dbdis::B0x0,
            true => Ucpd1Dbdis::B0x1,
        }
    }
    #[doc = "Enable USB Type-C dead battery pull-down behavior on UCPD1_CC1 and UCPD1_CC2 pins."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ucpd1Dbdis::B0x0
    }
    #[doc = "Disable USB Type-C dead battery pull-down behavior on UCPD1_CC1 and UCPD1_CC2 pins."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ucpd1Dbdis::B0x1
    }
}
#[doc = "Field `UCPD1_DBDIS` writer - USB Type-C and Power Delivery Dead Battery disable. After exiting reset, the USB Type-C dead battery behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to hand over control to the UCPD1 (which should therefore be initialized before doing the disable)."]
pub type Ucpd1DbdisW<'a, REG> = crate::BitWriter<'a, REG, Ucpd1Dbdis>;
impl<'a, REG> Ucpd1DbdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable USB Type-C dead battery pull-down behavior on UCPD1_CC1 and UCPD1_CC2 pins."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucpd1Dbdis::B0x0)
    }
    #[doc = "Disable USB Type-C dead battery pull-down behavior on UCPD1_CC1 and UCPD1_CC2 pins."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucpd1Dbdis::B0x1)
    }
}
#[doc = "Enable internal wakeup line\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eiwul {
    #[doc = "0: Internal wakeup line disable."]
    B0x0 = 0,
    #[doc = "1: Internal wakeup line enable."]
    B0x1 = 1,
}
impl From<Eiwul> for bool {
    #[inline(always)]
    fn from(variant: Eiwul) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EIWUL` reader - Enable internal wakeup line"]
pub type EiwulR = crate::BitReader<Eiwul>;
impl EiwulR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eiwul {
        match self.bits {
            false => Eiwul::B0x0,
            true => Eiwul::B0x1,
        }
    }
    #[doc = "Internal wakeup line disable."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eiwul::B0x0
    }
    #[doc = "Internal wakeup line enable."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eiwul::B0x1
    }
}
#[doc = "Field `EIWUL` writer - Enable internal wakeup line"]
pub type EiwulW<'a, REG> = crate::BitWriter<'a, REG, Eiwul>;
impl<'a, REG> EiwulW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal wakeup line disable."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Eiwul::B0x0)
    }
    #[doc = "Internal wakeup line enable."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Eiwul::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1 When this bit is set, the external wakeup pin WKUP1 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn ewup1(&self) -> Ewup1R {
        Ewup1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2 When this bit is set, the external wakeup pin WKUP2 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn ewup2(&self) -> Ewup2R {
        Ewup2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3 When this bit is set, the external wakeup pin WKUP3 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn ewup3(&self) -> Ewup3R {
        Ewup3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Wakeup pin WKUP4 When this bit is set, the external wakeup pin WKUP4 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn ewup4(&self) -> Ewup4R {
        Ewup4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Wakeup pin WKUP5 When this bit is set, the external wakeup pin WKUP5 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn ewup5(&self) -> Ewup5R {
        Ewup5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM2 retention in Standby mode"]
    #[inline(always)]
    pub fn rrs(&self) -> RrsR {
        RrsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os."]
    #[inline(always)]
    pub fn apc(&self) -> ApcR {
        ApcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - UCPD1_STDBY USB Type-C and Power Delivery standby mode."]
    #[inline(always)]
    pub fn ucpd1_stdby(&self) -> Ucpd1StdbyR {
        Ucpd1StdbyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USB Type-C and Power Delivery Dead Battery disable. After exiting reset, the USB Type-C dead battery behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to hand over control to the UCPD1 (which should therefore be initialized before doing the disable)."]
    #[inline(always)]
    pub fn ucpd1_dbdis(&self) -> Ucpd1DbdisR {
        Ucpd1DbdisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable internal wakeup line"]
    #[inline(always)]
    pub fn eiwul(&self) -> EiwulR {
        EiwulR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_CR3")
            .field("ewup1", &self.ewup1())
            .field("ewup2", &self.ewup2())
            .field("ewup3", &self.ewup3())
            .field("ewup4", &self.ewup4())
            .field("ewup5", &self.ewup5())
            .field("rrs", &self.rrs())
            .field("apc", &self.apc())
            .field("ucpd1_stdby", &self.ucpd1_stdby())
            .field("ucpd1_dbdis", &self.ucpd1_dbdis())
            .field("eiwul", &self.eiwul())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1 When this bit is set, the external wakeup pin WKUP1 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn ewup1(&mut self) -> Ewup1W<PwrCr3Spec> {
        Ewup1W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2 When this bit is set, the external wakeup pin WKUP2 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn ewup2(&mut self) -> Ewup2W<PwrCr3Spec> {
        Ewup2W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3 When this bit is set, the external wakeup pin WKUP3 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn ewup3(&mut self) -> Ewup3W<PwrCr3Spec> {
        Ewup3W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Wakeup pin WKUP4 When this bit is set, the external wakeup pin WKUP4 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn ewup4(&mut self) -> Ewup4W<PwrCr3Spec> {
        Ewup4W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Wakeup pin WKUP5 When this bit is set, the external wakeup pin WKUP5 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn ewup5(&mut self) -> Ewup5W<PwrCr3Spec> {
        Ewup5W::new(self, 4)
    }
    #[doc = "Bit 8 - SRAM2 retention in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn rrs(&mut self) -> RrsW<PwrCr3Spec> {
        RrsW::new(self, 8)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os."]
    #[inline(always)]
    #[must_use]
    pub fn apc(&mut self) -> ApcW<PwrCr3Spec> {
        ApcW::new(self, 10)
    }
    #[doc = "Bit 13 - UCPD1_STDBY USB Type-C and Power Delivery standby mode."]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1_stdby(&mut self) -> Ucpd1StdbyW<PwrCr3Spec> {
        Ucpd1StdbyW::new(self, 13)
    }
    #[doc = "Bit 14 - USB Type-C and Power Delivery Dead Battery disable. After exiting reset, the USB Type-C dead battery behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to hand over control to the UCPD1 (which should therefore be initialized before doing the disable)."]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1_dbdis(&mut self) -> Ucpd1DbdisW<PwrCr3Spec> {
        Ucpd1DbdisW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable internal wakeup line"]
    #[inline(always)]
    #[must_use]
    pub fn eiwul(&mut self) -> EiwulW<PwrCr3Spec> {
        EiwulW::new(self, 15)
    }
}
#[doc = "Power control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_cr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_cr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrCr3Spec;
impl crate::RegisterSpec for PwrCr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_cr3::R`](R) reader structure"]
impl crate::Readable for PwrCr3Spec {}
#[doc = "`write(|w| ..)` method takes [`pwr_cr3::W`](W) writer structure"]
impl crate::Writable for PwrCr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_CR3 to value 0x8000"]
impl crate::Resettable for PwrCr3Spec {
    const RESET_VALUE: u32 = 0x8000;
}

#[doc = "Register `PWR_SR1` reader"]
pub type R = crate::R<PwrSr1Spec>;
#[doc = "Field `WUF1` reader - Wakeup flag 1 This bit is set when a wakeup event is detected on wakeup pin, WKUP1. It is cleared by writing 1 in the CWUF1 bit of the PWR_SCR register."]
pub type Wuf1R = crate::BitReader;
#[doc = "Field `WUF2` reader - Wakeup flag 2 This bit is set when a wakeup event is detected on wakeup pin, WKUP2. It is cleared by writing 1 in the CWUF2 bit of the PWR_SCR register."]
pub type Wuf2R = crate::BitReader;
#[doc = "Field `WUF3` reader - Wakeup flag 3 This bit is set when a wakeup event is detected on wakeup pin, WKUP3. It is cleared by writing 1 in the CWUF3 bit of the PWR_SCR register."]
pub type Wuf3R = crate::BitReader;
#[doc = "Field `WUF4` reader - Wakeup flag 4 This bit is set when a wakeup event is detected on wakeup pin,WKUP4. It is cleared by writing 1 in the CWUF4 bit of the PWR_SCR register."]
pub type Wuf4R = crate::BitReader;
#[doc = "Field `WUF5` reader - Wakeup flag 5 This bit is set when a wakeup event is detected on wakeup pin, WKUP5. It is cleared by writing 1 in the CWUF5 bit of the PWR_SCR register."]
pub type Wuf5R = crate::BitReader;
#[doc = "Standby flag This bit is set by hardware when the device enters the Standby mode and is cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbf {
    #[doc = "0: The device did not enter the Standby mode"]
    B0x0 = 0,
    #[doc = "1: The device entered the Standby mode"]
    B0x1 = 1,
}
impl From<Sbf> for bool {
    #[inline(always)]
    fn from(variant: Sbf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBF` reader - Standby flag This bit is set by hardware when the device enters the Standby mode and is cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset."]
pub type SbfR = crate::BitReader<Sbf>;
impl SbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbf {
        match self.bits {
            false => Sbf::B0x0,
            true => Sbf::B0x1,
        }
    }
    #[doc = "The device did not enter the Standby mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sbf::B0x0
    }
    #[doc = "The device entered the Standby mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sbf::B0x1
    }
}
#[doc = "Field `WUFI` reader - Wakeup flag internal This bit is set when a wakeup is detected on the internal wakeup line. It is cleared when all internal wakeup sources are cleared."]
pub type WufiR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Wakeup flag 1 This bit is set when a wakeup event is detected on wakeup pin, WKUP1. It is cleared by writing 1 in the CWUF1 bit of the PWR_SCR register."]
    #[inline(always)]
    pub fn wuf1(&self) -> Wuf1R {
        Wuf1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup flag 2 This bit is set when a wakeup event is detected on wakeup pin, WKUP2. It is cleared by writing 1 in the CWUF2 bit of the PWR_SCR register."]
    #[inline(always)]
    pub fn wuf2(&self) -> Wuf2R {
        Wuf2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup flag 3 This bit is set when a wakeup event is detected on wakeup pin, WKUP3. It is cleared by writing 1 in the CWUF3 bit of the PWR_SCR register."]
    #[inline(always)]
    pub fn wuf3(&self) -> Wuf3R {
        Wuf3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup flag 4 This bit is set when a wakeup event is detected on wakeup pin,WKUP4. It is cleared by writing 1 in the CWUF4 bit of the PWR_SCR register."]
    #[inline(always)]
    pub fn wuf4(&self) -> Wuf4R {
        Wuf4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup flag 5 This bit is set when a wakeup event is detected on wakeup pin, WKUP5. It is cleared by writing 1 in the CWUF5 bit of the PWR_SCR register."]
    #[inline(always)]
    pub fn wuf5(&self) -> Wuf5R {
        Wuf5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Standby flag This bit is set by hardware when the device enters the Standby mode and is cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset."]
    #[inline(always)]
    pub fn sbf(&self) -> SbfR {
        SbfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - Wakeup flag internal This bit is set when a wakeup is detected on the internal wakeup line. It is cleared when all internal wakeup sources are cleared."]
    #[inline(always)]
    pub fn wufi(&self) -> WufiR {
        WufiR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_SR1")
            .field("wuf1", &self.wuf1())
            .field("wuf2", &self.wuf2())
            .field("wuf3", &self.wuf3())
            .field("wuf4", &self.wuf4())
            .field("wuf5", &self.wuf5())
            .field("sbf", &self.sbf())
            .field("wufi", &self.wufi())
            .finish()
    }
}
#[doc = "Power status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_sr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrSr1Spec;
impl crate::RegisterSpec for PwrSr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_sr1::R`](R) reader structure"]
impl crate::Readable for PwrSr1Spec {}
#[doc = "`reset()` method sets PWR_SR1 to value 0"]
impl crate::Resettable for PwrSr1Spec {
    const RESET_VALUE: u32 = 0;
}

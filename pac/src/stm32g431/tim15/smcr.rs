#[doc = "Register `SMCR` reader"]
pub type R = crate::R<SmcrSpec>;
#[doc = "Register `SMCR` writer"]
pub type W = crate::W<SmcrSpec>;
#[doc = "Field `SMS` reader - Slave mode selection"]
pub type SmsR = crate::FieldReader;
#[doc = "Field `SMS` writer - Slave mode selection"]
pub type SmsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TS` reader - Trigger selection"]
pub type TsR = crate::FieldReader;
#[doc = "Field `TS` writer - Trigger selection"]
pub type TsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSM` reader - Master/Slave mode"]
pub type MsmR = crate::BitReader;
#[doc = "Field `MSM` writer - Master/Slave mode"]
pub type MsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMS_3` reader - Slave mode selection - bit 3"]
pub type Sms3R = crate::BitReader;
#[doc = "Field `SMS_3` writer - Slave mode selection - bit 3"]
pub type Sms3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_4_3` reader - Trigger selection - bit 4:3"]
pub type Ts4_3R = crate::FieldReader;
#[doc = "Field `TS_4_3` writer - Trigger selection - bit 4:3"]
pub type Ts4_3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn sms(&self) -> SmsR {
        SmsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MsmR {
        MsmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave mode selection - bit 3"]
    #[inline(always)]
    pub fn sms_3(&self) -> Sms3R {
        Sms3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Trigger selection - bit 4:3"]
    #[inline(always)]
    pub fn ts_4_3(&self) -> Ts4_3R {
        Ts4_3R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMCR")
            .field("ts_4_3", &self.ts_4_3())
            .field("sms_3", &self.sms_3())
            .field("msm", &self.msm())
            .field("ts", &self.ts())
            .field("sms", &self.sms())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn sms(&mut self) -> SmsW<SmcrSpec> {
        SmsW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TsW<SmcrSpec> {
        TsW::new(self, 4)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MsmW<SmcrSpec> {
        MsmW::new(self, 7)
    }
    #[doc = "Bit 16 - Slave mode selection - bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn sms_3(&mut self) -> Sms3W<SmcrSpec> {
        Sms3W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Trigger selection - bit 4:3"]
    #[inline(always)]
    #[must_use]
    pub fn ts_4_3(&mut self) -> Ts4_3W<SmcrSpec> {
        Ts4_3W::new(self, 20)
    }
}
#[doc = "slave mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmcrSpec;
impl crate::RegisterSpec for SmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smcr::R`](R) reader structure"]
impl crate::Readable for SmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`smcr::W`](W) writer structure"]
impl crate::Writable for SmcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMCR to value 0"]
impl crate::Resettable for SmcrSpec {
    const RESET_VALUE: u32 = 0;
}

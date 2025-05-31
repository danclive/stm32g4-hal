#[doc = "Register `SMCR` reader"]
pub type R = crate::R<SmcrSpec>;
#[doc = "Register `SMCR` writer"]
pub type W = crate::W<SmcrSpec>;
#[doc = "Field `SMS` reader - Slave mode selection"]
pub type SmsR = crate::FieldReader;
#[doc = "Field `SMS` writer - Slave mode selection"]
pub type SmsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OCCS` reader - OCREF clear selection"]
pub type OccsR = crate::BitReader;
#[doc = "Field `OCCS` writer - OCREF clear selection"]
pub type OccsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS` reader - Trigger selection"]
pub type TsR = crate::FieldReader;
#[doc = "Field `TS` writer - Trigger selection"]
pub type TsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSM` reader - Master/Slave mode"]
pub type MsmR = crate::BitReader;
#[doc = "Field `MSM` writer - Master/Slave mode"]
pub type MsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETF` reader - External trigger filter"]
pub type EtfR = crate::FieldReader;
#[doc = "Field `ETF` writer - External trigger filter"]
pub type EtfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ETPS` reader - External trigger prescaler"]
pub type EtpsR = crate::FieldReader;
#[doc = "Field `ETPS` writer - External trigger prescaler"]
pub type EtpsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECE` reader - External clock enable"]
pub type EceR = crate::BitReader;
#[doc = "Field `ECE` writer - External clock enable"]
pub type EceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETP` reader - External trigger polarity"]
pub type EtpR = crate::BitReader;
#[doc = "Field `ETP` writer - External trigger polarity"]
pub type EtpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMS_3` reader - Slave mode selection - bit 3"]
pub type Sms3R = crate::BitReader;
#[doc = "Field `SMS_3` writer - Slave mode selection - bit 3"]
pub type Sms3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_4_3` reader - Trigger selection - bit 4:3"]
pub type Ts4_3R = crate::FieldReader;
#[doc = "Field `TS_4_3` writer - Trigger selection - bit 4:3"]
pub type Ts4_3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SMSPE` reader - SMS Preload Enable"]
pub type SmspeR = crate::BitReader;
#[doc = "Field `SMSPE` writer - SMS Preload Enable"]
pub type SmspeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMSPS` reader - SMS Preload Source"]
pub type SmspsR = crate::BitReader;
#[doc = "Field `SMSPS` writer - SMS Preload Source"]
pub type SmspsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn sms(&self) -> SmsR {
        SmsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - OCREF clear selection"]
    #[inline(always)]
    pub fn occs(&self) -> OccsR {
        OccsR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    pub fn etf(&self) -> EtfR {
        EtfR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etps(&self) -> EtpsR {
        EtpsR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline(always)]
    pub fn ece(&self) -> EceR {
        EceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etp(&self) -> EtpR {
        EtpR::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 24 - SMS Preload Enable"]
    #[inline(always)]
    pub fn smspe(&self) -> SmspeR {
        SmspeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SMS Preload Source"]
    #[inline(always)]
    pub fn smsps(&self) -> SmspsR {
        SmspsR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMCR")
            .field("smsps", &self.smsps())
            .field("smspe", &self.smspe())
            .field("ts_4_3", &self.ts_4_3())
            .field("sms_3", &self.sms_3())
            .field("etp", &self.etp())
            .field("ece", &self.ece())
            .field("etps", &self.etps())
            .field("etf", &self.etf())
            .field("msm", &self.msm())
            .field("ts", &self.ts())
            .field("occs", &self.occs())
            .field("sms", &self.sms())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn sms(&mut self) -> SmsW<SmcrSpec> {
        SmsW::new(self, 0)
    }
    #[doc = "Bit 3 - OCREF clear selection"]
    #[inline(always)]
    pub fn occs(&mut self) -> OccsW<SmcrSpec> {
        OccsW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn ts(&mut self) -> TsW<SmcrSpec> {
        TsW::new(self, 4)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&mut self) -> MsmW<SmcrSpec> {
        MsmW::new(self, 7)
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    pub fn etf(&mut self) -> EtfW<SmcrSpec> {
        EtfW::new(self, 8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etps(&mut self) -> EtpsW<SmcrSpec> {
        EtpsW::new(self, 12)
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline(always)]
    pub fn ece(&mut self) -> EceW<SmcrSpec> {
        EceW::new(self, 14)
    }
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etp(&mut self) -> EtpW<SmcrSpec> {
        EtpW::new(self, 15)
    }
    #[doc = "Bit 16 - Slave mode selection - bit 3"]
    #[inline(always)]
    pub fn sms_3(&mut self) -> Sms3W<SmcrSpec> {
        Sms3W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Trigger selection - bit 4:3"]
    #[inline(always)]
    pub fn ts_4_3(&mut self) -> Ts4_3W<SmcrSpec> {
        Ts4_3W::new(self, 20)
    }
    #[doc = "Bit 24 - SMS Preload Enable"]
    #[inline(always)]
    pub fn smspe(&mut self) -> SmspeW<SmcrSpec> {
        SmspeW::new(self, 24)
    }
    #[doc = "Bit 25 - SMS Preload Source"]
    #[inline(always)]
    pub fn smsps(&mut self) -> SmspsW<SmcrSpec> {
        SmspsW::new(self, 25)
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
}
#[doc = "`reset()` method sets SMCR to value 0"]
impl crate::Resettable for SmcrSpec {}

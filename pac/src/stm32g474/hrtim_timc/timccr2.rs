#[doc = "Register `TIMCCR2` reader"]
pub type R = crate::R<Timccr2Spec>;
#[doc = "Register `TIMCCR2` writer"]
pub type W = crate::W<Timccr2Spec>;
#[doc = "Field `DCDE` reader - Dual Channel DAC trigger enable"]
pub type DcdeR = crate::BitReader;
#[doc = "Field `DCDE` writer - Dual Channel DAC trigger enable"]
pub type DcdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDS` reader - Dual Channel DAC Step trigger"]
pub type DcdsR = crate::BitReader;
#[doc = "Field `DCDS` writer - Dual Channel DAC Step trigger"]
pub type DcdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDR` reader - Dual Channel DAC Reset trigger"]
pub type DcdrR = crate::BitReader;
#[doc = "Field `DCDR` writer - Dual Channel DAC Reset trigger"]
pub type DcdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDM` reader - Up-Down Mode"]
pub type UdmR = crate::BitReader;
#[doc = "Field `UDM` writer - Up-Down Mode"]
pub type UdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROM` reader - Roll-Over Mode"]
pub type RomR = crate::FieldReader;
#[doc = "Field `ROM` writer - Roll-Over Mode"]
pub type RomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OUTROM` reader - Output Roll-Over Mode"]
pub type OutromR = crate::FieldReader;
#[doc = "Field `OUTROM` writer - Output Roll-Over Mode"]
pub type OutromW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADROM` reader - ADC Roll-Over Mode"]
pub type AdromR = crate::FieldReader;
#[doc = "Field `ADROM` writer - ADC Roll-Over Mode"]
pub type AdromW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BMROM` reader - Burst Mode Roll-Over Mode"]
pub type BmromR = crate::FieldReader;
#[doc = "Field `BMROM` writer - Burst Mode Roll-Over Mode"]
pub type BmromW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FEROM` reader - Fault and Event Roll-Over Mode"]
pub type FeromR = crate::FieldReader;
#[doc = "Field `FEROM` writer - Fault and Event Roll-Over Mode"]
pub type FeromW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GTCMP1` reader - Greater than Compare 1 PWM mode"]
pub type Gtcmp1R = crate::BitReader;
#[doc = "Field `GTCMP1` writer - Greater than Compare 1 PWM mode"]
pub type Gtcmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTCMP3` reader - Greater than Compare 3 PWM mode"]
pub type Gtcmp3R = crate::BitReader;
#[doc = "Field `GTCMP3` writer - Greater than Compare 3 PWM mode"]
pub type Gtcmp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGHLF` reader - Triggered-half mode"]
pub type TrghlfR = crate::BitReader;
#[doc = "Field `TRGHLF` writer - Triggered-half mode"]
pub type TrghlfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Dual Channel DAC trigger enable"]
    #[inline(always)]
    pub fn dcde(&self) -> DcdeR {
        DcdeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Dual Channel DAC Step trigger"]
    #[inline(always)]
    pub fn dcds(&self) -> DcdsR {
        DcdsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Dual Channel DAC Reset trigger"]
    #[inline(always)]
    pub fn dcdr(&self) -> DcdrR {
        DcdrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Up-Down Mode"]
    #[inline(always)]
    pub fn udm(&self) -> UdmR {
        UdmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Roll-Over Mode"]
    #[inline(always)]
    pub fn rom(&self) -> RomR {
        RomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Output Roll-Over Mode"]
    #[inline(always)]
    pub fn outrom(&self) -> OutromR {
        OutromR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - ADC Roll-Over Mode"]
    #[inline(always)]
    pub fn adrom(&self) -> AdromR {
        AdromR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Burst Mode Roll-Over Mode"]
    #[inline(always)]
    pub fn bmrom(&self) -> BmromR {
        BmromR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Fault and Event Roll-Over Mode"]
    #[inline(always)]
    pub fn ferom(&self) -> FeromR {
        FeromR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Greater than Compare 1 PWM mode"]
    #[inline(always)]
    pub fn gtcmp1(&self) -> Gtcmp1R {
        Gtcmp1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Greater than Compare 3 PWM mode"]
    #[inline(always)]
    pub fn gtcmp3(&self) -> Gtcmp3R {
        Gtcmp3R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Triggered-half mode"]
    #[inline(always)]
    pub fn trghlf(&self) -> TrghlfR {
        TrghlfR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMCCR2")
            .field("trghlf", &self.trghlf())
            .field("gtcmp3", &self.gtcmp3())
            .field("gtcmp1", &self.gtcmp1())
            .field("ferom", &self.ferom())
            .field("bmrom", &self.bmrom())
            .field("adrom", &self.adrom())
            .field("outrom", &self.outrom())
            .field("rom", &self.rom())
            .field("udm", &self.udm())
            .field("dcdr", &self.dcdr())
            .field("dcds", &self.dcds())
            .field("dcde", &self.dcde())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Dual Channel DAC trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcde(&mut self) -> DcdeW<Timccr2Spec> {
        DcdeW::new(self, 0)
    }
    #[doc = "Bit 1 - Dual Channel DAC Step trigger"]
    #[inline(always)]
    #[must_use]
    pub fn dcds(&mut self) -> DcdsW<Timccr2Spec> {
        DcdsW::new(self, 1)
    }
    #[doc = "Bit 2 - Dual Channel DAC Reset trigger"]
    #[inline(always)]
    #[must_use]
    pub fn dcdr(&mut self) -> DcdrW<Timccr2Spec> {
        DcdrW::new(self, 2)
    }
    #[doc = "Bit 4 - Up-Down Mode"]
    #[inline(always)]
    #[must_use]
    pub fn udm(&mut self) -> UdmW<Timccr2Spec> {
        UdmW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Roll-Over Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> RomW<Timccr2Spec> {
        RomW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Output Roll-Over Mode"]
    #[inline(always)]
    #[must_use]
    pub fn outrom(&mut self) -> OutromW<Timccr2Spec> {
        OutromW::new(self, 8)
    }
    #[doc = "Bits 10:11 - ADC Roll-Over Mode"]
    #[inline(always)]
    #[must_use]
    pub fn adrom(&mut self) -> AdromW<Timccr2Spec> {
        AdromW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Burst Mode Roll-Over Mode"]
    #[inline(always)]
    #[must_use]
    pub fn bmrom(&mut self) -> BmromW<Timccr2Spec> {
        BmromW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Fault and Event Roll-Over Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ferom(&mut self) -> FeromW<Timccr2Spec> {
        FeromW::new(self, 14)
    }
    #[doc = "Bit 16 - Greater than Compare 1 PWM mode"]
    #[inline(always)]
    #[must_use]
    pub fn gtcmp1(&mut self) -> Gtcmp1W<Timccr2Spec> {
        Gtcmp1W::new(self, 16)
    }
    #[doc = "Bit 17 - Greater than Compare 3 PWM mode"]
    #[inline(always)]
    #[must_use]
    pub fn gtcmp3(&mut self) -> Gtcmp3W<Timccr2Spec> {
        Gtcmp3W::new(self, 17)
    }
    #[doc = "Bit 20 - Triggered-half mode"]
    #[inline(always)]
    #[must_use]
    pub fn trghlf(&mut self) -> TrghlfW<Timccr2Spec> {
        TrghlfW::new(self, 20)
    }
}
#[doc = "HRTIM Timerx Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timccr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timccr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timccr2Spec;
impl crate::RegisterSpec for Timccr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timccr2::R`](R) reader structure"]
impl crate::Readable for Timccr2Spec {}
#[doc = "`write(|w| ..)` method takes [`timccr2::W`](W) writer structure"]
impl crate::Writable for Timccr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMCCR2 to value 0"]
impl crate::Resettable for Timccr2Spec {
    const RESET_VALUE: u32 = 0;
}

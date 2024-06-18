#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `ADEN` reader - ADEN"]
pub type AdenR = crate::BitReader;
#[doc = "Field `ADEN` writer - ADEN"]
pub type AdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDIS` reader - ADDIS"]
pub type AddisR = crate::BitReader;
#[doc = "Field `ADDIS` writer - ADDIS"]
pub type AddisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSTART` reader - ADSTART"]
pub type AdstartR = crate::BitReader;
#[doc = "Field `ADSTART` writer - ADSTART"]
pub type AdstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JADSTART` reader - JADSTART"]
pub type JadstartR = crate::BitReader;
#[doc = "Field `JADSTART` writer - JADSTART"]
pub type JadstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSTP` reader - ADSTP"]
pub type AdstpR = crate::BitReader;
#[doc = "Field `ADSTP` writer - ADSTP"]
pub type AdstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JADSTP` reader - JADSTP"]
pub type JadstpR = crate::BitReader;
#[doc = "Field `JADSTP` writer - JADSTP"]
pub type JadstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADVREGEN` reader - ADVREGEN"]
pub type AdvregenR = crate::BitReader;
#[doc = "Field `ADVREGEN` writer - ADVREGEN"]
pub type AdvregenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEEPPWD` reader - DEEPPWD"]
pub type DeeppwdR = crate::BitReader;
#[doc = "Field `DEEPPWD` writer - DEEPPWD"]
pub type DeeppwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCALDIF` reader - ADCALDIF"]
pub type AdcaldifR = crate::BitReader;
#[doc = "Field `ADCALDIF` writer - ADCALDIF"]
pub type AdcaldifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCAL` reader - ADCAL"]
pub type AdcalR = crate::BitReader;
#[doc = "Field `ADCAL` writer - ADCAL"]
pub type AdcalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADEN"]
    #[inline(always)]
    pub fn aden(&self) -> AdenR {
        AdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADDIS"]
    #[inline(always)]
    pub fn addis(&self) -> AddisR {
        AddisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADSTART"]
    #[inline(always)]
    pub fn adstart(&self) -> AdstartR {
        AdstartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JADSTART"]
    #[inline(always)]
    pub fn jadstart(&self) -> JadstartR {
        JadstartR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADSTP"]
    #[inline(always)]
    pub fn adstp(&self) -> AdstpR {
        AdstpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JADSTP"]
    #[inline(always)]
    pub fn jadstp(&self) -> JadstpR {
        JadstpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 28 - ADVREGEN"]
    #[inline(always)]
    pub fn advregen(&self) -> AdvregenR {
        AdvregenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DEEPPWD"]
    #[inline(always)]
    pub fn deeppwd(&self) -> DeeppwdR {
        DeeppwdR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ADCALDIF"]
    #[inline(always)]
    pub fn adcaldif(&self) -> AdcaldifR {
        AdcaldifR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ADCAL"]
    #[inline(always)]
    pub fn adcal(&self) -> AdcalR {
        AdcalR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("adcal", &self.adcal())
            .field("adcaldif", &self.adcaldif())
            .field("deeppwd", &self.deeppwd())
            .field("advregen", &self.advregen())
            .field("jadstp", &self.jadstp())
            .field("adstp", &self.adstp())
            .field("jadstart", &self.jadstart())
            .field("adstart", &self.adstart())
            .field("addis", &self.addis())
            .field("aden", &self.aden())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ADEN"]
    #[inline(always)]
    #[must_use]
    pub fn aden(&mut self) -> AdenW<CrSpec> {
        AdenW::new(self, 0)
    }
    #[doc = "Bit 1 - ADDIS"]
    #[inline(always)]
    #[must_use]
    pub fn addis(&mut self) -> AddisW<CrSpec> {
        AddisW::new(self, 1)
    }
    #[doc = "Bit 2 - ADSTART"]
    #[inline(always)]
    #[must_use]
    pub fn adstart(&mut self) -> AdstartW<CrSpec> {
        AdstartW::new(self, 2)
    }
    #[doc = "Bit 3 - JADSTART"]
    #[inline(always)]
    #[must_use]
    pub fn jadstart(&mut self) -> JadstartW<CrSpec> {
        JadstartW::new(self, 3)
    }
    #[doc = "Bit 4 - ADSTP"]
    #[inline(always)]
    #[must_use]
    pub fn adstp(&mut self) -> AdstpW<CrSpec> {
        AdstpW::new(self, 4)
    }
    #[doc = "Bit 5 - JADSTP"]
    #[inline(always)]
    #[must_use]
    pub fn jadstp(&mut self) -> JadstpW<CrSpec> {
        JadstpW::new(self, 5)
    }
    #[doc = "Bit 28 - ADVREGEN"]
    #[inline(always)]
    #[must_use]
    pub fn advregen(&mut self) -> AdvregenW<CrSpec> {
        AdvregenW::new(self, 28)
    }
    #[doc = "Bit 29 - DEEPPWD"]
    #[inline(always)]
    #[must_use]
    pub fn deeppwd(&mut self) -> DeeppwdW<CrSpec> {
        DeeppwdW::new(self, 29)
    }
    #[doc = "Bit 30 - ADCALDIF"]
    #[inline(always)]
    #[must_use]
    pub fn adcaldif(&mut self) -> AdcaldifW<CrSpec> {
        AdcaldifW::new(self, 30)
    }
    #[doc = "Bit 31 - ADCAL"]
    #[inline(always)]
    #[must_use]
    pub fn adcal(&mut self) -> AdcalW<CrSpec> {
        AdcalW::new(self, 31)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0x2000_0000"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x2000_0000;
}

#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `MUDIS` reader - Master Update Disable"]
pub type MudisR = crate::BitReader;
#[doc = "Field `MUDIS` writer - Master Update Disable"]
pub type MudisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAUDIS` reader - Timer A Update Disable"]
pub type TaudisR = crate::BitReader;
#[doc = "Field `TAUDIS` writer - Timer A Update Disable"]
pub type TaudisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBUDIS` reader - Timer B Update Disable"]
pub type TbudisR = crate::BitReader;
#[doc = "Field `TBUDIS` writer - Timer B Update Disable"]
pub type TbudisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCUDIS` reader - Timer C Update Disable"]
pub type TcudisR = crate::BitReader;
#[doc = "Field `TCUDIS` writer - Timer C Update Disable"]
pub type TcudisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDUDIS` reader - Timer D Update Disable"]
pub type TdudisR = crate::BitReader;
#[doc = "Field `TDUDIS` writer - Timer D Update Disable"]
pub type TdudisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEUDIS` reader - Timer E Update Disable"]
pub type TeudisR = crate::BitReader;
#[doc = "Field `TEUDIS` writer - Timer E Update Disable"]
pub type TeudisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFUDIS` reader - Timer f Update Disable"]
pub type TfudisR = crate::BitReader;
#[doc = "Field `TFUDIS` writer - Timer f Update Disable"]
pub type TfudisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD1USRC` reader - ADC Trigger 1 Update Source"]
pub type Ad1usrcR = crate::FieldReader;
#[doc = "Field `AD1USRC` writer - ADC Trigger 1 Update Source"]
pub type Ad1usrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AD2USRC` reader - ADC Trigger 2 Update Source"]
pub type Ad2usrcR = crate::FieldReader;
#[doc = "Field `AD2USRC` writer - ADC Trigger 2 Update Source"]
pub type Ad2usrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AD3USRC` reader - ADC Trigger 3 Update Source"]
pub type Ad3usrcR = crate::FieldReader;
#[doc = "Field `AD3USRC` writer - ADC Trigger 3 Update Source"]
pub type Ad3usrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AD4USRC` reader - ADC Trigger 4 Update Source"]
pub type Ad4usrcR = crate::FieldReader;
#[doc = "Field `AD4USRC` writer - ADC Trigger 4 Update Source"]
pub type Ad4usrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Master Update Disable"]
    #[inline(always)]
    pub fn mudis(&self) -> MudisR {
        MudisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer A Update Disable"]
    #[inline(always)]
    pub fn taudis(&self) -> TaudisR {
        TaudisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer B Update Disable"]
    #[inline(always)]
    pub fn tbudis(&self) -> TbudisR {
        TbudisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer C Update Disable"]
    #[inline(always)]
    pub fn tcudis(&self) -> TcudisR {
        TcudisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer D Update Disable"]
    #[inline(always)]
    pub fn tdudis(&self) -> TdudisR {
        TdudisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer E Update Disable"]
    #[inline(always)]
    pub fn teudis(&self) -> TeudisR {
        TeudisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer f Update Disable"]
    #[inline(always)]
    pub fn tfudis(&self) -> TfudisR {
        TfudisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:18 - ADC Trigger 1 Update Source"]
    #[inline(always)]
    pub fn ad1usrc(&self) -> Ad1usrcR {
        Ad1usrcR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - ADC Trigger 2 Update Source"]
    #[inline(always)]
    pub fn ad2usrc(&self) -> Ad2usrcR {
        Ad2usrcR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - ADC Trigger 3 Update Source"]
    #[inline(always)]
    pub fn ad3usrc(&self) -> Ad3usrcR {
        Ad3usrcR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - ADC Trigger 4 Update Source"]
    #[inline(always)]
    pub fn ad4usrc(&self) -> Ad4usrcR {
        Ad4usrcR::new(((self.bits >> 25) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("ad4usrc", &self.ad4usrc())
            .field("ad3usrc", &self.ad3usrc())
            .field("ad2usrc", &self.ad2usrc())
            .field("ad1usrc", &self.ad1usrc())
            .field("tfudis", &self.tfudis())
            .field("teudis", &self.teudis())
            .field("tdudis", &self.tdudis())
            .field("tcudis", &self.tcudis())
            .field("tbudis", &self.tbudis())
            .field("taudis", &self.taudis())
            .field("mudis", &self.mudis())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Master Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mudis(&mut self) -> MudisW<Cr1Spec> {
        MudisW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer A Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn taudis(&mut self) -> TaudisW<Cr1Spec> {
        TaudisW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer B Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tbudis(&mut self) -> TbudisW<Cr1Spec> {
        TbudisW::new(self, 2)
    }
    #[doc = "Bit 3 - Timer C Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tcudis(&mut self) -> TcudisW<Cr1Spec> {
        TcudisW::new(self, 3)
    }
    #[doc = "Bit 4 - Timer D Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tdudis(&mut self) -> TdudisW<Cr1Spec> {
        TdudisW::new(self, 4)
    }
    #[doc = "Bit 5 - Timer E Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn teudis(&mut self) -> TeudisW<Cr1Spec> {
        TeudisW::new(self, 5)
    }
    #[doc = "Bit 6 - Timer f Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tfudis(&mut self) -> TfudisW<Cr1Spec> {
        TfudisW::new(self, 6)
    }
    #[doc = "Bits 16:18 - ADC Trigger 1 Update Source"]
    #[inline(always)]
    #[must_use]
    pub fn ad1usrc(&mut self) -> Ad1usrcW<Cr1Spec> {
        Ad1usrcW::new(self, 16)
    }
    #[doc = "Bits 19:21 - ADC Trigger 2 Update Source"]
    #[inline(always)]
    #[must_use]
    pub fn ad2usrc(&mut self) -> Ad2usrcW<Cr1Spec> {
        Ad2usrcW::new(self, 19)
    }
    #[doc = "Bits 22:24 - ADC Trigger 3 Update Source"]
    #[inline(always)]
    #[must_use]
    pub fn ad3usrc(&mut self) -> Ad3usrcW<Cr1Spec> {
        Ad3usrcW::new(self, 22)
    }
    #[doc = "Bits 25:27 - ADC Trigger 4 Update Source"]
    #[inline(always)]
    #[must_use]
    pub fn ad4usrc(&mut self) -> Ad4usrcW<Cr1Spec> {
        Ad4usrcW::new(self, 25)
    }
}
#[doc = "Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {
    const RESET_VALUE: u32 = 0;
}

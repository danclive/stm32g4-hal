#[doc = "Register `DIER` reader"]
pub type R = crate::R<DierSpec>;
#[doc = "Register `DIER` writer"]
pub type W = crate::W<DierSpec>;
#[doc = "Field `UIE` reader - Update interrupt enable"]
pub type UieR = crate::BitReader;
#[doc = "Field `UIE` writer - Update interrupt enable"]
pub type UieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IE` reader - Capture/Compare 1 interrupt enable"]
pub type Cc1ieR = crate::BitReader;
#[doc = "Field `CC1IE` writer - Capture/Compare 1 interrupt enable"]
pub type Cc1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMIE` reader - COM interrupt enable"]
pub type ComieR = crate::BitReader;
#[doc = "Field `COMIE` writer - COM interrupt enable"]
pub type ComieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIE` reader - Break interrupt enable"]
pub type BieR = crate::BitReader;
#[doc = "Field `BIE` writer - Break interrupt enable"]
pub type BieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDE` reader - Update DMA request enable"]
pub type UdeR = crate::BitReader;
#[doc = "Field `UDE` writer - Update DMA request enable"]
pub type UdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1DE` reader - Capture/Compare 1 DMA request enable"]
pub type Cc1deR = crate::BitReader;
#[doc = "Field `CC1DE` writer - Capture/Compare 1 DMA request enable"]
pub type Cc1deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMDE` reader - COM DMA request enable"]
pub type ComdeR = crate::BitReader;
#[doc = "Field `COMDE` writer - COM DMA request enable"]
pub type ComdeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn uie(&self) -> UieR {
        UieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cc1ie(&self) -> Cc1ieR {
        Cc1ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    pub fn comie(&self) -> ComieR {
        ComieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn bie(&self) -> BieR {
        BieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn ude(&self) -> UdeR {
        UdeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    pub fn cc1de(&self) -> Cc1deR {
        Cc1deR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - COM DMA request enable"]
    #[inline(always)]
    pub fn comde(&self) -> ComdeR {
        ComdeR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIER")
            .field("comde", &self.comde())
            .field("cc1de", &self.cc1de())
            .field("ude", &self.ude())
            .field("bie", &self.bie())
            .field("comie", &self.comie())
            .field("cc1ie", &self.cc1ie())
            .field("uie", &self.uie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn uie(&mut self) -> UieW<'_, DierSpec> {
        UieW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cc1ie(&mut self) -> Cc1ieW<'_, DierSpec> {
        Cc1ieW::new(self, 1)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    pub fn comie(&mut self) -> ComieW<'_, DierSpec> {
        ComieW::new(self, 5)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn bie(&mut self) -> BieW<'_, DierSpec> {
        BieW::new(self, 7)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn ude(&mut self) -> UdeW<'_, DierSpec> {
        UdeW::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    pub fn cc1de(&mut self) -> Cc1deW<'_, DierSpec> {
        Cc1deW::new(self, 9)
    }
    #[doc = "Bit 13 - COM DMA request enable"]
    #[inline(always)]
    pub fn comde(&mut self) -> ComdeW<'_, DierSpec> {
        ComdeW::new(self, 13)
    }
}
#[doc = "DMA/Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DierSpec;
impl crate::RegisterSpec for DierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dier::R`](R) reader structure"]
impl crate::Readable for DierSpec {}
#[doc = "`write(|w| ..)` method takes [`dier::W`](W) writer structure"]
impl crate::Writable for DierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIER to value 0"]
impl crate::Resettable for DierSpec {}

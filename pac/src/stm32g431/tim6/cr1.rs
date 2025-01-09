#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `CEN` reader - Counter enable"]
pub type CenR = crate::BitReader;
#[doc = "Field `CEN` writer - Counter enable"]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDIS` reader - Update disable"]
pub type UdisR = crate::BitReader;
#[doc = "Field `UDIS` writer - Update disable"]
pub type UdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URS` reader - Update request source"]
pub type UrsR = crate::BitReader;
#[doc = "Field `URS` writer - Update request source"]
pub type UrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPM` reader - One-pulse mode"]
pub type OpmR = crate::BitReader;
#[doc = "Field `OPM` writer - One-pulse mode"]
pub type OpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARPE` reader - Auto-reload preload enable"]
pub type ArpeR = crate::BitReader;
#[doc = "Field `ARPE` writer - Auto-reload preload enable"]
pub type ArpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UIFREMAP` reader - UIF status bit remapping"]
pub type UifremapR = crate::BitReader;
#[doc = "Field `UIFREMAP` writer - UIF status bit remapping"]
pub type UifremapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DITHEN` reader - Dithering Enable"]
pub type DithenR = crate::BitReader;
#[doc = "Field `DITHEN` writer - Dithering Enable"]
pub type DithenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn udis(&self) -> UdisR {
        UdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    pub fn urs(&self) -> UrsR {
        UrsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    pub fn opm(&self) -> OpmR {
        OpmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    pub fn arpe(&self) -> ArpeR {
        ArpeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - UIF status bit remapping"]
    #[inline(always)]
    pub fn uifremap(&self) -> UifremapR {
        UifremapR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Dithering Enable"]
    #[inline(always)]
    pub fn dithen(&self) -> DithenR {
        DithenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("dithen", &self.dithen())
            .field("uifremap", &self.uifremap())
            .field("arpe", &self.arpe())
            .field("opm", &self.opm())
            .field("urs", &self.urs())
            .field("udis", &self.udis())
            .field("cen", &self.cen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&mut self) -> CenW<Cr1Spec> {
        CenW::new(self, 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn udis(&mut self) -> UdisW<Cr1Spec> {
        UdisW::new(self, 1)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    pub fn urs(&mut self) -> UrsW<Cr1Spec> {
        UrsW::new(self, 2)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    pub fn opm(&mut self) -> OpmW<Cr1Spec> {
        OpmW::new(self, 3)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    pub fn arpe(&mut self) -> ArpeW<Cr1Spec> {
        ArpeW::new(self, 7)
    }
    #[doc = "Bit 11 - UIF status bit remapping"]
    #[inline(always)]
    pub fn uifremap(&mut self) -> UifremapW<Cr1Spec> {
        UifremapW::new(self, 11)
    }
    #[doc = "Bit 12 - Dithering Enable"]
    #[inline(always)]
    pub fn dithen(&mut self) -> DithenW<Cr1Spec> {
        DithenW::new(self, 12)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

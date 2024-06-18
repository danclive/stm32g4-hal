#[doc = "Register `CFG2` reader"]
pub type R = crate::R<Cfg2Spec>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<Cfg2Spec>;
#[doc = "Field `RXFILTDIS` reader - RXFILTDIS"]
pub type RxfiltdisR = crate::BitReader;
#[doc = "Field `RXFILTDIS` writer - RXFILTDIS"]
pub type RxfiltdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFILT2N3` reader - RXFILT2N3"]
pub type Rxfilt2n3R = crate::BitReader;
#[doc = "Field `RXFILT2N3` writer - RXFILT2N3"]
pub type Rxfilt2n3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCECLK` reader - FORCECLK"]
pub type ForceclkR = crate::BitReader;
#[doc = "Field `FORCECLK` writer - FORCECLK"]
pub type ForceclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPEN` reader - WUPEN"]
pub type WupenR = crate::BitReader;
#[doc = "Field `WUPEN` writer - WUPEN"]
pub type WupenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXFILTDIS"]
    #[inline(always)]
    pub fn rxfiltdis(&self) -> RxfiltdisR {
        RxfiltdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXFILT2N3"]
    #[inline(always)]
    pub fn rxfilt2n3(&self) -> Rxfilt2n3R {
        Rxfilt2n3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FORCECLK"]
    #[inline(always)]
    pub fn forceclk(&self) -> ForceclkR {
        ForceclkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WUPEN"]
    #[inline(always)]
    pub fn wupen(&self) -> WupenR {
        WupenR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG2")
            .field("rxfiltdis", &self.rxfiltdis())
            .field("rxfilt2n3", &self.rxfilt2n3())
            .field("forceclk", &self.forceclk())
            .field("wupen", &self.wupen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RXFILTDIS"]
    #[inline(always)]
    #[must_use]
    pub fn rxfiltdis(&mut self) -> RxfiltdisW<Cfg2Spec> {
        RxfiltdisW::new(self, 0)
    }
    #[doc = "Bit 1 - RXFILT2N3"]
    #[inline(always)]
    #[must_use]
    pub fn rxfilt2n3(&mut self) -> Rxfilt2n3W<Cfg2Spec> {
        Rxfilt2n3W::new(self, 1)
    }
    #[doc = "Bit 2 - FORCECLK"]
    #[inline(always)]
    #[must_use]
    pub fn forceclk(&mut self) -> ForceclkW<Cfg2Spec> {
        ForceclkW::new(self, 2)
    }
    #[doc = "Bit 3 - WUPEN"]
    #[inline(always)]
    #[must_use]
    pub fn wupen(&mut self) -> WupenW<Cfg2Spec> {
        WupenW::new(self, 3)
    }
}
#[doc = "UCPD configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg2Spec;
impl crate::RegisterSpec for Cfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for Cfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for Cfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for Cfg2Spec {
    const RESET_VALUE: u32 = 0;
}

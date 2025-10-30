#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `TXMSGDISCCF` reader - TXMSGDISCCF"]
pub type TxmsgdisccfR = crate::BitReader;
#[doc = "Field `TXMSGDISCCF` writer - TXMSGDISCCF"]
pub type TxmsgdisccfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMSGSENTCF` reader - TXMSGSENTCF"]
pub type TxmsgsentcfR = crate::BitReader;
#[doc = "Field `TXMSGSENTCF` writer - TXMSGSENTCF"]
pub type TxmsgsentcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMSGABTCF` reader - TXMSGABTCF"]
pub type TxmsgabtcfR = crate::BitReader;
#[doc = "Field `TXMSGABTCF` writer - TXMSGABTCF"]
pub type TxmsgabtcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRSTDISCCF` reader - HRSTDISCCF"]
pub type HrstdisccfR = crate::BitReader;
#[doc = "Field `HRSTDISCCF` writer - HRSTDISCCF"]
pub type HrstdisccfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRSTSENTCF` reader - HRSTSENTCF"]
pub type HrstsentcfR = crate::BitReader;
#[doc = "Field `HRSTSENTCF` writer - HRSTSENTCF"]
pub type HrstsentcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDCF` reader - TXUNDCF"]
pub type TxundcfR = crate::BitReader;
#[doc = "Field `TXUNDCF` writer - TXUNDCF"]
pub type TxundcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXORDDETCF` reader - RXORDDETCF"]
pub type RxorddetcfR = crate::BitReader;
#[doc = "Field `RXORDDETCF` writer - RXORDDETCF"]
pub type RxorddetcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXHRSTDETCF` reader - RXHRSTDETCF"]
pub type RxhrstdetcfR = crate::BitReader;
#[doc = "Field `RXHRSTDETCF` writer - RXHRSTDETCF"]
pub type RxhrstdetcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVRCF` reader - RXOVRCF"]
pub type RxovrcfR = crate::BitReader;
#[doc = "Field `RXOVRCF` writer - RXOVRCF"]
pub type RxovrcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMSGENDCF` reader - RXMSGENDCF"]
pub type RxmsgendcfR = crate::BitReader;
#[doc = "Field `RXMSGENDCF` writer - RXMSGENDCF"]
pub type RxmsgendcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPECEVT1CF` reader - TYPECEVT1CF"]
pub type Typecevt1cfR = crate::BitReader;
#[doc = "Field `TYPECEVT1CF` writer - TYPECEVT1CF"]
pub type Typecevt1cfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPECEVT2CF` reader - TYPECEVT2CF"]
pub type Typecevt2cfR = crate::BitReader;
#[doc = "Field `TYPECEVT2CF` writer - TYPECEVT2CF"]
pub type Typecevt2cfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRSEVTCF` reader - FRSEVTCF"]
pub type FrsevtcfR = crate::BitReader;
#[doc = "Field `FRSEVTCF` writer - FRSEVTCF"]
pub type FrsevtcfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - TXMSGDISCCF"]
    #[inline(always)]
    pub fn txmsgdisccf(&self) -> TxmsgdisccfR {
        TxmsgdisccfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXMSGSENTCF"]
    #[inline(always)]
    pub fn txmsgsentcf(&self) -> TxmsgsentcfR {
        TxmsgsentcfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TXMSGABTCF"]
    #[inline(always)]
    pub fn txmsgabtcf(&self) -> TxmsgabtcfR {
        TxmsgabtcfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HRSTDISCCF"]
    #[inline(always)]
    pub fn hrstdisccf(&self) -> HrstdisccfR {
        HrstdisccfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HRSTSENTCF"]
    #[inline(always)]
    pub fn hrstsentcf(&self) -> HrstsentcfR {
        HrstsentcfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXUNDCF"]
    #[inline(always)]
    pub fn txundcf(&self) -> TxundcfR {
        TxundcfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - RXORDDETCF"]
    #[inline(always)]
    pub fn rxorddetcf(&self) -> RxorddetcfR {
        RxorddetcfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RXHRSTDETCF"]
    #[inline(always)]
    pub fn rxhrstdetcf(&self) -> RxhrstdetcfR {
        RxhrstdetcfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RXOVRCF"]
    #[inline(always)]
    pub fn rxovrcf(&self) -> RxovrcfR {
        RxovrcfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RXMSGENDCF"]
    #[inline(always)]
    pub fn rxmsgendcf(&self) -> RxmsgendcfR {
        RxmsgendcfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - TYPECEVT1CF"]
    #[inline(always)]
    pub fn typecevt1cf(&self) -> Typecevt1cfR {
        Typecevt1cfR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TYPECEVT2CF"]
    #[inline(always)]
    pub fn typecevt2cf(&self) -> Typecevt2cfR {
        Typecevt2cfR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - FRSEVTCF"]
    #[inline(always)]
    pub fn frsevtcf(&self) -> FrsevtcfR {
        FrsevtcfR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR")
            .field("txmsgdisccf", &self.txmsgdisccf())
            .field("txmsgsentcf", &self.txmsgsentcf())
            .field("txmsgabtcf", &self.txmsgabtcf())
            .field("hrstdisccf", &self.hrstdisccf())
            .field("hrstsentcf", &self.hrstsentcf())
            .field("txundcf", &self.txundcf())
            .field("rxorddetcf", &self.rxorddetcf())
            .field("rxhrstdetcf", &self.rxhrstdetcf())
            .field("rxovrcf", &self.rxovrcf())
            .field("rxmsgendcf", &self.rxmsgendcf())
            .field("typecevt1cf", &self.typecevt1cf())
            .field("typecevt2cf", &self.typecevt2cf())
            .field("frsevtcf", &self.frsevtcf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - TXMSGDISCCF"]
    #[inline(always)]
    pub fn txmsgdisccf(&mut self) -> TxmsgdisccfW<'_, IcrSpec> {
        TxmsgdisccfW::new(self, 1)
    }
    #[doc = "Bit 2 - TXMSGSENTCF"]
    #[inline(always)]
    pub fn txmsgsentcf(&mut self) -> TxmsgsentcfW<'_, IcrSpec> {
        TxmsgsentcfW::new(self, 2)
    }
    #[doc = "Bit 3 - TXMSGABTCF"]
    #[inline(always)]
    pub fn txmsgabtcf(&mut self) -> TxmsgabtcfW<'_, IcrSpec> {
        TxmsgabtcfW::new(self, 3)
    }
    #[doc = "Bit 4 - HRSTDISCCF"]
    #[inline(always)]
    pub fn hrstdisccf(&mut self) -> HrstdisccfW<'_, IcrSpec> {
        HrstdisccfW::new(self, 4)
    }
    #[doc = "Bit 5 - HRSTSENTCF"]
    #[inline(always)]
    pub fn hrstsentcf(&mut self) -> HrstsentcfW<'_, IcrSpec> {
        HrstsentcfW::new(self, 5)
    }
    #[doc = "Bit 6 - TXUNDCF"]
    #[inline(always)]
    pub fn txundcf(&mut self) -> TxundcfW<'_, IcrSpec> {
        TxundcfW::new(self, 6)
    }
    #[doc = "Bit 9 - RXORDDETCF"]
    #[inline(always)]
    pub fn rxorddetcf(&mut self) -> RxorddetcfW<'_, IcrSpec> {
        RxorddetcfW::new(self, 9)
    }
    #[doc = "Bit 10 - RXHRSTDETCF"]
    #[inline(always)]
    pub fn rxhrstdetcf(&mut self) -> RxhrstdetcfW<'_, IcrSpec> {
        RxhrstdetcfW::new(self, 10)
    }
    #[doc = "Bit 11 - RXOVRCF"]
    #[inline(always)]
    pub fn rxovrcf(&mut self) -> RxovrcfW<'_, IcrSpec> {
        RxovrcfW::new(self, 11)
    }
    #[doc = "Bit 12 - RXMSGENDCF"]
    #[inline(always)]
    pub fn rxmsgendcf(&mut self) -> RxmsgendcfW<'_, IcrSpec> {
        RxmsgendcfW::new(self, 12)
    }
    #[doc = "Bit 14 - TYPECEVT1CF"]
    #[inline(always)]
    pub fn typecevt1cf(&mut self) -> Typecevt1cfW<'_, IcrSpec> {
        Typecevt1cfW::new(self, 14)
    }
    #[doc = "Bit 15 - TYPECEVT2CF"]
    #[inline(always)]
    pub fn typecevt2cf(&mut self) -> Typecevt2cfW<'_, IcrSpec> {
        Typecevt2cfW::new(self, 15)
    }
    #[doc = "Bit 20 - FRSEVTCF"]
    #[inline(always)]
    pub fn frsevtcf(&mut self) -> FrsevtcfW<'_, IcrSpec> {
        FrsevtcfW::new(self, 20)
    }
}
#[doc = "UCPD Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}

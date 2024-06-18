#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<ImrSpec>;
#[doc = "Field `TXISIE` reader - TXISIE"]
pub type TxisieR = crate::BitReader;
#[doc = "Field `TXISIE` writer - TXISIE"]
pub type TxisieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMSGDISCIE` reader - TXMSGDISCIE"]
pub type TxmsgdiscieR = crate::BitReader;
#[doc = "Field `TXMSGDISCIE` writer - TXMSGDISCIE"]
pub type TxmsgdiscieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMSGSENTIE` reader - TXMSGSENTIE"]
pub type TxmsgsentieR = crate::BitReader;
#[doc = "Field `TXMSGSENTIE` writer - TXMSGSENTIE"]
pub type TxmsgsentieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMSGABTIE` reader - TXMSGABTIE"]
pub type TxmsgabtieR = crate::BitReader;
#[doc = "Field `TXMSGABTIE` writer - TXMSGABTIE"]
pub type TxmsgabtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRSTDISCIE` reader - HRSTDISCIE"]
pub type HrstdiscieR = crate::BitReader;
#[doc = "Field `HRSTDISCIE` writer - HRSTDISCIE"]
pub type HrstdiscieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRSTSENTIE` reader - HRSTSENTIE"]
pub type HrstsentieR = crate::BitReader;
#[doc = "Field `HRSTSENTIE` writer - HRSTSENTIE"]
pub type HrstsentieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDIE` reader - TXUNDIE"]
pub type TxundieR = crate::BitReader;
#[doc = "Field `TXUNDIE` writer - TXUNDIE"]
pub type TxundieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNEIE` reader - RXNEIE"]
pub type RxneieR = crate::BitReader;
#[doc = "Field `RXNEIE` writer - RXNEIE"]
pub type RxneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXORDDETIE` reader - RXORDDETIE"]
pub type RxorddetieR = crate::BitReader;
#[doc = "Field `RXORDDETIE` writer - RXORDDETIE"]
pub type RxorddetieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXHRSTDETIE` reader - RXHRSTDETIE"]
pub type RxhrstdetieR = crate::BitReader;
#[doc = "Field `RXHRSTDETIE` writer - RXHRSTDETIE"]
pub type RxhrstdetieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVRIE` reader - RXOVRIE"]
pub type RxovrieR = crate::BitReader;
#[doc = "Field `RXOVRIE` writer - RXOVRIE"]
pub type RxovrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMSGENDIE` reader - RXMSGENDIE"]
pub type RxmsgendieR = crate::BitReader;
#[doc = "Field `RXMSGENDIE` writer - RXMSGENDIE"]
pub type RxmsgendieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPECEVT1IE` reader - TYPECEVT1IE"]
pub type Typecevt1ieR = crate::BitReader;
#[doc = "Field `TYPECEVT1IE` writer - TYPECEVT1IE"]
pub type Typecevt1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPECEVT2IE` reader - TYPECEVT2IE"]
pub type Typecevt2ieR = crate::BitReader;
#[doc = "Field `TYPECEVT2IE` writer - TYPECEVT2IE"]
pub type Typecevt2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRSEVTIE` reader - FRSEVTIE"]
pub type FrsevtieR = crate::BitReader;
#[doc = "Field `FRSEVTIE` writer - FRSEVTIE"]
pub type FrsevtieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TXISIE"]
    #[inline(always)]
    pub fn txisie(&self) -> TxisieR {
        TxisieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXMSGDISCIE"]
    #[inline(always)]
    pub fn txmsgdiscie(&self) -> TxmsgdiscieR {
        TxmsgdiscieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXMSGSENTIE"]
    #[inline(always)]
    pub fn txmsgsentie(&self) -> TxmsgsentieR {
        TxmsgsentieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TXMSGABTIE"]
    #[inline(always)]
    pub fn txmsgabtie(&self) -> TxmsgabtieR {
        TxmsgabtieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HRSTDISCIE"]
    #[inline(always)]
    pub fn hrstdiscie(&self) -> HrstdiscieR {
        HrstdiscieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HRSTSENTIE"]
    #[inline(always)]
    pub fn hrstsentie(&self) -> HrstsentieR {
        HrstsentieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXUNDIE"]
    #[inline(always)]
    pub fn txundie(&self) -> TxundieR {
        TxundieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - RXNEIE"]
    #[inline(always)]
    pub fn rxneie(&self) -> RxneieR {
        RxneieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXORDDETIE"]
    #[inline(always)]
    pub fn rxorddetie(&self) -> RxorddetieR {
        RxorddetieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RXHRSTDETIE"]
    #[inline(always)]
    pub fn rxhrstdetie(&self) -> RxhrstdetieR {
        RxhrstdetieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RXOVRIE"]
    #[inline(always)]
    pub fn rxovrie(&self) -> RxovrieR {
        RxovrieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RXMSGENDIE"]
    #[inline(always)]
    pub fn rxmsgendie(&self) -> RxmsgendieR {
        RxmsgendieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - TYPECEVT1IE"]
    #[inline(always)]
    pub fn typecevt1ie(&self) -> Typecevt1ieR {
        Typecevt1ieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TYPECEVT2IE"]
    #[inline(always)]
    pub fn typecevt2ie(&self) -> Typecevt2ieR {
        Typecevt2ieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - FRSEVTIE"]
    #[inline(always)]
    pub fn frsevtie(&self) -> FrsevtieR {
        FrsevtieR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR")
            .field("txisie", &self.txisie())
            .field("txmsgdiscie", &self.txmsgdiscie())
            .field("txmsgsentie", &self.txmsgsentie())
            .field("txmsgabtie", &self.txmsgabtie())
            .field("hrstdiscie", &self.hrstdiscie())
            .field("hrstsentie", &self.hrstsentie())
            .field("txundie", &self.txundie())
            .field("rxneie", &self.rxneie())
            .field("rxorddetie", &self.rxorddetie())
            .field("rxhrstdetie", &self.rxhrstdetie())
            .field("rxovrie", &self.rxovrie())
            .field("rxmsgendie", &self.rxmsgendie())
            .field("typecevt1ie", &self.typecevt1ie())
            .field("typecevt2ie", &self.typecevt2ie())
            .field("frsevtie", &self.frsevtie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - TXISIE"]
    #[inline(always)]
    #[must_use]
    pub fn txisie(&mut self) -> TxisieW<ImrSpec> {
        TxisieW::new(self, 0)
    }
    #[doc = "Bit 1 - TXMSGDISCIE"]
    #[inline(always)]
    #[must_use]
    pub fn txmsgdiscie(&mut self) -> TxmsgdiscieW<ImrSpec> {
        TxmsgdiscieW::new(self, 1)
    }
    #[doc = "Bit 2 - TXMSGSENTIE"]
    #[inline(always)]
    #[must_use]
    pub fn txmsgsentie(&mut self) -> TxmsgsentieW<ImrSpec> {
        TxmsgsentieW::new(self, 2)
    }
    #[doc = "Bit 3 - TXMSGABTIE"]
    #[inline(always)]
    #[must_use]
    pub fn txmsgabtie(&mut self) -> TxmsgabtieW<ImrSpec> {
        TxmsgabtieW::new(self, 3)
    }
    #[doc = "Bit 4 - HRSTDISCIE"]
    #[inline(always)]
    #[must_use]
    pub fn hrstdiscie(&mut self) -> HrstdiscieW<ImrSpec> {
        HrstdiscieW::new(self, 4)
    }
    #[doc = "Bit 5 - HRSTSENTIE"]
    #[inline(always)]
    #[must_use]
    pub fn hrstsentie(&mut self) -> HrstsentieW<ImrSpec> {
        HrstsentieW::new(self, 5)
    }
    #[doc = "Bit 6 - TXUNDIE"]
    #[inline(always)]
    #[must_use]
    pub fn txundie(&mut self) -> TxundieW<ImrSpec> {
        TxundieW::new(self, 6)
    }
    #[doc = "Bit 8 - RXNEIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RxneieW<ImrSpec> {
        RxneieW::new(self, 8)
    }
    #[doc = "Bit 9 - RXORDDETIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxorddetie(&mut self) -> RxorddetieW<ImrSpec> {
        RxorddetieW::new(self, 9)
    }
    #[doc = "Bit 10 - RXHRSTDETIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxhrstdetie(&mut self) -> RxhrstdetieW<ImrSpec> {
        RxhrstdetieW::new(self, 10)
    }
    #[doc = "Bit 11 - RXOVRIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxovrie(&mut self) -> RxovrieW<ImrSpec> {
        RxovrieW::new(self, 11)
    }
    #[doc = "Bit 12 - RXMSGENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxmsgendie(&mut self) -> RxmsgendieW<ImrSpec> {
        RxmsgendieW::new(self, 12)
    }
    #[doc = "Bit 14 - TYPECEVT1IE"]
    #[inline(always)]
    #[must_use]
    pub fn typecevt1ie(&mut self) -> Typecevt1ieW<ImrSpec> {
        Typecevt1ieW::new(self, 14)
    }
    #[doc = "Bit 15 - TYPECEVT2IE"]
    #[inline(always)]
    #[must_use]
    pub fn typecevt2ie(&mut self) -> Typecevt2ieW<ImrSpec> {
        Typecevt2ieW::new(self, 15)
    }
    #[doc = "Bit 20 - FRSEVTIE"]
    #[inline(always)]
    #[must_use]
    pub fn frsevtie(&mut self) -> FrsevtieW<ImrSpec> {
        FrsevtieW::new(self, 20)
    }
}
#[doc = "UCPD Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for ImrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0;
}

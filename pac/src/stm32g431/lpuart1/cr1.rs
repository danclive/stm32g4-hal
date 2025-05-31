#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `UE` reader - USART enable"]
pub type UeR = crate::BitReader;
#[doc = "Field `UE` writer - USART enable"]
pub type UeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UESM` reader - USART enable in Stop mode"]
pub type UesmR = crate::BitReader;
#[doc = "Field `UESM` writer - USART enable in Stop mode"]
pub type UesmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE` reader - Receiver enable"]
pub type ReR = crate::BitReader;
#[doc = "Field `RE` writer - Receiver enable"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - Transmitter enable"]
pub type TeR = crate::BitReader;
#[doc = "Field `TE` writer - Transmitter enable"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEIE` reader - IDLE interrupt enable"]
pub type IdleieR = crate::BitReader;
#[doc = "Field `IDLEIE` writer - IDLE interrupt enable"]
pub type IdleieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNEIE` reader - RXNE interrupt enable"]
pub type RxneieR = crate::BitReader;
#[doc = "Field `RXNEIE` writer - RXNE interrupt enable"]
pub type RxneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - Transmission complete interrupt enable"]
pub type TcieR = crate::BitReader;
#[doc = "Field `TCIE` writer - Transmission complete interrupt enable"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEIE` reader - interrupt enable"]
pub type TxeieR = crate::BitReader;
#[doc = "Field `TXEIE` writer - interrupt enable"]
pub type TxeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIE` reader - PE interrupt enable"]
pub type PeieR = crate::BitReader;
#[doc = "Field `PEIE` writer - PE interrupt enable"]
pub type PeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - Parity selection"]
pub type PsR = crate::BitReader;
#[doc = "Field `PS` writer - Parity selection"]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCE` reader - Parity control enable"]
pub type PceR = crate::BitReader;
#[doc = "Field `PCE` writer - Parity control enable"]
pub type PceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKE` reader - Receiver wakeup method"]
pub type WakeR = crate::BitReader;
#[doc = "Field `WAKE` writer - Receiver wakeup method"]
pub type WakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M0` reader - Word length"]
pub type M0R = crate::BitReader;
#[doc = "Field `M0` writer - Word length"]
pub type M0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MME` reader - Mute mode enable"]
pub type MmeR = crate::BitReader;
#[doc = "Field `MME` writer - Mute mode enable"]
pub type MmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMIE` reader - Character match interrupt enable"]
pub type CmieR = crate::BitReader;
#[doc = "Field `CMIE` writer - Character match interrupt enable"]
pub type CmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEDT0` reader - DEDT0"]
pub type Dedt0R = crate::BitReader;
#[doc = "Field `DEDT0` writer - DEDT0"]
pub type Dedt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEDT1` reader - DEDT1"]
pub type Dedt1R = crate::BitReader;
#[doc = "Field `DEDT1` writer - DEDT1"]
pub type Dedt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEDT2` reader - DEDT2"]
pub type Dedt2R = crate::BitReader;
#[doc = "Field `DEDT2` writer - DEDT2"]
pub type Dedt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEDT3` reader - DEDT3"]
pub type Dedt3R = crate::BitReader;
#[doc = "Field `DEDT3` writer - DEDT3"]
pub type Dedt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEDT4` reader - Driver Enable de-assertion time"]
pub type Dedt4R = crate::BitReader;
#[doc = "Field `DEDT4` writer - Driver Enable de-assertion time"]
pub type Dedt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEAT0` reader - DEAT0"]
pub type Deat0R = crate::BitReader;
#[doc = "Field `DEAT0` writer - DEAT0"]
pub type Deat0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEAT1` reader - DEAT1"]
pub type Deat1R = crate::BitReader;
#[doc = "Field `DEAT1` writer - DEAT1"]
pub type Deat1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEAT2` reader - DEAT2"]
pub type Deat2R = crate::BitReader;
#[doc = "Field `DEAT2` writer - DEAT2"]
pub type Deat2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEAT3` reader - DEAT3"]
pub type Deat3R = crate::BitReader;
#[doc = "Field `DEAT3` writer - DEAT3"]
pub type Deat3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEAT4` reader - Driver Enable assertion time"]
pub type Deat4R = crate::BitReader;
#[doc = "Field `DEAT4` writer - Driver Enable assertion time"]
pub type Deat4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M1` reader - Word length"]
pub type M1R = crate::BitReader;
#[doc = "Field `M1` writer - Word length"]
pub type M1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOEN` reader - FIFOEN"]
pub type FifoenR = crate::BitReader;
#[doc = "Field `FIFOEN` writer - FIFOEN"]
pub type FifoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFEIE` reader - TXFEIE"]
pub type TxfeieR = crate::BitReader;
#[doc = "Field `TXFEIE` writer - TXFEIE"]
pub type TxfeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFIE` reader - RXFFIE"]
pub type RxffieR = crate::BitReader;
#[doc = "Field `RXFFIE` writer - RXFFIE"]
pub type RxffieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USART enable"]
    #[inline(always)]
    pub fn ue(&self) -> UeR {
        UeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USART enable in Stop mode"]
    #[inline(always)]
    pub fn uesm(&self) -> UesmR {
        UesmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleie(&self) -> IdleieR {
        IdleieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RxneieR {
        RxneieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - interrupt enable"]
    #[inline(always)]
    pub fn txeie(&self) -> TxeieR {
        TxeieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline(always)]
    pub fn peie(&self) -> PeieR {
        PeieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receiver wakeup method"]
    #[inline(always)]
    pub fn wake(&self) -> WakeR {
        WakeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn m0(&self) -> M0R {
        M0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mute mode enable"]
    #[inline(always)]
    pub fn mme(&self) -> MmeR {
        MmeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Character match interrupt enable"]
    #[inline(always)]
    pub fn cmie(&self) -> CmieR {
        CmieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - DEDT0"]
    #[inline(always)]
    pub fn dedt0(&self) -> Dedt0R {
        Dedt0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DEDT1"]
    #[inline(always)]
    pub fn dedt1(&self) -> Dedt1R {
        Dedt1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DEDT2"]
    #[inline(always)]
    pub fn dedt2(&self) -> Dedt2R {
        Dedt2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DEDT3"]
    #[inline(always)]
    pub fn dedt3(&self) -> Dedt3R {
        Dedt3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Driver Enable de-assertion time"]
    #[inline(always)]
    pub fn dedt4(&self) -> Dedt4R {
        Dedt4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DEAT0"]
    #[inline(always)]
    pub fn deat0(&self) -> Deat0R {
        Deat0R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DEAT1"]
    #[inline(always)]
    pub fn deat1(&self) -> Deat1R {
        Deat1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DEAT2"]
    #[inline(always)]
    pub fn deat2(&self) -> Deat2R {
        Deat2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DEAT3"]
    #[inline(always)]
    pub fn deat3(&self) -> Deat3R {
        Deat3R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Driver Enable assertion time"]
    #[inline(always)]
    pub fn deat4(&self) -> Deat4R {
        Deat4R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Word length"]
    #[inline(always)]
    pub fn m1(&self) -> M1R {
        M1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - FIFOEN"]
    #[inline(always)]
    pub fn fifoen(&self) -> FifoenR {
        FifoenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TXFEIE"]
    #[inline(always)]
    pub fn txfeie(&self) -> TxfeieR {
        TxfeieR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RXFFIE"]
    #[inline(always)]
    pub fn rxffie(&self) -> RxffieR {
        RxffieR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("rxffie", &self.rxffie())
            .field("txfeie", &self.txfeie())
            .field("fifoen", &self.fifoen())
            .field("m1", &self.m1())
            .field("deat4", &self.deat4())
            .field("deat3", &self.deat3())
            .field("deat2", &self.deat2())
            .field("deat1", &self.deat1())
            .field("deat0", &self.deat0())
            .field("dedt4", &self.dedt4())
            .field("dedt3", &self.dedt3())
            .field("dedt2", &self.dedt2())
            .field("dedt1", &self.dedt1())
            .field("dedt0", &self.dedt0())
            .field("cmie", &self.cmie())
            .field("mme", &self.mme())
            .field("m0", &self.m0())
            .field("wake", &self.wake())
            .field("pce", &self.pce())
            .field("ps", &self.ps())
            .field("peie", &self.peie())
            .field("txeie", &self.txeie())
            .field("tcie", &self.tcie())
            .field("rxneie", &self.rxneie())
            .field("idleie", &self.idleie())
            .field("te", &self.te())
            .field("re", &self.re())
            .field("uesm", &self.uesm())
            .field("ue", &self.ue())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - USART enable"]
    #[inline(always)]
    pub fn ue(&mut self) -> UeW<Cr1Spec> {
        UeW::new(self, 0)
    }
    #[doc = "Bit 1 - USART enable in Stop mode"]
    #[inline(always)]
    pub fn uesm(&mut self) -> UesmW<Cr1Spec> {
        UesmW::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&mut self) -> ReW<Cr1Spec> {
        ReW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TeW<Cr1Spec> {
        TeW::new(self, 3)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleie(&mut self) -> IdleieW<Cr1Spec> {
        IdleieW::new(self, 4)
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&mut self) -> RxneieW<Cr1Spec> {
        RxneieW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<Cr1Spec> {
        TcieW::new(self, 6)
    }
    #[doc = "Bit 7 - interrupt enable"]
    #[inline(always)]
    pub fn txeie(&mut self) -> TxeieW<Cr1Spec> {
        TxeieW::new(self, 7)
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline(always)]
    pub fn peie(&mut self) -> PeieW<Cr1Spec> {
        PeieW::new(self, 8)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<Cr1Spec> {
        PsW::new(self, 9)
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    pub fn pce(&mut self) -> PceW<Cr1Spec> {
        PceW::new(self, 10)
    }
    #[doc = "Bit 11 - Receiver wakeup method"]
    #[inline(always)]
    pub fn wake(&mut self) -> WakeW<Cr1Spec> {
        WakeW::new(self, 11)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn m0(&mut self) -> M0W<Cr1Spec> {
        M0W::new(self, 12)
    }
    #[doc = "Bit 13 - Mute mode enable"]
    #[inline(always)]
    pub fn mme(&mut self) -> MmeW<Cr1Spec> {
        MmeW::new(self, 13)
    }
    #[doc = "Bit 14 - Character match interrupt enable"]
    #[inline(always)]
    pub fn cmie(&mut self) -> CmieW<Cr1Spec> {
        CmieW::new(self, 14)
    }
    #[doc = "Bit 16 - DEDT0"]
    #[inline(always)]
    pub fn dedt0(&mut self) -> Dedt0W<Cr1Spec> {
        Dedt0W::new(self, 16)
    }
    #[doc = "Bit 17 - DEDT1"]
    #[inline(always)]
    pub fn dedt1(&mut self) -> Dedt1W<Cr1Spec> {
        Dedt1W::new(self, 17)
    }
    #[doc = "Bit 18 - DEDT2"]
    #[inline(always)]
    pub fn dedt2(&mut self) -> Dedt2W<Cr1Spec> {
        Dedt2W::new(self, 18)
    }
    #[doc = "Bit 19 - DEDT3"]
    #[inline(always)]
    pub fn dedt3(&mut self) -> Dedt3W<Cr1Spec> {
        Dedt3W::new(self, 19)
    }
    #[doc = "Bit 20 - Driver Enable de-assertion time"]
    #[inline(always)]
    pub fn dedt4(&mut self) -> Dedt4W<Cr1Spec> {
        Dedt4W::new(self, 20)
    }
    #[doc = "Bit 21 - DEAT0"]
    #[inline(always)]
    pub fn deat0(&mut self) -> Deat0W<Cr1Spec> {
        Deat0W::new(self, 21)
    }
    #[doc = "Bit 22 - DEAT1"]
    #[inline(always)]
    pub fn deat1(&mut self) -> Deat1W<Cr1Spec> {
        Deat1W::new(self, 22)
    }
    #[doc = "Bit 23 - DEAT2"]
    #[inline(always)]
    pub fn deat2(&mut self) -> Deat2W<Cr1Spec> {
        Deat2W::new(self, 23)
    }
    #[doc = "Bit 24 - DEAT3"]
    #[inline(always)]
    pub fn deat3(&mut self) -> Deat3W<Cr1Spec> {
        Deat3W::new(self, 24)
    }
    #[doc = "Bit 25 - Driver Enable assertion time"]
    #[inline(always)]
    pub fn deat4(&mut self) -> Deat4W<Cr1Spec> {
        Deat4W::new(self, 25)
    }
    #[doc = "Bit 28 - Word length"]
    #[inline(always)]
    pub fn m1(&mut self) -> M1W<Cr1Spec> {
        M1W::new(self, 28)
    }
    #[doc = "Bit 29 - FIFOEN"]
    #[inline(always)]
    pub fn fifoen(&mut self) -> FifoenW<Cr1Spec> {
        FifoenW::new(self, 29)
    }
    #[doc = "Bit 30 - TXFEIE"]
    #[inline(always)]
    pub fn txfeie(&mut self) -> TxfeieW<Cr1Spec> {
        TxfeieW::new(self, 30)
    }
    #[doc = "Bit 31 - RXFFIE"]
    #[inline(always)]
    pub fn rxffie(&mut self) -> RxffieW<Cr1Spec> {
        RxffieW::new(self, 31)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}

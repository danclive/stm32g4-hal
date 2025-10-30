#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `TXIS` reader - TXIS"]
pub type TxisR = crate::BitReader;
#[doc = "Field `TXIS` writer - TXIS"]
pub type TxisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMSGDISC` reader - TXMSGDISC"]
pub type TxmsgdiscR = crate::BitReader;
#[doc = "Field `TXMSGDISC` writer - TXMSGDISC"]
pub type TxmsgdiscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMSGSENT` reader - TXMSGSENT"]
pub type TxmsgsentR = crate::BitReader;
#[doc = "Field `TXMSGSENT` writer - TXMSGSENT"]
pub type TxmsgsentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMSGABT` reader - TXMSGABT"]
pub type TxmsgabtR = crate::BitReader;
#[doc = "Field `TXMSGABT` writer - TXMSGABT"]
pub type TxmsgabtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRSTDISC` reader - HRSTDISC"]
pub type HrstdiscR = crate::BitReader;
#[doc = "Field `HRSTDISC` writer - HRSTDISC"]
pub type HrstdiscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRSTSENT` reader - HRSTSENT"]
pub type HrstsentR = crate::BitReader;
#[doc = "Field `HRSTSENT` writer - HRSTSENT"]
pub type HrstsentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUND` reader - TXUND"]
pub type TxundR = crate::BitReader;
#[doc = "Field `TXUND` writer - TXUND"]
pub type TxundW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNE` reader - RXNE"]
pub type RxneR = crate::BitReader;
#[doc = "Field `RXNE` writer - RXNE"]
pub type RxneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXORDDET` reader - RXORDDET"]
pub type RxorddetR = crate::BitReader;
#[doc = "Field `RXORDDET` writer - RXORDDET"]
pub type RxorddetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXHRSTDET` reader - RXHRSTDET"]
pub type RxhrstdetR = crate::BitReader;
#[doc = "Field `RXHRSTDET` writer - RXHRSTDET"]
pub type RxhrstdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVR` reader - RXOVR"]
pub type RxovrR = crate::BitReader;
#[doc = "Field `RXOVR` writer - RXOVR"]
pub type RxovrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMSGEND` reader - RXMSGEND"]
pub type RxmsgendR = crate::BitReader;
#[doc = "Field `RXMSGEND` writer - RXMSGEND"]
pub type RxmsgendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXERR` reader - RXERR"]
pub type RxerrR = crate::BitReader;
#[doc = "Field `RXERR` writer - RXERR"]
pub type RxerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPECEVT1` reader - TYPECEVT1"]
pub type Typecevt1R = crate::BitReader;
#[doc = "Field `TYPECEVT1` writer - TYPECEVT1"]
pub type Typecevt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPECEVT2` reader - TYPECEVT2"]
pub type Typecevt2R = crate::BitReader;
#[doc = "Field `TYPECEVT2` writer - TYPECEVT2"]
pub type Typecevt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPEC_VSTATE_CC1` reader - TYPEC_VSTATE_CC1"]
pub type TypecVstateCc1R = crate::FieldReader;
#[doc = "Field `TYPEC_VSTATE_CC1` writer - TYPEC_VSTATE_CC1"]
pub type TypecVstateCc1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TYPEC_VSTATE_CC2` reader - TYPEC_VSTATE_CC2"]
pub type TypecVstateCc2R = crate::FieldReader;
#[doc = "Field `TYPEC_VSTATE_CC2` writer - TYPEC_VSTATE_CC2"]
pub type TypecVstateCc2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FRSEVT` reader - FRSEVT"]
pub type FrsevtR = crate::BitReader;
#[doc = "Field `FRSEVT` writer - FRSEVT"]
pub type FrsevtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TXIS"]
    #[inline(always)]
    pub fn txis(&self) -> TxisR {
        TxisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXMSGDISC"]
    #[inline(always)]
    pub fn txmsgdisc(&self) -> TxmsgdiscR {
        TxmsgdiscR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXMSGSENT"]
    #[inline(always)]
    pub fn txmsgsent(&self) -> TxmsgsentR {
        TxmsgsentR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TXMSGABT"]
    #[inline(always)]
    pub fn txmsgabt(&self) -> TxmsgabtR {
        TxmsgabtR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HRSTDISC"]
    #[inline(always)]
    pub fn hrstdisc(&self) -> HrstdiscR {
        HrstdiscR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HRSTSENT"]
    #[inline(always)]
    pub fn hrstsent(&self) -> HrstsentR {
        HrstsentR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXUND"]
    #[inline(always)]
    pub fn txund(&self) -> TxundR {
        TxundR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - RXNE"]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXORDDET"]
    #[inline(always)]
    pub fn rxorddet(&self) -> RxorddetR {
        RxorddetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RXHRSTDET"]
    #[inline(always)]
    pub fn rxhrstdet(&self) -> RxhrstdetR {
        RxhrstdetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RXOVR"]
    #[inline(always)]
    pub fn rxovr(&self) -> RxovrR {
        RxovrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RXMSGEND"]
    #[inline(always)]
    pub fn rxmsgend(&self) -> RxmsgendR {
        RxmsgendR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RXERR"]
    #[inline(always)]
    pub fn rxerr(&self) -> RxerrR {
        RxerrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TYPECEVT1"]
    #[inline(always)]
    pub fn typecevt1(&self) -> Typecevt1R {
        Typecevt1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TYPECEVT2"]
    #[inline(always)]
    pub fn typecevt2(&self) -> Typecevt2R {
        Typecevt2R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - TYPEC_VSTATE_CC1"]
    #[inline(always)]
    pub fn typec_vstate_cc1(&self) -> TypecVstateCc1R {
        TypecVstateCc1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - TYPEC_VSTATE_CC2"]
    #[inline(always)]
    pub fn typec_vstate_cc2(&self) -> TypecVstateCc2R {
        TypecVstateCc2R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - FRSEVT"]
    #[inline(always)]
    pub fn frsevt(&self) -> FrsevtR {
        FrsevtR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("txis", &self.txis())
            .field("txmsgdisc", &self.txmsgdisc())
            .field("txmsgsent", &self.txmsgsent())
            .field("txmsgabt", &self.txmsgabt())
            .field("hrstdisc", &self.hrstdisc())
            .field("hrstsent", &self.hrstsent())
            .field("txund", &self.txund())
            .field("rxne", &self.rxne())
            .field("rxorddet", &self.rxorddet())
            .field("rxhrstdet", &self.rxhrstdet())
            .field("rxovr", &self.rxovr())
            .field("rxmsgend", &self.rxmsgend())
            .field("rxerr", &self.rxerr())
            .field("typecevt1", &self.typecevt1())
            .field("typecevt2", &self.typecevt2())
            .field("typec_vstate_cc1", &self.typec_vstate_cc1())
            .field("typec_vstate_cc2", &self.typec_vstate_cc2())
            .field("frsevt", &self.frsevt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - TXIS"]
    #[inline(always)]
    pub fn txis(&mut self) -> TxisW<'_, SrSpec> {
        TxisW::new(self, 0)
    }
    #[doc = "Bit 1 - TXMSGDISC"]
    #[inline(always)]
    pub fn txmsgdisc(&mut self) -> TxmsgdiscW<'_, SrSpec> {
        TxmsgdiscW::new(self, 1)
    }
    #[doc = "Bit 2 - TXMSGSENT"]
    #[inline(always)]
    pub fn txmsgsent(&mut self) -> TxmsgsentW<'_, SrSpec> {
        TxmsgsentW::new(self, 2)
    }
    #[doc = "Bit 3 - TXMSGABT"]
    #[inline(always)]
    pub fn txmsgabt(&mut self) -> TxmsgabtW<'_, SrSpec> {
        TxmsgabtW::new(self, 3)
    }
    #[doc = "Bit 4 - HRSTDISC"]
    #[inline(always)]
    pub fn hrstdisc(&mut self) -> HrstdiscW<'_, SrSpec> {
        HrstdiscW::new(self, 4)
    }
    #[doc = "Bit 5 - HRSTSENT"]
    #[inline(always)]
    pub fn hrstsent(&mut self) -> HrstsentW<'_, SrSpec> {
        HrstsentW::new(self, 5)
    }
    #[doc = "Bit 6 - TXUND"]
    #[inline(always)]
    pub fn txund(&mut self) -> TxundW<'_, SrSpec> {
        TxundW::new(self, 6)
    }
    #[doc = "Bit 8 - RXNE"]
    #[inline(always)]
    pub fn rxne(&mut self) -> RxneW<'_, SrSpec> {
        RxneW::new(self, 8)
    }
    #[doc = "Bit 9 - RXORDDET"]
    #[inline(always)]
    pub fn rxorddet(&mut self) -> RxorddetW<'_, SrSpec> {
        RxorddetW::new(self, 9)
    }
    #[doc = "Bit 10 - RXHRSTDET"]
    #[inline(always)]
    pub fn rxhrstdet(&mut self) -> RxhrstdetW<'_, SrSpec> {
        RxhrstdetW::new(self, 10)
    }
    #[doc = "Bit 11 - RXOVR"]
    #[inline(always)]
    pub fn rxovr(&mut self) -> RxovrW<'_, SrSpec> {
        RxovrW::new(self, 11)
    }
    #[doc = "Bit 12 - RXMSGEND"]
    #[inline(always)]
    pub fn rxmsgend(&mut self) -> RxmsgendW<'_, SrSpec> {
        RxmsgendW::new(self, 12)
    }
    #[doc = "Bit 13 - RXERR"]
    #[inline(always)]
    pub fn rxerr(&mut self) -> RxerrW<'_, SrSpec> {
        RxerrW::new(self, 13)
    }
    #[doc = "Bit 14 - TYPECEVT1"]
    #[inline(always)]
    pub fn typecevt1(&mut self) -> Typecevt1W<'_, SrSpec> {
        Typecevt1W::new(self, 14)
    }
    #[doc = "Bit 15 - TYPECEVT2"]
    #[inline(always)]
    pub fn typecevt2(&mut self) -> Typecevt2W<'_, SrSpec> {
        Typecevt2W::new(self, 15)
    }
    #[doc = "Bits 16:17 - TYPEC_VSTATE_CC1"]
    #[inline(always)]
    pub fn typec_vstate_cc1(&mut self) -> TypecVstateCc1W<'_, SrSpec> {
        TypecVstateCc1W::new(self, 16)
    }
    #[doc = "Bits 18:19 - TYPEC_VSTATE_CC2"]
    #[inline(always)]
    pub fn typec_vstate_cc2(&mut self) -> TypecVstateCc2W<'_, SrSpec> {
        TypecVstateCc2W::new(self, 18)
    }
    #[doc = "Bit 20 - FRSEVT"]
    #[inline(always)]
    pub fn frsevt(&mut self) -> FrsevtW<'_, SrSpec> {
        FrsevtW::new(self, 20)
    }
}
#[doc = "UCPD Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}

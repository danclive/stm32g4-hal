#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `TXMODE` reader - TXMODE"]
pub type TxmodeR = crate::FieldReader;
#[doc = "Field `TXMODE` writer - TXMODE"]
pub type TxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXSEND` reader - TXSEND"]
pub type TxsendR = crate::BitReader;
#[doc = "Field `TXSEND` writer - TXSEND"]
pub type TxsendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXHRST` reader - TXHRST"]
pub type TxhrstR = crate::BitReader;
#[doc = "Field `TXHRST` writer - TXHRST"]
pub type TxhrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMODE` reader - RXMODE"]
pub type RxmodeR = crate::BitReader;
#[doc = "Field `RXMODE` writer - RXMODE"]
pub type RxmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYRXEN` reader - PHYRXEN"]
pub type PhyrxenR = crate::BitReader;
#[doc = "Field `PHYRXEN` writer - PHYRXEN"]
pub type PhyrxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYCCSEL` reader - PHYCCSEL"]
pub type PhyccselR = crate::BitReader;
#[doc = "Field `PHYCCSEL` writer - PHYCCSEL"]
pub type PhyccselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANASUBMODE` reader - ANASUBMODE"]
pub type AnasubmodeR = crate::FieldReader;
#[doc = "Field `ANASUBMODE` writer - ANASUBMODE"]
pub type AnasubmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ANAMODE` reader - ANAMODE"]
pub type AnamodeR = crate::BitReader;
#[doc = "Field `ANAMODE` writer - ANAMODE"]
pub type AnamodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCENABLE` reader - CCENABLE"]
pub type CcenableR = crate::FieldReader;
#[doc = "Field `CCENABLE` writer - CCENABLE"]
pub type CcenableW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FRSRXEN` reader - FRSRXEN"]
pub type FrsrxenR = crate::BitReader;
#[doc = "Field `FRSRXEN` writer - FRSRXEN"]
pub type FrsrxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRSTX` reader - FRSTX"]
pub type FrstxR = crate::BitReader;
#[doc = "Field `FRSTX` writer - FRSTX"]
pub type FrstxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDCH` reader - RDCH"]
pub type RdchR = crate::BitReader;
#[doc = "Field `RDCH` writer - RDCH"]
pub type RdchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1TCDIS` reader - CC1TCDIS"]
pub type Cc1tcdisR = crate::BitReader;
#[doc = "Field `CC1TCDIS` writer - CC1TCDIS"]
pub type Cc1tcdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2TCDIS` reader - CC2TCDIS"]
pub type Cc2tcdisR = crate::BitReader;
#[doc = "Field `CC2TCDIS` writer - CC2TCDIS"]
pub type Cc2tcdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - TXMODE"]
    #[inline(always)]
    pub fn txmode(&self) -> TxmodeR {
        TxmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - TXSEND"]
    #[inline(always)]
    pub fn txsend(&self) -> TxsendR {
        TxsendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TXHRST"]
    #[inline(always)]
    pub fn txhrst(&self) -> TxhrstR {
        TxhrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXMODE"]
    #[inline(always)]
    pub fn rxmode(&self) -> RxmodeR {
        RxmodeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PHYRXEN"]
    #[inline(always)]
    pub fn phyrxen(&self) -> PhyrxenR {
        PhyrxenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PHYCCSEL"]
    #[inline(always)]
    pub fn phyccsel(&self) -> PhyccselR {
        PhyccselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - ANASUBMODE"]
    #[inline(always)]
    pub fn anasubmode(&self) -> AnasubmodeR {
        AnasubmodeR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - ANAMODE"]
    #[inline(always)]
    pub fn anamode(&self) -> AnamodeR {
        AnamodeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - CCENABLE"]
    #[inline(always)]
    pub fn ccenable(&self) -> CcenableR {
        CcenableR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 16 - FRSRXEN"]
    #[inline(always)]
    pub fn frsrxen(&self) -> FrsrxenR {
        FrsrxenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FRSTX"]
    #[inline(always)]
    pub fn frstx(&self) -> FrstxR {
        FrstxR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RDCH"]
    #[inline(always)]
    pub fn rdch(&self) -> RdchR {
        RdchR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - CC1TCDIS"]
    #[inline(always)]
    pub fn cc1tcdis(&self) -> Cc1tcdisR {
        Cc1tcdisR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CC2TCDIS"]
    #[inline(always)]
    pub fn cc2tcdis(&self) -> Cc2tcdisR {
        Cc2tcdisR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("txmode", &self.txmode())
            .field("txsend", &self.txsend())
            .field("txhrst", &self.txhrst())
            .field("rxmode", &self.rxmode())
            .field("phyrxen", &self.phyrxen())
            .field("phyccsel", &self.phyccsel())
            .field("anasubmode", &self.anasubmode())
            .field("anamode", &self.anamode())
            .field("ccenable", &self.ccenable())
            .field("frsrxen", &self.frsrxen())
            .field("frstx", &self.frstx())
            .field("rdch", &self.rdch())
            .field("cc1tcdis", &self.cc1tcdis())
            .field("cc2tcdis", &self.cc2tcdis())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - TXMODE"]
    #[inline(always)]
    pub fn txmode(&mut self) -> TxmodeW<'_, CrSpec> {
        TxmodeW::new(self, 0)
    }
    #[doc = "Bit 2 - TXSEND"]
    #[inline(always)]
    pub fn txsend(&mut self) -> TxsendW<'_, CrSpec> {
        TxsendW::new(self, 2)
    }
    #[doc = "Bit 3 - TXHRST"]
    #[inline(always)]
    pub fn txhrst(&mut self) -> TxhrstW<'_, CrSpec> {
        TxhrstW::new(self, 3)
    }
    #[doc = "Bit 4 - RXMODE"]
    #[inline(always)]
    pub fn rxmode(&mut self) -> RxmodeW<'_, CrSpec> {
        RxmodeW::new(self, 4)
    }
    #[doc = "Bit 5 - PHYRXEN"]
    #[inline(always)]
    pub fn phyrxen(&mut self) -> PhyrxenW<'_, CrSpec> {
        PhyrxenW::new(self, 5)
    }
    #[doc = "Bit 6 - PHYCCSEL"]
    #[inline(always)]
    pub fn phyccsel(&mut self) -> PhyccselW<'_, CrSpec> {
        PhyccselW::new(self, 6)
    }
    #[doc = "Bits 7:8 - ANASUBMODE"]
    #[inline(always)]
    pub fn anasubmode(&mut self) -> AnasubmodeW<'_, CrSpec> {
        AnasubmodeW::new(self, 7)
    }
    #[doc = "Bit 9 - ANAMODE"]
    #[inline(always)]
    pub fn anamode(&mut self) -> AnamodeW<'_, CrSpec> {
        AnamodeW::new(self, 9)
    }
    #[doc = "Bits 10:11 - CCENABLE"]
    #[inline(always)]
    pub fn ccenable(&mut self) -> CcenableW<'_, CrSpec> {
        CcenableW::new(self, 10)
    }
    #[doc = "Bit 16 - FRSRXEN"]
    #[inline(always)]
    pub fn frsrxen(&mut self) -> FrsrxenW<'_, CrSpec> {
        FrsrxenW::new(self, 16)
    }
    #[doc = "Bit 17 - FRSTX"]
    #[inline(always)]
    pub fn frstx(&mut self) -> FrstxW<'_, CrSpec> {
        FrstxW::new(self, 17)
    }
    #[doc = "Bit 18 - RDCH"]
    #[inline(always)]
    pub fn rdch(&mut self) -> RdchW<'_, CrSpec> {
        RdchW::new(self, 18)
    }
    #[doc = "Bit 20 - CC1TCDIS"]
    #[inline(always)]
    pub fn cc1tcdis(&mut self) -> Cc1tcdisW<'_, CrSpec> {
        Cc1tcdisW::new(self, 20)
    }
    #[doc = "Bit 21 - CC2TCDIS"]
    #[inline(always)]
    pub fn cc2tcdis(&mut self) -> Cc2tcdisW<'_, CrSpec> {
        Cc2tcdisW::new(self, 21)
    }
}
#[doc = "UCPD configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}

#[doc = "Register `EP6R` reader"]
pub type R = crate::R<Ep6rSpec>;
#[doc = "Register `EP6R` writer"]
pub type W = crate::W<Ep6rSpec>;
#[doc = "Field `EA` reader - EA"]
pub type EaR = crate::FieldReader;
#[doc = "Field `EA` writer - EA"]
pub type EaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STAT_TX` reader - STAT_TX"]
pub type StatTxR = crate::FieldReader;
#[doc = "Field `STAT_TX` writer - STAT_TX"]
pub type StatTxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTOG_TX` reader - DTOG_TX"]
pub type DtogTxR = crate::BitReader;
#[doc = "Field `DTOG_TX` writer - DTOG_TX"]
pub type DtogTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR_TX` reader - CTR_TX"]
pub type CtrTxR = crate::BitReader;
#[doc = "Field `CTR_TX` writer - CTR_TX"]
pub type CtrTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_KIND` reader - EP_KIND"]
pub type EpKindR = crate::BitReader;
#[doc = "Field `EP_KIND` writer - EP_KIND"]
pub type EpKindW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_TYPE` reader - EP_TYPE"]
pub type EpTypeR = crate::FieldReader;
#[doc = "Field `EP_TYPE` writer - EP_TYPE"]
pub type EpTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SETUP` reader - SETUP"]
pub type SetupR = crate::BitReader;
#[doc = "Field `SETUP` writer - SETUP"]
pub type SetupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STAT_RX` reader - STAT_RX"]
pub type StatRxR = crate::FieldReader;
#[doc = "Field `STAT_RX` writer - STAT_RX"]
pub type StatRxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTOG_RX` reader - DTOG_RX"]
pub type DtogRxR = crate::BitReader;
#[doc = "Field `DTOG_RX` writer - DTOG_RX"]
pub type DtogRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR_RX` reader - CTR_RX"]
pub type CtrRxR = crate::BitReader;
#[doc = "Field `CTR_RX` writer - CTR_RX"]
pub type CtrRxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - EA"]
    #[inline(always)]
    pub fn ea(&self) -> EaR {
        EaR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - STAT_TX"]
    #[inline(always)]
    pub fn stat_tx(&self) -> StatTxR {
        StatTxR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - DTOG_TX"]
    #[inline(always)]
    pub fn dtog_tx(&self) -> DtogTxR {
        DtogTxR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CTR_TX"]
    #[inline(always)]
    pub fn ctr_tx(&self) -> CtrTxR {
        CtrTxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EP_KIND"]
    #[inline(always)]
    pub fn ep_kind(&self) -> EpKindR {
        EpKindR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - EP_TYPE"]
    #[inline(always)]
    pub fn ep_type(&self) -> EpTypeR {
        EpTypeR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - SETUP"]
    #[inline(always)]
    pub fn setup(&self) -> SetupR {
        SetupR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STAT_RX"]
    #[inline(always)]
    pub fn stat_rx(&self) -> StatRxR {
        StatRxR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - DTOG_RX"]
    #[inline(always)]
    pub fn dtog_rx(&self) -> DtogRxR {
        DtogRxR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CTR_RX"]
    #[inline(always)]
    pub fn ctr_rx(&self) -> CtrRxR {
        CtrRxR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EP6R")
            .field("ea", &self.ea())
            .field("stat_tx", &self.stat_tx())
            .field("dtog_tx", &self.dtog_tx())
            .field("ctr_tx", &self.ctr_tx())
            .field("ep_kind", &self.ep_kind())
            .field("ep_type", &self.ep_type())
            .field("setup", &self.setup())
            .field("stat_rx", &self.stat_rx())
            .field("dtog_rx", &self.dtog_rx())
            .field("ctr_rx", &self.ctr_rx())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - EA"]
    #[inline(always)]
    pub fn ea(&mut self) -> EaW<Ep6rSpec> {
        EaW::new(self, 0)
    }
    #[doc = "Bits 4:5 - STAT_TX"]
    #[inline(always)]
    pub fn stat_tx(&mut self) -> StatTxW<Ep6rSpec> {
        StatTxW::new(self, 4)
    }
    #[doc = "Bit 6 - DTOG_TX"]
    #[inline(always)]
    pub fn dtog_tx(&mut self) -> DtogTxW<Ep6rSpec> {
        DtogTxW::new(self, 6)
    }
    #[doc = "Bit 7 - CTR_TX"]
    #[inline(always)]
    pub fn ctr_tx(&mut self) -> CtrTxW<Ep6rSpec> {
        CtrTxW::new(self, 7)
    }
    #[doc = "Bit 8 - EP_KIND"]
    #[inline(always)]
    pub fn ep_kind(&mut self) -> EpKindW<Ep6rSpec> {
        EpKindW::new(self, 8)
    }
    #[doc = "Bits 9:10 - EP_TYPE"]
    #[inline(always)]
    pub fn ep_type(&mut self) -> EpTypeW<Ep6rSpec> {
        EpTypeW::new(self, 9)
    }
    #[doc = "Bit 11 - SETUP"]
    #[inline(always)]
    pub fn setup(&mut self) -> SetupW<Ep6rSpec> {
        SetupW::new(self, 11)
    }
    #[doc = "Bits 12:13 - STAT_RX"]
    #[inline(always)]
    pub fn stat_rx(&mut self) -> StatRxW<Ep6rSpec> {
        StatRxW::new(self, 12)
    }
    #[doc = "Bit 14 - DTOG_RX"]
    #[inline(always)]
    pub fn dtog_rx(&mut self) -> DtogRxW<Ep6rSpec> {
        DtogRxW::new(self, 14)
    }
    #[doc = "Bit 15 - CTR_RX"]
    #[inline(always)]
    pub fn ctr_rx(&mut self) -> CtrRxW<Ep6rSpec> {
        CtrRxW::new(self, 15)
    }
}
#[doc = "USB endpoint n register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep6r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep6r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep6rSpec;
impl crate::RegisterSpec for Ep6rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep6r::R`](R) reader structure"]
impl crate::Readable for Ep6rSpec {}
#[doc = "`write(|w| ..)` method takes [`ep6r::W`](W) writer structure"]
impl crate::Writable for Ep6rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP6R to value 0"]
impl crate::Resettable for Ep6rSpec {}

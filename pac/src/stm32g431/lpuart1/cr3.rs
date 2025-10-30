#[doc = "Register `CR3` reader"]
pub type R = crate::R<Cr3Spec>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<Cr3Spec>;
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub type EieR = crate::BitReader;
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub type EieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDSEL` reader - Half-duplex selection"]
pub type HdselR = crate::BitReader;
#[doc = "Field `HDSEL` writer - Half-duplex selection"]
pub type HdselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAR` reader - DMA enable receiver"]
pub type DmarR = crate::BitReader;
#[doc = "Field `DMAR` writer - DMA enable receiver"]
pub type DmarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAT` reader - DMA enable transmitter"]
pub type DmatR = crate::BitReader;
#[doc = "Field `DMAT` writer - DMA enable transmitter"]
pub type DmatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSE` reader - RTS enable"]
pub type RtseR = crate::BitReader;
#[doc = "Field `RTSE` writer - RTS enable"]
pub type RtseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSE` reader - CTS enable"]
pub type CtseR = crate::BitReader;
#[doc = "Field `CTSE` writer - CTS enable"]
pub type CtseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSIE` reader - CTS interrupt enable"]
pub type CtsieR = crate::BitReader;
#[doc = "Field `CTSIE` writer - CTS interrupt enable"]
pub type CtsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRDIS` reader - Overrun Disable"]
pub type OvrdisR = crate::BitReader;
#[doc = "Field `OVRDIS` writer - Overrun Disable"]
pub type OvrdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRE` reader - DMA Disable on Reception Error"]
pub type DdreR = crate::BitReader;
#[doc = "Field `DDRE` writer - DMA Disable on Reception Error"]
pub type DdreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEM` reader - Driver enable mode"]
pub type DemR = crate::BitReader;
#[doc = "Field `DEM` writer - Driver enable mode"]
pub type DemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEP` reader - Driver enable polarity selection"]
pub type DepR = crate::BitReader;
#[doc = "Field `DEP` writer - Driver enable polarity selection"]
pub type DepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUS` reader - Wakeup from Stop mode interrupt flag selection"]
pub type WusR = crate::FieldReader;
#[doc = "Field `WUS` writer - Wakeup from Stop mode interrupt flag selection"]
pub type WusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WUFIE` reader - Wakeup from Stop mode interrupt enable"]
pub type WufieR = crate::BitReader;
#[doc = "Field `WUFIE` writer - Wakeup from Stop mode interrupt enable"]
pub type WufieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFTIE` reader - TXFTIE"]
pub type TxftieR = crate::BitReader;
#[doc = "Field `TXFTIE` writer - TXFTIE"]
pub type TxftieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFTCFG` reader - RXFTCFG"]
pub type RxftcfgR = crate::FieldReader;
#[doc = "Field `RXFTCFG` writer - RXFTCFG"]
pub type RxftcfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RXFTIE` reader - RXFTIE"]
pub type RxftieR = crate::BitReader;
#[doc = "Field `RXFTIE` writer - RXFTIE"]
pub type RxftieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFTCFG` reader - TXFTCFG"]
pub type TxftcfgR = crate::FieldReader;
#[doc = "Field `TXFTCFG` writer - TXFTCFG"]
pub type TxftcfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EieR {
        EieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hdsel(&self) -> HdselR {
        HdselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn dmar(&self) -> DmarR {
        DmarR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dmat(&self) -> DmatR {
        DmatR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtse(&self) -> RtseR {
        RtseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctse(&self) -> CtseR {
        CtseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&self) -> CtsieR {
        CtsieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Overrun Disable"]
    #[inline(always)]
    pub fn ovrdis(&self) -> OvrdisR {
        OvrdisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error"]
    #[inline(always)]
    pub fn ddre(&self) -> DdreR {
        DdreR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Driver enable mode"]
    #[inline(always)]
    pub fn dem(&self) -> DemR {
        DemR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Driver enable polarity selection"]
    #[inline(always)]
    pub fn dep(&self) -> DepR {
        DepR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Wakeup from Stop mode interrupt flag selection"]
    #[inline(always)]
    pub fn wus(&self) -> WusR {
        WusR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Wakeup from Stop mode interrupt enable"]
    #[inline(always)]
    pub fn wufie(&self) -> WufieR {
        WufieR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TXFTIE"]
    #[inline(always)]
    pub fn txftie(&self) -> TxftieR {
        TxftieR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 25:27 - RXFTCFG"]
    #[inline(always)]
    pub fn rxftcfg(&self) -> RxftcfgR {
        RxftcfgR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - RXFTIE"]
    #[inline(always)]
    pub fn rxftie(&self) -> RxftieR {
        RxftieR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - TXFTCFG"]
    #[inline(always)]
    pub fn txftcfg(&self) -> TxftcfgR {
        TxftcfgR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("txftcfg", &self.txftcfg())
            .field("rxftie", &self.rxftie())
            .field("rxftcfg", &self.rxftcfg())
            .field("txftie", &self.txftie())
            .field("wufie", &self.wufie())
            .field("wus", &self.wus())
            .field("dep", &self.dep())
            .field("dem", &self.dem())
            .field("ddre", &self.ddre())
            .field("ovrdis", &self.ovrdis())
            .field("ctsie", &self.ctsie())
            .field("ctse", &self.ctse())
            .field("rtse", &self.rtse())
            .field("dmat", &self.dmat())
            .field("dmar", &self.dmar())
            .field("hdsel", &self.hdsel())
            .field("eie", &self.eie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&mut self) -> EieW<'_, Cr3Spec> {
        EieW::new(self, 0)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hdsel(&mut self) -> HdselW<'_, Cr3Spec> {
        HdselW::new(self, 3)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn dmar(&mut self) -> DmarW<'_, Cr3Spec> {
        DmarW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dmat(&mut self) -> DmatW<'_, Cr3Spec> {
        DmatW::new(self, 7)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtse(&mut self) -> RtseW<'_, Cr3Spec> {
        RtseW::new(self, 8)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctse(&mut self) -> CtseW<'_, Cr3Spec> {
        CtseW::new(self, 9)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&mut self) -> CtsieW<'_, Cr3Spec> {
        CtsieW::new(self, 10)
    }
    #[doc = "Bit 12 - Overrun Disable"]
    #[inline(always)]
    pub fn ovrdis(&mut self) -> OvrdisW<'_, Cr3Spec> {
        OvrdisW::new(self, 12)
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error"]
    #[inline(always)]
    pub fn ddre(&mut self) -> DdreW<'_, Cr3Spec> {
        DdreW::new(self, 13)
    }
    #[doc = "Bit 14 - Driver enable mode"]
    #[inline(always)]
    pub fn dem(&mut self) -> DemW<'_, Cr3Spec> {
        DemW::new(self, 14)
    }
    #[doc = "Bit 15 - Driver enable polarity selection"]
    #[inline(always)]
    pub fn dep(&mut self) -> DepW<'_, Cr3Spec> {
        DepW::new(self, 15)
    }
    #[doc = "Bits 20:21 - Wakeup from Stop mode interrupt flag selection"]
    #[inline(always)]
    pub fn wus(&mut self) -> WusW<'_, Cr3Spec> {
        WusW::new(self, 20)
    }
    #[doc = "Bit 22 - Wakeup from Stop mode interrupt enable"]
    #[inline(always)]
    pub fn wufie(&mut self) -> WufieW<'_, Cr3Spec> {
        WufieW::new(self, 22)
    }
    #[doc = "Bit 23 - TXFTIE"]
    #[inline(always)]
    pub fn txftie(&mut self) -> TxftieW<'_, Cr3Spec> {
        TxftieW::new(self, 23)
    }
    #[doc = "Bits 25:27 - RXFTCFG"]
    #[inline(always)]
    pub fn rxftcfg(&mut self) -> RxftcfgW<'_, Cr3Spec> {
        RxftcfgW::new(self, 25)
    }
    #[doc = "Bit 28 - RXFTIE"]
    #[inline(always)]
    pub fn rxftie(&mut self) -> RxftieW<'_, Cr3Spec> {
        RxftieW::new(self, 28)
    }
    #[doc = "Bits 29:31 - TXFTCFG"]
    #[inline(always)]
    pub fn txftcfg(&mut self) -> TxftcfgW<'_, Cr3Spec> {
        TxftcfgW::new(self, 29)
    }
}
#[doc = "Control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr3Spec;
impl crate::RegisterSpec for Cr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for Cr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for Cr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for Cr3Spec {}

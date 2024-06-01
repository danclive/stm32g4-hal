#[doc = "Register `OUTAR` reader"]
pub type R = crate::R<OutarSpec>;
#[doc = "Register `OUTAR` writer"]
pub type W = crate::W<OutarSpec>;
#[doc = "Field `POL1` reader - Output 1 polarity"]
pub type Pol1R = crate::BitReader;
#[doc = "Field `POL1` writer - Output 1 polarity"]
pub type Pol1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEM1` reader - Output 1 Idle mode"]
pub type Idlem1R = crate::BitReader;
#[doc = "Field `IDLEM1` writer - Output 1 Idle mode"]
pub type Idlem1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLES1` reader - Output 1 Idle State"]
pub type Idles1R = crate::BitReader;
#[doc = "Field `IDLES1` writer - Output 1 Idle State"]
pub type Idles1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT1` reader - Output 1 Fault state"]
pub type Fault1R = crate::FieldReader;
#[doc = "Field `FAULT1` writer - Output 1 Fault state"]
pub type Fault1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHP1` reader - Output 1 Chopper enable"]
pub type Chp1R = crate::BitReader;
#[doc = "Field `CHP1` writer - Output 1 Chopper enable"]
pub type Chp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIDL1` reader - Output 1 Deadtime upon burst mode Idle entry"]
pub type Didl1R = crate::BitReader;
#[doc = "Field `DIDL1` writer - Output 1 Deadtime upon burst mode Idle entry"]
pub type Didl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTEN` reader - Deadtime enable"]
pub type DtenR = crate::BitReader;
#[doc = "Field `DTEN` writer - Deadtime enable"]
pub type DtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLYPRTEN` reader - Delayed Protection Enable"]
pub type DlyprtenR = crate::BitReader;
#[doc = "Field `DLYPRTEN` writer - Delayed Protection Enable"]
pub type DlyprtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLYPRT` reader - Delayed Protection"]
pub type DlyprtR = crate::FieldReader;
#[doc = "Field `DLYPRT` writer - Delayed Protection"]
pub type DlyprtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIAR` reader - Balanced Idle Automatic Resume"]
pub type BiarR = crate::BitReader;
#[doc = "Field `BIAR` writer - Balanced Idle Automatic Resume"]
pub type BiarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL2` reader - Output 2 polarity"]
pub type Pol2R = crate::BitReader;
#[doc = "Field `POL2` writer - Output 2 polarity"]
pub type Pol2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEM2` reader - Output 2 Idle mode"]
pub type Idlem2R = crate::BitReader;
#[doc = "Field `IDLEM2` writer - Output 2 Idle mode"]
pub type Idlem2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLES2` reader - Output 2 Idle State"]
pub type Idles2R = crate::BitReader;
#[doc = "Field `IDLES2` writer - Output 2 Idle State"]
pub type Idles2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT2` reader - Output 2 Fault state"]
pub type Fault2R = crate::FieldReader;
#[doc = "Field `FAULT2` writer - Output 2 Fault state"]
pub type Fault2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHP2` reader - Output 2 Chopper enable"]
pub type Chp2R = crate::BitReader;
#[doc = "Field `CHP2` writer - Output 2 Chopper enable"]
pub type Chp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIDL2` reader - Output 2 Deadtime upon burst mode Idle entry"]
pub type Didl2R = crate::BitReader;
#[doc = "Field `DIDL2` writer - Output 2 Deadtime upon burst mode Idle entry"]
pub type Didl2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Output 1 polarity"]
    #[inline(always)]
    pub fn pol1(&self) -> Pol1R {
        Pol1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output 1 Idle mode"]
    #[inline(always)]
    pub fn idlem1(&self) -> Idlem1R {
        Idlem1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output 1 Idle State"]
    #[inline(always)]
    pub fn idles1(&self) -> Idles1R {
        Idles1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Output 1 Fault state"]
    #[inline(always)]
    pub fn fault1(&self) -> Fault1R {
        Fault1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Output 1 Chopper enable"]
    #[inline(always)]
    pub fn chp1(&self) -> Chp1R {
        Chp1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output 1 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    pub fn didl1(&self) -> Didl1R {
        Didl1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Deadtime enable"]
    #[inline(always)]
    pub fn dten(&self) -> DtenR {
        DtenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Delayed Protection Enable"]
    #[inline(always)]
    pub fn dlyprten(&self) -> DlyprtenR {
        DlyprtenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Delayed Protection"]
    #[inline(always)]
    pub fn dlyprt(&self) -> DlyprtR {
        DlyprtR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 14 - Balanced Idle Automatic Resume"]
    #[inline(always)]
    pub fn biar(&self) -> BiarR {
        BiarR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - Output 2 polarity"]
    #[inline(always)]
    pub fn pol2(&self) -> Pol2R {
        Pol2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output 2 Idle mode"]
    #[inline(always)]
    pub fn idlem2(&self) -> Idlem2R {
        Idlem2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output 2 Idle State"]
    #[inline(always)]
    pub fn idles2(&self) -> Idles2R {
        Idles2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Output 2 Fault state"]
    #[inline(always)]
    pub fn fault2(&self) -> Fault2R {
        Fault2R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Output 2 Chopper enable"]
    #[inline(always)]
    pub fn chp2(&self) -> Chp2R {
        Chp2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Output 2 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    pub fn didl2(&self) -> Didl2R {
        Didl2R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTAR")
            .field("didl2", &self.didl2())
            .field("chp2", &self.chp2())
            .field("fault2", &self.fault2())
            .field("idles2", &self.idles2())
            .field("idlem2", &self.idlem2())
            .field("pol2", &self.pol2())
            .field("biar", &self.biar())
            .field("dlyprt", &self.dlyprt())
            .field("dlyprten", &self.dlyprten())
            .field("dten", &self.dten())
            .field("didl1", &self.didl1())
            .field("chp1", &self.chp1())
            .field("fault1", &self.fault1())
            .field("idles1", &self.idles1())
            .field("idlem1", &self.idlem1())
            .field("pol1", &self.pol1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Output 1 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol1(&mut self) -> Pol1W<OutarSpec> {
        Pol1W::new(self, 1)
    }
    #[doc = "Bit 2 - Output 1 Idle mode"]
    #[inline(always)]
    #[must_use]
    pub fn idlem1(&mut self) -> Idlem1W<OutarSpec> {
        Idlem1W::new(self, 2)
    }
    #[doc = "Bit 3 - Output 1 Idle State"]
    #[inline(always)]
    #[must_use]
    pub fn idles1(&mut self) -> Idles1W<OutarSpec> {
        Idles1W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Output 1 Fault state"]
    #[inline(always)]
    #[must_use]
    pub fn fault1(&mut self) -> Fault1W<OutarSpec> {
        Fault1W::new(self, 4)
    }
    #[doc = "Bit 6 - Output 1 Chopper enable"]
    #[inline(always)]
    #[must_use]
    pub fn chp1(&mut self) -> Chp1W<OutarSpec> {
        Chp1W::new(self, 6)
    }
    #[doc = "Bit 7 - Output 1 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    #[must_use]
    pub fn didl1(&mut self) -> Didl1W<OutarSpec> {
        Didl1W::new(self, 7)
    }
    #[doc = "Bit 8 - Deadtime enable"]
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DtenW<OutarSpec> {
        DtenW::new(self, 8)
    }
    #[doc = "Bit 9 - Delayed Protection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dlyprten(&mut self) -> DlyprtenW<OutarSpec> {
        DlyprtenW::new(self, 9)
    }
    #[doc = "Bits 10:12 - Delayed Protection"]
    #[inline(always)]
    #[must_use]
    pub fn dlyprt(&mut self) -> DlyprtW<OutarSpec> {
        DlyprtW::new(self, 10)
    }
    #[doc = "Bit 14 - Balanced Idle Automatic Resume"]
    #[inline(always)]
    #[must_use]
    pub fn biar(&mut self) -> BiarW<OutarSpec> {
        BiarW::new(self, 14)
    }
    #[doc = "Bit 17 - Output 2 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol2(&mut self) -> Pol2W<OutarSpec> {
        Pol2W::new(self, 17)
    }
    #[doc = "Bit 18 - Output 2 Idle mode"]
    #[inline(always)]
    #[must_use]
    pub fn idlem2(&mut self) -> Idlem2W<OutarSpec> {
        Idlem2W::new(self, 18)
    }
    #[doc = "Bit 19 - Output 2 Idle State"]
    #[inline(always)]
    #[must_use]
    pub fn idles2(&mut self) -> Idles2W<OutarSpec> {
        Idles2W::new(self, 19)
    }
    #[doc = "Bits 20:21 - Output 2 Fault state"]
    #[inline(always)]
    #[must_use]
    pub fn fault2(&mut self) -> Fault2W<OutarSpec> {
        Fault2W::new(self, 20)
    }
    #[doc = "Bit 22 - Output 2 Chopper enable"]
    #[inline(always)]
    #[must_use]
    pub fn chp2(&mut self) -> Chp2W<OutarSpec> {
        Chp2W::new(self, 22)
    }
    #[doc = "Bit 23 - Output 2 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    #[must_use]
    pub fn didl2(&mut self) -> Didl2W<OutarSpec> {
        Didl2W::new(self, 23)
    }
}
#[doc = "Timerx Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutarSpec;
impl crate::RegisterSpec for OutarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outar::R`](R) reader structure"]
impl crate::Readable for OutarSpec {}
#[doc = "`write(|w| ..)` method takes [`outar::W`](W) writer structure"]
impl crate::Writable for OutarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTAR to value 0"]
impl crate::Resettable for OutarSpec {
    const RESET_VALUE: u32 = 0;
}

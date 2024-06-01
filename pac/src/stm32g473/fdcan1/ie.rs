#[doc = "Register `IE` reader"]
pub type R = crate::R<IeSpec>;
#[doc = "Register `IE` writer"]
pub type W = crate::W<IeSpec>;
#[doc = "Field `RF0NE` reader - RF0NE"]
pub type Rf0neR = crate::BitReader;
#[doc = "Field `RF0NE` writer - RF0NE"]
pub type Rf0neW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0FE` reader - RF0FE"]
pub type Rf0feR = crate::BitReader;
#[doc = "Field `RF0FE` writer - RF0FE"]
pub type Rf0feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0LE` reader - RF0LE"]
pub type Rf0leR = crate::BitReader;
#[doc = "Field `RF0LE` writer - RF0LE"]
pub type Rf0leW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1NE` reader - RF1NE"]
pub type Rf1neR = crate::BitReader;
#[doc = "Field `RF1NE` writer - RF1NE"]
pub type Rf1neW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1FE` reader - RF1FE"]
pub type Rf1feR = crate::BitReader;
#[doc = "Field `RF1FE` writer - RF1FE"]
pub type Rf1feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1LE` reader - RF1LE"]
pub type Rf1leR = crate::BitReader;
#[doc = "Field `RF1LE` writer - RF1LE"]
pub type Rf1leW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPME` reader - HPME"]
pub type HpmeR = crate::BitReader;
#[doc = "Field `HPME` writer - HPME"]
pub type HpmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCE` reader - TCE"]
pub type TceR = crate::BitReader;
#[doc = "Field `TCE` writer - TCE"]
pub type TceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCFE` reader - TCFE"]
pub type TcfeR = crate::BitReader;
#[doc = "Field `TCFE` writer - TCFE"]
pub type TcfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFEE` reader - TFEE"]
pub type TfeeR = crate::BitReader;
#[doc = "Field `TFEE` writer - TFEE"]
pub type TfeeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFNE` reader - TEFNE"]
pub type TefneR = crate::BitReader;
#[doc = "Field `TEFNE` writer - TEFNE"]
pub type TefneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFFE` reader - TEFFE"]
pub type TeffeR = crate::BitReader;
#[doc = "Field `TEFFE` writer - TEFFE"]
pub type TeffeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFLE` reader - TEFLE"]
pub type TefleR = crate::BitReader;
#[doc = "Field `TEFLE` writer - TEFLE"]
pub type TefleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSWE` reader - TSWE"]
pub type TsweR = crate::BitReader;
#[doc = "Field `TSWE` writer - TSWE"]
pub type TsweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRAFE` reader - MRAFE"]
pub type MrafeR = crate::BitReader;
#[doc = "Field `MRAFE` writer - MRAFE"]
pub type MrafeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOOE` reader - TOOE"]
pub type TooeR = crate::BitReader;
#[doc = "Field `TOOE` writer - TOOE"]
pub type TooeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELOE` reader - ELOE"]
pub type EloeR = crate::BitReader;
#[doc = "Field `ELOE` writer - ELOE"]
pub type EloeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPE` reader - EPE"]
pub type EpeR = crate::BitReader;
#[doc = "Field `EPE` writer - EPE"]
pub type EpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWE` reader - EWE"]
pub type EweR = crate::BitReader;
#[doc = "Field `EWE` writer - EWE"]
pub type EweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOE` reader - BOE"]
pub type BoeR = crate::BitReader;
#[doc = "Field `BOE` writer - BOE"]
pub type BoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDIE` reader - WDIE"]
pub type WdieR = crate::BitReader;
#[doc = "Field `WDIE` writer - WDIE"]
pub type WdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEAE` reader - PEAE"]
pub type PeaeR = crate::BitReader;
#[doc = "Field `PEAE` writer - PEAE"]
pub type PeaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEDE` reader - PEDE"]
pub type PedeR = crate::BitReader;
#[doc = "Field `PEDE` writer - PEDE"]
pub type PedeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARAE` reader - ARAE"]
pub type AraeR = crate::BitReader;
#[doc = "Field `ARAE` writer - ARAE"]
pub type AraeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RF0NE"]
    #[inline(always)]
    pub fn rf0ne(&self) -> Rf0neR {
        Rf0neR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RF0FE"]
    #[inline(always)]
    pub fn rf0fe(&self) -> Rf0feR {
        Rf0feR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RF0LE"]
    #[inline(always)]
    pub fn rf0le(&self) -> Rf0leR {
        Rf0leR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RF1NE"]
    #[inline(always)]
    pub fn rf1ne(&self) -> Rf1neR {
        Rf1neR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RF1FE"]
    #[inline(always)]
    pub fn rf1fe(&self) -> Rf1feR {
        Rf1feR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RF1LE"]
    #[inline(always)]
    pub fn rf1le(&self) -> Rf1leR {
        Rf1leR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HPME"]
    #[inline(always)]
    pub fn hpme(&self) -> HpmeR {
        HpmeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TCE"]
    #[inline(always)]
    pub fn tce(&self) -> TceR {
        TceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TCFE"]
    #[inline(always)]
    pub fn tcfe(&self) -> TcfeR {
        TcfeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TFEE"]
    #[inline(always)]
    pub fn tfee(&self) -> TfeeR {
        TfeeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TEFNE"]
    #[inline(always)]
    pub fn tefne(&self) -> TefneR {
        TefneR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TEFFE"]
    #[inline(always)]
    pub fn teffe(&self) -> TeffeR {
        TeffeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TEFLE"]
    #[inline(always)]
    pub fn tefle(&self) -> TefleR {
        TefleR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TSWE"]
    #[inline(always)]
    pub fn tswe(&self) -> TsweR {
        TsweR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MRAFE"]
    #[inline(always)]
    pub fn mrafe(&self) -> MrafeR {
        MrafeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TOOE"]
    #[inline(always)]
    pub fn tooe(&self) -> TooeR {
        TooeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ELOE"]
    #[inline(always)]
    pub fn eloe(&self) -> EloeR {
        EloeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - EPE"]
    #[inline(always)]
    pub fn epe(&self) -> EpeR {
        EpeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - EWE"]
    #[inline(always)]
    pub fn ewe(&self) -> EweR {
        EweR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - BOE"]
    #[inline(always)]
    pub fn boe(&self) -> BoeR {
        BoeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - WDIE"]
    #[inline(always)]
    pub fn wdie(&self) -> WdieR {
        WdieR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PEAE"]
    #[inline(always)]
    pub fn peae(&self) -> PeaeR {
        PeaeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PEDE"]
    #[inline(always)]
    pub fn pede(&self) -> PedeR {
        PedeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ARAE"]
    #[inline(always)]
    pub fn arae(&self) -> AraeR {
        AraeR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IE")
            .field("rf0ne", &self.rf0ne())
            .field("rf0fe", &self.rf0fe())
            .field("rf0le", &self.rf0le())
            .field("rf1ne", &self.rf1ne())
            .field("rf1fe", &self.rf1fe())
            .field("rf1le", &self.rf1le())
            .field("hpme", &self.hpme())
            .field("tce", &self.tce())
            .field("tcfe", &self.tcfe())
            .field("tfee", &self.tfee())
            .field("tefne", &self.tefne())
            .field("teffe", &self.teffe())
            .field("tefle", &self.tefle())
            .field("tswe", &self.tswe())
            .field("mrafe", &self.mrafe())
            .field("tooe", &self.tooe())
            .field("eloe", &self.eloe())
            .field("epe", &self.epe())
            .field("ewe", &self.ewe())
            .field("boe", &self.boe())
            .field("wdie", &self.wdie())
            .field("peae", &self.peae())
            .field("pede", &self.pede())
            .field("arae", &self.arae())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RF0NE"]
    #[inline(always)]
    #[must_use]
    pub fn rf0ne(&mut self) -> Rf0neW<IeSpec> {
        Rf0neW::new(self, 0)
    }
    #[doc = "Bit 1 - RF0FE"]
    #[inline(always)]
    #[must_use]
    pub fn rf0fe(&mut self) -> Rf0feW<IeSpec> {
        Rf0feW::new(self, 1)
    }
    #[doc = "Bit 2 - RF0LE"]
    #[inline(always)]
    #[must_use]
    pub fn rf0le(&mut self) -> Rf0leW<IeSpec> {
        Rf0leW::new(self, 2)
    }
    #[doc = "Bit 3 - RF1NE"]
    #[inline(always)]
    #[must_use]
    pub fn rf1ne(&mut self) -> Rf1neW<IeSpec> {
        Rf1neW::new(self, 3)
    }
    #[doc = "Bit 4 - RF1FE"]
    #[inline(always)]
    #[must_use]
    pub fn rf1fe(&mut self) -> Rf1feW<IeSpec> {
        Rf1feW::new(self, 4)
    }
    #[doc = "Bit 5 - RF1LE"]
    #[inline(always)]
    #[must_use]
    pub fn rf1le(&mut self) -> Rf1leW<IeSpec> {
        Rf1leW::new(self, 5)
    }
    #[doc = "Bit 6 - HPME"]
    #[inline(always)]
    #[must_use]
    pub fn hpme(&mut self) -> HpmeW<IeSpec> {
        HpmeW::new(self, 6)
    }
    #[doc = "Bit 7 - TCE"]
    #[inline(always)]
    #[must_use]
    pub fn tce(&mut self) -> TceW<IeSpec> {
        TceW::new(self, 7)
    }
    #[doc = "Bit 8 - TCFE"]
    #[inline(always)]
    #[must_use]
    pub fn tcfe(&mut self) -> TcfeW<IeSpec> {
        TcfeW::new(self, 8)
    }
    #[doc = "Bit 9 - TFEE"]
    #[inline(always)]
    #[must_use]
    pub fn tfee(&mut self) -> TfeeW<IeSpec> {
        TfeeW::new(self, 9)
    }
    #[doc = "Bit 10 - TEFNE"]
    #[inline(always)]
    #[must_use]
    pub fn tefne(&mut self) -> TefneW<IeSpec> {
        TefneW::new(self, 10)
    }
    #[doc = "Bit 11 - TEFFE"]
    #[inline(always)]
    #[must_use]
    pub fn teffe(&mut self) -> TeffeW<IeSpec> {
        TeffeW::new(self, 11)
    }
    #[doc = "Bit 12 - TEFLE"]
    #[inline(always)]
    #[must_use]
    pub fn tefle(&mut self) -> TefleW<IeSpec> {
        TefleW::new(self, 12)
    }
    #[doc = "Bit 13 - TSWE"]
    #[inline(always)]
    #[must_use]
    pub fn tswe(&mut self) -> TsweW<IeSpec> {
        TsweW::new(self, 13)
    }
    #[doc = "Bit 14 - MRAFE"]
    #[inline(always)]
    #[must_use]
    pub fn mrafe(&mut self) -> MrafeW<IeSpec> {
        MrafeW::new(self, 14)
    }
    #[doc = "Bit 15 - TOOE"]
    #[inline(always)]
    #[must_use]
    pub fn tooe(&mut self) -> TooeW<IeSpec> {
        TooeW::new(self, 15)
    }
    #[doc = "Bit 16 - ELOE"]
    #[inline(always)]
    #[must_use]
    pub fn eloe(&mut self) -> EloeW<IeSpec> {
        EloeW::new(self, 16)
    }
    #[doc = "Bit 17 - EPE"]
    #[inline(always)]
    #[must_use]
    pub fn epe(&mut self) -> EpeW<IeSpec> {
        EpeW::new(self, 17)
    }
    #[doc = "Bit 18 - EWE"]
    #[inline(always)]
    #[must_use]
    pub fn ewe(&mut self) -> EweW<IeSpec> {
        EweW::new(self, 18)
    }
    #[doc = "Bit 19 - BOE"]
    #[inline(always)]
    #[must_use]
    pub fn boe(&mut self) -> BoeW<IeSpec> {
        BoeW::new(self, 19)
    }
    #[doc = "Bit 20 - WDIE"]
    #[inline(always)]
    #[must_use]
    pub fn wdie(&mut self) -> WdieW<IeSpec> {
        WdieW::new(self, 20)
    }
    #[doc = "Bit 21 - PEAE"]
    #[inline(always)]
    #[must_use]
    pub fn peae(&mut self) -> PeaeW<IeSpec> {
        PeaeW::new(self, 21)
    }
    #[doc = "Bit 22 - PEDE"]
    #[inline(always)]
    #[must_use]
    pub fn pede(&mut self) -> PedeW<IeSpec> {
        PedeW::new(self, 22)
    }
    #[doc = "Bit 23 - ARAE"]
    #[inline(always)]
    #[must_use]
    pub fn arae(&mut self) -> AraeW<IeSpec> {
        AraeW::new(self, 23)
    }
}
#[doc = "The settings in the Interrupt Enable register determine which status changes in the Interrupt Register will be signaled on an interrupt line.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IeSpec;
impl crate::RegisterSpec for IeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ie::R`](R) reader structure"]
impl crate::Readable for IeSpec {}
#[doc = "`write(|w| ..)` method takes [`ie::W`](W) writer structure"]
impl crate::Writable for IeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IeSpec {
    const RESET_VALUE: u32 = 0;
}

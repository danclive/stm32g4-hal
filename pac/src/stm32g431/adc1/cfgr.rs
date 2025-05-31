#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `DMAEN` reader - DMAEN"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMAEN"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACFG` reader - DMACFG"]
pub type DmacfgR = crate::BitReader;
#[doc = "Field `DMACFG` writer - DMACFG"]
pub type DmacfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES` reader - RES"]
pub type ResR = crate::FieldReader;
#[doc = "Field `RES` writer - RES"]
pub type ResW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EXTSEL` reader - External trigger selection for regular group"]
pub type ExtselR = crate::FieldReader;
#[doc = "Field `EXTSEL` writer - External trigger selection for regular group"]
pub type ExtselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EXTEN` reader - EXTEN"]
pub type ExtenR = crate::FieldReader;
#[doc = "Field `EXTEN` writer - EXTEN"]
pub type ExtenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OVRMOD` reader - OVRMOD"]
pub type OvrmodR = crate::BitReader;
#[doc = "Field `OVRMOD` writer - OVRMOD"]
pub type OvrmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONT` reader - CONT"]
pub type ContR = crate::BitReader;
#[doc = "Field `CONT` writer - CONT"]
pub type ContW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTDLY` reader - AUTDLY"]
pub type AutdlyR = crate::BitReader;
#[doc = "Field `AUTDLY` writer - AUTDLY"]
pub type AutdlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALIGN` reader - ALIGN"]
pub type AlignR = crate::BitReader;
#[doc = "Field `ALIGN` writer - ALIGN"]
pub type AlignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCEN` reader - DISCEN"]
pub type DiscenR = crate::BitReader;
#[doc = "Field `DISCEN` writer - DISCEN"]
pub type DiscenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCNUM` reader - DISCNUM"]
pub type DiscnumR = crate::FieldReader;
#[doc = "Field `DISCNUM` writer - DISCNUM"]
pub type DiscnumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JDISCEN` reader - JDISCEN"]
pub type JdiscenR = crate::BitReader;
#[doc = "Field `JDISCEN` writer - JDISCEN"]
pub type JdiscenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JQM` reader - JQM"]
pub type JqmR = crate::BitReader;
#[doc = "Field `JQM` writer - JQM"]
pub type JqmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1SGL` reader - AWD1SGL"]
pub type Awd1sglR = crate::BitReader;
#[doc = "Field `AWD1SGL` writer - AWD1SGL"]
pub type Awd1sglW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1EN` reader - AWD1EN"]
pub type Awd1enR = crate::BitReader;
#[doc = "Field `AWD1EN` writer - AWD1EN"]
pub type Awd1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JAWD1EN` reader - JAWD1EN"]
pub type Jawd1enR = crate::BitReader;
#[doc = "Field `JAWD1EN` writer - JAWD1EN"]
pub type Jawd1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JAUTO` reader - JAUTO"]
pub type JautoR = crate::BitReader;
#[doc = "Field `JAUTO` writer - JAUTO"]
pub type JautoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1CH` reader - Analog watchdog 1 channel selection"]
pub type Awd1chR = crate::FieldReader;
#[doc = "Field `AWD1CH` writer - Analog watchdog 1 channel selection"]
pub type Awd1chW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JQDIS` reader - Injected Queue disable"]
pub type JqdisR = crate::BitReader;
#[doc = "Field `JQDIS` writer - Injected Queue disable"]
pub type JqdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    pub fn dmacfg(&self) -> DmacfgR {
        DmacfgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:4 - RES"]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:9 - External trigger selection for regular group"]
    #[inline(always)]
    pub fn extsel(&self) -> ExtselR {
        ExtselR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:11 - EXTEN"]
    #[inline(always)]
    pub fn exten(&self) -> ExtenR {
        ExtenR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - OVRMOD"]
    #[inline(always)]
    pub fn ovrmod(&self) -> OvrmodR {
        OvrmodR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CONT"]
    #[inline(always)]
    pub fn cont(&self) -> ContR {
        ContR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AUTDLY"]
    #[inline(always)]
    pub fn autdly(&self) -> AutdlyR {
        AutdlyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ALIGN"]
    #[inline(always)]
    pub fn align(&self) -> AlignR {
        AlignR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DISCEN"]
    #[inline(always)]
    pub fn discen(&self) -> DiscenR {
        DiscenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - DISCNUM"]
    #[inline(always)]
    pub fn discnum(&self) -> DiscnumR {
        DiscnumR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - JDISCEN"]
    #[inline(always)]
    pub fn jdiscen(&self) -> JdiscenR {
        JdiscenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - JQM"]
    #[inline(always)]
    pub fn jqm(&self) -> JqmR {
        JqmR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - AWD1SGL"]
    #[inline(always)]
    pub fn awd1sgl(&self) -> Awd1sglR {
        Awd1sglR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - AWD1EN"]
    #[inline(always)]
    pub fn awd1en(&self) -> Awd1enR {
        Awd1enR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - JAWD1EN"]
    #[inline(always)]
    pub fn jawd1en(&self) -> Jawd1enR {
        Jawd1enR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - JAUTO"]
    #[inline(always)]
    pub fn jauto(&self) -> JautoR {
        JautoR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - Analog watchdog 1 channel selection"]
    #[inline(always)]
    pub fn awd1ch(&self) -> Awd1chR {
        Awd1chR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Injected Queue disable"]
    #[inline(always)]
    pub fn jqdis(&self) -> JqdisR {
        JqdisR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("jqdis", &self.jqdis())
            .field("awd1ch", &self.awd1ch())
            .field("jauto", &self.jauto())
            .field("jawd1en", &self.jawd1en())
            .field("awd1en", &self.awd1en())
            .field("awd1sgl", &self.awd1sgl())
            .field("jqm", &self.jqm())
            .field("jdiscen", &self.jdiscen())
            .field("discnum", &self.discnum())
            .field("discen", &self.discen())
            .field("align", &self.align())
            .field("autdly", &self.autdly())
            .field("cont", &self.cont())
            .field("ovrmod", &self.ovrmod())
            .field("exten", &self.exten())
            .field("extsel", &self.extsel())
            .field("res", &self.res())
            .field("dmacfg", &self.dmacfg())
            .field("dmaen", &self.dmaen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<CfgrSpec> {
        DmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DmacfgW<CfgrSpec> {
        DmacfgW::new(self, 1)
    }
    #[doc = "Bits 3:4 - RES"]
    #[inline(always)]
    pub fn res(&mut self) -> ResW<CfgrSpec> {
        ResW::new(self, 3)
    }
    #[doc = "Bits 5:9 - External trigger selection for regular group"]
    #[inline(always)]
    pub fn extsel(&mut self) -> ExtselW<CfgrSpec> {
        ExtselW::new(self, 5)
    }
    #[doc = "Bits 10:11 - EXTEN"]
    #[inline(always)]
    pub fn exten(&mut self) -> ExtenW<CfgrSpec> {
        ExtenW::new(self, 10)
    }
    #[doc = "Bit 12 - OVRMOD"]
    #[inline(always)]
    pub fn ovrmod(&mut self) -> OvrmodW<CfgrSpec> {
        OvrmodW::new(self, 12)
    }
    #[doc = "Bit 13 - CONT"]
    #[inline(always)]
    pub fn cont(&mut self) -> ContW<CfgrSpec> {
        ContW::new(self, 13)
    }
    #[doc = "Bit 14 - AUTDLY"]
    #[inline(always)]
    pub fn autdly(&mut self) -> AutdlyW<CfgrSpec> {
        AutdlyW::new(self, 14)
    }
    #[doc = "Bit 15 - ALIGN"]
    #[inline(always)]
    pub fn align(&mut self) -> AlignW<CfgrSpec> {
        AlignW::new(self, 15)
    }
    #[doc = "Bit 16 - DISCEN"]
    #[inline(always)]
    pub fn discen(&mut self) -> DiscenW<CfgrSpec> {
        DiscenW::new(self, 16)
    }
    #[doc = "Bits 17:19 - DISCNUM"]
    #[inline(always)]
    pub fn discnum(&mut self) -> DiscnumW<CfgrSpec> {
        DiscnumW::new(self, 17)
    }
    #[doc = "Bit 20 - JDISCEN"]
    #[inline(always)]
    pub fn jdiscen(&mut self) -> JdiscenW<CfgrSpec> {
        JdiscenW::new(self, 20)
    }
    #[doc = "Bit 21 - JQM"]
    #[inline(always)]
    pub fn jqm(&mut self) -> JqmW<CfgrSpec> {
        JqmW::new(self, 21)
    }
    #[doc = "Bit 22 - AWD1SGL"]
    #[inline(always)]
    pub fn awd1sgl(&mut self) -> Awd1sglW<CfgrSpec> {
        Awd1sglW::new(self, 22)
    }
    #[doc = "Bit 23 - AWD1EN"]
    #[inline(always)]
    pub fn awd1en(&mut self) -> Awd1enW<CfgrSpec> {
        Awd1enW::new(self, 23)
    }
    #[doc = "Bit 24 - JAWD1EN"]
    #[inline(always)]
    pub fn jawd1en(&mut self) -> Jawd1enW<CfgrSpec> {
        Jawd1enW::new(self, 24)
    }
    #[doc = "Bit 25 - JAUTO"]
    #[inline(always)]
    pub fn jauto(&mut self) -> JautoW<CfgrSpec> {
        JautoW::new(self, 25)
    }
    #[doc = "Bits 26:30 - Analog watchdog 1 channel selection"]
    #[inline(always)]
    pub fn awd1ch(&mut self) -> Awd1chW<CfgrSpec> {
        Awd1chW::new(self, 26)
    }
    #[doc = "Bit 31 - Injected Queue disable"]
    #[inline(always)]
    pub fn jqdis(&mut self) -> JqdisW<CfgrSpec> {
        JqdisW::new(self, 31)
    }
}
#[doc = "configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR to value 0x8000_0000"]
impl crate::Resettable for CfgrSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}

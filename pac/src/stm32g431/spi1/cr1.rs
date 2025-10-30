#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `CPHA` reader - Clock phase"]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - Clock phase"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Clock polarity"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTR` reader - Master selection"]
pub type MstrR = crate::BitReader;
#[doc = "Field `MSTR` writer - Master selection"]
pub type MstrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR` reader - Baud rate control"]
pub type BrR = crate::FieldReader;
#[doc = "Field `BR` writer - Baud rate control"]
pub type BrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPE` reader - SPI enable"]
pub type SpeR = crate::BitReader;
#[doc = "Field `SPE` writer - SPI enable"]
pub type SpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSBFIRST` reader - Frame format"]
pub type LsbfirstR = crate::BitReader;
#[doc = "Field `LSBFIRST` writer - Frame format"]
pub type LsbfirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSI` reader - Internal slave select"]
pub type SsiR = crate::BitReader;
#[doc = "Field `SSI` writer - Internal slave select"]
pub type SsiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSM` reader - Software slave management"]
pub type SsmR = crate::BitReader;
#[doc = "Field `SSM` writer - Software slave management"]
pub type SsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXONLY` reader - Receive only"]
pub type RxonlyR = crate::BitReader;
#[doc = "Field `RXONLY` writer - Receive only"]
pub type RxonlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFF` reader - Data frame format"]
pub type DffR = crate::BitReader;
#[doc = "Field `DFF` writer - Data frame format"]
pub type DffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCNEXT` reader - CRC transfer next"]
pub type CrcnextR = crate::BitReader;
#[doc = "Field `CRCNEXT` writer - CRC transfer next"]
pub type CrcnextW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - Hardware CRC calculation enable"]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - Hardware CRC calculation enable"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIDIOE` reader - Output enable in bidirectional mode"]
pub type BidioeR = crate::BitReader;
#[doc = "Field `BIDIOE` writer - Output enable in bidirectional mode"]
pub type BidioeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIDIMODE` reader - Bidirectional data mode enable"]
pub type BidimodeR = crate::BitReader;
#[doc = "Field `BIDIMODE` writer - Bidirectional data mode enable"]
pub type BidimodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master selection"]
    #[inline(always)]
    pub fn mstr(&self) -> MstrR {
        MstrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Baud rate control"]
    #[inline(always)]
    pub fn br(&self) -> BrR {
        BrR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    pub fn spe(&self) -> SpeR {
        SpeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline(always)]
    pub fn lsbfirst(&self) -> LsbfirstR {
        LsbfirstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Internal slave select"]
    #[inline(always)]
    pub fn ssi(&self) -> SsiR {
        SsiR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software slave management"]
    #[inline(always)]
    pub fn ssm(&self) -> SsmR {
        SsmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    pub fn rxonly(&self) -> RxonlyR {
        RxonlyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data frame format"]
    #[inline(always)]
    pub fn dff(&self) -> DffR {
        DffR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline(always)]
    pub fn crcnext(&self) -> CrcnextR {
        CrcnextR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode"]
    #[inline(always)]
    pub fn bidioe(&self) -> BidioeR {
        BidioeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bidirectional data mode enable"]
    #[inline(always)]
    pub fn bidimode(&self) -> BidimodeR {
        BidimodeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("bidimode", &self.bidimode())
            .field("bidioe", &self.bidioe())
            .field("crcen", &self.crcen())
            .field("crcnext", &self.crcnext())
            .field("dff", &self.dff())
            .field("rxonly", &self.rxonly())
            .field("ssm", &self.ssm())
            .field("ssi", &self.ssi())
            .field("lsbfirst", &self.lsbfirst())
            .field("spe", &self.spe())
            .field("br", &self.br())
            .field("mstr", &self.mstr())
            .field("cpol", &self.cpol())
            .field("cpha", &self.cpha())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<'_, Cr1Spec> {
        CphaW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, Cr1Spec> {
        CpolW::new(self, 1)
    }
    #[doc = "Bit 2 - Master selection"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MstrW<'_, Cr1Spec> {
        MstrW::new(self, 2)
    }
    #[doc = "Bits 3:5 - Baud rate control"]
    #[inline(always)]
    pub fn br(&mut self) -> BrW<'_, Cr1Spec> {
        BrW::new(self, 3)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    pub fn spe(&mut self) -> SpeW<'_, Cr1Spec> {
        SpeW::new(self, 6)
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline(always)]
    pub fn lsbfirst(&mut self) -> LsbfirstW<'_, Cr1Spec> {
        LsbfirstW::new(self, 7)
    }
    #[doc = "Bit 8 - Internal slave select"]
    #[inline(always)]
    pub fn ssi(&mut self) -> SsiW<'_, Cr1Spec> {
        SsiW::new(self, 8)
    }
    #[doc = "Bit 9 - Software slave management"]
    #[inline(always)]
    pub fn ssm(&mut self) -> SsmW<'_, Cr1Spec> {
        SsmW::new(self, 9)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    pub fn rxonly(&mut self) -> RxonlyW<'_, Cr1Spec> {
        RxonlyW::new(self, 10)
    }
    #[doc = "Bit 11 - Data frame format"]
    #[inline(always)]
    pub fn dff(&mut self) -> DffW<'_, Cr1Spec> {
        DffW::new(self, 11)
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline(always)]
    pub fn crcnext(&mut self) -> CrcnextW<'_, Cr1Spec> {
        CrcnextW::new(self, 12)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CrcenW<'_, Cr1Spec> {
        CrcenW::new(self, 13)
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode"]
    #[inline(always)]
    pub fn bidioe(&mut self) -> BidioeW<'_, Cr1Spec> {
        BidioeW::new(self, 14)
    }
    #[doc = "Bit 15 - Bidirectional data mode enable"]
    #[inline(always)]
    pub fn bidimode(&mut self) -> BidimodeW<'_, Cr1Spec> {
        BidimodeW::new(self, 15)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

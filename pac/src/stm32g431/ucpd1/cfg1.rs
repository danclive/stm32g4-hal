#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "Field `HBITCLKDIV` reader - HBITCLKDIV"]
pub type HbitclkdivR = crate::FieldReader;
#[doc = "Field `HBITCLKDIV` writer - HBITCLKDIV"]
pub type HbitclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `IFRGAP` reader - IFRGAP"]
pub type IfrgapR = crate::FieldReader;
#[doc = "Field `IFRGAP` writer - IFRGAP"]
pub type IfrgapW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRANSWIN` reader - TRANSWIN"]
pub type TranswinR = crate::FieldReader;
#[doc = "Field `TRANSWIN` writer - TRANSWIN"]
pub type TranswinW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PSC_USBPDCLK` reader - PSC_USBPDCLK"]
pub type PscUsbpdclkR = crate::FieldReader;
#[doc = "Field `PSC_USBPDCLK` writer - PSC_USBPDCLK"]
pub type PscUsbpdclkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RXORDSETEN` reader - RXORDSETEN"]
pub type RxordsetenR = crate::FieldReader<u16>;
#[doc = "Field `RXORDSETEN` writer - RXORDSETEN"]
pub type RxordsetenW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TXDMAEN` reader - TXDMAEN"]
pub type TxdmaenR = crate::BitReader;
#[doc = "Field `TXDMAEN` writer - TXDMAEN"]
pub type TxdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMAEN` reader - RXDMAEN"]
pub type RxdmaenR = crate::BitReader;
#[doc = "Field `RXDMAEN` writer - RXDMAEN"]
pub type RxdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPDEN` reader - UCPDEN"]
pub type UcpdenR = crate::BitReader;
#[doc = "Field `UCPDEN` writer - UCPDEN"]
pub type UcpdenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - HBITCLKDIV"]
    #[inline(always)]
    pub fn hbitclkdiv(&self) -> HbitclkdivR {
        HbitclkdivR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:10 - IFRGAP"]
    #[inline(always)]
    pub fn ifrgap(&self) -> IfrgapR {
        IfrgapR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - TRANSWIN"]
    #[inline(always)]
    pub fn transwin(&self) -> TranswinR {
        TranswinR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 17:19 - PSC_USBPDCLK"]
    #[inline(always)]
    pub fn psc_usbpdclk(&self) -> PscUsbpdclkR {
        PscUsbpdclkR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:28 - RXORDSETEN"]
    #[inline(always)]
    pub fn rxordseten(&self) -> RxordsetenR {
        RxordsetenR::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bit 29 - TXDMAEN"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TxdmaenR {
        TxdmaenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - RXDMAEN"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RxdmaenR {
        RxdmaenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UCPDEN"]
    #[inline(always)]
    pub fn ucpden(&self) -> UcpdenR {
        UcpdenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG1")
            .field("hbitclkdiv", &self.hbitclkdiv())
            .field("ifrgap", &self.ifrgap())
            .field("transwin", &self.transwin())
            .field("psc_usbpdclk", &self.psc_usbpdclk())
            .field("rxordseten", &self.rxordseten())
            .field("txdmaen", &self.txdmaen())
            .field("rxdmaen", &self.rxdmaen())
            .field("ucpden", &self.ucpden())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - HBITCLKDIV"]
    #[inline(always)]
    pub fn hbitclkdiv(&mut self) -> HbitclkdivW<'_, Cfg1Spec> {
        HbitclkdivW::new(self, 0)
    }
    #[doc = "Bits 6:10 - IFRGAP"]
    #[inline(always)]
    pub fn ifrgap(&mut self) -> IfrgapW<'_, Cfg1Spec> {
        IfrgapW::new(self, 6)
    }
    #[doc = "Bits 11:15 - TRANSWIN"]
    #[inline(always)]
    pub fn transwin(&mut self) -> TranswinW<'_, Cfg1Spec> {
        TranswinW::new(self, 11)
    }
    #[doc = "Bits 17:19 - PSC_USBPDCLK"]
    #[inline(always)]
    pub fn psc_usbpdclk(&mut self) -> PscUsbpdclkW<'_, Cfg1Spec> {
        PscUsbpdclkW::new(self, 17)
    }
    #[doc = "Bits 20:28 - RXORDSETEN"]
    #[inline(always)]
    pub fn rxordseten(&mut self) -> RxordsetenW<'_, Cfg1Spec> {
        RxordsetenW::new(self, 20)
    }
    #[doc = "Bit 29 - TXDMAEN"]
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TxdmaenW<'_, Cfg1Spec> {
        TxdmaenW::new(self, 29)
    }
    #[doc = "Bit 30 - RXDMAEN"]
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RxdmaenW<'_, Cfg1Spec> {
        RxdmaenW::new(self, 30)
    }
    #[doc = "Bit 31 - UCPDEN"]
    #[inline(always)]
    pub fn ucpden(&mut self) -> UcpdenW<'_, Cfg1Spec> {
        UcpdenW::new(self, 31)
    }
}
#[doc = "UCPD configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for Cfg1Spec {}

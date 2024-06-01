#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `RXNE` reader - Receive buffer not empty"]
pub type RxneR = crate::BitReader;
#[doc = "Field `TXE` reader - Transmit buffer empty"]
pub type TxeR = crate::BitReader;
#[doc = "Field `CRCERR` reader - CRC error flag"]
pub type CrcerrR = crate::BitReader;
#[doc = "Field `CRCERR` writer - CRC error flag"]
pub type CrcerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODF` reader - Mode fault"]
pub type ModfR = crate::BitReader;
#[doc = "Field `OVR` reader - Overrun flag"]
pub type OvrR = crate::BitReader;
#[doc = "Field `BSY` reader - Busy flag"]
pub type BsyR = crate::BitReader;
#[doc = "Field `TIFRFE` reader - TI frame format error"]
pub type TifrfeR = crate::BitReader;
#[doc = "Field `FRLVL` reader - FIFO reception level"]
pub type FrlvlR = crate::FieldReader;
#[doc = "Field `FTLVL` reader - FIFO transmission level"]
pub type FtlvlR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Receive buffer not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit buffer empty"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC error flag"]
    #[inline(always)]
    pub fn crcerr(&self) -> CrcerrR {
        CrcerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mode fault"]
    #[inline(always)]
    pub fn modf(&self) -> ModfR {
        ModfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun flag"]
    #[inline(always)]
    pub fn ovr(&self) -> OvrR {
        OvrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Busy flag"]
    #[inline(always)]
    pub fn bsy(&self) -> BsyR {
        BsyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TI frame format error"]
    #[inline(always)]
    pub fn tifrfe(&self) -> TifrfeR {
        TifrfeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - FIFO reception level"]
    #[inline(always)]
    pub fn frlvl(&self) -> FrlvlR {
        FrlvlR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - FIFO transmission level"]
    #[inline(always)]
    pub fn ftlvl(&self) -> FtlvlR {
        FtlvlR::new(((self.bits >> 11) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("rxne", &self.rxne())
            .field("txe", &self.txe())
            .field("crcerr", &self.crcerr())
            .field("modf", &self.modf())
            .field("ovr", &self.ovr())
            .field("bsy", &self.bsy())
            .field("tifrfe", &self.tifrfe())
            .field("frlvl", &self.frlvl())
            .field("ftlvl", &self.ftlvl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - CRC error flag"]
    #[inline(always)]
    #[must_use]
    pub fn crcerr(&mut self) -> CrcerrW<SrSpec> {
        CrcerrW::new(self, 4)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0x02"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x02;
}

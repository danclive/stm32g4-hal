#[doc = "Register `DBTP` reader"]
pub type R = crate::R<DbtpSpec>;
#[doc = "Register `DBTP` writer"]
pub type W = crate::W<DbtpSpec>;
#[doc = "Field `DSJW` reader - DSJW"]
pub type DsjwR = crate::FieldReader;
#[doc = "Field `DSJW` writer - DSJW"]
pub type DsjwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTSEG2` reader - DTSEG2"]
pub type Dtseg2R = crate::FieldReader;
#[doc = "Field `DTSEG2` writer - DTSEG2"]
pub type Dtseg2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTSEG1` reader - DTSEG1"]
pub type Dtseg1R = crate::FieldReader;
#[doc = "Field `DTSEG1` writer - DTSEG1"]
pub type Dtseg1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DBRP` reader - DBRP"]
pub type DbrpR = crate::FieldReader;
#[doc = "Field `DBRP` writer - DBRP"]
pub type DbrpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TDC` reader - TDC"]
pub type TdcR = crate::BitReader;
#[doc = "Field `TDC` writer - TDC"]
pub type TdcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - DSJW"]
    #[inline(always)]
    pub fn dsjw(&self) -> DsjwR {
        DsjwR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DTSEG2"]
    #[inline(always)]
    pub fn dtseg2(&self) -> Dtseg2R {
        Dtseg2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - DTSEG1"]
    #[inline(always)]
    pub fn dtseg1(&self) -> Dtseg1R {
        Dtseg1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - DBRP"]
    #[inline(always)]
    pub fn dbrp(&self) -> DbrpR {
        DbrpR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - TDC"]
    #[inline(always)]
    pub fn tdc(&self) -> TdcR {
        TdcR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBTP")
            .field("dsjw", &self.dsjw())
            .field("dtseg2", &self.dtseg2())
            .field("dtseg1", &self.dtseg1())
            .field("dbrp", &self.dbrp())
            .field("tdc", &self.tdc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - DSJW"]
    #[inline(always)]
    #[must_use]
    pub fn dsjw(&mut self) -> DsjwW<DbtpSpec> {
        DsjwW::new(self, 0)
    }
    #[doc = "Bits 4:7 - DTSEG2"]
    #[inline(always)]
    #[must_use]
    pub fn dtseg2(&mut self) -> Dtseg2W<DbtpSpec> {
        Dtseg2W::new(self, 4)
    }
    #[doc = "Bits 8:12 - DTSEG1"]
    #[inline(always)]
    #[must_use]
    pub fn dtseg1(&mut self) -> Dtseg1W<DbtpSpec> {
        Dtseg1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - DBRP"]
    #[inline(always)]
    #[must_use]
    pub fn dbrp(&mut self) -> DbrpW<DbtpSpec> {
        DbrpW::new(self, 16)
    }
    #[doc = "Bit 23 - TDC"]
    #[inline(always)]
    #[must_use]
    pub fn tdc(&mut self) -> TdcW<DbtpSpec> {
        TdcW::new(self, 23)
    }
}
#[doc = "This register is only writable if bits CCCR.CCE and CCCR.INIT are set. The CAN bit time may be programed in the range of 4 to 25 time quanta. The CAN time quantum may be programmed in the range of 1 to 1024 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock period. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (programmed values) \\[DTSEG1 + DTSEG2 + 3\\]
tq or (functional values) \\[Sync_Seg + Prop_Seg + Phase_Seg1 + Phase_Seg2\\]
tq. The Information Processing Time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbtp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbtp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbtpSpec;
impl crate::RegisterSpec for DbtpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbtp::R`](R) reader structure"]
impl crate::Readable for DbtpSpec {}
#[doc = "`write(|w| ..)` method takes [`dbtp::W`](W) writer structure"]
impl crate::Writable for DbtpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBTP to value 0x0a33"]
impl crate::Resettable for DbtpSpec {
    const RESET_VALUE: u32 = 0x0a33;
}

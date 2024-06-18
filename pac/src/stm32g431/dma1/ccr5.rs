#[doc = "Register `CCR5` reader"]
pub type R = crate::R<Ccr5Spec>;
#[doc = "Register `CCR5` writer"]
pub type W = crate::W<Ccr5Spec>;
#[doc = "Field `EN` reader - channel enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - channel enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - TCIE"]
pub type TcieR = crate::BitReader;
#[doc = "Field `TCIE` writer - TCIE"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTIE` reader - HTIE"]
pub type HtieR = crate::BitReader;
#[doc = "Field `HTIE` writer - HTIE"]
pub type HtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIE` reader - TEIE"]
pub type TeieR = crate::BitReader;
#[doc = "Field `TEIE` writer - TEIE"]
pub type TeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - DIR"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - DIR"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIRC` reader - CIRC"]
pub type CircR = crate::BitReader;
#[doc = "Field `CIRC` writer - CIRC"]
pub type CircW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINC` reader - PINC"]
pub type PincR = crate::BitReader;
#[doc = "Field `PINC` writer - PINC"]
pub type PincW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINC` reader - MINC"]
pub type MincR = crate::BitReader;
#[doc = "Field `MINC` writer - MINC"]
pub type MincW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSIZE` reader - PSIZE"]
pub type PsizeR = crate::FieldReader;
#[doc = "Field `PSIZE` writer - PSIZE"]
pub type PsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MSIZE` reader - MSIZE"]
pub type MsizeR = crate::FieldReader;
#[doc = "Field `MSIZE` writer - MSIZE"]
pub type MsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PL` reader - PL"]
pub type PlR = crate::FieldReader;
#[doc = "Field `PL` writer - PL"]
pub type PlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM2MEM` reader - MEM2MEM"]
pub type Mem2memR = crate::BitReader;
#[doc = "Field `MEM2MEM` writer - MEM2MEM"]
pub type Mem2memW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - channel enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TCIE"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HTIE"]
    #[inline(always)]
    pub fn htie(&self) -> HtieR {
        HtieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TEIE"]
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        TeieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CIRC"]
    #[inline(always)]
    pub fn circ(&self) -> CircR {
        CircR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PINC"]
    #[inline(always)]
    pub fn pinc(&self) -> PincR {
        PincR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MINC"]
    #[inline(always)]
    pub fn minc(&self) -> MincR {
        MincR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - PSIZE"]
    #[inline(always)]
    pub fn psize(&self) -> PsizeR {
        PsizeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - MSIZE"]
    #[inline(always)]
    pub fn msize(&self) -> MsizeR {
        MsizeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PL"]
    #[inline(always)]
    pub fn pl(&self) -> PlR {
        PlR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - MEM2MEM"]
    #[inline(always)]
    pub fn mem2mem(&self) -> Mem2memR {
        Mem2memR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR5")
            .field("en", &self.en())
            .field("tcie", &self.tcie())
            .field("htie", &self.htie())
            .field("teie", &self.teie())
            .field("dir", &self.dir())
            .field("circ", &self.circ())
            .field("pinc", &self.pinc())
            .field("minc", &self.minc())
            .field("psize", &self.psize())
            .field("msize", &self.msize())
            .field("pl", &self.pl())
            .field("mem2mem", &self.mem2mem())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<Ccr5Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - TCIE"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TcieW<Ccr5Spec> {
        TcieW::new(self, 1)
    }
    #[doc = "Bit 2 - HTIE"]
    #[inline(always)]
    #[must_use]
    pub fn htie(&mut self) -> HtieW<Ccr5Spec> {
        HtieW::new(self, 2)
    }
    #[doc = "Bit 3 - TEIE"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TeieW<Ccr5Spec> {
        TeieW::new(self, 3)
    }
    #[doc = "Bit 4 - DIR"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DirW<Ccr5Spec> {
        DirW::new(self, 4)
    }
    #[doc = "Bit 5 - CIRC"]
    #[inline(always)]
    #[must_use]
    pub fn circ(&mut self) -> CircW<Ccr5Spec> {
        CircW::new(self, 5)
    }
    #[doc = "Bit 6 - PINC"]
    #[inline(always)]
    #[must_use]
    pub fn pinc(&mut self) -> PincW<Ccr5Spec> {
        PincW::new(self, 6)
    }
    #[doc = "Bit 7 - MINC"]
    #[inline(always)]
    #[must_use]
    pub fn minc(&mut self) -> MincW<Ccr5Spec> {
        MincW::new(self, 7)
    }
    #[doc = "Bits 8:9 - PSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PsizeW<Ccr5Spec> {
        PsizeW::new(self, 8)
    }
    #[doc = "Bits 10:11 - MSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn msize(&mut self) -> MsizeW<Ccr5Spec> {
        MsizeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - PL"]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PlW<Ccr5Spec> {
        PlW::new(self, 12)
    }
    #[doc = "Bit 14 - MEM2MEM"]
    #[inline(always)]
    #[must_use]
    pub fn mem2mem(&mut self) -> Mem2memW<Ccr5Spec> {
        Mem2memW::new(self, 14)
    }
}
#[doc = "DMA channel 4 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr5Spec;
impl crate::RegisterSpec for Ccr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr5::R`](R) reader structure"]
impl crate::Readable for Ccr5Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr5::W`](W) writer structure"]
impl crate::Writable for Ccr5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR5 to value 0"]
impl crate::Resettable for Ccr5Spec {
    const RESET_VALUE: u32 = 0;
}

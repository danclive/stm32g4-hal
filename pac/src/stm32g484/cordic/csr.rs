#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `FUNC` reader - FUNC"]
pub type FuncR = crate::FieldReader;
#[doc = "Field `FUNC` writer - FUNC"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRECISION` reader - PRECISION"]
pub type PrecisionR = crate::FieldReader;
#[doc = "Field `PRECISION` writer - PRECISION"]
pub type PrecisionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCALE` reader - SCALE"]
pub type ScaleR = crate::FieldReader;
#[doc = "Field `SCALE` writer - SCALE"]
pub type ScaleW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IEN` reader - IEN"]
pub type IenR = crate::BitReader;
#[doc = "Field `IEN` writer - IEN"]
pub type IenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAREN` reader - DMAREN"]
pub type DmarenR = crate::BitReader;
#[doc = "Field `DMAREN` writer - DMAREN"]
pub type DmarenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAWEN` reader - DMAWEN"]
pub type DmawenR = crate::BitReader;
#[doc = "Field `DMAWEN` writer - DMAWEN"]
pub type DmawenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRES` reader - NRES"]
pub type NresR = crate::BitReader;
#[doc = "Field `NRES` writer - NRES"]
pub type NresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NARGS` reader - NARGS"]
pub type NargsR = crate::BitReader;
#[doc = "Field `NARGS` writer - NARGS"]
pub type NargsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESSIZE` reader - RESSIZE"]
pub type RessizeR = crate::BitReader;
#[doc = "Field `RESSIZE` writer - RESSIZE"]
pub type RessizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARGSIZE` reader - ARGSIZE"]
pub type ArgsizeR = crate::BitReader;
#[doc = "Field `ARGSIZE` writer - ARGSIZE"]
pub type ArgsizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRDY` reader - RRDY"]
pub type RrdyR = crate::BitReader;
#[doc = "Field `RRDY` writer - RRDY"]
pub type RrdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - FUNC"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PRECISION"]
    #[inline(always)]
    pub fn precision(&self) -> PrecisionR {
        PrecisionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - SCALE"]
    #[inline(always)]
    pub fn scale(&self) -> ScaleR {
        ScaleR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - IEN"]
    #[inline(always)]
    pub fn ien(&self) -> IenR {
        IenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMAREN"]
    #[inline(always)]
    pub fn dmaren(&self) -> DmarenR {
        DmarenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMAWEN"]
    #[inline(always)]
    pub fn dmawen(&self) -> DmawenR {
        DmawenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - NRES"]
    #[inline(always)]
    pub fn nres(&self) -> NresR {
        NresR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - NARGS"]
    #[inline(always)]
    pub fn nargs(&self) -> NargsR {
        NargsR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RESSIZE"]
    #[inline(always)]
    pub fn ressize(&self) -> RessizeR {
        RessizeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ARGSIZE"]
    #[inline(always)]
    pub fn argsize(&self) -> ArgsizeR {
        ArgsizeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 31 - RRDY"]
    #[inline(always)]
    pub fn rrdy(&self) -> RrdyR {
        RrdyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("func", &self.func())
            .field("precision", &self.precision())
            .field("scale", &self.scale())
            .field("ien", &self.ien())
            .field("dmaren", &self.dmaren())
            .field("dmawen", &self.dmawen())
            .field("nres", &self.nres())
            .field("nargs", &self.nargs())
            .field("ressize", &self.ressize())
            .field("argsize", &self.argsize())
            .field("rrdy", &self.rrdy())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - FUNC"]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FuncW<CsrSpec> {
        FuncW::new(self, 0)
    }
    #[doc = "Bits 4:7 - PRECISION"]
    #[inline(always)]
    #[must_use]
    pub fn precision(&mut self) -> PrecisionW<CsrSpec> {
        PrecisionW::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCALE"]
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> ScaleW<CsrSpec> {
        ScaleW::new(self, 8)
    }
    #[doc = "Bit 16 - IEN"]
    #[inline(always)]
    #[must_use]
    pub fn ien(&mut self) -> IenW<CsrSpec> {
        IenW::new(self, 16)
    }
    #[doc = "Bit 17 - DMAREN"]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DmarenW<CsrSpec> {
        DmarenW::new(self, 17)
    }
    #[doc = "Bit 18 - DMAWEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmawen(&mut self) -> DmawenW<CsrSpec> {
        DmawenW::new(self, 18)
    }
    #[doc = "Bit 19 - NRES"]
    #[inline(always)]
    #[must_use]
    pub fn nres(&mut self) -> NresW<CsrSpec> {
        NresW::new(self, 19)
    }
    #[doc = "Bit 20 - NARGS"]
    #[inline(always)]
    #[must_use]
    pub fn nargs(&mut self) -> NargsW<CsrSpec> {
        NargsW::new(self, 20)
    }
    #[doc = "Bit 21 - RESSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn ressize(&mut self) -> RessizeW<CsrSpec> {
        RessizeW::new(self, 21)
    }
    #[doc = "Bit 22 - ARGSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn argsize(&mut self) -> ArgsizeW<CsrSpec> {
        ArgsizeW::new(self, 22)
    }
    #[doc = "Bit 31 - RRDY"]
    #[inline(always)]
    #[must_use]
    pub fn rrdy(&mut self) -> RrdyW<CsrSpec> {
        RrdyW::new(self, 31)
    }
}
#[doc = "CORDIC Control Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {
    const RESET_VALUE: u32 = 0;
}

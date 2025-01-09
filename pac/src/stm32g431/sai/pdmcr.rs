#[doc = "Register `PDMCR` reader"]
pub type R = crate::R<PdmcrSpec>;
#[doc = "Register `PDMCR` writer"]
pub type W = crate::W<PdmcrSpec>;
#[doc = "Field `PDMEN` reader - PDMEN"]
pub type PdmenR = crate::BitReader;
#[doc = "Field `PDMEN` writer - PDMEN"]
pub type PdmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MICNBR` reader - MICNBR"]
pub type MicnbrR = crate::FieldReader;
#[doc = "Field `MICNBR` writer - MICNBR"]
pub type MicnbrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKEN1` reader - CKEN1"]
pub type Cken1R = crate::BitReader;
#[doc = "Field `CKEN1` writer - CKEN1"]
pub type Cken1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKEN2` reader - CKEN2"]
pub type Cken2R = crate::BitReader;
#[doc = "Field `CKEN2` writer - CKEN2"]
pub type Cken2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKEN3` reader - CKEN3"]
pub type Cken3R = crate::BitReader;
#[doc = "Field `CKEN3` writer - CKEN3"]
pub type Cken3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKEN4` reader - CKEN4"]
pub type Cken4R = crate::BitReader;
#[doc = "Field `CKEN4` writer - CKEN4"]
pub type Cken4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PDMEN"]
    #[inline(always)]
    pub fn pdmen(&self) -> PdmenR {
        PdmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - MICNBR"]
    #[inline(always)]
    pub fn micnbr(&self) -> MicnbrR {
        MicnbrR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - CKEN1"]
    #[inline(always)]
    pub fn cken1(&self) -> Cken1R {
        Cken1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CKEN2"]
    #[inline(always)]
    pub fn cken2(&self) -> Cken2R {
        Cken2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CKEN3"]
    #[inline(always)]
    pub fn cken3(&self) -> Cken3R {
        Cken3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CKEN4"]
    #[inline(always)]
    pub fn cken4(&self) -> Cken4R {
        Cken4R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDMCR")
            .field("pdmen", &self.pdmen())
            .field("micnbr", &self.micnbr())
            .field("cken1", &self.cken1())
            .field("cken2", &self.cken2())
            .field("cken3", &self.cken3())
            .field("cken4", &self.cken4())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PDMEN"]
    #[inline(always)]
    pub fn pdmen(&mut self) -> PdmenW<PdmcrSpec> {
        PdmenW::new(self, 0)
    }
    #[doc = "Bits 4:5 - MICNBR"]
    #[inline(always)]
    pub fn micnbr(&mut self) -> MicnbrW<PdmcrSpec> {
        MicnbrW::new(self, 4)
    }
    #[doc = "Bit 8 - CKEN1"]
    #[inline(always)]
    pub fn cken1(&mut self) -> Cken1W<PdmcrSpec> {
        Cken1W::new(self, 8)
    }
    #[doc = "Bit 9 - CKEN2"]
    #[inline(always)]
    pub fn cken2(&mut self) -> Cken2W<PdmcrSpec> {
        Cken2W::new(self, 9)
    }
    #[doc = "Bit 10 - CKEN3"]
    #[inline(always)]
    pub fn cken3(&mut self) -> Cken3W<PdmcrSpec> {
        Cken3W::new(self, 10)
    }
    #[doc = "Bit 11 - CKEN4"]
    #[inline(always)]
    pub fn cken4(&mut self) -> Cken4W<PdmcrSpec> {
        Cken4W::new(self, 11)
    }
}
#[doc = "PDM control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdmcrSpec;
impl crate::RegisterSpec for PdmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdmcr::R`](R) reader structure"]
impl crate::Readable for PdmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pdmcr::W`](W) writer structure"]
impl crate::Writable for PdmcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDMCR to value 0"]
impl crate::Resettable for PdmcrSpec {
    const RESET_VALUE: u32 = 0;
}

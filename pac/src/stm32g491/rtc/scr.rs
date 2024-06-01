#[doc = "Register `SCR` writer"]
pub type W = crate::W<ScrSpec>;
#[doc = "Field `CALRAF` writer - CALRAF"]
pub type CalrafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRBF` writer - CALRBF"]
pub type CalrbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUTF` writer - CWUTF"]
pub type CwutfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSF` writer - CTSF"]
pub type CtsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSOVF` writer - CTSOVF"]
pub type CtsovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITSF` writer - CITSF"]
pub type CitsfW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ScrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - CALRAF"]
    #[inline(always)]
    #[must_use]
    pub fn calraf(&mut self) -> CalrafW<ScrSpec> {
        CalrafW::new(self, 0)
    }
    #[doc = "Bit 1 - CALRBF"]
    #[inline(always)]
    #[must_use]
    pub fn calrbf(&mut self) -> CalrbfW<ScrSpec> {
        CalrbfW::new(self, 1)
    }
    #[doc = "Bit 2 - CWUTF"]
    #[inline(always)]
    #[must_use]
    pub fn cwutf(&mut self) -> CwutfW<ScrSpec> {
        CwutfW::new(self, 2)
    }
    #[doc = "Bit 3 - CTSF"]
    #[inline(always)]
    #[must_use]
    pub fn ctsf(&mut self) -> CtsfW<ScrSpec> {
        CtsfW::new(self, 3)
    }
    #[doc = "Bit 4 - CTSOVF"]
    #[inline(always)]
    #[must_use]
    pub fn ctsovf(&mut self) -> CtsovfW<ScrSpec> {
        CtsovfW::new(self, 4)
    }
    #[doc = "Bit 5 - CITSF"]
    #[inline(always)]
    #[must_use]
    pub fn citsf(&mut self) -> CitsfW<ScrSpec> {
        CitsfW::new(self, 5)
    }
}
#[doc = "status register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrSpec;
impl crate::RegisterSpec for ScrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for ScrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for ScrSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `ALRAF` reader - ALRAF"]
pub type AlrafR = crate::BitReader;
#[doc = "Field `ALRBF` reader - ALRBF"]
pub type AlrbfR = crate::BitReader;
#[doc = "Field `WUTF` reader - WUTF"]
pub type WutfR = crate::BitReader;
#[doc = "Field `TSF` reader - TSF"]
pub type TsfR = crate::BitReader;
#[doc = "Field `TSOVF` reader - TSOVF"]
pub type TsovfR = crate::BitReader;
#[doc = "Field `ITSF` reader - ITSF"]
pub type ItsfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ALRAF"]
    #[inline(always)]
    pub fn alraf(&self) -> AlrafR {
        AlrafR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ALRBF"]
    #[inline(always)]
    pub fn alrbf(&self) -> AlrbfR {
        AlrbfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WUTF"]
    #[inline(always)]
    pub fn wutf(&self) -> WutfR {
        WutfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TSF"]
    #[inline(always)]
    pub fn tsf(&self) -> TsfR {
        TsfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TSOVF"]
    #[inline(always)]
    pub fn tsovf(&self) -> TsovfR {
        TsovfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ITSF"]
    #[inline(always)]
    pub fn itsf(&self) -> ItsfR {
        ItsfR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("alraf", &self.alraf())
            .field("alrbf", &self.alrbf())
            .field("wutf", &self.wutf())
            .field("tsf", &self.tsf())
            .field("tsovf", &self.tsovf())
            .field("itsf", &self.itsf())
            .finish()
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0;
}

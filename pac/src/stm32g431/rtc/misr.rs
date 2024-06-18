#[doc = "Register `MISR` reader"]
pub type R = crate::R<MisrSpec>;
#[doc = "Field `ALRAMF` reader - ALRAMF"]
pub type AlramfR = crate::BitReader;
#[doc = "Field `ALRBMF` reader - ALRBMF"]
pub type AlrbmfR = crate::BitReader;
#[doc = "Field `WUTMF` reader - WUTMF"]
pub type WutmfR = crate::BitReader;
#[doc = "Field `TSMF` reader - TSMF"]
pub type TsmfR = crate::BitReader;
#[doc = "Field `TSOVMF` reader - TSOVMF"]
pub type TsovmfR = crate::BitReader;
#[doc = "Field `ITSMF` reader - ITSMF"]
pub type ItsmfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ALRAMF"]
    #[inline(always)]
    pub fn alramf(&self) -> AlramfR {
        AlramfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ALRBMF"]
    #[inline(always)]
    pub fn alrbmf(&self) -> AlrbmfR {
        AlrbmfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WUTMF"]
    #[inline(always)]
    pub fn wutmf(&self) -> WutmfR {
        WutmfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TSMF"]
    #[inline(always)]
    pub fn tsmf(&self) -> TsmfR {
        TsmfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TSOVMF"]
    #[inline(always)]
    pub fn tsovmf(&self) -> TsovmfR {
        TsovmfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ITSMF"]
    #[inline(always)]
    pub fn itsmf(&self) -> ItsmfR {
        ItsmfR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISR")
            .field("alramf", &self.alramf())
            .field("alrbmf", &self.alrbmf())
            .field("wutmf", &self.wutmf())
            .field("tsmf", &self.tsmf())
            .field("tsovmf", &self.tsovmf())
            .field("itsmf", &self.itsmf())
            .finish()
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisrSpec;
impl crate::RegisterSpec for MisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misr::R`](R) reader structure"]
impl crate::Readable for MisrSpec {}
#[doc = "`reset()` method sets MISR to value 0"]
impl crate::Resettable for MisrSpec {
    const RESET_VALUE: u32 = 0;
}

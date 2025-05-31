#[doc = "Register `MISR` reader"]
pub type R = crate::R<MisrSpec>;
#[doc = "Field `TAMP1MF` reader - TAMP1MF:"]
pub type Tamp1mfR = crate::BitReader;
#[doc = "Field `TAMP2MF` reader - TAMP2MF"]
pub type Tamp2mfR = crate::BitReader;
#[doc = "Field `TAMP3MF` reader - TAMP3MF"]
pub type Tamp3mfR = crate::BitReader;
#[doc = "Field `ITAMP3MF` reader - ITAMP3MF"]
pub type Itamp3mfR = crate::BitReader;
#[doc = "Field `ITAMP4MF` reader - ITAMP4MF"]
pub type Itamp4mfR = crate::BitReader;
#[doc = "Field `ITAMP5MF` reader - ITAMP5MF"]
pub type Itamp5mfR = crate::BitReader;
#[doc = "Field `ITAMP6MF` reader - ITAMP6MF"]
pub type Itamp6mfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TAMP1MF:"]
    #[inline(always)]
    pub fn tamp1mf(&self) -> Tamp1mfR {
        Tamp1mfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2MF"]
    #[inline(always)]
    pub fn tamp2mf(&self) -> Tamp2mfR {
        Tamp2mfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMP3MF"]
    #[inline(always)]
    pub fn tamp3mf(&self) -> Tamp3mfR {
        Tamp3mfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - ITAMP3MF"]
    #[inline(always)]
    pub fn itamp3mf(&self) -> Itamp3mfR {
        Itamp3mfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ITAMP4MF"]
    #[inline(always)]
    pub fn itamp4mf(&self) -> Itamp4mfR {
        Itamp4mfR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ITAMP5MF"]
    #[inline(always)]
    pub fn itamp5mf(&self) -> Itamp5mfR {
        Itamp5mfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ITAMP6MF"]
    #[inline(always)]
    pub fn itamp6mf(&self) -> Itamp6mfR {
        Itamp6mfR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISR")
            .field("tamp1mf", &self.tamp1mf())
            .field("tamp2mf", &self.tamp2mf())
            .field("tamp3mf", &self.tamp3mf())
            .field("itamp3mf", &self.itamp3mf())
            .field("itamp4mf", &self.itamp4mf())
            .field("itamp5mf", &self.itamp5mf())
            .field("itamp6mf", &self.itamp6mf())
            .finish()
    }
}
#[doc = "TAMP masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisrSpec;
impl crate::RegisterSpec for MisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misr::R`](R) reader structure"]
impl crate::Readable for MisrSpec {}
#[doc = "`reset()` method sets MISR to value 0"]
impl crate::Resettable for MisrSpec {}

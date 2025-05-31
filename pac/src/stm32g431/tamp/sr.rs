#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `TAMP1F` reader - TAMP1F"]
pub type Tamp1fR = crate::BitReader;
#[doc = "Field `TAMP2F` reader - TAMP2F"]
pub type Tamp2fR = crate::BitReader;
#[doc = "Field `TAMP3F` reader - TAMP3F"]
pub type Tamp3fR = crate::BitReader;
#[doc = "Field `ITAMP3F` reader - ITAMP3F"]
pub type Itamp3fR = crate::BitReader;
#[doc = "Field `ITAMP4F` reader - ITAMP4F"]
pub type Itamp4fR = crate::BitReader;
#[doc = "Field `ITAMP5F` reader - ITAMP5F"]
pub type Itamp5fR = crate::BitReader;
#[doc = "Field `ITAMP6F` reader - ITAMP6F"]
pub type Itamp6fR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TAMP1F"]
    #[inline(always)]
    pub fn tamp1f(&self) -> Tamp1fR {
        Tamp1fR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2F"]
    #[inline(always)]
    pub fn tamp2f(&self) -> Tamp2fR {
        Tamp2fR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMP3F"]
    #[inline(always)]
    pub fn tamp3f(&self) -> Tamp3fR {
        Tamp3fR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - ITAMP3F"]
    #[inline(always)]
    pub fn itamp3f(&self) -> Itamp3fR {
        Itamp3fR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ITAMP4F"]
    #[inline(always)]
    pub fn itamp4f(&self) -> Itamp4fR {
        Itamp4fR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ITAMP5F"]
    #[inline(always)]
    pub fn itamp5f(&self) -> Itamp5fR {
        Itamp5fR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ITAMP6F"]
    #[inline(always)]
    pub fn itamp6f(&self) -> Itamp6fR {
        Itamp6fR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("tamp1f", &self.tamp1f())
            .field("tamp2f", &self.tamp2f())
            .field("tamp3f", &self.tamp3f())
            .field("itamp3f", &self.itamp3f())
            .field("itamp4f", &self.itamp4f())
            .field("itamp5f", &self.itamp5f())
            .field("itamp6f", &self.itamp6f())
            .finish()
    }
}
#[doc = "TAMP status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}

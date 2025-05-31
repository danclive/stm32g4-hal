#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `YEMPTY` reader - YEMPTY"]
pub type YemptyR = crate::BitReader;
#[doc = "Field `X1FULL` reader - X1FULL"]
pub type X1fullR = crate::BitReader;
#[doc = "Field `OVFL` reader - OVFL"]
pub type OvflR = crate::BitReader;
#[doc = "Field `UNFL` reader - UNFL"]
pub type UnflR = crate::BitReader;
#[doc = "Field `SAT` reader - SAT"]
pub type SatR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - YEMPTY"]
    #[inline(always)]
    pub fn yempty(&self) -> YemptyR {
        YemptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - X1FULL"]
    #[inline(always)]
    pub fn x1full(&self) -> X1fullR {
        X1fullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - OVFL"]
    #[inline(always)]
    pub fn ovfl(&self) -> OvflR {
        OvflR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UNFL"]
    #[inline(always)]
    pub fn unfl(&self) -> UnflR {
        UnflR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SAT"]
    #[inline(always)]
    pub fn sat(&self) -> SatR {
        SatR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("yempty", &self.yempty())
            .field("x1full", &self.x1full())
            .field("ovfl", &self.ovfl())
            .field("unfl", &self.unfl())
            .field("sat", &self.sat())
            .finish()
    }
}
#[doc = "FMAC Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}

#[doc = "Register `RXF0S` reader"]
pub type R = crate::R<Rxf0sSpec>;
#[doc = "Field `F0FL` reader - F0FL"]
pub type F0flR = crate::FieldReader;
#[doc = "Field `F0GI` reader - F0GI"]
pub type F0giR = crate::FieldReader;
#[doc = "Field `F0PI` reader - F0PI"]
pub type F0piR = crate::FieldReader;
#[doc = "Field `F0F` reader - F0F"]
pub type F0fR = crate::BitReader;
#[doc = "Field `RF0L` reader - RF0L"]
pub type Rf0lR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - F0FL"]
    #[inline(always)]
    pub fn f0fl(&self) -> F0flR {
        F0flR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - F0GI"]
    #[inline(always)]
    pub fn f0gi(&self) -> F0giR {
        F0giR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - F0PI"]
    #[inline(always)]
    pub fn f0pi(&self) -> F0piR {
        F0piR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - F0F"]
    #[inline(always)]
    pub fn f0f(&self) -> F0fR {
        F0fR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RF0L"]
    #[inline(always)]
    pub fn rf0l(&self) -> Rf0lR {
        Rf0lR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXF0S")
            .field("f0fl", &self.f0fl())
            .field("f0gi", &self.f0gi())
            .field("f0pi", &self.f0pi())
            .field("f0f", &self.f0f())
            .field("rf0l", &self.rf0l())
            .finish()
    }
}
#[doc = "FDCAN Rx FIFO 0 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0s::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxf0sSpec;
impl crate::RegisterSpec for Rxf0sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf0s::R`](R) reader structure"]
impl crate::Readable for Rxf0sSpec {}
#[doc = "`reset()` method sets RXF0S to value 0"]
impl crate::Resettable for Rxf0sSpec {
    const RESET_VALUE: u32 = 0;
}

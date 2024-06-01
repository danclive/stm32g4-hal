#[doc = "Register `CPT2FR` reader"]
pub type R = crate::R<Cpt2frSpec>;
#[doc = "Field `CPT2x` reader - Timerx Capture 2 value"]
pub type Cpt2xR = crate::FieldReader<u16>;
#[doc = "Field `DIR` reader - Timerx Capture 1 Direction status"]
pub type DirR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Timerx Capture 2 value"]
    #[inline(always)]
    pub fn cpt2x(&self) -> Cpt2xR {
        Cpt2xR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Timerx Capture 1 Direction status"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPT2FR")
            .field("cpt2x", &self.cpt2x())
            .field("dir", &self.dir())
            .finish()
    }
}
#[doc = "Timerx Capture 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2fr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpt2frSpec;
impl crate::RegisterSpec for Cpt2frSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpt2fr::R`](R) reader structure"]
impl crate::Readable for Cpt2frSpec {}
#[doc = "`reset()` method sets CPT2FR to value 0"]
impl crate::Resettable for Cpt2frSpec {
    const RESET_VALUE: u32 = 0;
}

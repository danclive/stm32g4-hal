#[doc = "Register `CPT1DR` reader"]
pub type R = crate::R<Cpt1drSpec>;
#[doc = "Field `CPT1x` reader - Timerx Capture 1 value"]
pub type Cpt1xR = crate::FieldReader<u16>;
#[doc = "Field `DIR` reader - Timerx Capture 1 Direction status"]
pub type DirR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Timerx Capture 1 value"]
    #[inline(always)]
    pub fn cpt1x(&self) -> Cpt1xR {
        Cpt1xR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Timerx Capture 1 Direction status"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPT1DR")
            .field("cpt1x", &self.cpt1x())
            .field("dir", &self.dir())
            .finish()
    }
}
#[doc = "Timerx Capture 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpt1drSpec;
impl crate::RegisterSpec for Cpt1drSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpt1dr::R`](R) reader structure"]
impl crate::Readable for Cpt1drSpec {}
#[doc = "`reset()` method sets CPT1DR to value 0"]
impl crate::Resettable for Cpt1drSpec {
    const RESET_VALUE: u32 = 0;
}

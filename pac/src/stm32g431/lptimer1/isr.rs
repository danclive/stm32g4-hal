#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `CMPM` reader - Compare match"]
pub type CmpmR = crate::BitReader;
#[doc = "Field `ARRM` reader - Autoreload match"]
pub type ArrmR = crate::BitReader;
#[doc = "Field `EXTTRIG` reader - External trigger edge event"]
pub type ExttrigR = crate::BitReader;
#[doc = "Field `CMPOK` reader - Compare register update OK"]
pub type CmpokR = crate::BitReader;
#[doc = "Field `ARROK` reader - Autoreload register update OK"]
pub type ArrokR = crate::BitReader;
#[doc = "Field `UP` reader - Counter direction change down to up"]
pub type UpR = crate::BitReader;
#[doc = "Field `DOWN` reader - Counter direction change up to down"]
pub type DownR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Compare match"]
    #[inline(always)]
    pub fn cmpm(&self) -> CmpmR {
        CmpmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Autoreload match"]
    #[inline(always)]
    pub fn arrm(&self) -> ArrmR {
        ArrmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External trigger edge event"]
    #[inline(always)]
    pub fn exttrig(&self) -> ExttrigR {
        ExttrigR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare register update OK"]
    #[inline(always)]
    pub fn cmpok(&self) -> CmpokR {
        CmpokR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK"]
    #[inline(always)]
    pub fn arrok(&self) -> ArrokR {
        ArrokR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Counter direction change down to up"]
    #[inline(always)]
    pub fn up(&self) -> UpR {
        UpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Counter direction change up to down"]
    #[inline(always)]
    pub fn down(&self) -> DownR {
        DownR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("down", &self.down())
            .field("up", &self.up())
            .field("arrok", &self.arrok())
            .field("cmpok", &self.cmpok())
            .field("exttrig", &self.exttrig())
            .field("arrm", &self.arrm())
            .field("cmpm", &self.cmpm())
            .finish()
    }
}
#[doc = "Interrupt and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}

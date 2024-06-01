#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `FLT1` reader - Fault 1 Interrupt Flag"]
pub type Flt1R = crate::BitReader;
#[doc = "Field `FLT2` reader - Fault 2 Interrupt Flag"]
pub type Flt2R = crate::BitReader;
#[doc = "Field `FLT3` reader - Fault 3 Interrupt Flag"]
pub type Flt3R = crate::BitReader;
#[doc = "Field `FLT4` reader - Fault 4 Interrupt Flag"]
pub type Flt4R = crate::BitReader;
#[doc = "Field `FLT5` reader - Fault 5 Interrupt Flag"]
pub type Flt5R = crate::BitReader;
#[doc = "Field `SYSFLT` reader - System Fault Interrupt Flag"]
pub type SysfltR = crate::BitReader;
#[doc = "Field `FLT6` reader - Fault 6 Interrupt Flag"]
pub type Flt6R = crate::BitReader;
#[doc = "Field `DLLRDY` reader - DLL Ready Interrupt Flag"]
pub type DllrdyR = crate::BitReader;
#[doc = "Field `BMPER` reader - Burst mode Period Interrupt Flag"]
pub type BmperR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Fault 1 Interrupt Flag"]
    #[inline(always)]
    pub fn flt1(&self) -> Flt1R {
        Flt1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Flag"]
    #[inline(always)]
    pub fn flt2(&self) -> Flt2R {
        Flt2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Flag"]
    #[inline(always)]
    pub fn flt3(&self) -> Flt3R {
        Flt3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Flag"]
    #[inline(always)]
    pub fn flt4(&self) -> Flt4R {
        Flt4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Flag"]
    #[inline(always)]
    pub fn flt5(&self) -> Flt5R {
        Flt5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - System Fault Interrupt Flag"]
    #[inline(always)]
    pub fn sysflt(&self) -> SysfltR {
        SysfltR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fault 6 Interrupt Flag"]
    #[inline(always)]
    pub fn flt6(&self) -> Flt6R {
        Flt6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt Flag"]
    #[inline(always)]
    pub fn dllrdy(&self) -> DllrdyR {
        DllrdyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Burst mode Period Interrupt Flag"]
    #[inline(always)]
    pub fn bmper(&self) -> BmperR {
        BmperR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("bmper", &self.bmper())
            .field("dllrdy", &self.dllrdy())
            .field("flt6", &self.flt6())
            .field("sysflt", &self.sysflt())
            .field("flt5", &self.flt5())
            .field("flt4", &self.flt4())
            .field("flt3", &self.flt3())
            .field("flt2", &self.flt2())
            .field("flt1", &self.flt1())
            .finish()
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0;
}

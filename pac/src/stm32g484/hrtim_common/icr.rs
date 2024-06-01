#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `FLT1C` writer - Fault 1 Interrupt Flag Clear"]
pub type Flt1cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2C` writer - Fault 2 Interrupt Flag Clear"]
pub type Flt2cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT3C` writer - Fault 3 Interrupt Flag Clear"]
pub type Flt3cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT4C` writer - Fault 4 Interrupt Flag Clear"]
pub type Flt4cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5C` writer - Fault 5 Interrupt Flag Clear"]
pub type Flt5cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSFLTC` writer - System Fault Interrupt Flag Clear"]
pub type SysfltcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6C` writer - Fault 6 Interrupt Flag Clear"]
pub type Flt6cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLLRDYC` writer - DLL Ready Interrupt flag Clear"]
pub type DllrdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMPERC` writer - Burst mode period flag Clear"]
pub type BmpercW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IcrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Fault 1 Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flt1c(&mut self) -> Flt1cW<IcrSpec> {
        Flt1cW::new(self, 0)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flt2c(&mut self) -> Flt2cW<IcrSpec> {
        Flt2cW::new(self, 1)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flt3c(&mut self) -> Flt3cW<IcrSpec> {
        Flt3cW::new(self, 2)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flt4c(&mut self) -> Flt4cW<IcrSpec> {
        Flt4cW::new(self, 3)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flt5c(&mut self) -> Flt5cW<IcrSpec> {
        Flt5cW::new(self, 4)
    }
    #[doc = "Bit 5 - System Fault Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn sysfltc(&mut self) -> SysfltcW<IcrSpec> {
        SysfltcW::new(self, 5)
    }
    #[doc = "Bit 6 - Fault 6 Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flt6c(&mut self) -> Flt6cW<IcrSpec> {
        Flt6cW::new(self, 6)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dllrdyc(&mut self) -> DllrdycW<IcrSpec> {
        DllrdycW::new(self, 16)
    }
    #[doc = "Bit 17 - Burst mode period flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bmperc(&mut self) -> BmpercW<IcrSpec> {
        BmpercW::new(self, 17)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {
    const RESET_VALUE: u32 = 0;
}

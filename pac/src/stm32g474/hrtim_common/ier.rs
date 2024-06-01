#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `FLT1IE` reader - Fault 1 Interrupt Enable"]
pub type Flt1ieR = crate::BitReader;
#[doc = "Field `FLT1IE` writer - Fault 1 Interrupt Enable"]
pub type Flt1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2IE` reader - Fault 2 Interrupt Enable"]
pub type Flt2ieR = crate::BitReader;
#[doc = "Field `FLT2IE` writer - Fault 2 Interrupt Enable"]
pub type Flt2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT3IE` reader - Fault 3 Interrupt Enable"]
pub type Flt3ieR = crate::BitReader;
#[doc = "Field `FLT3IE` writer - Fault 3 Interrupt Enable"]
pub type Flt3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT4IE` reader - Fault 4 Interrupt Enable"]
pub type Flt4ieR = crate::BitReader;
#[doc = "Field `FLT4IE` writer - Fault 4 Interrupt Enable"]
pub type Flt4ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5IE` reader - Fault 5 Interrupt Enable"]
pub type Flt5ieR = crate::BitReader;
#[doc = "Field `FLT5IE` writer - Fault 5 Interrupt Enable"]
pub type Flt5ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSFLTE` reader - System Fault Interrupt Enable"]
pub type SysflteR = crate::BitReader;
#[doc = "Field `SYSFLTE` writer - System Fault Interrupt Enable"]
pub type SysflteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6IE` reader - Fault 6 Interrupt Enable"]
pub type Flt6ieR = crate::BitReader;
#[doc = "Field `FLT6IE` writer - Fault 6 Interrupt Enable"]
pub type Flt6ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLLRDYIE` reader - DLL Ready Interrupt Enable"]
pub type DllrdyieR = crate::BitReader;
#[doc = "Field `DLLRDYIE` writer - DLL Ready Interrupt Enable"]
pub type DllrdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMPERIE` reader - Burst mode period Interrupt Enable"]
pub type BmperieR = crate::BitReader;
#[doc = "Field `BMPERIE` writer - Burst mode period Interrupt Enable"]
pub type BmperieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fault 1 Interrupt Enable"]
    #[inline(always)]
    pub fn flt1ie(&self) -> Flt1ieR {
        Flt1ieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Enable"]
    #[inline(always)]
    pub fn flt2ie(&self) -> Flt2ieR {
        Flt2ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Enable"]
    #[inline(always)]
    pub fn flt3ie(&self) -> Flt3ieR {
        Flt3ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Enable"]
    #[inline(always)]
    pub fn flt4ie(&self) -> Flt4ieR {
        Flt4ieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Enable"]
    #[inline(always)]
    pub fn flt5ie(&self) -> Flt5ieR {
        Flt5ieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - System Fault Interrupt Enable"]
    #[inline(always)]
    pub fn sysflte(&self) -> SysflteR {
        SysflteR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fault 6 Interrupt Enable"]
    #[inline(always)]
    pub fn flt6ie(&self) -> Flt6ieR {
        Flt6ieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dllrdyie(&self) -> DllrdyieR {
        DllrdyieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Burst mode period Interrupt Enable"]
    #[inline(always)]
    pub fn bmperie(&self) -> BmperieR {
        BmperieR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("bmperie", &self.bmperie())
            .field("dllrdyie", &self.dllrdyie())
            .field("flt6ie", &self.flt6ie())
            .field("sysflte", &self.sysflte())
            .field("flt5ie", &self.flt5ie())
            .field("flt4ie", &self.flt4ie())
            .field("flt3ie", &self.flt3ie())
            .field("flt2ie", &self.flt2ie())
            .field("flt1ie", &self.flt1ie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Fault 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt1ie(&mut self) -> Flt1ieW<IerSpec> {
        Flt1ieW::new(self, 0)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt2ie(&mut self) -> Flt2ieW<IerSpec> {
        Flt2ieW::new(self, 1)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt3ie(&mut self) -> Flt3ieW<IerSpec> {
        Flt3ieW::new(self, 2)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt4ie(&mut self) -> Flt4ieW<IerSpec> {
        Flt4ieW::new(self, 3)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt5ie(&mut self) -> Flt5ieW<IerSpec> {
        Flt5ieW::new(self, 4)
    }
    #[doc = "Bit 5 - System Fault Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sysflte(&mut self) -> SysflteW<IerSpec> {
        SysflteW::new(self, 5)
    }
    #[doc = "Bit 6 - Fault 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt6ie(&mut self) -> Flt6ieW<IerSpec> {
        Flt6ieW::new(self, 6)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dllrdyie(&mut self) -> DllrdyieW<IerSpec> {
        DllrdyieW::new(self, 16)
    }
    #[doc = "Bit 17 - Burst mode period Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bmperie(&mut self) -> BmperieW<IerSpec> {
        BmperieW::new(self, 17)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {
    const RESET_VALUE: u32 = 0;
}

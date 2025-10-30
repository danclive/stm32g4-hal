#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `TAMP1E` reader - TAMP1E"]
pub type Tamp1eR = crate::BitReader;
#[doc = "Field `TAMP1E` writer - TAMP1E"]
pub type Tamp1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2E` reader - TAMP2E"]
pub type Tamp2eR = crate::BitReader;
#[doc = "Field `TAMP2E` writer - TAMP2E"]
pub type Tamp2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3E` reader - TAMP2E"]
pub type Tamp3eR = crate::BitReader;
#[doc = "Field `TAMP3E` writer - TAMP2E"]
pub type Tamp3eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP3E` reader - ITAMP3E"]
pub type Itamp3eR = crate::BitReader;
#[doc = "Field `ITAMP3E` writer - ITAMP3E"]
pub type Itamp3eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP4E` reader - ITAMP4E"]
pub type Itamp4eR = crate::BitReader;
#[doc = "Field `ITAMP4E` writer - ITAMP4E"]
pub type Itamp4eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP5E` reader - ITAMP5E"]
pub type Itamp5eR = crate::BitReader;
#[doc = "Field `ITAMP5E` writer - ITAMP5E"]
pub type Itamp5eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP6E` reader - ITAMP6E"]
pub type Itamp6eR = crate::BitReader;
#[doc = "Field `ITAMP6E` writer - ITAMP6E"]
pub type Itamp6eW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TAMP1E"]
    #[inline(always)]
    pub fn tamp1e(&self) -> Tamp1eR {
        Tamp1eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2E"]
    #[inline(always)]
    pub fn tamp2e(&self) -> Tamp2eR {
        Tamp2eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMP2E"]
    #[inline(always)]
    pub fn tamp3e(&self) -> Tamp3eR {
        Tamp3eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - ITAMP3E"]
    #[inline(always)]
    pub fn itamp3e(&self) -> Itamp3eR {
        Itamp3eR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ITAMP4E"]
    #[inline(always)]
    pub fn itamp4e(&self) -> Itamp4eR {
        Itamp4eR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ITAMP5E"]
    #[inline(always)]
    pub fn itamp5e(&self) -> Itamp5eR {
        Itamp5eR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ITAMP6E"]
    #[inline(always)]
    pub fn itamp6e(&self) -> Itamp6eR {
        Itamp6eR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("tamp1e", &self.tamp1e())
            .field("tamp2e", &self.tamp2e())
            .field("tamp3e", &self.tamp3e())
            .field("itamp3e", &self.itamp3e())
            .field("itamp4e", &self.itamp4e())
            .field("itamp5e", &self.itamp5e())
            .field("itamp6e", &self.itamp6e())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - TAMP1E"]
    #[inline(always)]
    pub fn tamp1e(&mut self) -> Tamp1eW<'_, Cr1Spec> {
        Tamp1eW::new(self, 0)
    }
    #[doc = "Bit 1 - TAMP2E"]
    #[inline(always)]
    pub fn tamp2e(&mut self) -> Tamp2eW<'_, Cr1Spec> {
        Tamp2eW::new(self, 1)
    }
    #[doc = "Bit 2 - TAMP2E"]
    #[inline(always)]
    pub fn tamp3e(&mut self) -> Tamp3eW<'_, Cr1Spec> {
        Tamp3eW::new(self, 2)
    }
    #[doc = "Bit 18 - ITAMP3E"]
    #[inline(always)]
    pub fn itamp3e(&mut self) -> Itamp3eW<'_, Cr1Spec> {
        Itamp3eW::new(self, 18)
    }
    #[doc = "Bit 19 - ITAMP4E"]
    #[inline(always)]
    pub fn itamp4e(&mut self) -> Itamp4eW<'_, Cr1Spec> {
        Itamp4eW::new(self, 19)
    }
    #[doc = "Bit 20 - ITAMP5E"]
    #[inline(always)]
    pub fn itamp5e(&mut self) -> Itamp5eW<'_, Cr1Spec> {
        Itamp5eW::new(self, 20)
    }
    #[doc = "Bit 21 - ITAMP6E"]
    #[inline(always)]
    pub fn itamp6e(&mut self) -> Itamp6eW<'_, Cr1Spec> {
        Itamp6eW::new(self, 21)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0xffff_0000"]
impl crate::Resettable for Cr1Spec {
    const RESET_VALUE: u32 = 0xffff_0000;
}

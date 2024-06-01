#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `TAMP1IE` reader - TAMP1IE"]
pub type Tamp1ieR = crate::BitReader;
#[doc = "Field `TAMP1IE` writer - TAMP1IE"]
pub type Tamp1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2IE` reader - TAMP2IE"]
pub type Tamp2ieR = crate::BitReader;
#[doc = "Field `TAMP2IE` writer - TAMP2IE"]
pub type Tamp2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3IE` reader - TAMP3IE"]
pub type Tamp3ieR = crate::BitReader;
#[doc = "Field `TAMP3IE` writer - TAMP3IE"]
pub type Tamp3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP3IE` reader - ITAMP3IE"]
pub type Itamp3ieR = crate::BitReader;
#[doc = "Field `ITAMP3IE` writer - ITAMP3IE"]
pub type Itamp3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP4IE` reader - ITAMP4IE"]
pub type Itamp4ieR = crate::BitReader;
#[doc = "Field `ITAMP4IE` writer - ITAMP4IE"]
pub type Itamp4ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP5IE` reader - ITAMP5IE"]
pub type Itamp5ieR = crate::BitReader;
#[doc = "Field `ITAMP5IE` writer - ITAMP5IE"]
pub type Itamp5ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP6IE` reader - ITAMP6IE"]
pub type Itamp6ieR = crate::BitReader;
#[doc = "Field `ITAMP6IE` writer - ITAMP6IE"]
pub type Itamp6ieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TAMP1IE"]
    #[inline(always)]
    pub fn tamp1ie(&self) -> Tamp1ieR {
        Tamp1ieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2IE"]
    #[inline(always)]
    pub fn tamp2ie(&self) -> Tamp2ieR {
        Tamp2ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMP3IE"]
    #[inline(always)]
    pub fn tamp3ie(&self) -> Tamp3ieR {
        Tamp3ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - ITAMP3IE"]
    #[inline(always)]
    pub fn itamp3ie(&self) -> Itamp3ieR {
        Itamp3ieR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ITAMP4IE"]
    #[inline(always)]
    pub fn itamp4ie(&self) -> Itamp4ieR {
        Itamp4ieR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ITAMP5IE"]
    #[inline(always)]
    pub fn itamp5ie(&self) -> Itamp5ieR {
        Itamp5ieR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ITAMP6IE"]
    #[inline(always)]
    pub fn itamp6ie(&self) -> Itamp6ieR {
        Itamp6ieR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("tamp1ie", &self.tamp1ie())
            .field("tamp2ie", &self.tamp2ie())
            .field("tamp3ie", &self.tamp3ie())
            .field("itamp3ie", &self.itamp3ie())
            .field("itamp4ie", &self.itamp4ie())
            .field("itamp5ie", &self.itamp5ie())
            .field("itamp6ie", &self.itamp6ie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - TAMP1IE"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1ie(&mut self) -> Tamp1ieW<IerSpec> {
        Tamp1ieW::new(self, 0)
    }
    #[doc = "Bit 1 - TAMP2IE"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2ie(&mut self) -> Tamp2ieW<IerSpec> {
        Tamp2ieW::new(self, 1)
    }
    #[doc = "Bit 2 - TAMP3IE"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3ie(&mut self) -> Tamp3ieW<IerSpec> {
        Tamp3ieW::new(self, 2)
    }
    #[doc = "Bit 18 - ITAMP3IE"]
    #[inline(always)]
    #[must_use]
    pub fn itamp3ie(&mut self) -> Itamp3ieW<IerSpec> {
        Itamp3ieW::new(self, 18)
    }
    #[doc = "Bit 19 - ITAMP4IE"]
    #[inline(always)]
    #[must_use]
    pub fn itamp4ie(&mut self) -> Itamp4ieW<IerSpec> {
        Itamp4ieW::new(self, 19)
    }
    #[doc = "Bit 20 - ITAMP5IE"]
    #[inline(always)]
    #[must_use]
    pub fn itamp5ie(&mut self) -> Itamp5ieW<IerSpec> {
        Itamp5ieW::new(self, 20)
    }
    #[doc = "Bit 21 - ITAMP6IE"]
    #[inline(always)]
    #[must_use]
    pub fn itamp6ie(&mut self) -> Itamp6ieW<IerSpec> {
        Itamp6ieW::new(self, 21)
    }
}
#[doc = "TAMP interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

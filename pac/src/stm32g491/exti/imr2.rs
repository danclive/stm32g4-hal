#[doc = "Register `IMR2` reader"]
pub type R = crate::R<Imr2Spec>;
#[doc = "Register `IMR2` writer"]
pub type W = crate::W<Imr2Spec>;
#[doc = "Field `IM32` reader - Interrupt Mask on external/internal line 32"]
pub type Im32R = crate::BitReader;
#[doc = "Field `IM32` writer - Interrupt Mask on external/internal line 32"]
pub type Im32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM33` reader - Interrupt Mask on external/internal line 33"]
pub type Im33R = crate::BitReader;
#[doc = "Field `IM33` writer - Interrupt Mask on external/internal line 33"]
pub type Im33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM34` reader - Interrupt Mask on external/internal line 34"]
pub type Im34R = crate::BitReader;
#[doc = "Field `IM34` writer - Interrupt Mask on external/internal line 34"]
pub type Im34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM35` reader - Interrupt Mask on external/internal line 35"]
pub type Im35R = crate::BitReader;
#[doc = "Field `IM35` writer - Interrupt Mask on external/internal line 35"]
pub type Im35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM36` reader - Interrupt Mask on external/internal line 36"]
pub type Im36R = crate::BitReader;
#[doc = "Field `IM36` writer - Interrupt Mask on external/internal line 36"]
pub type Im36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM37` reader - Interrupt Mask on external/internal line 37"]
pub type Im37R = crate::BitReader;
#[doc = "Field `IM37` writer - Interrupt Mask on external/internal line 37"]
pub type Im37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM38` reader - Interrupt Mask on external/internal line 38"]
pub type Im38R = crate::BitReader;
#[doc = "Field `IM38` writer - Interrupt Mask on external/internal line 38"]
pub type Im38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM39` reader - Interrupt Mask on external/internal line 39"]
pub type Im39R = crate::BitReader;
#[doc = "Field `IM39` writer - Interrupt Mask on external/internal line 39"]
pub type Im39W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM40` reader - Interrupt Mask on external/internal line 40"]
pub type Im40R = crate::BitReader;
#[doc = "Field `IM40` writer - Interrupt Mask on external/internal line 40"]
pub type Im40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM41` reader - Interrupt Mask on external/internal line 41"]
pub type Im41R = crate::BitReader;
#[doc = "Field `IM41` writer - Interrupt Mask on external/internal line 41"]
pub type Im41W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM42` reader - Interrupt Mask on external/internal line 42"]
pub type Im42R = crate::BitReader;
#[doc = "Field `IM42` writer - Interrupt Mask on external/internal line 42"]
pub type Im42W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM43` reader - Interrupt Mask on external/internal line 43"]
pub type Im43R = crate::BitReader;
#[doc = "Field `IM43` writer - Interrupt Mask on external/internal line 43"]
pub type Im43W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt Mask on external/internal line 32"]
    #[inline(always)]
    pub fn im32(&self) -> Im32R {
        Im32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Mask on external/internal line 33"]
    #[inline(always)]
    pub fn im33(&self) -> Im33R {
        Im33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Mask on external/internal line 34"]
    #[inline(always)]
    pub fn im34(&self) -> Im34R {
        Im34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Mask on external/internal line 35"]
    #[inline(always)]
    pub fn im35(&self) -> Im35R {
        Im35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Mask on external/internal line 36"]
    #[inline(always)]
    pub fn im36(&self) -> Im36R {
        Im36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Mask on external/internal line 37"]
    #[inline(always)]
    pub fn im37(&self) -> Im37R {
        Im37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Mask on external/internal line 38"]
    #[inline(always)]
    pub fn im38(&self) -> Im38R {
        Im38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Mask on external/internal line 39"]
    #[inline(always)]
    pub fn im39(&self) -> Im39R {
        Im39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Mask on external/internal line 40"]
    #[inline(always)]
    pub fn im40(&self) -> Im40R {
        Im40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Mask on external/internal line 41"]
    #[inline(always)]
    pub fn im41(&self) -> Im41R {
        Im41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Mask on external/internal line 42"]
    #[inline(always)]
    pub fn im42(&self) -> Im42R {
        Im42R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Mask on external/internal line 43"]
    #[inline(always)]
    pub fn im43(&self) -> Im43R {
        Im43R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR2")
            .field("im32", &self.im32())
            .field("im33", &self.im33())
            .field("im34", &self.im34())
            .field("im35", &self.im35())
            .field("im36", &self.im36())
            .field("im37", &self.im37())
            .field("im38", &self.im38())
            .field("im39", &self.im39())
            .field("im40", &self.im40())
            .field("im41", &self.im41())
            .field("im42", &self.im42())
            .field("im43", &self.im43())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Mask on external/internal line 32"]
    #[inline(always)]
    #[must_use]
    pub fn im32(&mut self) -> Im32W<Imr2Spec> {
        Im32W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt Mask on external/internal line 33"]
    #[inline(always)]
    #[must_use]
    pub fn im33(&mut self) -> Im33W<Imr2Spec> {
        Im33W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt Mask on external/internal line 34"]
    #[inline(always)]
    #[must_use]
    pub fn im34(&mut self) -> Im34W<Imr2Spec> {
        Im34W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt Mask on external/internal line 35"]
    #[inline(always)]
    #[must_use]
    pub fn im35(&mut self) -> Im35W<Imr2Spec> {
        Im35W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt Mask on external/internal line 36"]
    #[inline(always)]
    #[must_use]
    pub fn im36(&mut self) -> Im36W<Imr2Spec> {
        Im36W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt Mask on external/internal line 37"]
    #[inline(always)]
    #[must_use]
    pub fn im37(&mut self) -> Im37W<Imr2Spec> {
        Im37W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt Mask on external/internal line 38"]
    #[inline(always)]
    #[must_use]
    pub fn im38(&mut self) -> Im38W<Imr2Spec> {
        Im38W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt Mask on external/internal line 39"]
    #[inline(always)]
    #[must_use]
    pub fn im39(&mut self) -> Im39W<Imr2Spec> {
        Im39W::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt Mask on external/internal line 40"]
    #[inline(always)]
    #[must_use]
    pub fn im40(&mut self) -> Im40W<Imr2Spec> {
        Im40W::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt Mask on external/internal line 41"]
    #[inline(always)]
    #[must_use]
    pub fn im41(&mut self) -> Im41W<Imr2Spec> {
        Im41W::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt Mask on external/internal line 42"]
    #[inline(always)]
    #[must_use]
    pub fn im42(&mut self) -> Im42W<Imr2Spec> {
        Im42W::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt Mask on external/internal line 43"]
    #[inline(always)]
    #[must_use]
    pub fn im43(&mut self) -> Im43W<Imr2Spec> {
        Im43W::new(self, 11)
    }
}
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Imr2Spec;
impl crate::RegisterSpec for Imr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr2::R`](R) reader structure"]
impl crate::Readable for Imr2Spec {}
#[doc = "`write(|w| ..)` method takes [`imr2::W`](W) writer structure"]
impl crate::Writable for Imr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR2 to value 0xffff_ff87"]
impl crate::Resettable for Imr2Spec {
    const RESET_VALUE: u32 = 0xffff_ff87;
}

#[doc = "Register `ILE` reader"]
pub type R = crate::R<IleSpec>;
#[doc = "Register `ILE` writer"]
pub type W = crate::W<IleSpec>;
#[doc = "Field `EINT0` reader - EINT0"]
pub type Eint0R = crate::BitReader;
#[doc = "Field `EINT0` writer - EINT0"]
pub type Eint0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EINT1` reader - EINT1"]
pub type Eint1R = crate::BitReader;
#[doc = "Field `EINT1` writer - EINT1"]
pub type Eint1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EINT0"]
    #[inline(always)]
    pub fn eint0(&self) -> Eint0R {
        Eint0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EINT1"]
    #[inline(always)]
    pub fn eint1(&self) -> Eint1R {
        Eint1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ILE")
            .field("eint0", &self.eint0())
            .field("eint1", &self.eint1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - EINT0"]
    #[inline(always)]
    #[must_use]
    pub fn eint0(&mut self) -> Eint0W<IleSpec> {
        Eint0W::new(self, 0)
    }
    #[doc = "Bit 1 - EINT1"]
    #[inline(always)]
    #[must_use]
    pub fn eint1(&mut self) -> Eint1W<IleSpec> {
        Eint1W::new(self, 1)
    }
}
#[doc = "Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ile::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ile::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IleSpec;
impl crate::RegisterSpec for IleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ile::R`](R) reader structure"]
impl crate::Readable for IleSpec {}
#[doc = "`write(|w| ..)` method takes [`ile::W`](W) writer structure"]
impl crate::Writable for IleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ILE to value 0"]
impl crate::Resettable for IleSpec {
    const RESET_VALUE: u32 = 0;
}

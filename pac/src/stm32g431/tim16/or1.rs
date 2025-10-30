#[doc = "Register `OR1` reader"]
pub type R = crate::R<Or1Spec>;
#[doc = "Register `OR1` writer"]
pub type W = crate::W<Or1Spec>;
#[doc = "Field `HSE32EN` reader - HSE Divided by 32 enable"]
pub type Hse32enR = crate::BitReader;
#[doc = "Field `HSE32EN` writer - HSE Divided by 32 enable"]
pub type Hse32enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HSE Divided by 32 enable"]
    #[inline(always)]
    pub fn hse32en(&self) -> Hse32enR {
        Hse32enR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR1")
            .field("hse32en", &self.hse32en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - HSE Divided by 32 enable"]
    #[inline(always)]
    pub fn hse32en(&mut self) -> Hse32enW<'_, Or1Spec> {
        Hse32enW::new(self, 0)
    }
}
#[doc = "TIM option register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`or1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Or1Spec;
impl crate::RegisterSpec for Or1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or1::R`](R) reader structure"]
impl crate::Readable for Or1Spec {}
#[doc = "`write(|w| ..)` method takes [`or1::W`](W) writer structure"]
impl crate::Writable for Or1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OR1 to value 0"]
impl crate::Resettable for Or1Spec {}

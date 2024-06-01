#[doc = "Register `RXF0A` reader"]
pub type R = crate::R<Rxf0aSpec>;
#[doc = "Register `RXF0A` writer"]
pub type W = crate::W<Rxf0aSpec>;
#[doc = "Field `F0AI` reader - F0AI"]
pub type F0aiR = crate::FieldReader;
#[doc = "Field `F0AI` writer - F0AI"]
pub type F0aiW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - F0AI"]
    #[inline(always)]
    pub fn f0ai(&self) -> F0aiR {
        F0aiR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXF0A").field("f0ai", &self.f0ai()).finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - F0AI"]
    #[inline(always)]
    #[must_use]
    pub fn f0ai(&mut self) -> F0aiW<Rxf0aSpec> {
        F0aiW::new(self, 0)
    }
}
#[doc = "CAN Rx FIFO 0 Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf0a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxf0aSpec;
impl crate::RegisterSpec for Rxf0aSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf0a::R`](R) reader structure"]
impl crate::Readable for Rxf0aSpec {}
#[doc = "`write(|w| ..)` method takes [`rxf0a::W`](W) writer structure"]
impl crate::Writable for Rxf0aSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXF0A to value 0"]
impl crate::Resettable for Rxf0aSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `RX_ORDEXT1` reader"]
pub type R = crate::R<RxOrdext1Spec>;
#[doc = "Register `RX_ORDEXT1` writer"]
pub type W = crate::W<RxOrdext1Spec>;
#[doc = "Field `RXSOPX1` reader - RXSOPX1"]
pub type Rxsopx1R = crate::FieldReader<u32>;
#[doc = "Field `RXSOPX1` writer - RXSOPX1"]
pub type Rxsopx1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - RXSOPX1"]
    #[inline(always)]
    pub fn rxsopx1(&self) -> Rxsopx1R {
        Rxsopx1R::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_ORDEXT1")
            .field("rxsopx1", &self.rxsopx1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - RXSOPX1"]
    #[inline(always)]
    #[must_use]
    pub fn rxsopx1(&mut self) -> Rxsopx1W<RxOrdext1Spec> {
        Rxsopx1W::new(self, 0)
    }
}
#[doc = "UCPD Rx Ordered Set Extension Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ordext1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ordext1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxOrdext1Spec;
impl crate::RegisterSpec for RxOrdext1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ordext1::R`](R) reader structure"]
impl crate::Readable for RxOrdext1Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_ordext1::W`](W) writer structure"]
impl crate::Writable for RxOrdext1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_ORDEXT1 to value 0"]
impl crate::Resettable for RxOrdext1Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `RX_ORDEXT2` reader"]
pub type R = crate::R<RxOrdext2Spec>;
#[doc = "Register `RX_ORDEXT2` writer"]
pub type W = crate::W<RxOrdext2Spec>;
#[doc = "Field `RXSOPX2` reader - RXSOPX2"]
pub type Rxsopx2R = crate::FieldReader<u32>;
#[doc = "Field `RXSOPX2` writer - RXSOPX2"]
pub type Rxsopx2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - RXSOPX2"]
    #[inline(always)]
    pub fn rxsopx2(&self) -> Rxsopx2R {
        Rxsopx2R::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_ORDEXT2")
            .field("rxsopx2", &self.rxsopx2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - RXSOPX2"]
    #[inline(always)]
    #[must_use]
    pub fn rxsopx2(&mut self) -> Rxsopx2W<RxOrdext2Spec> {
        Rxsopx2W::new(self, 0)
    }
}
#[doc = "UCPD Rx Ordered Set Extension Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ordext2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_ordext2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxOrdext2Spec;
impl crate::RegisterSpec for RxOrdext2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ordext2::R`](R) reader structure"]
impl crate::Readable for RxOrdext2Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_ordext2::W`](W) writer structure"]
impl crate::Writable for RxOrdext2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_ORDEXT2 to value 0"]
impl crate::Resettable for RxOrdext2Spec {
    const RESET_VALUE: u32 = 0;
}

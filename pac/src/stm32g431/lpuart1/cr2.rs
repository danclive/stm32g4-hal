#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `ADDM7` reader - 7-bit Address Detection/4-bit Address Detection"]
pub type Addm7R = crate::BitReader;
#[doc = "Field `ADDM7` writer - 7-bit Address Detection/4-bit Address Detection"]
pub type Addm7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - STOP bits"]
pub type StopR = crate::FieldReader;
#[doc = "Field `STOP` writer - STOP bits"]
pub type StopW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWAP` reader - Swap TX/RX pins"]
pub type SwapR = crate::BitReader;
#[doc = "Field `SWAP` writer - Swap TX/RX pins"]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINV` reader - RX pin active level inversion"]
pub type RxinvR = crate::BitReader;
#[doc = "Field `RXINV` writer - RX pin active level inversion"]
pub type RxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXINV` reader - TX pin active level inversion"]
pub type TxinvR = crate::BitReader;
#[doc = "Field `TXINV` writer - TX pin active level inversion"]
pub type TxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAINV` reader - Binary data inversion"]
pub type TainvR = crate::BitReader;
#[doc = "Field `TAINV` writer - Binary data inversion"]
pub type TainvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSBFIRST` reader - Most significant bit first"]
pub type MsbfirstR = crate::BitReader;
#[doc = "Field `MSBFIRST` writer - Most significant bit first"]
pub type MsbfirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD0_3` reader - Address of the USART node"]
pub type Add0_3R = crate::FieldReader;
#[doc = "Field `ADD0_3` writer - Address of the USART node"]
pub type Add0_3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADD4_7` reader - Address of the USART node"]
pub type Add4_7R = crate::FieldReader;
#[doc = "Field `ADD4_7` writer - Address of the USART node"]
pub type Add4_7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    pub fn addm7(&self) -> Addm7R {
        Addm7R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    pub fn rxinv(&self) -> RxinvR {
        RxinvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    pub fn txinv(&self) -> TxinvR {
        TxinvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    pub fn tainv(&self) -> TainvR {
        TainvR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    pub fn msbfirst(&self) -> MsbfirstR {
        MsbfirstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Address of the USART node"]
    #[inline(always)]
    pub fn add0_3(&self) -> Add0_3R {
        Add0_3R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Address of the USART node"]
    #[inline(always)]
    pub fn add4_7(&self) -> Add4_7R {
        Add4_7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("add4_7", &self.add4_7())
            .field("add0_3", &self.add0_3())
            .field("msbfirst", &self.msbfirst())
            .field("tainv", &self.tainv())
            .field("txinv", &self.txinv())
            .field("rxinv", &self.rxinv())
            .field("swap", &self.swap())
            .field("stop", &self.stop())
            .field("addm7", &self.addm7())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    pub fn addm7(&mut self) -> Addm7W<Cr2Spec> {
        Addm7W::new(self, 4)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<Cr2Spec> {
        StopW::new(self, 12)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    pub fn swap(&mut self) -> SwapW<Cr2Spec> {
        SwapW::new(self, 15)
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RxinvW<Cr2Spec> {
        RxinvW::new(self, 16)
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    pub fn txinv(&mut self) -> TxinvW<Cr2Spec> {
        TxinvW::new(self, 17)
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    pub fn tainv(&mut self) -> TainvW<Cr2Spec> {
        TainvW::new(self, 18)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    pub fn msbfirst(&mut self) -> MsbfirstW<Cr2Spec> {
        MsbfirstW::new(self, 19)
    }
    #[doc = "Bits 24:27 - Address of the USART node"]
    #[inline(always)]
    pub fn add0_3(&mut self) -> Add0_3W<Cr2Spec> {
        Add0_3W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Address of the USART node"]
    #[inline(always)]
    pub fn add4_7(&mut self) -> Add4_7W<Cr2Spec> {
        Add4_7W::new(self, 28)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {
    const RESET_VALUE: u32 = 0;
}

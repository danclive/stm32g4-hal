#[doc = "Register `FDCAN_RXF1A` reader"]
pub type R = crate::R<FdcanRxf1aSpec>;
#[doc = "Register `FDCAN_RXF1A` writer"]
pub type W = crate::W<FdcanRxf1aSpec>;
#[doc = "Field `F1AI` reader - Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFO 1 get index RXF1S\\[F1GI\\] to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\\[F1FL\\]."]
pub type F1aiR = crate::FieldReader;
#[doc = "Field `F1AI` writer - Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFO 1 get index RXF1S\\[F1GI\\] to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\\[F1FL\\]."]
pub type F1aiW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFO 1 get index RXF1S\\[F1GI\\] to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\\[F1FL\\]."]
    #[inline(always)]
    pub fn f1ai(&self) -> F1aiR {
        F1aiR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXF1A")
            .field("f1ai", &self.f1ai())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFO 1 get index RXF1S\\[F1GI\\] to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\\[F1FL\\]."]
    #[inline(always)]
    pub fn f1ai(&mut self) -> F1aiW<FdcanRxf1aSpec> {
        F1aiW::new(self, 0)
    }
}
#[doc = "FDCAN Rx FIFO 1 acknowledge register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rxf1a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf1a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanRxf1aSpec;
impl crate::RegisterSpec for FdcanRxf1aSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rxf1a::R`](R) reader structure"]
impl crate::Readable for FdcanRxf1aSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_rxf1a::W`](W) writer structure"]
impl crate::Writable for FdcanRxf1aSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_RXF1A to value 0"]
impl crate::Resettable for FdcanRxf1aSpec {}

#[doc = "Register `FDCAN_RXF0A` reader"]
pub type R = crate::R<FdcanRxf0aSpec>;
#[doc = "Register `FDCAN_RXF0A` writer"]
pub type W = crate::W<FdcanRxf0aSpec>;
#[doc = "Field `F0AI` reader - Rx FIFO 0 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This sets the Rx FIFO 0 get index RXF0S\\[F0GI\\]
to F0AI + 1 and update the FIFO 0 fill level RXF0S\\[F0FL\\]."]
pub type F0aiR = crate::FieldReader;
#[doc = "Field `F0AI` writer - Rx FIFO 0 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This sets the Rx FIFO 0 get index RXF0S\\[F0GI\\]
to F0AI + 1 and update the FIFO 0 fill level RXF0S\\[F0FL\\]."]
pub type F0aiW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Rx FIFO 0 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This sets the Rx FIFO 0 get index RXF0S\\[F0GI\\]
to F0AI + 1 and update the FIFO 0 fill level RXF0S\\[F0FL\\]."]
    #[inline(always)]
    pub fn f0ai(&self) -> F0aiR {
        F0aiR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXF0A")
            .field("f0ai", &self.f0ai())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Rx FIFO 0 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This sets the Rx FIFO 0 get index RXF0S\\[F0GI\\]
to F0AI + 1 and update the FIFO 0 fill level RXF0S\\[F0FL\\]."]
    #[inline(always)]
    pub fn f0ai(&mut self) -> F0aiW<FdcanRxf0aSpec> {
        F0aiW::new(self, 0)
    }
}
#[doc = "CAN Rx FIFO 0 acknowledge register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rxf0a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf0a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanRxf0aSpec;
impl crate::RegisterSpec for FdcanRxf0aSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rxf0a::R`](R) reader structure"]
impl crate::Readable for FdcanRxf0aSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_rxf0a::W`](W) writer structure"]
impl crate::Writable for FdcanRxf0aSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_RXF0A to value 0"]
impl crate::Resettable for FdcanRxf0aSpec {
    const RESET_VALUE: u32 = 0;
}

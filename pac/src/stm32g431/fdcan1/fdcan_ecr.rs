#[doc = "Register `FDCAN_ECR` reader"]
pub type R = crate::R<FdcanEcrSpec>;
#[doc = "Register `FDCAN_ECR` writer"]
pub type W = crate::W<FdcanEcrSpec>;
#[doc = "Field `TEC` reader - Transmit error counter Actual state of the transmit error counter, values between 0 and 255. When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented."]
pub type TecR = crate::FieldReader;
#[doc = "Field `REC` reader - Receive error counter Actual state of the receive error counter, values between 0 and 127."]
pub type RecR = crate::FieldReader;
#[doc = "Receive error passive\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rp {
    #[doc = "0: The receive error counter is below the error passive level of 128."]
    B0x0 = 0,
    #[doc = "1: The receive error counter has reached the error passive level of 128."]
    B0x1 = 1,
}
impl From<Rp> for bool {
    #[inline(always)]
    fn from(variant: Rp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RP` reader - Receive error passive"]
pub type RpR = crate::BitReader<Rp>;
impl RpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rp {
        match self.bits {
            false => Rp::B0x0,
            true => Rp::B0x1,
        }
    }
    #[doc = "The receive error counter is below the error passive level of 128."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rp::B0x0
    }
    #[doc = "The receive error counter has reached the error passive level of 128."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rp::B0x1
    }
}
#[doc = "Field `CEL` reader - CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\\[ELO\\]. Access type is RX: reset on read."]
pub type CelR = crate::FieldReader;
#[doc = "Field `CEL` writer - CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\\[ELO\\]. Access type is RX: reset on read."]
pub type CelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit error counter Actual state of the transmit error counter, values between 0 and 255. When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented."]
    #[inline(always)]
    pub fn tec(&self) -> TecR {
        TecR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive error counter Actual state of the receive error counter, values between 0 and 127."]
    #[inline(always)]
    pub fn rec(&self) -> RecR {
        RecR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Receive error passive"]
    #[inline(always)]
    pub fn rp(&self) -> RpR {
        RpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\\[ELO\\]. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn cel(&self) -> CelR {
        CelR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_ECR")
            .field("tec", &self.tec())
            .field("rec", &self.rec())
            .field("rp", &self.rp())
            .field("cel", &self.cel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:23 - CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\\[ELO\\]. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn cel(&mut self) -> CelW<FdcanEcrSpec> {
        CelW::new(self, 16)
    }
}
#[doc = "FDCAN error counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanEcrSpec;
impl crate::RegisterSpec for FdcanEcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ecr::R`](R) reader structure"]
impl crate::Readable for FdcanEcrSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ecr::W`](W) writer structure"]
impl crate::Writable for FdcanEcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_ECR to value 0"]
impl crate::Resettable for FdcanEcrSpec {}

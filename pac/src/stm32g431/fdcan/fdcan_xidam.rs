#[doc = "Register `FDCAN_XIDAM` reader"]
pub type R = crate::R<FdcanXidamSpec>;
#[doc = "Register `FDCAN_XIDAM` writer"]
pub type W = crate::W<FdcanXidamSpec>;
#[doc = "Field `EIDM` reader - Extended ID mask For acceptance filtering of extended frames the Extended ID AND Mask is AND-ed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to 1 the mask is not active. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type EidmR = crate::FieldReader<u32>;
#[doc = "Field `EIDM` writer - Extended ID mask For acceptance filtering of extended frames the Extended ID AND Mask is AND-ed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to 1 the mask is not active. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type EidmW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - Extended ID mask For acceptance filtering of extended frames the Extended ID AND Mask is AND-ed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to 1 the mask is not active. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn eidm(&self) -> EidmR {
        EidmR::new(self.bits & 0x1fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_XIDAM")
            .field("eidm", &self.eidm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:28 - Extended ID mask For acceptance filtering of extended frames the Extended ID AND Mask is AND-ed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to 1 the mask is not active. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn eidm(&mut self) -> EidmW<FdcanXidamSpec> {
        EidmW::new(self, 0)
    }
}
#[doc = "FDCAN extended ID and mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_xidam::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_xidam::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanXidamSpec;
impl crate::RegisterSpec for FdcanXidamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_xidam::R`](R) reader structure"]
impl crate::Readable for FdcanXidamSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_xidam::W`](W) writer structure"]
impl crate::Writable for FdcanXidamSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_XIDAM to value 0x1fff_ffff"]
impl crate::Resettable for FdcanXidamSpec {
    const RESET_VALUE: u32 = 0x1fff_ffff;
}

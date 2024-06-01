#[doc = "Register `FDCAN_ENDN` reader"]
pub type R = crate::R<FdcanEndnSpec>;
#[doc = "Field `ETV` reader - Endianness test value The endianness test value is 0x8765 4321."]
pub type EtvR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Endianness test value The endianness test value is 0x8765 4321."]
    #[inline(always)]
    pub fn etv(&self) -> EtvR {
        EtvR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_ENDN")
            .field("etv", &self.etv())
            .finish()
    }
}
#[doc = "FDCAN endian register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_endn::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanEndnSpec;
impl crate::RegisterSpec for FdcanEndnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_endn::R`](R) reader structure"]
impl crate::Readable for FdcanEndnSpec {}
#[doc = "`reset()` method sets FDCAN_ENDN to value 0x8765_4321"]
impl crate::Resettable for FdcanEndnSpec {
    const RESET_VALUE: u32 = 0x8765_4321;
}

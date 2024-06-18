#[doc = "Register `INIT` reader"]
pub type R = crate::R<InitSpec>;
#[doc = "Register `INIT` writer"]
pub type W = crate::W<InitSpec>;
#[doc = "Field `CRC_INIT` reader - Programmable initial CRC value"]
pub type CrcInitR = crate::FieldReader<u32>;
#[doc = "Field `CRC_INIT` writer - Programmable initial CRC value"]
pub type CrcInitW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Programmable initial CRC value"]
    #[inline(always)]
    pub fn crc_init(&self) -> CrcInitR {
        CrcInitR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INIT")
            .field("crc_init", &self.crc_init())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Programmable initial CRC value"]
    #[inline(always)]
    #[must_use]
    pub fn crc_init(&mut self) -> CrcInitW<InitSpec> {
        CrcInitW::new(self, 0)
    }
}
#[doc = "Initial CRC value\n\nYou can [`read`](crate::Reg::read) this register and get [`init::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InitSpec;
impl crate::RegisterSpec for InitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`init::R`](R) reader structure"]
impl crate::Readable for InitSpec {}
#[doc = "`write(|w| ..)` method takes [`init::W`](W) writer structure"]
impl crate::Writable for InitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INIT to value 0xffff_ffff"]
impl crate::Resettable for InitSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

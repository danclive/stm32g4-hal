#[doc = "Register `RCC_ICSCR` reader"]
pub type R = crate::R<RccIcscrSpec>;
#[doc = "Register `RCC_ICSCR` writer"]
pub type W = crate::W<RccIcscrSpec>;
#[doc = "Field `HSICAL` reader - HSI16 clock calibration These bits are initialized at startup with the factory-programmed HSI16 calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value."]
pub type HsicalR = crate::FieldReader;
#[doc = "Field `HSITRIM` reader - HSI16 clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\\[7:0\\]
bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI16. The default value is 16, which, when added to the HSICAL value, should trim the HSI16 to 16 MHz 1 %."]
pub type HsitrimR = crate::FieldReader;
#[doc = "Field `HSITRIM` writer - HSI16 clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\\[7:0\\]
bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI16. The default value is 16, which, when added to the HSICAL value, should trim the HSI16 to 16 MHz 1 %."]
pub type HsitrimW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 16:23 - HSI16 clock calibration These bits are initialized at startup with the factory-programmed HSI16 calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value."]
    #[inline(always)]
    pub fn hsical(&self) -> HsicalR {
        HsicalR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - HSI16 clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\\[7:0\\]
bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI16. The default value is 16, which, when added to the HSICAL value, should trim the HSI16 to 16 MHz 1 %."]
    #[inline(always)]
    pub fn hsitrim(&self) -> HsitrimR {
        HsitrimR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_ICSCR")
            .field("hsical", &self.hsical())
            .field("hsitrim", &self.hsitrim())
            .finish()
    }
}
impl W {
    #[doc = "Bits 24:30 - HSI16 clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\\[7:0\\]
bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI16. The default value is 16, which, when added to the HSICAL value, should trim the HSI16 to 16 MHz 1 %."]
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HsitrimW<RccIcscrSpec> {
        HsitrimW::new(self, 24)
    }
}
#[doc = "Internal clock sources calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_icscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_icscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccIcscrSpec;
impl crate::RegisterSpec for RccIcscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_icscr::R`](R) reader structure"]
impl crate::Readable for RccIcscrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_icscr::W`](W) writer structure"]
impl crate::Writable for RccIcscrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_ICSCR to value 0x4000_0000"]
impl crate::Resettable for RccIcscrSpec {
    const RESET_VALUE: u32 = 0x4000_0000;
}

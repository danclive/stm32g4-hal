#[doc = "Register `VREFBUF_CSR` reader"]
pub type R = crate::R<VrefbufCsrSpec>;
#[doc = "Register `VREFBUF_CSR` writer"]
pub type W = crate::W<VrefbufCsrSpec>;
#[doc = "Field `ENVR` reader - Enable Voltage Reference"]
pub type EnvrR = crate::BitReader;
#[doc = "Field `ENVR` writer - Enable Voltage Reference"]
pub type EnvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIZ` reader - High impedence mode for the VREF_BUF"]
pub type HizR = crate::BitReader;
#[doc = "Field `HIZ` writer - High impedence mode for the VREF_BUF"]
pub type HizW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VRR` reader - Voltage reference buffer ready"]
pub type VrrR = crate::BitReader;
#[doc = "Field `VRS` reader - Voltage reference scale"]
pub type VrsR = crate::FieldReader;
#[doc = "Field `VRS` writer - Voltage reference scale"]
pub type VrsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Enable Voltage Reference"]
    #[inline(always)]
    pub fn envr(&self) -> EnvrR {
        EnvrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High impedence mode for the VREF_BUF"]
    #[inline(always)]
    pub fn hiz(&self) -> HizR {
        HizR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage reference buffer ready"]
    #[inline(always)]
    pub fn vrr(&self) -> VrrR {
        VrrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Voltage reference scale"]
    #[inline(always)]
    pub fn vrs(&self) -> VrsR {
        VrsR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VREFBUF_CSR")
            .field("envr", &self.envr())
            .field("hiz", &self.hiz())
            .field("vrr", &self.vrr())
            .field("vrs", &self.vrs())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable Voltage Reference"]
    #[inline(always)]
    pub fn envr(&mut self) -> EnvrW<'_, VrefbufCsrSpec> {
        EnvrW::new(self, 0)
    }
    #[doc = "Bit 1 - High impedence mode for the VREF_BUF"]
    #[inline(always)]
    pub fn hiz(&mut self) -> HizW<'_, VrefbufCsrSpec> {
        HizW::new(self, 1)
    }
    #[doc = "Bits 4:5 - Voltage reference scale"]
    #[inline(always)]
    pub fn vrs(&mut self) -> VrsW<'_, VrefbufCsrSpec> {
        VrsW::new(self, 4)
    }
}
#[doc = "VREF_BUF Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vrefbuf_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrefbuf_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrefbufCsrSpec;
impl crate::RegisterSpec for VrefbufCsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vrefbuf_csr::R`](R) reader structure"]
impl crate::Readable for VrefbufCsrSpec {}
#[doc = "`write(|w| ..)` method takes [`vrefbuf_csr::W`](W) writer structure"]
impl crate::Writable for VrefbufCsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VREFBUF_CSR to value 0x02"]
impl crate::Resettable for VrefbufCsrSpec {
    const RESET_VALUE: u32 = 0x02;
}

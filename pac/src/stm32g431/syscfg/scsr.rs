#[doc = "Register `SCSR` reader"]
pub type R = crate::R<ScsrSpec>;
#[doc = "Register `SCSR` writer"]
pub type W = crate::W<ScsrSpec>;
#[doc = "Field `CCMER` reader - CCM SRAM Erase"]
pub type CcmerR = crate::BitReader;
#[doc = "Field `CCMER` writer - CCM SRAM Erase"]
pub type CcmerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCMBSY` reader - CCM SRAM busy by erase operation"]
pub type CcmbsyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CCM SRAM Erase"]
    #[inline(always)]
    pub fn ccmer(&self) -> CcmerR {
        CcmerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CCM SRAM busy by erase operation"]
    #[inline(always)]
    pub fn ccmbsy(&self) -> CcmbsyR {
        CcmbsyR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCSR")
            .field("ccmer", &self.ccmer())
            .field("ccmbsy", &self.ccmbsy())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CCM SRAM Erase"]
    #[inline(always)]
    pub fn ccmer(&mut self) -> CcmerW<ScsrSpec> {
        CcmerW::new(self, 0)
    }
}
#[doc = "CCM SRAM control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`scsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScsrSpec;
impl crate::RegisterSpec for ScsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scsr::R`](R) reader structure"]
impl crate::Readable for ScsrSpec {}
#[doc = "`write(|w| ..)` method takes [`scsr::W`](W) writer structure"]
impl crate::Writable for ScsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCSR to value 0"]
impl crate::Resettable for ScsrSpec {}

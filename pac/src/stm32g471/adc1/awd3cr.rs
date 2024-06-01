#[doc = "Register `AWD3CR` reader"]
pub type R = crate::R<Awd3crSpec>;
#[doc = "Register `AWD3CR` writer"]
pub type W = crate::W<Awd3crSpec>;
#[doc = "Field `AWD3CH` reader - AWD3CH"]
pub type Awd3chR = crate::FieldReader<u32>;
#[doc = "Field `AWD3CH` writer - AWD3CH"]
pub type Awd3chW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:18 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch(&self) -> Awd3chR {
        Awd3chR::new(self.bits & 0x0007_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWD3CR")
            .field("awd3ch", &self.awd3ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:18 - AWD3CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch(&mut self) -> Awd3chW<Awd3crSpec> {
        Awd3chW::new(self, 0)
    }
}
#[doc = "Analog Watchdog 3 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awd3cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awd3cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Awd3crSpec;
impl crate::RegisterSpec for Awd3crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awd3cr::R`](R) reader structure"]
impl crate::Readable for Awd3crSpec {}
#[doc = "`write(|w| ..)` method takes [`awd3cr::W`](W) writer structure"]
impl crate::Writable for Awd3crSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWD3CR to value 0"]
impl crate::Resettable for Awd3crSpec {
    const RESET_VALUE: u32 = 0;
}

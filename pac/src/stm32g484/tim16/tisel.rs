#[doc = "Register `TISEL` reader"]
pub type R = crate::R<TiselSpec>;
#[doc = "Register `TISEL` writer"]
pub type W = crate::W<TiselSpec>;
#[doc = "Field `TI1SEL` reader - TI1\\[0\\]
to TI1\\[15\\]
input selection"]
pub type Ti1selR = crate::FieldReader;
#[doc = "Field `TI1SEL` writer - TI1\\[0\\]
to TI1\\[15\\]
input selection"]
pub type Ti1selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TI1\\[0\\]
to TI1\\[15\\]
input selection"]
    #[inline(always)]
    pub fn ti1sel(&self) -> Ti1selR {
        Ti1selR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TISEL")
            .field("ti1sel", &self.ti1sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - TI1\\[0\\]
to TI1\\[15\\]
input selection"]
    #[inline(always)]
    #[must_use]
    pub fn ti1sel(&mut self) -> Ti1selW<TiselSpec> {
        Ti1selW::new(self, 0)
    }
}
#[doc = "TIM timer input selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tisel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tisel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TiselSpec;
impl crate::RegisterSpec for TiselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tisel::R`](R) reader structure"]
impl crate::Readable for TiselSpec {}
#[doc = "`write(|w| ..)` method takes [`tisel::W`](W) writer structure"]
impl crate::Writable for TiselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TISEL to value 0"]
impl crate::Resettable for TiselSpec {
    const RESET_VALUE: u32 = 0;
}

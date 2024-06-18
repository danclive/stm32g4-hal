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
#[doc = "Field `TI2SEL` reader - TI2\\[0\\]
to TI2\\[15\\]
input selection"]
pub type Ti2selR = crate::FieldReader;
#[doc = "Field `TI2SEL` writer - TI2\\[0\\]
to TI2\\[15\\]
input selection"]
pub type Ti2selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TI3SEL` reader - TI3\\[0\\]
to TI3\\[15\\]
input selection"]
pub type Ti3selR = crate::FieldReader;
#[doc = "Field `TI3SEL` writer - TI3\\[0\\]
to TI3\\[15\\]
input selection"]
pub type Ti3selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TI4SEL` reader - TI4\\[0\\]
to TI4\\[15\\]
input selection"]
pub type Ti4selR = crate::FieldReader;
#[doc = "Field `TI4SEL` writer - TI4\\[0\\]
to TI4\\[15\\]
input selection"]
pub type Ti4selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TI1\\[0\\]
to TI1\\[15\\]
input selection"]
    #[inline(always)]
    pub fn ti1sel(&self) -> Ti1selR {
        Ti1selR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - TI2\\[0\\]
to TI2\\[15\\]
input selection"]
    #[inline(always)]
    pub fn ti2sel(&self) -> Ti2selR {
        Ti2selR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - TI3\\[0\\]
to TI3\\[15\\]
input selection"]
    #[inline(always)]
    pub fn ti3sel(&self) -> Ti3selR {
        Ti3selR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - TI4\\[0\\]
to TI4\\[15\\]
input selection"]
    #[inline(always)]
    pub fn ti4sel(&self) -> Ti4selR {
        Ti4selR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TISEL")
            .field("ti1sel", &self.ti1sel())
            .field("ti2sel", &self.ti2sel())
            .field("ti3sel", &self.ti3sel())
            .field("ti4sel", &self.ti4sel())
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
    #[doc = "Bits 8:11 - TI2\\[0\\]
to TI2\\[15\\]
input selection"]
    #[inline(always)]
    #[must_use]
    pub fn ti2sel(&mut self) -> Ti2selW<TiselSpec> {
        Ti2selW::new(self, 8)
    }
    #[doc = "Bits 16:19 - TI3\\[0\\]
to TI3\\[15\\]
input selection"]
    #[inline(always)]
    #[must_use]
    pub fn ti3sel(&mut self) -> Ti3selW<TiselSpec> {
        Ti3selW::new(self, 16)
    }
    #[doc = "Bits 24:27 - TI4\\[0\\]
to TI4\\[15\\]
input selection"]
    #[inline(always)]
    #[must_use]
    pub fn ti4sel(&mut self) -> Ti4selW<TiselSpec> {
        Ti4selW::new(self, 24)
    }
}
#[doc = "TIM timer input selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`tisel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

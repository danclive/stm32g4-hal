#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `T` reader - 7-bit counter (MSB to LSB)"]
pub type TR = crate::FieldReader;
#[doc = "Field `T` writer - 7-bit counter (MSB to LSB)"]
pub type TW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WDGA` reader - Activation bit"]
pub type WdgaR = crate::BitReader;
#[doc = "Field `WDGA` writer - Activation bit"]
pub type WdgaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 7-bit counter (MSB to LSB)"]
    #[inline(always)]
    pub fn t(&self) -> TR {
        TR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Activation bit"]
    #[inline(always)]
    pub fn wdga(&self) -> WdgaR {
        WdgaR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("wdga", &self.wdga())
            .field("t", &self.t())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit counter (MSB to LSB)"]
    #[inline(always)]
    pub fn t(&mut self) -> TW<'_, CrSpec> {
        TW::new(self, 0)
    }
    #[doc = "Bit 7 - Activation bit"]
    #[inline(always)]
    pub fn wdga(&mut self) -> WdgaW<'_, CrSpec> {
        WdgaW::new(self, 7)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0x7f"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x7f;
}

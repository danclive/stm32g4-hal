#[doc = "Register `DEEFR3` reader"]
pub type R = crate::R<Deefr3Spec>;
#[doc = "Register `DEEFR3` writer"]
pub type W = crate::W<Deefr3Spec>;
#[doc = "Field `EEVACE` reader - External Event A Counter Enable"]
pub type EevaceR = crate::BitReader;
#[doc = "Field `EEVACE` writer - External Event A Counter Enable"]
pub type EevaceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEVACRES` reader - External Event A Counter Reset"]
pub type EevacresR = crate::BitReader;
#[doc = "Field `EEVACRES` writer - External Event A Counter Reset"]
pub type EevacresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEVARSTM` reader - External Event A Reset Mode"]
pub type EevarstmR = crate::BitReader;
#[doc = "Field `EEVARSTM` writer - External Event A Reset Mode"]
pub type EevarstmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEVASEL` reader - External Event A Selection"]
pub type EevaselR = crate::FieldReader;
#[doc = "Field `EEVASEL` writer - External Event A Selection"]
pub type EevaselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EEVACNT` reader - External Event A counter"]
pub type EevacntR = crate::FieldReader;
#[doc = "Field `EEVACNT` writer - External Event A counter"]
pub type EevacntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - External Event A Counter Enable"]
    #[inline(always)]
    pub fn eevace(&self) -> EevaceR {
        EevaceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Event A Counter Reset"]
    #[inline(always)]
    pub fn eevacres(&self) -> EevacresR {
        EevacresR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Event A Reset Mode"]
    #[inline(always)]
    pub fn eevarstm(&self) -> EevarstmR {
        EevarstmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - External Event A Selection"]
    #[inline(always)]
    pub fn eevasel(&self) -> EevaselR {
        EevaselR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - External Event A counter"]
    #[inline(always)]
    pub fn eevacnt(&self) -> EevacntR {
        EevacntR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEEFR3")
            .field("eevacnt", &self.eevacnt())
            .field("eevasel", &self.eevasel())
            .field("eevarstm", &self.eevarstm())
            .field("eevacres", &self.eevacres())
            .field("eevace", &self.eevace())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - External Event A Counter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eevace(&mut self) -> EevaceW<Deefr3Spec> {
        EevaceW::new(self, 0)
    }
    #[doc = "Bit 1 - External Event A Counter Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eevacres(&mut self) -> EevacresW<Deefr3Spec> {
        EevacresW::new(self, 1)
    }
    #[doc = "Bit 2 - External Event A Reset Mode"]
    #[inline(always)]
    #[must_use]
    pub fn eevarstm(&mut self) -> EevarstmW<Deefr3Spec> {
        EevarstmW::new(self, 2)
    }
    #[doc = "Bits 4:7 - External Event A Selection"]
    #[inline(always)]
    #[must_use]
    pub fn eevasel(&mut self) -> EevaselW<Deefr3Spec> {
        EevaselW::new(self, 4)
    }
    #[doc = "Bits 8:13 - External Event A counter"]
    #[inline(always)]
    #[must_use]
    pub fn eevacnt(&mut self) -> EevacntW<Deefr3Spec> {
        EevacntW::new(self, 8)
    }
}
#[doc = "HRTIM Timerx External Event Filtering Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deefr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deefr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Deefr3Spec;
impl crate::RegisterSpec for Deefr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deefr3::R`](R) reader structure"]
impl crate::Readable for Deefr3Spec {}
#[doc = "`write(|w| ..)` method takes [`deefr3::W`](W) writer structure"]
impl crate::Writable for Deefr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEEFR3 to value 0"]
impl crate::Resettable for Deefr3Spec {
    const RESET_VALUE: u32 = 0;
}

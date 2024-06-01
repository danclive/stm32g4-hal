#[doc = "Register `TOCC` reader"]
pub type R = crate::R<ToccSpec>;
#[doc = "Register `TOCC` writer"]
pub type W = crate::W<ToccSpec>;
#[doc = "Field `ETOC` reader - ETOC"]
pub type EtocR = crate::BitReader;
#[doc = "Field `ETOC` writer - ETOC"]
pub type EtocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS` reader - TOS"]
pub type TosR = crate::FieldReader;
#[doc = "Field `TOS` writer - TOS"]
pub type TosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOP` reader - TOP"]
pub type TopR = crate::FieldReader<u16>;
#[doc = "Field `TOP` writer - TOP"]
pub type TopW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - ETOC"]
    #[inline(always)]
    pub fn etoc(&self) -> EtocR {
        EtocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - TOS"]
    #[inline(always)]
    pub fn tos(&self) -> TosR {
        TosR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 16:31 - TOP"]
    #[inline(always)]
    pub fn top(&self) -> TopR {
        TopR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOCC")
            .field("etoc", &self.etoc())
            .field("tos", &self.tos())
            .field("top", &self.top())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ETOC"]
    #[inline(always)]
    #[must_use]
    pub fn etoc(&mut self) -> EtocW<ToccSpec> {
        EtocW::new(self, 0)
    }
    #[doc = "Bits 1:2 - TOS"]
    #[inline(always)]
    #[must_use]
    pub fn tos(&mut self) -> TosW<ToccSpec> {
        TosW::new(self, 1)
    }
    #[doc = "Bits 16:31 - TOP"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TopW<ToccSpec> {
        TopW::new(self, 16)
    }
}
#[doc = "FDCAN Timeout Counter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ToccSpec;
impl crate::RegisterSpec for ToccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tocc::R`](R) reader structure"]
impl crate::Readable for ToccSpec {}
#[doc = "`write(|w| ..)` method takes [`tocc::W`](W) writer structure"]
impl crate::Writable for ToccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOCC to value 0xffff_0000"]
impl crate::Resettable for ToccSpec {
    const RESET_VALUE: u32 = 0xffff_0000;
}

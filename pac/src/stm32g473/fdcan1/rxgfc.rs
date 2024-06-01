#[doc = "Register `RXGFC` reader"]
pub type R = crate::R<RxgfcSpec>;
#[doc = "Register `RXGFC` writer"]
pub type W = crate::W<RxgfcSpec>;
#[doc = "Field `RRFE` reader - RRFE"]
pub type RrfeR = crate::BitReader;
#[doc = "Field `RRFE` writer - RRFE"]
pub type RrfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRFS` reader - RRFS"]
pub type RrfsR = crate::BitReader;
#[doc = "Field `RRFS` writer - RRFS"]
pub type RrfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANFE` reader - ANFE"]
pub type AnfeR = crate::FieldReader;
#[doc = "Field `ANFE` writer - ANFE"]
pub type AnfeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ANFS` reader - ANFS"]
pub type AnfsR = crate::FieldReader;
#[doc = "Field `ANFS` writer - ANFS"]
pub type AnfsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `F1OM` reader - F1OM"]
pub type F1omR = crate::BitReader;
#[doc = "Field `F1OM` writer - F1OM"]
pub type F1omW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F0OM` reader - F0OM"]
pub type F0omR = crate::BitReader;
#[doc = "Field `F0OM` writer - F0OM"]
pub type F0omW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSS` reader - LSS"]
pub type LssR = crate::FieldReader;
#[doc = "Field `LSS` writer - LSS"]
pub type LssW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LSE` reader - LSE"]
pub type LseR = crate::FieldReader;
#[doc = "Field `LSE` writer - LSE"]
pub type LseW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - RRFE"]
    #[inline(always)]
    pub fn rrfe(&self) -> RrfeR {
        RrfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RRFS"]
    #[inline(always)]
    pub fn rrfs(&self) -> RrfsR {
        RrfsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - ANFE"]
    #[inline(always)]
    pub fn anfe(&self) -> AnfeR {
        AnfeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - ANFS"]
    #[inline(always)]
    pub fn anfs(&self) -> AnfsR {
        AnfsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - F1OM"]
    #[inline(always)]
    pub fn f1om(&self) -> F1omR {
        F1omR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - F0OM"]
    #[inline(always)]
    pub fn f0om(&self) -> F0omR {
        F0omR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:20 - LSS"]
    #[inline(always)]
    pub fn lss(&self) -> LssR {
        LssR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - LSE"]
    #[inline(always)]
    pub fn lse(&self) -> LseR {
        LseR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXGFC")
            .field("rrfe", &self.rrfe())
            .field("rrfs", &self.rrfs())
            .field("anfe", &self.anfe())
            .field("anfs", &self.anfs())
            .field("f1om", &self.f1om())
            .field("f0om", &self.f0om())
            .field("lss", &self.lss())
            .field("lse", &self.lse())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RRFE"]
    #[inline(always)]
    #[must_use]
    pub fn rrfe(&mut self) -> RrfeW<RxgfcSpec> {
        RrfeW::new(self, 0)
    }
    #[doc = "Bit 1 - RRFS"]
    #[inline(always)]
    #[must_use]
    pub fn rrfs(&mut self) -> RrfsW<RxgfcSpec> {
        RrfsW::new(self, 1)
    }
    #[doc = "Bits 2:3 - ANFE"]
    #[inline(always)]
    #[must_use]
    pub fn anfe(&mut self) -> AnfeW<RxgfcSpec> {
        AnfeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - ANFS"]
    #[inline(always)]
    #[must_use]
    pub fn anfs(&mut self) -> AnfsW<RxgfcSpec> {
        AnfsW::new(self, 4)
    }
    #[doc = "Bit 8 - F1OM"]
    #[inline(always)]
    #[must_use]
    pub fn f1om(&mut self) -> F1omW<RxgfcSpec> {
        F1omW::new(self, 8)
    }
    #[doc = "Bit 9 - F0OM"]
    #[inline(always)]
    #[must_use]
    pub fn f0om(&mut self) -> F0omW<RxgfcSpec> {
        F0omW::new(self, 9)
    }
    #[doc = "Bits 16:20 - LSS"]
    #[inline(always)]
    #[must_use]
    pub fn lss(&mut self) -> LssW<RxgfcSpec> {
        LssW::new(self, 16)
    }
    #[doc = "Bits 24:27 - LSE"]
    #[inline(always)]
    #[must_use]
    pub fn lse(&mut self) -> LseW<RxgfcSpec> {
        LseW::new(self, 24)
    }
}
#[doc = "Global settings for Message ID filtering. The Global Filter Configuration controls the filter path for standard and extended messages as described in Figure706: Standard Message ID filter path and Figure707: Extended Message ID filter path.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxgfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxgfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxgfcSpec;
impl crate::RegisterSpec for RxgfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxgfc::R`](R) reader structure"]
impl crate::Readable for RxgfcSpec {}
#[doc = "`write(|w| ..)` method takes [`rxgfc::W`](W) writer structure"]
impl crate::Writable for RxgfcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXGFC to value 0"]
impl crate::Resettable for RxgfcSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `PARAM` reader"]
pub type R = crate::R<ParamSpec>;
#[doc = "Register `PARAM` writer"]
pub type W = crate::W<ParamSpec>;
#[doc = "Field `P` reader - P"]
pub type PR = crate::FieldReader;
#[doc = "Field `P` writer - P"]
pub type PW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Q` reader - Q"]
pub type QR = crate::FieldReader;
#[doc = "Field `Q` writer - Q"]
pub type QW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R` reader - R"]
pub type RR = crate::FieldReader;
#[doc = "Field `R` writer - R"]
pub type RW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FUNC` reader - FUNC"]
pub type FuncR = crate::FieldReader;
#[doc = "Field `FUNC` writer - FUNC"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `START` reader - START"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - START"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - P"]
    #[inline(always)]
    pub fn p(&self) -> PR {
        PR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Q"]
    #[inline(always)]
    pub fn q(&self) -> QR {
        QR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - R"]
    #[inline(always)]
    pub fn r(&self) -> RR {
        RR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - FUNC"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - START"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PARAM")
            .field("start", &self.start())
            .field("func", &self.func())
            .field("r", &self.r())
            .field("q", &self.q())
            .field("p", &self.p())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - P"]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> PW<ParamSpec> {
        PW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Q"]
    #[inline(always)]
    #[must_use]
    pub fn q(&mut self) -> QW<ParamSpec> {
        QW::new(self, 8)
    }
    #[doc = "Bits 16:23 - R"]
    #[inline(always)]
    #[must_use]
    pub fn r(&mut self) -> RW<ParamSpec> {
        RW::new(self, 16)
    }
    #[doc = "Bits 24:30 - FUNC"]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FuncW<ParamSpec> {
        FuncW::new(self, 24)
    }
    #[doc = "Bit 31 - START"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<ParamSpec> {
        StartW::new(self, 31)
    }
}
#[doc = "FMAC Parameter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`param::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`param::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParamSpec;
impl crate::RegisterSpec for ParamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`param::R`](R) reader structure"]
impl crate::Readable for ParamSpec {}
#[doc = "`write(|w| ..)` method takes [`param::W`](W) writer structure"]
impl crate::Writable for ParamSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PARAM to value 0"]
impl crate::Resettable for ParamSpec {
    const RESET_VALUE: u32 = 0;
}

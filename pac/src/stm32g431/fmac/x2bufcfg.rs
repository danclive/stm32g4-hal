#[doc = "Register `X2BUFCFG` reader"]
pub type R = crate::R<X2bufcfgSpec>;
#[doc = "Register `X2BUFCFG` writer"]
pub type W = crate::W<X2bufcfgSpec>;
#[doc = "Field `X2_BASE` reader - X1_BASE"]
pub type X2BaseR = crate::FieldReader;
#[doc = "Field `X2_BASE` writer - X1_BASE"]
pub type X2BaseW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `X2_BUF_SIZE` reader - X1_BUF_SIZE"]
pub type X2BufSizeR = crate::FieldReader;
#[doc = "Field `X2_BUF_SIZE` writer - X1_BUF_SIZE"]
pub type X2BufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - X1_BASE"]
    #[inline(always)]
    pub fn x2_base(&self) -> X2BaseR {
        X2BaseR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - X1_BUF_SIZE"]
    #[inline(always)]
    pub fn x2_buf_size(&self) -> X2BufSizeR {
        X2BufSizeR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("X2BUFCFG")
            .field("x2_base", &self.x2_base())
            .field("x2_buf_size", &self.x2_buf_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - X1_BASE"]
    #[inline(always)]
    #[must_use]
    pub fn x2_base(&mut self) -> X2BaseW<X2bufcfgSpec> {
        X2BaseW::new(self, 0)
    }
    #[doc = "Bits 8:15 - X1_BUF_SIZE"]
    #[inline(always)]
    #[must_use]
    pub fn x2_buf_size(&mut self) -> X2BufSizeW<X2bufcfgSpec> {
        X2BufSizeW::new(self, 8)
    }
}
#[doc = "FMAC X2 Buffer Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`x2bufcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`x2bufcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct X2bufcfgSpec;
impl crate::RegisterSpec for X2bufcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`x2bufcfg::R`](R) reader structure"]
impl crate::Readable for X2bufcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`x2bufcfg::W`](W) writer structure"]
impl crate::Writable for X2bufcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets X2BUFCFG to value 0"]
impl crate::Resettable for X2bufcfgSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `X1BUFCFG` reader"]
pub type R = crate::R<X1bufcfgSpec>;
#[doc = "Register `X1BUFCFG` writer"]
pub type W = crate::W<X1bufcfgSpec>;
#[doc = "Field `X1_BASE` reader - X1_BASE"]
pub type X1BaseR = crate::FieldReader;
#[doc = "Field `X1_BASE` writer - X1_BASE"]
pub type X1BaseW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `X1_BUF_SIZE` reader - X1_BUF_SIZE"]
pub type X1BufSizeR = crate::FieldReader;
#[doc = "Field `X1_BUF_SIZE` writer - X1_BUF_SIZE"]
pub type X1BufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FULL_WM` reader - FULL_WM"]
pub type FullWmR = crate::FieldReader;
#[doc = "Field `FULL_WM` writer - FULL_WM"]
pub type FullWmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - X1_BASE"]
    #[inline(always)]
    pub fn x1_base(&self) -> X1BaseR {
        X1BaseR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - X1_BUF_SIZE"]
    #[inline(always)]
    pub fn x1_buf_size(&self) -> X1BufSizeR {
        X1BufSizeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - FULL_WM"]
    #[inline(always)]
    pub fn full_wm(&self) -> FullWmR {
        FullWmR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("X1BUFCFG")
            .field("x1_base", &self.x1_base())
            .field("x1_buf_size", &self.x1_buf_size())
            .field("full_wm", &self.full_wm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - X1_BASE"]
    #[inline(always)]
    pub fn x1_base(&mut self) -> X1BaseW<X1bufcfgSpec> {
        X1BaseW::new(self, 0)
    }
    #[doc = "Bits 8:15 - X1_BUF_SIZE"]
    #[inline(always)]
    pub fn x1_buf_size(&mut self) -> X1BufSizeW<X1bufcfgSpec> {
        X1BufSizeW::new(self, 8)
    }
    #[doc = "Bits 24:25 - FULL_WM"]
    #[inline(always)]
    pub fn full_wm(&mut self) -> FullWmW<X1bufcfgSpec> {
        FullWmW::new(self, 24)
    }
}
#[doc = "FMAC X1 Buffer Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`x1bufcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`x1bufcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct X1bufcfgSpec;
impl crate::RegisterSpec for X1bufcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`x1bufcfg::R`](R) reader structure"]
impl crate::Readable for X1bufcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`x1bufcfg::W`](W) writer structure"]
impl crate::Writable for X1bufcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets X1BUFCFG to value 0"]
impl crate::Resettable for X1bufcfgSpec {}

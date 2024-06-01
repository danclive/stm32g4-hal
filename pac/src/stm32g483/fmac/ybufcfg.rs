#[doc = "Register `YBUFCFG` reader"]
pub type R = crate::R<YbufcfgSpec>;
#[doc = "Register `YBUFCFG` writer"]
pub type W = crate::W<YbufcfgSpec>;
#[doc = "Field `Y_BASE` reader - X1_BASE"]
pub type YBaseR = crate::FieldReader;
#[doc = "Field `Y_BASE` writer - X1_BASE"]
pub type YBaseW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Y_BUF_SIZE` reader - X1_BUF_SIZE"]
pub type YBufSizeR = crate::FieldReader;
#[doc = "Field `Y_BUF_SIZE` writer - X1_BUF_SIZE"]
pub type YBufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EMPTY_WM` reader - EMPTY_WM"]
pub type EmptyWmR = crate::FieldReader;
#[doc = "Field `EMPTY_WM` writer - EMPTY_WM"]
pub type EmptyWmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - X1_BASE"]
    #[inline(always)]
    pub fn y_base(&self) -> YBaseR {
        YBaseR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - X1_BUF_SIZE"]
    #[inline(always)]
    pub fn y_buf_size(&self) -> YBufSizeR {
        YBufSizeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - EMPTY_WM"]
    #[inline(always)]
    pub fn empty_wm(&self) -> EmptyWmR {
        EmptyWmR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("YBUFCFG")
            .field("y_base", &self.y_base())
            .field("y_buf_size", &self.y_buf_size())
            .field("empty_wm", &self.empty_wm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - X1_BASE"]
    #[inline(always)]
    #[must_use]
    pub fn y_base(&mut self) -> YBaseW<YbufcfgSpec> {
        YBaseW::new(self, 0)
    }
    #[doc = "Bits 8:15 - X1_BUF_SIZE"]
    #[inline(always)]
    #[must_use]
    pub fn y_buf_size(&mut self) -> YBufSizeW<YbufcfgSpec> {
        YBufSizeW::new(self, 8)
    }
    #[doc = "Bits 24:25 - EMPTY_WM"]
    #[inline(always)]
    #[must_use]
    pub fn empty_wm(&mut self) -> EmptyWmW<YbufcfgSpec> {
        EmptyWmW::new(self, 24)
    }
}
#[doc = "FMAC Y Buffer Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ybufcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ybufcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct YbufcfgSpec;
impl crate::RegisterSpec for YbufcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ybufcfg::R`](R) reader structure"]
impl crate::Readable for YbufcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ybufcfg::W`](W) writer structure"]
impl crate::Writable for YbufcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets YBUFCFG to value 0"]
impl crate::Resettable for YbufcfgSpec {
    const RESET_VALUE: u32 = 0;
}

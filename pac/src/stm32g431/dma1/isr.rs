#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `GIF1` reader - GIF1"]
pub type Gif1R = crate::BitReader;
#[doc = "Field `TCIF1` reader - TCIF1"]
pub type Tcif1R = crate::BitReader;
#[doc = "Field `HTIF1` reader - HTIF1"]
pub type Htif1R = crate::BitReader;
#[doc = "Field `TEIF1` reader - TEIF1"]
pub type Teif1R = crate::BitReader;
#[doc = "Field `GIF2` reader - GIF2"]
pub type Gif2R = crate::BitReader;
#[doc = "Field `TCIF2` reader - TCIF2"]
pub type Tcif2R = crate::BitReader;
#[doc = "Field `HTIF2` reader - HTIF2"]
pub type Htif2R = crate::BitReader;
#[doc = "Field `TEIF2` reader - TEIF2"]
pub type Teif2R = crate::BitReader;
#[doc = "Field `GIF3` reader - GIF3"]
pub type Gif3R = crate::BitReader;
#[doc = "Field `TCIF3` reader - TCIF3"]
pub type Tcif3R = crate::BitReader;
#[doc = "Field `HTIF3` reader - HTIF3"]
pub type Htif3R = crate::BitReader;
#[doc = "Field `TEIF3` reader - TEIF3"]
pub type Teif3R = crate::BitReader;
#[doc = "Field `GIF4` reader - GIF4"]
pub type Gif4R = crate::BitReader;
#[doc = "Field `TCIF4` reader - TCIF4"]
pub type Tcif4R = crate::BitReader;
#[doc = "Field `HTIF4` reader - HTIF4"]
pub type Htif4R = crate::BitReader;
#[doc = "Field `TEIF4` reader - TEIF4"]
pub type Teif4R = crate::BitReader;
#[doc = "Field `GIF5` reader - GIF5"]
pub type Gif5R = crate::BitReader;
#[doc = "Field `TCIF5` reader - TCIF5"]
pub type Tcif5R = crate::BitReader;
#[doc = "Field `HTIF5` reader - HTIF5"]
pub type Htif5R = crate::BitReader;
#[doc = "Field `TEIF5` reader - TEIF5"]
pub type Teif5R = crate::BitReader;
#[doc = "Field `GIF6` reader - GIF6"]
pub type Gif6R = crate::BitReader;
#[doc = "Field `TCIF6` reader - TCIF6"]
pub type Tcif6R = crate::BitReader;
#[doc = "Field `HTIF6` reader - HTIF6"]
pub type Htif6R = crate::BitReader;
#[doc = "Field `TEIF6` reader - TEIF6"]
pub type Teif6R = crate::BitReader;
#[doc = "Field `GIF7` reader - GIF7"]
pub type Gif7R = crate::BitReader;
#[doc = "Field `TCIF7` reader - TCIF7"]
pub type Tcif7R = crate::BitReader;
#[doc = "Field `HTIF7` reader - HTIF7"]
pub type Htif7R = crate::BitReader;
#[doc = "Field `TEIF7` reader - TEIF7"]
pub type Teif7R = crate::BitReader;
#[doc = "Field `GIF8` reader - GIF8"]
pub type Gif8R = crate::BitReader;
#[doc = "Field `TCIF8` reader - TCIF8"]
pub type Tcif8R = crate::BitReader;
#[doc = "Field `HTIF8` reader - HTIF8"]
pub type Htif8R = crate::BitReader;
#[doc = "Field `TEIF8` reader - TEIF8"]
pub type Teif8R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - GIF1"]
    #[inline(always)]
    pub fn gif1(&self) -> Gif1R {
        Gif1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TCIF1"]
    #[inline(always)]
    pub fn tcif1(&self) -> Tcif1R {
        Tcif1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HTIF1"]
    #[inline(always)]
    pub fn htif1(&self) -> Htif1R {
        Htif1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TEIF1"]
    #[inline(always)]
    pub fn teif1(&self) -> Teif1R {
        Teif1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GIF2"]
    #[inline(always)]
    pub fn gif2(&self) -> Gif2R {
        Gif2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TCIF2"]
    #[inline(always)]
    pub fn tcif2(&self) -> Tcif2R {
        Tcif2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HTIF2"]
    #[inline(always)]
    pub fn htif2(&self) -> Htif2R {
        Htif2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TEIF2"]
    #[inline(always)]
    pub fn teif2(&self) -> Teif2R {
        Teif2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GIF3"]
    #[inline(always)]
    pub fn gif3(&self) -> Gif3R {
        Gif3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TCIF3"]
    #[inline(always)]
    pub fn tcif3(&self) -> Tcif3R {
        Tcif3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HTIF3"]
    #[inline(always)]
    pub fn htif3(&self) -> Htif3R {
        Htif3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TEIF3"]
    #[inline(always)]
    pub fn teif3(&self) -> Teif3R {
        Teif3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GIF4"]
    #[inline(always)]
    pub fn gif4(&self) -> Gif4R {
        Gif4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TCIF4"]
    #[inline(always)]
    pub fn tcif4(&self) -> Tcif4R {
        Tcif4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - HTIF4"]
    #[inline(always)]
    pub fn htif4(&self) -> Htif4R {
        Htif4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TEIF4"]
    #[inline(always)]
    pub fn teif4(&self) -> Teif4R {
        Teif4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GIF5"]
    #[inline(always)]
    pub fn gif5(&self) -> Gif5R {
        Gif5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TCIF5"]
    #[inline(always)]
    pub fn tcif5(&self) -> Tcif5R {
        Tcif5R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HTIF5"]
    #[inline(always)]
    pub fn htif5(&self) -> Htif5R {
        Htif5R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TEIF5"]
    #[inline(always)]
    pub fn teif5(&self) -> Teif5R {
        Teif5R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GIF6"]
    #[inline(always)]
    pub fn gif6(&self) -> Gif6R {
        Gif6R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TCIF6"]
    #[inline(always)]
    pub fn tcif6(&self) -> Tcif6R {
        Tcif6R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - HTIF6"]
    #[inline(always)]
    pub fn htif6(&self) -> Htif6R {
        Htif6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TEIF6"]
    #[inline(always)]
    pub fn teif6(&self) -> Teif6R {
        Teif6R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GIF7"]
    #[inline(always)]
    pub fn gif7(&self) -> Gif7R {
        Gif7R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TCIF7"]
    #[inline(always)]
    pub fn tcif7(&self) -> Tcif7R {
        Tcif7R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - HTIF7"]
    #[inline(always)]
    pub fn htif7(&self) -> Htif7R {
        Htif7R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TEIF7"]
    #[inline(always)]
    pub fn teif7(&self) -> Teif7R {
        Teif7R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GIF8"]
    #[inline(always)]
    pub fn gif8(&self) -> Gif8R {
        Gif8R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TCIF8"]
    #[inline(always)]
    pub fn tcif8(&self) -> Tcif8R {
        Tcif8R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - HTIF8"]
    #[inline(always)]
    pub fn htif8(&self) -> Htif8R {
        Htif8R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TEIF8"]
    #[inline(always)]
    pub fn teif8(&self) -> Teif8R {
        Teif8R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("teif8", &self.teif8())
            .field("htif8", &self.htif8())
            .field("tcif8", &self.tcif8())
            .field("gif8", &self.gif8())
            .field("teif7", &self.teif7())
            .field("htif7", &self.htif7())
            .field("tcif7", &self.tcif7())
            .field("gif7", &self.gif7())
            .field("teif6", &self.teif6())
            .field("htif6", &self.htif6())
            .field("tcif6", &self.tcif6())
            .field("gif6", &self.gif6())
            .field("teif5", &self.teif5())
            .field("htif5", &self.htif5())
            .field("tcif5", &self.tcif5())
            .field("gif5", &self.gif5())
            .field("teif4", &self.teif4())
            .field("htif4", &self.htif4())
            .field("tcif4", &self.tcif4())
            .field("gif4", &self.gif4())
            .field("teif3", &self.teif3())
            .field("htif3", &self.htif3())
            .field("tcif3", &self.tcif3())
            .field("gif3", &self.gif3())
            .field("teif2", &self.teif2())
            .field("htif2", &self.htif2())
            .field("tcif2", &self.tcif2())
            .field("gif2", &self.gif2())
            .field("teif1", &self.teif1())
            .field("htif1", &self.htif1())
            .field("tcif1", &self.tcif1())
            .field("gif1", &self.gif1())
            .finish()
    }
}
#[doc = "interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0;
}

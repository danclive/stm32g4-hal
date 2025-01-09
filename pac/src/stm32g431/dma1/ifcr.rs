#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IfcrSpec>;
#[doc = "Field `GIF1` writer - GIF1"]
pub type Gif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIF1` writer - TCIF1"]
pub type Tcif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTIF1` writer - HTIF1"]
pub type Htif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIF1` writer - TEIF1"]
pub type Teif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIF2` writer - GIF2"]
pub type Gif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIF2` writer - TCIF2"]
pub type Tcif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTIF2` writer - HTIF2"]
pub type Htif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIF2` writer - TEIF2"]
pub type Teif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIF3` writer - GIF3"]
pub type Gif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIF3` writer - TCIF3"]
pub type Tcif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTIF3` writer - HTIF3"]
pub type Htif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIF3` writer - TEIF3"]
pub type Teif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIF4` writer - GIF4"]
pub type Gif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIF4` writer - TCIF4"]
pub type Tcif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTIF4` writer - HTIF4"]
pub type Htif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIF4` writer - TEIF4"]
pub type Teif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIF5` writer - GIF5"]
pub type Gif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIF5` writer - TCIF5"]
pub type Tcif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTIF5` writer - HTIF5"]
pub type Htif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIF5` writer - TEIF5"]
pub type Teif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIF6` writer - GIF6"]
pub type Gif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIF6` writer - TCIF6"]
pub type Tcif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTIF6` writer - HTIF6"]
pub type Htif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIF6` writer - TEIF6"]
pub type Teif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIF7` writer - GIF7"]
pub type Gif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIF7` writer - TCIF7"]
pub type Tcif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTIF7` writer - HTIF7"]
pub type Htif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIF7` writer - TEIF7"]
pub type Teif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIF8` writer - GIF8"]
pub type Gif8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIF8` writer - TCIF8"]
pub type Tcif8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTIF8` writer - HTIF8"]
pub type Htif8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIF8` writer - TEIF8"]
pub type Teif8W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IfcrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - GIF1"]
    #[inline(always)]
    pub fn gif1(&mut self) -> Gif1W<IfcrSpec> {
        Gif1W::new(self, 0)
    }
    #[doc = "Bit 1 - TCIF1"]
    #[inline(always)]
    pub fn tcif1(&mut self) -> Tcif1W<IfcrSpec> {
        Tcif1W::new(self, 1)
    }
    #[doc = "Bit 2 - HTIF1"]
    #[inline(always)]
    pub fn htif1(&mut self) -> Htif1W<IfcrSpec> {
        Htif1W::new(self, 2)
    }
    #[doc = "Bit 3 - TEIF1"]
    #[inline(always)]
    pub fn teif1(&mut self) -> Teif1W<IfcrSpec> {
        Teif1W::new(self, 3)
    }
    #[doc = "Bit 4 - GIF2"]
    #[inline(always)]
    pub fn gif2(&mut self) -> Gif2W<IfcrSpec> {
        Gif2W::new(self, 4)
    }
    #[doc = "Bit 5 - TCIF2"]
    #[inline(always)]
    pub fn tcif2(&mut self) -> Tcif2W<IfcrSpec> {
        Tcif2W::new(self, 5)
    }
    #[doc = "Bit 6 - HTIF2"]
    #[inline(always)]
    pub fn htif2(&mut self) -> Htif2W<IfcrSpec> {
        Htif2W::new(self, 6)
    }
    #[doc = "Bit 7 - TEIF2"]
    #[inline(always)]
    pub fn teif2(&mut self) -> Teif2W<IfcrSpec> {
        Teif2W::new(self, 7)
    }
    #[doc = "Bit 8 - GIF3"]
    #[inline(always)]
    pub fn gif3(&mut self) -> Gif3W<IfcrSpec> {
        Gif3W::new(self, 8)
    }
    #[doc = "Bit 9 - TCIF3"]
    #[inline(always)]
    pub fn tcif3(&mut self) -> Tcif3W<IfcrSpec> {
        Tcif3W::new(self, 9)
    }
    #[doc = "Bit 10 - HTIF3"]
    #[inline(always)]
    pub fn htif3(&mut self) -> Htif3W<IfcrSpec> {
        Htif3W::new(self, 10)
    }
    #[doc = "Bit 11 - TEIF3"]
    #[inline(always)]
    pub fn teif3(&mut self) -> Teif3W<IfcrSpec> {
        Teif3W::new(self, 11)
    }
    #[doc = "Bit 12 - GIF4"]
    #[inline(always)]
    pub fn gif4(&mut self) -> Gif4W<IfcrSpec> {
        Gif4W::new(self, 12)
    }
    #[doc = "Bit 13 - TCIF4"]
    #[inline(always)]
    pub fn tcif4(&mut self) -> Tcif4W<IfcrSpec> {
        Tcif4W::new(self, 13)
    }
    #[doc = "Bit 14 - HTIF4"]
    #[inline(always)]
    pub fn htif4(&mut self) -> Htif4W<IfcrSpec> {
        Htif4W::new(self, 14)
    }
    #[doc = "Bit 15 - TEIF4"]
    #[inline(always)]
    pub fn teif4(&mut self) -> Teif4W<IfcrSpec> {
        Teif4W::new(self, 15)
    }
    #[doc = "Bit 16 - GIF5"]
    #[inline(always)]
    pub fn gif5(&mut self) -> Gif5W<IfcrSpec> {
        Gif5W::new(self, 16)
    }
    #[doc = "Bit 17 - TCIF5"]
    #[inline(always)]
    pub fn tcif5(&mut self) -> Tcif5W<IfcrSpec> {
        Tcif5W::new(self, 17)
    }
    #[doc = "Bit 18 - HTIF5"]
    #[inline(always)]
    pub fn htif5(&mut self) -> Htif5W<IfcrSpec> {
        Htif5W::new(self, 18)
    }
    #[doc = "Bit 19 - TEIF5"]
    #[inline(always)]
    pub fn teif5(&mut self) -> Teif5W<IfcrSpec> {
        Teif5W::new(self, 19)
    }
    #[doc = "Bit 20 - GIF6"]
    #[inline(always)]
    pub fn gif6(&mut self) -> Gif6W<IfcrSpec> {
        Gif6W::new(self, 20)
    }
    #[doc = "Bit 21 - TCIF6"]
    #[inline(always)]
    pub fn tcif6(&mut self) -> Tcif6W<IfcrSpec> {
        Tcif6W::new(self, 21)
    }
    #[doc = "Bit 22 - HTIF6"]
    #[inline(always)]
    pub fn htif6(&mut self) -> Htif6W<IfcrSpec> {
        Htif6W::new(self, 22)
    }
    #[doc = "Bit 23 - TEIF6"]
    #[inline(always)]
    pub fn teif6(&mut self) -> Teif6W<IfcrSpec> {
        Teif6W::new(self, 23)
    }
    #[doc = "Bit 24 - GIF7"]
    #[inline(always)]
    pub fn gif7(&mut self) -> Gif7W<IfcrSpec> {
        Gif7W::new(self, 24)
    }
    #[doc = "Bit 25 - TCIF7"]
    #[inline(always)]
    pub fn tcif7(&mut self) -> Tcif7W<IfcrSpec> {
        Tcif7W::new(self, 25)
    }
    #[doc = "Bit 26 - HTIF7"]
    #[inline(always)]
    pub fn htif7(&mut self) -> Htif7W<IfcrSpec> {
        Htif7W::new(self, 26)
    }
    #[doc = "Bit 27 - TEIF7"]
    #[inline(always)]
    pub fn teif7(&mut self) -> Teif7W<IfcrSpec> {
        Teif7W::new(self, 27)
    }
    #[doc = "Bit 28 - GIF8"]
    #[inline(always)]
    pub fn gif8(&mut self) -> Gif8W<IfcrSpec> {
        Gif8W::new(self, 28)
    }
    #[doc = "Bit 29 - TCIF8"]
    #[inline(always)]
    pub fn tcif8(&mut self) -> Tcif8W<IfcrSpec> {
        Tcif8W::new(self, 29)
    }
    #[doc = "Bit 30 - HTIF8"]
    #[inline(always)]
    pub fn htif8(&mut self) -> Htif8W<IfcrSpec> {
        Htif8W::new(self, 30)
    }
    #[doc = "Bit 31 - TEIF8"]
    #[inline(always)]
    pub fn teif8(&mut self) -> Teif8W<IfcrSpec> {
        Teif8W::new(self, 31)
    }
}
#[doc = "DMA interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfcrSpec;
impl crate::RegisterSpec for IfcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifcr::W`](W) writer structure"]
impl crate::Writable for IfcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IfcrSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `BSRR` writer"]
pub type W = crate::W<BsrrSpec>;
#[doc = "Field `BS0` writer - Port x set bit y (y= 0..15)"]
pub type Bs0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS1` writer - Port x set bit y (y= 0..15)"]
pub type Bs1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS2` writer - Port x set bit y (y= 0..15)"]
pub type Bs2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS3` writer - Port x set bit y (y= 0..15)"]
pub type Bs3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS4` writer - Port x set bit y (y= 0..15)"]
pub type Bs4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS5` writer - Port x set bit y (y= 0..15)"]
pub type Bs5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS6` writer - Port x set bit y (y= 0..15)"]
pub type Bs6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS7` writer - Port x set bit y (y= 0..15)"]
pub type Bs7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS8` writer - Port x set bit y (y= 0..15)"]
pub type Bs8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS9` writer - Port x set bit y (y= 0..15)"]
pub type Bs9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS10` writer - Port x set bit y (y= 0..15)"]
pub type Bs10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS11` writer - Port x set bit y (y= 0..15)"]
pub type Bs11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS12` writer - Port x set bit y (y= 0..15)"]
pub type Bs12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS13` writer - Port x set bit y (y= 0..15)"]
pub type Bs13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS14` writer - Port x set bit y (y= 0..15)"]
pub type Bs14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS15` writer - Port x set bit y (y= 0..15)"]
pub type Bs15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR0` writer - Port x set bit y (y= 0..15)"]
pub type Br0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR1` writer - Port x reset bit y (y = 0..15)"]
pub type Br1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR2` writer - Port x reset bit y (y = 0..15)"]
pub type Br2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR3` writer - Port x reset bit y (y = 0..15)"]
pub type Br3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR4` writer - Port x reset bit y (y = 0..15)"]
pub type Br4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR5` writer - Port x reset bit y (y = 0..15)"]
pub type Br5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR6` writer - Port x reset bit y (y = 0..15)"]
pub type Br6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR7` writer - Port x reset bit y (y = 0..15)"]
pub type Br7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR8` writer - Port x reset bit y (y = 0..15)"]
pub type Br8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR9` writer - Port x reset bit y (y = 0..15)"]
pub type Br9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR10` writer - Port x reset bit y (y = 0..15)"]
pub type Br10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR11` writer - Port x reset bit y (y = 0..15)"]
pub type Br11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR12` writer - Port x reset bit y (y = 0..15)"]
pub type Br12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR13` writer - Port x reset bit y (y = 0..15)"]
pub type Br13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR14` writer - Port x reset bit y (y = 0..15)"]
pub type Br14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR15` writer - Port x reset bit y (y = 0..15)"]
pub type Br15W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<BsrrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs0(&mut self) -> Bs0W<BsrrSpec> {
        Bs0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs1(&mut self) -> Bs1W<BsrrSpec> {
        Bs1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs2(&mut self) -> Bs2W<BsrrSpec> {
        Bs2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs3(&mut self) -> Bs3W<BsrrSpec> {
        Bs3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs4(&mut self) -> Bs4W<BsrrSpec> {
        Bs4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs5(&mut self) -> Bs5W<BsrrSpec> {
        Bs5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs6(&mut self) -> Bs6W<BsrrSpec> {
        Bs6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs7(&mut self) -> Bs7W<BsrrSpec> {
        Bs7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs8(&mut self) -> Bs8W<BsrrSpec> {
        Bs8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs9(&mut self) -> Bs9W<BsrrSpec> {
        Bs9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs10(&mut self) -> Bs10W<BsrrSpec> {
        Bs10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs11(&mut self) -> Bs11W<BsrrSpec> {
        Bs11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs12(&mut self) -> Bs12W<BsrrSpec> {
        Bs12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs13(&mut self) -> Bs13W<BsrrSpec> {
        Bs13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs14(&mut self) -> Bs14W<BsrrSpec> {
        Bs14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs15(&mut self) -> Bs15W<BsrrSpec> {
        Bs15W::new(self, 15)
    }
    #[doc = "Bit 16 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn br0(&mut self) -> Br0W<BsrrSpec> {
        Br0W::new(self, 16)
    }
    #[doc = "Bit 17 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br1(&mut self) -> Br1W<BsrrSpec> {
        Br1W::new(self, 17)
    }
    #[doc = "Bit 18 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br2(&mut self) -> Br2W<BsrrSpec> {
        Br2W::new(self, 18)
    }
    #[doc = "Bit 19 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br3(&mut self) -> Br3W<BsrrSpec> {
        Br3W::new(self, 19)
    }
    #[doc = "Bit 20 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br4(&mut self) -> Br4W<BsrrSpec> {
        Br4W::new(self, 20)
    }
    #[doc = "Bit 21 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br5(&mut self) -> Br5W<BsrrSpec> {
        Br5W::new(self, 21)
    }
    #[doc = "Bit 22 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br6(&mut self) -> Br6W<BsrrSpec> {
        Br6W::new(self, 22)
    }
    #[doc = "Bit 23 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br7(&mut self) -> Br7W<BsrrSpec> {
        Br7W::new(self, 23)
    }
    #[doc = "Bit 24 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br8(&mut self) -> Br8W<BsrrSpec> {
        Br8W::new(self, 24)
    }
    #[doc = "Bit 25 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br9(&mut self) -> Br9W<BsrrSpec> {
        Br9W::new(self, 25)
    }
    #[doc = "Bit 26 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br10(&mut self) -> Br10W<BsrrSpec> {
        Br10W::new(self, 26)
    }
    #[doc = "Bit 27 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br11(&mut self) -> Br11W<BsrrSpec> {
        Br11W::new(self, 27)
    }
    #[doc = "Bit 28 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br12(&mut self) -> Br12W<BsrrSpec> {
        Br12W::new(self, 28)
    }
    #[doc = "Bit 29 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br13(&mut self) -> Br13W<BsrrSpec> {
        Br13W::new(self, 29)
    }
    #[doc = "Bit 30 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br14(&mut self) -> Br14W<BsrrSpec> {
        Br14W::new(self, 30)
    }
    #[doc = "Bit 31 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br15(&mut self) -> Br15W<BsrrSpec> {
        Br15W::new(self, 31)
    }
}
#[doc = "GPIO port bit set/reset register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BsrrSpec;
impl crate::RegisterSpec for BsrrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bsrr::W`](W) writer structure"]
impl crate::Writable for BsrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BSRR to value 0"]
impl crate::Resettable for BsrrSpec {}

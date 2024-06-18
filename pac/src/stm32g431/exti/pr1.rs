#[doc = "Register `PR1` reader"]
pub type R = crate::R<Pr1Spec>;
#[doc = "Register `PR1` writer"]
pub type W = crate::W<Pr1Spec>;
#[doc = "Field `PIF0` reader - Pending bit 0"]
pub type Pif0R = crate::BitReader;
#[doc = "Field `PIF0` writer - Pending bit 0"]
pub type Pif0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF1` reader - Pending bit 1"]
pub type Pif1R = crate::BitReader;
#[doc = "Field `PIF1` writer - Pending bit 1"]
pub type Pif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF2` reader - Pending bit 2"]
pub type Pif2R = crate::BitReader;
#[doc = "Field `PIF2` writer - Pending bit 2"]
pub type Pif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF3` reader - Pending bit 3"]
pub type Pif3R = crate::BitReader;
#[doc = "Field `PIF3` writer - Pending bit 3"]
pub type Pif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF4` reader - Pending bit 4"]
pub type Pif4R = crate::BitReader;
#[doc = "Field `PIF4` writer - Pending bit 4"]
pub type Pif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF5` reader - Pending bit 5"]
pub type Pif5R = crate::BitReader;
#[doc = "Field `PIF5` writer - Pending bit 5"]
pub type Pif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF6` reader - Pending bit 6"]
pub type Pif6R = crate::BitReader;
#[doc = "Field `PIF6` writer - Pending bit 6"]
pub type Pif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF7` reader - Pending bit 7"]
pub type Pif7R = crate::BitReader;
#[doc = "Field `PIF7` writer - Pending bit 7"]
pub type Pif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF8` reader - Pending bit 8"]
pub type Pif8R = crate::BitReader;
#[doc = "Field `PIF8` writer - Pending bit 8"]
pub type Pif8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF9` reader - Pending bit 9"]
pub type Pif9R = crate::BitReader;
#[doc = "Field `PIF9` writer - Pending bit 9"]
pub type Pif9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF10` reader - Pending bit 10"]
pub type Pif10R = crate::BitReader;
#[doc = "Field `PIF10` writer - Pending bit 10"]
pub type Pif10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF11` reader - Pending bit 11"]
pub type Pif11R = crate::BitReader;
#[doc = "Field `PIF11` writer - Pending bit 11"]
pub type Pif11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF12` reader - Pending bit 12"]
pub type Pif12R = crate::BitReader;
#[doc = "Field `PIF12` writer - Pending bit 12"]
pub type Pif12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF13` reader - Pending bit 13"]
pub type Pif13R = crate::BitReader;
#[doc = "Field `PIF13` writer - Pending bit 13"]
pub type Pif13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF14` reader - Pending bit 14"]
pub type Pif14R = crate::BitReader;
#[doc = "Field `PIF14` writer - Pending bit 14"]
pub type Pif14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF15` reader - Pending bit 15"]
pub type Pif15R = crate::BitReader;
#[doc = "Field `PIF15` writer - Pending bit 15"]
pub type Pif15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF16` reader - Pending bit 16"]
pub type Pif16R = crate::BitReader;
#[doc = "Field `PIF16` writer - Pending bit 16"]
pub type Pif16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF18` reader - Pending bit 18"]
pub type Pif18R = crate::BitReader;
#[doc = "Field `PIF18` writer - Pending bit 18"]
pub type Pif18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF19` reader - Pending bit 19"]
pub type Pif19R = crate::BitReader;
#[doc = "Field `PIF19` writer - Pending bit 19"]
pub type Pif19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF20` reader - Pending bit 20"]
pub type Pif20R = crate::BitReader;
#[doc = "Field `PIF20` writer - Pending bit 20"]
pub type Pif20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF21` reader - Pending bit 21"]
pub type Pif21R = crate::BitReader;
#[doc = "Field `PIF21` writer - Pending bit 21"]
pub type Pif21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF22` reader - Pending bit 22"]
pub type Pif22R = crate::BitReader;
#[doc = "Field `PIF22` writer - Pending bit 22"]
pub type Pif22W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pending bit 0"]
    #[inline(always)]
    pub fn pif0(&self) -> Pif0R {
        Pif0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pending bit 1"]
    #[inline(always)]
    pub fn pif1(&self) -> Pif1R {
        Pif1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pending bit 2"]
    #[inline(always)]
    pub fn pif2(&self) -> Pif2R {
        Pif2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pending bit 3"]
    #[inline(always)]
    pub fn pif3(&self) -> Pif3R {
        Pif3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pending bit 4"]
    #[inline(always)]
    pub fn pif4(&self) -> Pif4R {
        Pif4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pending bit 5"]
    #[inline(always)]
    pub fn pif5(&self) -> Pif5R {
        Pif5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pending bit 6"]
    #[inline(always)]
    pub fn pif6(&self) -> Pif6R {
        Pif6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pending bit 7"]
    #[inline(always)]
    pub fn pif7(&self) -> Pif7R {
        Pif7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pending bit 8"]
    #[inline(always)]
    pub fn pif8(&self) -> Pif8R {
        Pif8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pending bit 9"]
    #[inline(always)]
    pub fn pif9(&self) -> Pif9R {
        Pif9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pending bit 10"]
    #[inline(always)]
    pub fn pif10(&self) -> Pif10R {
        Pif10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pending bit 11"]
    #[inline(always)]
    pub fn pif11(&self) -> Pif11R {
        Pif11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pending bit 12"]
    #[inline(always)]
    pub fn pif12(&self) -> Pif12R {
        Pif12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pending bit 13"]
    #[inline(always)]
    pub fn pif13(&self) -> Pif13R {
        Pif13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pending bit 14"]
    #[inline(always)]
    pub fn pif14(&self) -> Pif14R {
        Pif14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pending bit 15"]
    #[inline(always)]
    pub fn pif15(&self) -> Pif15R {
        Pif15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Pending bit 16"]
    #[inline(always)]
    pub fn pif16(&self) -> Pif16R {
        Pif16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Pending bit 18"]
    #[inline(always)]
    pub fn pif18(&self) -> Pif18R {
        Pif18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Pending bit 19"]
    #[inline(always)]
    pub fn pif19(&self) -> Pif19R {
        Pif19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Pending bit 20"]
    #[inline(always)]
    pub fn pif20(&self) -> Pif20R {
        Pif20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pending bit 21"]
    #[inline(always)]
    pub fn pif21(&self) -> Pif21R {
        Pif21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Pending bit 22"]
    #[inline(always)]
    pub fn pif22(&self) -> Pif22R {
        Pif22R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PR1")
            .field("pif0", &self.pif0())
            .field("pif1", &self.pif1())
            .field("pif2", &self.pif2())
            .field("pif3", &self.pif3())
            .field("pif4", &self.pif4())
            .field("pif5", &self.pif5())
            .field("pif6", &self.pif6())
            .field("pif7", &self.pif7())
            .field("pif8", &self.pif8())
            .field("pif9", &self.pif9())
            .field("pif10", &self.pif10())
            .field("pif11", &self.pif11())
            .field("pif12", &self.pif12())
            .field("pif13", &self.pif13())
            .field("pif14", &self.pif14())
            .field("pif15", &self.pif15())
            .field("pif16", &self.pif16())
            .field("pif18", &self.pif18())
            .field("pif19", &self.pif19())
            .field("pif20", &self.pif20())
            .field("pif21", &self.pif21())
            .field("pif22", &self.pif22())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Pending bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pif0(&mut self) -> Pif0W<Pr1Spec> {
        Pif0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pending bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pif1(&mut self) -> Pif1W<Pr1Spec> {
        Pif1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pending bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pif2(&mut self) -> Pif2W<Pr1Spec> {
        Pif2W::new(self, 2)
    }
    #[doc = "Bit 3 - Pending bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pif3(&mut self) -> Pif3W<Pr1Spec> {
        Pif3W::new(self, 3)
    }
    #[doc = "Bit 4 - Pending bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pif4(&mut self) -> Pif4W<Pr1Spec> {
        Pif4W::new(self, 4)
    }
    #[doc = "Bit 5 - Pending bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pif5(&mut self) -> Pif5W<Pr1Spec> {
        Pif5W::new(self, 5)
    }
    #[doc = "Bit 6 - Pending bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pif6(&mut self) -> Pif6W<Pr1Spec> {
        Pif6W::new(self, 6)
    }
    #[doc = "Bit 7 - Pending bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pif7(&mut self) -> Pif7W<Pr1Spec> {
        Pif7W::new(self, 7)
    }
    #[doc = "Bit 8 - Pending bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pif8(&mut self) -> Pif8W<Pr1Spec> {
        Pif8W::new(self, 8)
    }
    #[doc = "Bit 9 - Pending bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pif9(&mut self) -> Pif9W<Pr1Spec> {
        Pif9W::new(self, 9)
    }
    #[doc = "Bit 10 - Pending bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pif10(&mut self) -> Pif10W<Pr1Spec> {
        Pif10W::new(self, 10)
    }
    #[doc = "Bit 11 - Pending bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pif11(&mut self) -> Pif11W<Pr1Spec> {
        Pif11W::new(self, 11)
    }
    #[doc = "Bit 12 - Pending bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pif12(&mut self) -> Pif12W<Pr1Spec> {
        Pif12W::new(self, 12)
    }
    #[doc = "Bit 13 - Pending bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pif13(&mut self) -> Pif13W<Pr1Spec> {
        Pif13W::new(self, 13)
    }
    #[doc = "Bit 14 - Pending bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pif14(&mut self) -> Pif14W<Pr1Spec> {
        Pif14W::new(self, 14)
    }
    #[doc = "Bit 15 - Pending bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pif15(&mut self) -> Pif15W<Pr1Spec> {
        Pif15W::new(self, 15)
    }
    #[doc = "Bit 16 - Pending bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pif16(&mut self) -> Pif16W<Pr1Spec> {
        Pif16W::new(self, 16)
    }
    #[doc = "Bit 18 - Pending bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn pif18(&mut self) -> Pif18W<Pr1Spec> {
        Pif18W::new(self, 18)
    }
    #[doc = "Bit 19 - Pending bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pif19(&mut self) -> Pif19W<Pr1Spec> {
        Pif19W::new(self, 19)
    }
    #[doc = "Bit 20 - Pending bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn pif20(&mut self) -> Pif20W<Pr1Spec> {
        Pif20W::new(self, 20)
    }
    #[doc = "Bit 21 - Pending bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn pif21(&mut self) -> Pif21W<Pr1Spec> {
        Pif21W::new(self, 21)
    }
    #[doc = "Bit 22 - Pending bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pif22(&mut self) -> Pif22W<Pr1Spec> {
        Pif22W::new(self, 22)
    }
}
#[doc = "Pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`pr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Spec;
impl crate::RegisterSpec for Pr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1::R`](R) reader structure"]
impl crate::Readable for Pr1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1::W`](W) writer structure"]
impl crate::Writable for Pr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1 to value 0"]
impl crate::Resettable for Pr1Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `SWIER1` reader"]
pub type R = crate::R<Swier1Spec>;
#[doc = "Register `SWIER1` writer"]
pub type W = crate::W<Swier1Spec>;
#[doc = "Field `SWI0` reader - Software Interrupt on line 0"]
pub type Swi0R = crate::BitReader;
#[doc = "Field `SWI0` writer - Software Interrupt on line 0"]
pub type Swi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI1` reader - Software Interrupt on line 1"]
pub type Swi1R = crate::BitReader;
#[doc = "Field `SWI1` writer - Software Interrupt on line 1"]
pub type Swi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI2` reader - Software Interrupt on line 2"]
pub type Swi2R = crate::BitReader;
#[doc = "Field `SWI2` writer - Software Interrupt on line 2"]
pub type Swi2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI3` reader - Software Interrupt on line 3"]
pub type Swi3R = crate::BitReader;
#[doc = "Field `SWI3` writer - Software Interrupt on line 3"]
pub type Swi3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI4` reader - Software Interrupt on line 4"]
pub type Swi4R = crate::BitReader;
#[doc = "Field `SWI4` writer - Software Interrupt on line 4"]
pub type Swi4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI5` reader - Software Interrupt on line 5"]
pub type Swi5R = crate::BitReader;
#[doc = "Field `SWI5` writer - Software Interrupt on line 5"]
pub type Swi5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI6` reader - Software Interrupt on line 6"]
pub type Swi6R = crate::BitReader;
#[doc = "Field `SWI6` writer - Software Interrupt on line 6"]
pub type Swi6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI7` reader - Software Interrupt on line 7"]
pub type Swi7R = crate::BitReader;
#[doc = "Field `SWI7` writer - Software Interrupt on line 7"]
pub type Swi7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI8` reader - Software Interrupt on line 8"]
pub type Swi8R = crate::BitReader;
#[doc = "Field `SWI8` writer - Software Interrupt on line 8"]
pub type Swi8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI9` reader - Software Interrupt on line 9"]
pub type Swi9R = crate::BitReader;
#[doc = "Field `SWI9` writer - Software Interrupt on line 9"]
pub type Swi9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI10` reader - Software Interrupt on line 10"]
pub type Swi10R = crate::BitReader;
#[doc = "Field `SWI10` writer - Software Interrupt on line 10"]
pub type Swi10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI11` reader - Software Interrupt on line 11"]
pub type Swi11R = crate::BitReader;
#[doc = "Field `SWI11` writer - Software Interrupt on line 11"]
pub type Swi11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI12` reader - Software Interrupt on line 12"]
pub type Swi12R = crate::BitReader;
#[doc = "Field `SWI12` writer - Software Interrupt on line 12"]
pub type Swi12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI13` reader - Software Interrupt on line 13"]
pub type Swi13R = crate::BitReader;
#[doc = "Field `SWI13` writer - Software Interrupt on line 13"]
pub type Swi13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI14` reader - Software Interrupt on line 14"]
pub type Swi14R = crate::BitReader;
#[doc = "Field `SWI14` writer - Software Interrupt on line 14"]
pub type Swi14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI15` reader - Software Interrupt on line 15"]
pub type Swi15R = crate::BitReader;
#[doc = "Field `SWI15` writer - Software Interrupt on line 15"]
pub type Swi15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI16` reader - Software Interrupt on line 16"]
pub type Swi16R = crate::BitReader;
#[doc = "Field `SWI16` writer - Software Interrupt on line 16"]
pub type Swi16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI17` reader - Software Interrupt on line 17"]
pub type Swi17R = crate::BitReader;
#[doc = "Field `SWI17` writer - Software Interrupt on line 17"]
pub type Swi17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI19` reader - Software Interrupt on line 19"]
pub type Swi19R = crate::BitReader;
#[doc = "Field `SWI19` writer - Software Interrupt on line 19"]
pub type Swi19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI20` reader - Software Interrupt on line 20"]
pub type Swi20R = crate::BitReader;
#[doc = "Field `SWI20` writer - Software Interrupt on line 20"]
pub type Swi20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI21` reader - Software Interrupt on line 21"]
pub type Swi21R = crate::BitReader;
#[doc = "Field `SWI21` writer - Software Interrupt on line 21"]
pub type Swi21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI22` reader - Software Interrupt on line 22"]
pub type Swi22R = crate::BitReader;
#[doc = "Field `SWI22` writer - Software Interrupt on line 22"]
pub type Swi22W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Interrupt on line 0"]
    #[inline(always)]
    pub fn swi0(&self) -> Swi0R {
        Swi0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Interrupt on line 1"]
    #[inline(always)]
    pub fn swi1(&self) -> Swi1R {
        Swi1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Interrupt on line 2"]
    #[inline(always)]
    pub fn swi2(&self) -> Swi2R {
        Swi2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software Interrupt on line 3"]
    #[inline(always)]
    pub fn swi3(&self) -> Swi3R {
        Swi3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software Interrupt on line 4"]
    #[inline(always)]
    pub fn swi4(&self) -> Swi4R {
        Swi4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software Interrupt on line 5"]
    #[inline(always)]
    pub fn swi5(&self) -> Swi5R {
        Swi5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software Interrupt on line 6"]
    #[inline(always)]
    pub fn swi6(&self) -> Swi6R {
        Swi6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software Interrupt on line 7"]
    #[inline(always)]
    pub fn swi7(&self) -> Swi7R {
        Swi7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software Interrupt on line 8"]
    #[inline(always)]
    pub fn swi8(&self) -> Swi8R {
        Swi8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software Interrupt on line 9"]
    #[inline(always)]
    pub fn swi9(&self) -> Swi9R {
        Swi9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software Interrupt on line 10"]
    #[inline(always)]
    pub fn swi10(&self) -> Swi10R {
        Swi10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software Interrupt on line 11"]
    #[inline(always)]
    pub fn swi11(&self) -> Swi11R {
        Swi11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software Interrupt on line 12"]
    #[inline(always)]
    pub fn swi12(&self) -> Swi12R {
        Swi12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software Interrupt on line 13"]
    #[inline(always)]
    pub fn swi13(&self) -> Swi13R {
        Swi13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software Interrupt on line 14"]
    #[inline(always)]
    pub fn swi14(&self) -> Swi14R {
        Swi14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software Interrupt on line 15"]
    #[inline(always)]
    pub fn swi15(&self) -> Swi15R {
        Swi15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Software Interrupt on line 16"]
    #[inline(always)]
    pub fn swi16(&self) -> Swi16R {
        Swi16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Software Interrupt on line 17"]
    #[inline(always)]
    pub fn swi17(&self) -> Swi17R {
        Swi17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Software Interrupt on line 19"]
    #[inline(always)]
    pub fn swi19(&self) -> Swi19R {
        Swi19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Software Interrupt on line 20"]
    #[inline(always)]
    pub fn swi20(&self) -> Swi20R {
        Swi20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Software Interrupt on line 21"]
    #[inline(always)]
    pub fn swi21(&self) -> Swi21R {
        Swi21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Software Interrupt on line 22"]
    #[inline(always)]
    pub fn swi22(&self) -> Swi22R {
        Swi22R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWIER1")
            .field("swi0", &self.swi0())
            .field("swi1", &self.swi1())
            .field("swi2", &self.swi2())
            .field("swi3", &self.swi3())
            .field("swi4", &self.swi4())
            .field("swi5", &self.swi5())
            .field("swi6", &self.swi6())
            .field("swi7", &self.swi7())
            .field("swi8", &self.swi8())
            .field("swi9", &self.swi9())
            .field("swi10", &self.swi10())
            .field("swi11", &self.swi11())
            .field("swi12", &self.swi12())
            .field("swi13", &self.swi13())
            .field("swi14", &self.swi14())
            .field("swi15", &self.swi15())
            .field("swi16", &self.swi16())
            .field("swi17", &self.swi17())
            .field("swi19", &self.swi19())
            .field("swi20", &self.swi20())
            .field("swi21", &self.swi21())
            .field("swi22", &self.swi22())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Software Interrupt on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn swi0(&mut self) -> Swi0W<Swier1Spec> {
        Swi0W::new(self, 0)
    }
    #[doc = "Bit 1 - Software Interrupt on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn swi1(&mut self) -> Swi1W<Swier1Spec> {
        Swi1W::new(self, 1)
    }
    #[doc = "Bit 2 - Software Interrupt on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn swi2(&mut self) -> Swi2W<Swier1Spec> {
        Swi2W::new(self, 2)
    }
    #[doc = "Bit 3 - Software Interrupt on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn swi3(&mut self) -> Swi3W<Swier1Spec> {
        Swi3W::new(self, 3)
    }
    #[doc = "Bit 4 - Software Interrupt on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn swi4(&mut self) -> Swi4W<Swier1Spec> {
        Swi4W::new(self, 4)
    }
    #[doc = "Bit 5 - Software Interrupt on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn swi5(&mut self) -> Swi5W<Swier1Spec> {
        Swi5W::new(self, 5)
    }
    #[doc = "Bit 6 - Software Interrupt on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn swi6(&mut self) -> Swi6W<Swier1Spec> {
        Swi6W::new(self, 6)
    }
    #[doc = "Bit 7 - Software Interrupt on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn swi7(&mut self) -> Swi7W<Swier1Spec> {
        Swi7W::new(self, 7)
    }
    #[doc = "Bit 8 - Software Interrupt on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn swi8(&mut self) -> Swi8W<Swier1Spec> {
        Swi8W::new(self, 8)
    }
    #[doc = "Bit 9 - Software Interrupt on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn swi9(&mut self) -> Swi9W<Swier1Spec> {
        Swi9W::new(self, 9)
    }
    #[doc = "Bit 10 - Software Interrupt on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn swi10(&mut self) -> Swi10W<Swier1Spec> {
        Swi10W::new(self, 10)
    }
    #[doc = "Bit 11 - Software Interrupt on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn swi11(&mut self) -> Swi11W<Swier1Spec> {
        Swi11W::new(self, 11)
    }
    #[doc = "Bit 12 - Software Interrupt on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn swi12(&mut self) -> Swi12W<Swier1Spec> {
        Swi12W::new(self, 12)
    }
    #[doc = "Bit 13 - Software Interrupt on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn swi13(&mut self) -> Swi13W<Swier1Spec> {
        Swi13W::new(self, 13)
    }
    #[doc = "Bit 14 - Software Interrupt on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn swi14(&mut self) -> Swi14W<Swier1Spec> {
        Swi14W::new(self, 14)
    }
    #[doc = "Bit 15 - Software Interrupt on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn swi15(&mut self) -> Swi15W<Swier1Spec> {
        Swi15W::new(self, 15)
    }
    #[doc = "Bit 16 - Software Interrupt on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn swi16(&mut self) -> Swi16W<Swier1Spec> {
        Swi16W::new(self, 16)
    }
    #[doc = "Bit 17 - Software Interrupt on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn swi17(&mut self) -> Swi17W<Swier1Spec> {
        Swi17W::new(self, 17)
    }
    #[doc = "Bit 19 - Software Interrupt on line 19"]
    #[inline(always)]
    #[must_use]
    pub fn swi19(&mut self) -> Swi19W<Swier1Spec> {
        Swi19W::new(self, 19)
    }
    #[doc = "Bit 20 - Software Interrupt on line 20"]
    #[inline(always)]
    #[must_use]
    pub fn swi20(&mut self) -> Swi20W<Swier1Spec> {
        Swi20W::new(self, 20)
    }
    #[doc = "Bit 21 - Software Interrupt on line 21"]
    #[inline(always)]
    #[must_use]
    pub fn swi21(&mut self) -> Swi21W<Swier1Spec> {
        Swi21W::new(self, 21)
    }
    #[doc = "Bit 22 - Software Interrupt on line 22"]
    #[inline(always)]
    #[must_use]
    pub fn swi22(&mut self) -> Swi22W<Swier1Spec> {
        Swi22W::new(self, 22)
    }
}
#[doc = "Software interrupt event register\n\nYou can [`read`](crate::Reg::read) this register and get [`swier1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swier1Spec;
impl crate::RegisterSpec for Swier1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swier1::R`](R) reader structure"]
impl crate::Readable for Swier1Spec {}
#[doc = "`write(|w| ..)` method takes [`swier1::W`](W) writer structure"]
impl crate::Writable for Swier1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWIER1 to value 0"]
impl crate::Resettable for Swier1Spec {
    const RESET_VALUE: u32 = 0;
}

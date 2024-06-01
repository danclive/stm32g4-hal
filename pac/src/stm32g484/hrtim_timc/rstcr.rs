#[doc = "Register `RSTCR` reader"]
pub type R = crate::R<RstcrSpec>;
#[doc = "Register `RSTCR` writer"]
pub type W = crate::W<RstcrSpec>;
#[doc = "Field `TIMFCMP1` reader - Timer A Update reset"]
pub type Timfcmp1R = crate::BitReader;
#[doc = "Field `TIMFCMP1` writer - Timer A Update reset"]
pub type Timfcmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDT` reader - Timer A Update reset"]
pub type UpdtR = crate::BitReader;
#[doc = "Field `UPDT` writer - Timer A Update reset"]
pub type UpdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2` reader - Timer A compare 2 reset"]
pub type Cmp2R = crate::BitReader;
#[doc = "Field `CMP2` writer - Timer A compare 2 reset"]
pub type Cmp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP4` reader - Timer A compare 4 reset"]
pub type Cmp4R = crate::BitReader;
#[doc = "Field `CMP4` writer - Timer A compare 4 reset"]
pub type Cmp4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTPER` reader - Master timer Period"]
pub type MstperR = crate::BitReader;
#[doc = "Field `MSTPER` writer - Master timer Period"]
pub type MstperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTCMP1` reader - Master compare 1"]
pub type Mstcmp1R = crate::BitReader;
#[doc = "Field `MSTCMP1` writer - Master compare 1"]
pub type Mstcmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTCMP2` reader - Master compare 2"]
pub type Mstcmp2R = crate::BitReader;
#[doc = "Field `MSTCMP2` writer - Master compare 2"]
pub type Mstcmp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTCMP3` reader - Master compare 3"]
pub type Mstcmp3R = crate::BitReader;
#[doc = "Field `MSTCMP3` writer - Master compare 3"]
pub type Mstcmp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTCMP4` reader - Master compare 4"]
pub type Mstcmp4R = crate::BitReader;
#[doc = "Field `MSTCMP4` writer - Master compare 4"]
pub type Mstcmp4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEVNT1` reader - External Event 1"]
pub type Extevnt1R = crate::BitReader;
#[doc = "Field `EXTEVNT1` writer - External Event 1"]
pub type Extevnt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEVNT2` reader - External Event 2"]
pub type Extevnt2R = crate::BitReader;
#[doc = "Field `EXTEVNT2` writer - External Event 2"]
pub type Extevnt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEVNT3` reader - External Event 3"]
pub type Extevnt3R = crate::BitReader;
#[doc = "Field `EXTEVNT3` writer - External Event 3"]
pub type Extevnt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEVNT4` reader - External Event 4"]
pub type Extevnt4R = crate::BitReader;
#[doc = "Field `EXTEVNT4` writer - External Event 4"]
pub type Extevnt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEVNT5` reader - External Event 5"]
pub type Extevnt5R = crate::BitReader;
#[doc = "Field `EXTEVNT5` writer - External Event 5"]
pub type Extevnt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEVNT6` reader - External Event 6"]
pub type Extevnt6R = crate::BitReader;
#[doc = "Field `EXTEVNT6` writer - External Event 6"]
pub type Extevnt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEVNT7` reader - External Event 7"]
pub type Extevnt7R = crate::BitReader;
#[doc = "Field `EXTEVNT7` writer - External Event 7"]
pub type Extevnt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEVNT8` reader - External Event 8"]
pub type Extevnt8R = crate::BitReader;
#[doc = "Field `EXTEVNT8` writer - External Event 8"]
pub type Extevnt8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEVNT9` reader - External Event 9"]
pub type Extevnt9R = crate::BitReader;
#[doc = "Field `EXTEVNT9` writer - External Event 9"]
pub type Extevnt9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEVNT10` reader - External Event 10"]
pub type Extevnt10R = crate::BitReader;
#[doc = "Field `EXTEVNT10` writer - External Event 10"]
pub type Extevnt10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMACMP1` reader - Timer A Compare 1"]
pub type Timacmp1R = crate::BitReader;
#[doc = "Field `TIMACMP1` writer - Timer A Compare 1"]
pub type Timacmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMACMP2` reader - Timer A Compare 2"]
pub type Timacmp2R = crate::BitReader;
#[doc = "Field `TIMACMP2` writer - Timer A Compare 2"]
pub type Timacmp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMACMP4` reader - Timer A Compare 4"]
pub type Timacmp4R = crate::BitReader;
#[doc = "Field `TIMACMP4` writer - Timer A Compare 4"]
pub type Timacmp4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMBCMP1` reader - Timer B Compare 1"]
pub type Timbcmp1R = crate::BitReader;
#[doc = "Field `TIMBCMP1` writer - Timer B Compare 1"]
pub type Timbcmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMBCMP2` reader - Timer B Compare 2"]
pub type Timbcmp2R = crate::BitReader;
#[doc = "Field `TIMBCMP2` writer - Timer B Compare 2"]
pub type Timbcmp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMBCMP4` reader - Timer B Compare 4"]
pub type Timbcmp4R = crate::BitReader;
#[doc = "Field `TIMBCMP4` writer - Timer B Compare 4"]
pub type Timbcmp4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMDCMP1` reader - Timer D Compare 1"]
pub type Timdcmp1R = crate::BitReader;
#[doc = "Field `TIMDCMP1` writer - Timer D Compare 1"]
pub type Timdcmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMDCMP2` reader - Timer D Compare 2"]
pub type Timdcmp2R = crate::BitReader;
#[doc = "Field `TIMDCMP2` writer - Timer D Compare 2"]
pub type Timdcmp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMDCMP4` reader - Timer D Compare 4"]
pub type Timdcmp4R = crate::BitReader;
#[doc = "Field `TIMDCMP4` writer - Timer D Compare 4"]
pub type Timdcmp4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMECMP1` reader - Timer E Compare 1"]
pub type Timecmp1R = crate::BitReader;
#[doc = "Field `TIMECMP1` writer - Timer E Compare 1"]
pub type Timecmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMECMP2` reader - Timer E Compare 2"]
pub type Timecmp2R = crate::BitReader;
#[doc = "Field `TIMECMP2` writer - Timer E Compare 2"]
pub type Timecmp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMECMP4` reader - Timer E Compare 4"]
pub type Timecmp4R = crate::BitReader;
#[doc = "Field `TIMECMP4` writer - Timer E Compare 4"]
pub type Timecmp4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMFCPM2` reader - Timer F Compare 2"]
pub type Timfcpm2R = crate::BitReader;
#[doc = "Field `TIMFCPM2` writer - Timer F Compare 2"]
pub type Timfcpm2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer A Update reset"]
    #[inline(always)]
    pub fn timfcmp1(&self) -> Timfcmp1R {
        Timfcmp1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer A Update reset"]
    #[inline(always)]
    pub fn updt(&self) -> UpdtR {
        UpdtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer A compare 2 reset"]
    #[inline(always)]
    pub fn cmp2(&self) -> Cmp2R {
        Cmp2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer A compare 4 reset"]
    #[inline(always)]
    pub fn cmp4(&self) -> Cmp4R {
        Cmp4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master timer Period"]
    #[inline(always)]
    pub fn mstper(&self) -> MstperR {
        MstperR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master compare 1"]
    #[inline(always)]
    pub fn mstcmp1(&self) -> Mstcmp1R {
        Mstcmp1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Master compare 2"]
    #[inline(always)]
    pub fn mstcmp2(&self) -> Mstcmp2R {
        Mstcmp2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Master compare 3"]
    #[inline(always)]
    pub fn mstcmp3(&self) -> Mstcmp3R {
        Mstcmp3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Master compare 4"]
    #[inline(always)]
    pub fn mstcmp4(&self) -> Mstcmp4R {
        Mstcmp4R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External Event 1"]
    #[inline(always)]
    pub fn extevnt1(&self) -> Extevnt1R {
        Extevnt1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External Event 2"]
    #[inline(always)]
    pub fn extevnt2(&self) -> Extevnt2R {
        Extevnt2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External Event 3"]
    #[inline(always)]
    pub fn extevnt3(&self) -> Extevnt3R {
        Extevnt3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - External Event 4"]
    #[inline(always)]
    pub fn extevnt4(&self) -> Extevnt4R {
        Extevnt4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - External Event 5"]
    #[inline(always)]
    pub fn extevnt5(&self) -> Extevnt5R {
        Extevnt5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - External Event 6"]
    #[inline(always)]
    pub fn extevnt6(&self) -> Extevnt6R {
        Extevnt6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External Event 7"]
    #[inline(always)]
    pub fn extevnt7(&self) -> Extevnt7R {
        Extevnt7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - External Event 8"]
    #[inline(always)]
    pub fn extevnt8(&self) -> Extevnt8R {
        Extevnt8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - External Event 9"]
    #[inline(always)]
    pub fn extevnt9(&self) -> Extevnt9R {
        Extevnt9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - External Event 10"]
    #[inline(always)]
    pub fn extevnt10(&self) -> Extevnt10R {
        Extevnt10R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer A Compare 1"]
    #[inline(always)]
    pub fn timacmp1(&self) -> Timacmp1R {
        Timacmp1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer A Compare 2"]
    #[inline(always)]
    pub fn timacmp2(&self) -> Timacmp2R {
        Timacmp2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer A Compare 4"]
    #[inline(always)]
    pub fn timacmp4(&self) -> Timacmp4R {
        Timacmp4R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timer B Compare 1"]
    #[inline(always)]
    pub fn timbcmp1(&self) -> Timbcmp1R {
        Timbcmp1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Timer B Compare 2"]
    #[inline(always)]
    pub fn timbcmp2(&self) -> Timbcmp2R {
        Timbcmp2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Timer B Compare 4"]
    #[inline(always)]
    pub fn timbcmp4(&self) -> Timbcmp4R {
        Timbcmp4R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Timer D Compare 1"]
    #[inline(always)]
    pub fn timdcmp1(&self) -> Timdcmp1R {
        Timdcmp1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Timer D Compare 2"]
    #[inline(always)]
    pub fn timdcmp2(&self) -> Timdcmp2R {
        Timdcmp2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Timer D Compare 4"]
    #[inline(always)]
    pub fn timdcmp4(&self) -> Timdcmp4R {
        Timdcmp4R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Timer E Compare 1"]
    #[inline(always)]
    pub fn timecmp1(&self) -> Timecmp1R {
        Timecmp1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Timer E Compare 2"]
    #[inline(always)]
    pub fn timecmp2(&self) -> Timecmp2R {
        Timecmp2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Timer E Compare 4"]
    #[inline(always)]
    pub fn timecmp4(&self) -> Timecmp4R {
        Timecmp4R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Timer F Compare 2"]
    #[inline(always)]
    pub fn timfcpm2(&self) -> Timfcpm2R {
        Timfcpm2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTCR")
            .field("timfcpm2", &self.timfcpm2())
            .field("timecmp4", &self.timecmp4())
            .field("timecmp2", &self.timecmp2())
            .field("timecmp1", &self.timecmp1())
            .field("timdcmp4", &self.timdcmp4())
            .field("timdcmp2", &self.timdcmp2())
            .field("timdcmp1", &self.timdcmp1())
            .field("timbcmp4", &self.timbcmp4())
            .field("timbcmp2", &self.timbcmp2())
            .field("timbcmp1", &self.timbcmp1())
            .field("timacmp4", &self.timacmp4())
            .field("timacmp2", &self.timacmp2())
            .field("timacmp1", &self.timacmp1())
            .field("extevnt10", &self.extevnt10())
            .field("extevnt9", &self.extevnt9())
            .field("extevnt8", &self.extevnt8())
            .field("extevnt7", &self.extevnt7())
            .field("extevnt6", &self.extevnt6())
            .field("extevnt5", &self.extevnt5())
            .field("extevnt4", &self.extevnt4())
            .field("extevnt3", &self.extevnt3())
            .field("extevnt2", &self.extevnt2())
            .field("extevnt1", &self.extevnt1())
            .field("mstcmp4", &self.mstcmp4())
            .field("mstcmp3", &self.mstcmp3())
            .field("mstcmp2", &self.mstcmp2())
            .field("mstcmp1", &self.mstcmp1())
            .field("mstper", &self.mstper())
            .field("cmp4", &self.cmp4())
            .field("cmp2", &self.cmp2())
            .field("updt", &self.updt())
            .field("timfcmp1", &self.timfcmp1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timer A Update reset"]
    #[inline(always)]
    #[must_use]
    pub fn timfcmp1(&mut self) -> Timfcmp1W<RstcrSpec> {
        Timfcmp1W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer A Update reset"]
    #[inline(always)]
    #[must_use]
    pub fn updt(&mut self) -> UpdtW<RstcrSpec> {
        UpdtW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer A compare 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2(&mut self) -> Cmp2W<RstcrSpec> {
        Cmp2W::new(self, 2)
    }
    #[doc = "Bit 3 - Timer A compare 4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn cmp4(&mut self) -> Cmp4W<RstcrSpec> {
        Cmp4W::new(self, 3)
    }
    #[doc = "Bit 4 - Master timer Period"]
    #[inline(always)]
    #[must_use]
    pub fn mstper(&mut self) -> MstperW<RstcrSpec> {
        MstperW::new(self, 4)
    }
    #[doc = "Bit 5 - Master compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp1(&mut self) -> Mstcmp1W<RstcrSpec> {
        Mstcmp1W::new(self, 5)
    }
    #[doc = "Bit 6 - Master compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp2(&mut self) -> Mstcmp2W<RstcrSpec> {
        Mstcmp2W::new(self, 6)
    }
    #[doc = "Bit 7 - Master compare 3"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp3(&mut self) -> Mstcmp3W<RstcrSpec> {
        Mstcmp3W::new(self, 7)
    }
    #[doc = "Bit 8 - Master compare 4"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp4(&mut self) -> Mstcmp4W<RstcrSpec> {
        Mstcmp4W::new(self, 8)
    }
    #[doc = "Bit 9 - External Event 1"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt1(&mut self) -> Extevnt1W<RstcrSpec> {
        Extevnt1W::new(self, 9)
    }
    #[doc = "Bit 10 - External Event 2"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt2(&mut self) -> Extevnt2W<RstcrSpec> {
        Extevnt2W::new(self, 10)
    }
    #[doc = "Bit 11 - External Event 3"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt3(&mut self) -> Extevnt3W<RstcrSpec> {
        Extevnt3W::new(self, 11)
    }
    #[doc = "Bit 12 - External Event 4"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt4(&mut self) -> Extevnt4W<RstcrSpec> {
        Extevnt4W::new(self, 12)
    }
    #[doc = "Bit 13 - External Event 5"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt5(&mut self) -> Extevnt5W<RstcrSpec> {
        Extevnt5W::new(self, 13)
    }
    #[doc = "Bit 14 - External Event 6"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt6(&mut self) -> Extevnt6W<RstcrSpec> {
        Extevnt6W::new(self, 14)
    }
    #[doc = "Bit 15 - External Event 7"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt7(&mut self) -> Extevnt7W<RstcrSpec> {
        Extevnt7W::new(self, 15)
    }
    #[doc = "Bit 16 - External Event 8"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt8(&mut self) -> Extevnt8W<RstcrSpec> {
        Extevnt8W::new(self, 16)
    }
    #[doc = "Bit 17 - External Event 9"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt9(&mut self) -> Extevnt9W<RstcrSpec> {
        Extevnt9W::new(self, 17)
    }
    #[doc = "Bit 18 - External Event 10"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt10(&mut self) -> Extevnt10W<RstcrSpec> {
        Extevnt10W::new(self, 18)
    }
    #[doc = "Bit 19 - Timer A Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn timacmp1(&mut self) -> Timacmp1W<RstcrSpec> {
        Timacmp1W::new(self, 19)
    }
    #[doc = "Bit 20 - Timer A Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn timacmp2(&mut self) -> Timacmp2W<RstcrSpec> {
        Timacmp2W::new(self, 20)
    }
    #[doc = "Bit 21 - Timer A Compare 4"]
    #[inline(always)]
    #[must_use]
    pub fn timacmp4(&mut self) -> Timacmp4W<RstcrSpec> {
        Timacmp4W::new(self, 21)
    }
    #[doc = "Bit 22 - Timer B Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn timbcmp1(&mut self) -> Timbcmp1W<RstcrSpec> {
        Timbcmp1W::new(self, 22)
    }
    #[doc = "Bit 23 - Timer B Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn timbcmp2(&mut self) -> Timbcmp2W<RstcrSpec> {
        Timbcmp2W::new(self, 23)
    }
    #[doc = "Bit 24 - Timer B Compare 4"]
    #[inline(always)]
    #[must_use]
    pub fn timbcmp4(&mut self) -> Timbcmp4W<RstcrSpec> {
        Timbcmp4W::new(self, 24)
    }
    #[doc = "Bit 25 - Timer D Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn timdcmp1(&mut self) -> Timdcmp1W<RstcrSpec> {
        Timdcmp1W::new(self, 25)
    }
    #[doc = "Bit 26 - Timer D Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn timdcmp2(&mut self) -> Timdcmp2W<RstcrSpec> {
        Timdcmp2W::new(self, 26)
    }
    #[doc = "Bit 27 - Timer D Compare 4"]
    #[inline(always)]
    #[must_use]
    pub fn timdcmp4(&mut self) -> Timdcmp4W<RstcrSpec> {
        Timdcmp4W::new(self, 27)
    }
    #[doc = "Bit 28 - Timer E Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn timecmp1(&mut self) -> Timecmp1W<RstcrSpec> {
        Timecmp1W::new(self, 28)
    }
    #[doc = "Bit 29 - Timer E Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn timecmp2(&mut self) -> Timecmp2W<RstcrSpec> {
        Timecmp2W::new(self, 29)
    }
    #[doc = "Bit 30 - Timer E Compare 4"]
    #[inline(always)]
    #[must_use]
    pub fn timecmp4(&mut self) -> Timecmp4W<RstcrSpec> {
        Timecmp4W::new(self, 30)
    }
    #[doc = "Bit 31 - Timer F Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn timfcpm2(&mut self) -> Timfcpm2W<RstcrSpec> {
        Timfcpm2W::new(self, 31)
    }
}
#[doc = "TimerA Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstcrSpec;
impl crate::RegisterSpec for RstcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstcr::R`](R) reader structure"]
impl crate::Readable for RstcrSpec {}
#[doc = "`write(|w| ..)` method takes [`rstcr::W`](W) writer structure"]
impl crate::Writable for RstcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSTCR to value 0"]
impl crate::Resettable for RstcrSpec {
    const RESET_VALUE: u32 = 0;
}

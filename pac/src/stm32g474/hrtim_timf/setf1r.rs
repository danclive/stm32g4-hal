#[doc = "Register `SETF1R` reader"]
pub type R = crate::R<Setf1rSpec>;
#[doc = "Register `SETF1R` writer"]
pub type W = crate::W<Setf1rSpec>;
#[doc = "Field `SST` reader - Software Set trigger"]
pub type SstR = crate::BitReader;
#[doc = "Field `SST` writer - Software Set trigger"]
pub type SstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESYNC` reader - Timer A resynchronizaton"]
pub type ResyncR = crate::BitReader;
#[doc = "Field `RESYNC` writer - Timer A resynchronizaton"]
pub type ResyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER` reader - Timer A Period"]
pub type PerR = crate::BitReader;
#[doc = "Field `PER` writer - Timer A Period"]
pub type PerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1` reader - Timer A compare 1"]
pub type Cmp1R = crate::BitReader;
#[doc = "Field `CMP1` writer - Timer A compare 1"]
pub type Cmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2` reader - Timer A compare 2"]
pub type Cmp2R = crate::BitReader;
#[doc = "Field `CMP2` writer - Timer A compare 2"]
pub type Cmp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP3` reader - Timer A compare 3"]
pub type Cmp3R = crate::BitReader;
#[doc = "Field `CMP3` writer - Timer A compare 3"]
pub type Cmp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP4` reader - Timer A compare 4"]
pub type Cmp4R = crate::BitReader;
#[doc = "Field `CMP4` writer - Timer A compare 4"]
pub type Cmp4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTPER` reader - Master Period"]
pub type MstperR = crate::BitReader;
#[doc = "Field `MSTPER` writer - Master Period"]
pub type MstperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTCMP1` reader - Master Compare 1"]
pub type Mstcmp1R = crate::BitReader;
#[doc = "Field `MSTCMP1` writer - Master Compare 1"]
pub type Mstcmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTCMP2` reader - Master Compare 2"]
pub type Mstcmp2R = crate::BitReader;
#[doc = "Field `MSTCMP2` writer - Master Compare 2"]
pub type Mstcmp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTCMP3` reader - Master Compare 3"]
pub type Mstcmp3R = crate::BitReader;
#[doc = "Field `MSTCMP3` writer - Master Compare 3"]
pub type Mstcmp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTCMP4` reader - Master Compare 4"]
pub type Mstcmp4R = crate::BitReader;
#[doc = "Field `MSTCMP4` writer - Master Compare 4"]
pub type Mstcmp4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEVNT1` reader - Timer Event 1"]
pub type Timevnt1R = crate::BitReader;
#[doc = "Field `TIMEVNT1` writer - Timer Event 1"]
pub type Timevnt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEVNT2` reader - Timer Event 2"]
pub type Timevnt2R = crate::BitReader;
#[doc = "Field `TIMEVNT2` writer - Timer Event 2"]
pub type Timevnt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEVNT3` reader - Timer Event 3"]
pub type Timevnt3R = crate::BitReader;
#[doc = "Field `TIMEVNT3` writer - Timer Event 3"]
pub type Timevnt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEVNT4` reader - Timer Event 4"]
pub type Timevnt4R = crate::BitReader;
#[doc = "Field `TIMEVNT4` writer - Timer Event 4"]
pub type Timevnt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEVNT5` reader - Timer Event 5"]
pub type Timevnt5R = crate::BitReader;
#[doc = "Field `TIMEVNT5` writer - Timer Event 5"]
pub type Timevnt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEVNT6` reader - Timer Event 6"]
pub type Timevnt6R = crate::BitReader;
#[doc = "Field `TIMEVNT6` writer - Timer Event 6"]
pub type Timevnt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEVNT7` reader - Timer Event 7"]
pub type Timevnt7R = crate::BitReader;
#[doc = "Field `TIMEVNT7` writer - Timer Event 7"]
pub type Timevnt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEVNT8` reader - Timer Event 8"]
pub type Timevnt8R = crate::BitReader;
#[doc = "Field `TIMEVNT8` writer - Timer Event 8"]
pub type Timevnt8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEVNT9` reader - Timer Event 9"]
pub type Timevnt9R = crate::BitReader;
#[doc = "Field `TIMEVNT9` writer - Timer Event 9"]
pub type Timevnt9W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `UPDATE` reader - Registers update (transfer preload to active)"]
pub type UpdateR = crate::BitReader;
#[doc = "Field `UPDATE` writer - Registers update (transfer preload to active)"]
pub type UpdateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Set trigger"]
    #[inline(always)]
    pub fn sst(&self) -> SstR {
        SstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer A resynchronizaton"]
    #[inline(always)]
    pub fn resync(&self) -> ResyncR {
        ResyncR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer A Period"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer A compare 1"]
    #[inline(always)]
    pub fn cmp1(&self) -> Cmp1R {
        Cmp1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer A compare 2"]
    #[inline(always)]
    pub fn cmp2(&self) -> Cmp2R {
        Cmp2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer A compare 3"]
    #[inline(always)]
    pub fn cmp3(&self) -> Cmp3R {
        Cmp3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer A compare 4"]
    #[inline(always)]
    pub fn cmp4(&self) -> Cmp4R {
        Cmp4R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Master Period"]
    #[inline(always)]
    pub fn mstper(&self) -> MstperR {
        MstperR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Master Compare 1"]
    #[inline(always)]
    pub fn mstcmp1(&self) -> Mstcmp1R {
        Mstcmp1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master Compare 2"]
    #[inline(always)]
    pub fn mstcmp2(&self) -> Mstcmp2R {
        Mstcmp2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Master Compare 3"]
    #[inline(always)]
    pub fn mstcmp3(&self) -> Mstcmp3R {
        Mstcmp3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Master Compare 4"]
    #[inline(always)]
    pub fn mstcmp4(&self) -> Mstcmp4R {
        Mstcmp4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer Event 1"]
    #[inline(always)]
    pub fn timevnt1(&self) -> Timevnt1R {
        Timevnt1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer Event 2"]
    #[inline(always)]
    pub fn timevnt2(&self) -> Timevnt2R {
        Timevnt2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Timer Event 3"]
    #[inline(always)]
    pub fn timevnt3(&self) -> Timevnt3R {
        Timevnt3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timer Event 4"]
    #[inline(always)]
    pub fn timevnt4(&self) -> Timevnt4R {
        Timevnt4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer Event 5"]
    #[inline(always)]
    pub fn timevnt5(&self) -> Timevnt5R {
        Timevnt5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer Event 6"]
    #[inline(always)]
    pub fn timevnt6(&self) -> Timevnt6R {
        Timevnt6R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer Event 7"]
    #[inline(always)]
    pub fn timevnt7(&self) -> Timevnt7R {
        Timevnt7R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer Event 8"]
    #[inline(always)]
    pub fn timevnt8(&self) -> Timevnt8R {
        Timevnt8R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer Event 9"]
    #[inline(always)]
    pub fn timevnt9(&self) -> Timevnt9R {
        Timevnt9R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - External Event 1"]
    #[inline(always)]
    pub fn extevnt1(&self) -> Extevnt1R {
        Extevnt1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - External Event 2"]
    #[inline(always)]
    pub fn extevnt2(&self) -> Extevnt2R {
        Extevnt2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - External Event 3"]
    #[inline(always)]
    pub fn extevnt3(&self) -> Extevnt3R {
        Extevnt3R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - External Event 4"]
    #[inline(always)]
    pub fn extevnt4(&self) -> Extevnt4R {
        Extevnt4R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - External Event 5"]
    #[inline(always)]
    pub fn extevnt5(&self) -> Extevnt5R {
        Extevnt5R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - External Event 6"]
    #[inline(always)]
    pub fn extevnt6(&self) -> Extevnt6R {
        Extevnt6R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - External Event 7"]
    #[inline(always)]
    pub fn extevnt7(&self) -> Extevnt7R {
        Extevnt7R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - External Event 8"]
    #[inline(always)]
    pub fn extevnt8(&self) -> Extevnt8R {
        Extevnt8R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - External Event 9"]
    #[inline(always)]
    pub fn extevnt9(&self) -> Extevnt9R {
        Extevnt9R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - External Event 10"]
    #[inline(always)]
    pub fn extevnt10(&self) -> Extevnt10R {
        Extevnt10R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Registers update (transfer preload to active)"]
    #[inline(always)]
    pub fn update(&self) -> UpdateR {
        UpdateR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SETF1R")
            .field("update", &self.update())
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
            .field("timevnt9", &self.timevnt9())
            .field("timevnt8", &self.timevnt8())
            .field("timevnt7", &self.timevnt7())
            .field("timevnt6", &self.timevnt6())
            .field("timevnt5", &self.timevnt5())
            .field("timevnt4", &self.timevnt4())
            .field("timevnt3", &self.timevnt3())
            .field("timevnt2", &self.timevnt2())
            .field("timevnt1", &self.timevnt1())
            .field("mstcmp4", &self.mstcmp4())
            .field("mstcmp3", &self.mstcmp3())
            .field("mstcmp2", &self.mstcmp2())
            .field("mstcmp1", &self.mstcmp1())
            .field("mstper", &self.mstper())
            .field("cmp4", &self.cmp4())
            .field("cmp3", &self.cmp3())
            .field("cmp2", &self.cmp2())
            .field("cmp1", &self.cmp1())
            .field("per", &self.per())
            .field("resync", &self.resync())
            .field("sst", &self.sst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Software Set trigger"]
    #[inline(always)]
    #[must_use]
    pub fn sst(&mut self) -> SstW<Setf1rSpec> {
        SstW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer A resynchronizaton"]
    #[inline(always)]
    #[must_use]
    pub fn resync(&mut self) -> ResyncW<Setf1rSpec> {
        ResyncW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer A Period"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PerW<Setf1rSpec> {
        PerW::new(self, 2)
    }
    #[doc = "Bit 3 - Timer A compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1(&mut self) -> Cmp1W<Setf1rSpec> {
        Cmp1W::new(self, 3)
    }
    #[doc = "Bit 4 - Timer A compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2(&mut self) -> Cmp2W<Setf1rSpec> {
        Cmp2W::new(self, 4)
    }
    #[doc = "Bit 5 - Timer A compare 3"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3(&mut self) -> Cmp3W<Setf1rSpec> {
        Cmp3W::new(self, 5)
    }
    #[doc = "Bit 6 - Timer A compare 4"]
    #[inline(always)]
    #[must_use]
    pub fn cmp4(&mut self) -> Cmp4W<Setf1rSpec> {
        Cmp4W::new(self, 6)
    }
    #[doc = "Bit 7 - Master Period"]
    #[inline(always)]
    #[must_use]
    pub fn mstper(&mut self) -> MstperW<Setf1rSpec> {
        MstperW::new(self, 7)
    }
    #[doc = "Bit 8 - Master Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp1(&mut self) -> Mstcmp1W<Setf1rSpec> {
        Mstcmp1W::new(self, 8)
    }
    #[doc = "Bit 9 - Master Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp2(&mut self) -> Mstcmp2W<Setf1rSpec> {
        Mstcmp2W::new(self, 9)
    }
    #[doc = "Bit 10 - Master Compare 3"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp3(&mut self) -> Mstcmp3W<Setf1rSpec> {
        Mstcmp3W::new(self, 10)
    }
    #[doc = "Bit 11 - Master Compare 4"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp4(&mut self) -> Mstcmp4W<Setf1rSpec> {
        Mstcmp4W::new(self, 11)
    }
    #[doc = "Bit 12 - Timer Event 1"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt1(&mut self) -> Timevnt1W<Setf1rSpec> {
        Timevnt1W::new(self, 12)
    }
    #[doc = "Bit 13 - Timer Event 2"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt2(&mut self) -> Timevnt2W<Setf1rSpec> {
        Timevnt2W::new(self, 13)
    }
    #[doc = "Bit 14 - Timer Event 3"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt3(&mut self) -> Timevnt3W<Setf1rSpec> {
        Timevnt3W::new(self, 14)
    }
    #[doc = "Bit 15 - Timer Event 4"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt4(&mut self) -> Timevnt4W<Setf1rSpec> {
        Timevnt4W::new(self, 15)
    }
    #[doc = "Bit 16 - Timer Event 5"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt5(&mut self) -> Timevnt5W<Setf1rSpec> {
        Timevnt5W::new(self, 16)
    }
    #[doc = "Bit 17 - Timer Event 6"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt6(&mut self) -> Timevnt6W<Setf1rSpec> {
        Timevnt6W::new(self, 17)
    }
    #[doc = "Bit 18 - Timer Event 7"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt7(&mut self) -> Timevnt7W<Setf1rSpec> {
        Timevnt7W::new(self, 18)
    }
    #[doc = "Bit 19 - Timer Event 8"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt8(&mut self) -> Timevnt8W<Setf1rSpec> {
        Timevnt8W::new(self, 19)
    }
    #[doc = "Bit 20 - Timer Event 9"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt9(&mut self) -> Timevnt9W<Setf1rSpec> {
        Timevnt9W::new(self, 20)
    }
    #[doc = "Bit 21 - External Event 1"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt1(&mut self) -> Extevnt1W<Setf1rSpec> {
        Extevnt1W::new(self, 21)
    }
    #[doc = "Bit 22 - External Event 2"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt2(&mut self) -> Extevnt2W<Setf1rSpec> {
        Extevnt2W::new(self, 22)
    }
    #[doc = "Bit 23 - External Event 3"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt3(&mut self) -> Extevnt3W<Setf1rSpec> {
        Extevnt3W::new(self, 23)
    }
    #[doc = "Bit 24 - External Event 4"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt4(&mut self) -> Extevnt4W<Setf1rSpec> {
        Extevnt4W::new(self, 24)
    }
    #[doc = "Bit 25 - External Event 5"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt5(&mut self) -> Extevnt5W<Setf1rSpec> {
        Extevnt5W::new(self, 25)
    }
    #[doc = "Bit 26 - External Event 6"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt6(&mut self) -> Extevnt6W<Setf1rSpec> {
        Extevnt6W::new(self, 26)
    }
    #[doc = "Bit 27 - External Event 7"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt7(&mut self) -> Extevnt7W<Setf1rSpec> {
        Extevnt7W::new(self, 27)
    }
    #[doc = "Bit 28 - External Event 8"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt8(&mut self) -> Extevnt8W<Setf1rSpec> {
        Extevnt8W::new(self, 28)
    }
    #[doc = "Bit 29 - External Event 9"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt9(&mut self) -> Extevnt9W<Setf1rSpec> {
        Extevnt9W::new(self, 29)
    }
    #[doc = "Bit 30 - External Event 10"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt10(&mut self) -> Extevnt10W<Setf1rSpec> {
        Extevnt10W::new(self, 30)
    }
    #[doc = "Bit 31 - Registers update (transfer preload to active)"]
    #[inline(always)]
    #[must_use]
    pub fn update(&mut self) -> UpdateW<Setf1rSpec> {
        UpdateW::new(self, 31)
    }
}
#[doc = "Timerx Output1 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setf1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setf1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Setf1rSpec;
impl crate::RegisterSpec for Setf1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`setf1r::R`](R) reader structure"]
impl crate::Readable for Setf1rSpec {}
#[doc = "`write(|w| ..)` method takes [`setf1r::W`](W) writer structure"]
impl crate::Writable for Setf1rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SETF1R to value 0"]
impl crate::Resettable for Setf1rSpec {
    const RESET_VALUE: u32 = 0;
}

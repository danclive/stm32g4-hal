#[doc = "Register `ALRMBR` reader"]
pub type R = crate::R<AlrmbrSpec>;
#[doc = "Register `ALRMBR` writer"]
pub type W = crate::W<AlrmbrSpec>;
#[doc = "Field `SU` reader - Second units in BCD format"]
pub type SuR = crate::FieldReader;
#[doc = "Field `SU` writer - Second units in BCD format"]
pub type SuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ST` reader - Second tens in BCD format"]
pub type StR = crate::FieldReader;
#[doc = "Field `ST` writer - Second tens in BCD format"]
pub type StW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSK1` reader - Alarm B seconds mask"]
pub type Msk1R = crate::BitReader;
#[doc = "Field `MSK1` writer - Alarm B seconds mask"]
pub type Msk1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MNU` reader - Minute units in BCD format"]
pub type MnuR = crate::FieldReader;
#[doc = "Field `MNU` writer - Minute units in BCD format"]
pub type MnuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MNT` reader - Minute tens in BCD format"]
pub type MntR = crate::FieldReader;
#[doc = "Field `MNT` writer - Minute tens in BCD format"]
pub type MntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSK2` reader - Alarm B minutes mask"]
pub type Msk2R = crate::BitReader;
#[doc = "Field `MSK2` writer - Alarm B minutes mask"]
pub type Msk2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HU` reader - Hour units in BCD format"]
pub type HuR = crate::FieldReader;
#[doc = "Field `HU` writer - Hour units in BCD format"]
pub type HuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HT` reader - Hour tens in BCD format"]
pub type HtR = crate::FieldReader;
#[doc = "Field `HT` writer - Hour tens in BCD format"]
pub type HtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PM` reader - AM/PM notation"]
pub type PmR = crate::BitReader;
#[doc = "Field `PM` writer - AM/PM notation"]
pub type PmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSK3` reader - Alarm B hours mask"]
pub type Msk3R = crate::BitReader;
#[doc = "Field `MSK3` writer - Alarm B hours mask"]
pub type Msk3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DU` reader - Date units or day in BCD format"]
pub type DuR = crate::FieldReader;
#[doc = "Field `DU` writer - Date units or day in BCD format"]
pub type DuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DT` reader - Date tens in BCD format"]
pub type DtR = crate::FieldReader;
#[doc = "Field `DT` writer - Date tens in BCD format"]
pub type DtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WDSEL` reader - Week day selection"]
pub type WdselR = crate::BitReader;
#[doc = "Field `WDSEL` writer - Week day selection"]
pub type WdselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSK4` reader - Alarm B date mask"]
pub type Msk4R = crate::BitReader;
#[doc = "Field `MSK4` writer - Alarm B date mask"]
pub type Msk4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Second units in BCD format"]
    #[inline(always)]
    pub fn su(&self) -> SuR {
        SuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format"]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Alarm B seconds mask"]
    #[inline(always)]
    pub fn msk1(&self) -> Msk1R {
        Msk1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    pub fn mnu(&self) -> MnuR {
        MnuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    pub fn mnt(&self) -> MntR {
        MntR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Alarm B minutes mask"]
    #[inline(always)]
    pub fn msk2(&self) -> Msk2R {
        Msk2R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn hu(&self) -> HuR {
        HuR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    pub fn ht(&self) -> HtR {
        HtR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Alarm B hours mask"]
    #[inline(always)]
    pub fn msk3(&self) -> Msk3R {
        Msk3R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format"]
    #[inline(always)]
    pub fn du(&self) -> DuR {
        DuR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    pub fn wdsel(&self) -> WdselR {
        WdselR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Alarm B date mask"]
    #[inline(always)]
    pub fn msk4(&self) -> Msk4R {
        Msk4R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALRMBR")
            .field("msk4", &self.msk4())
            .field("wdsel", &self.wdsel())
            .field("dt", &self.dt())
            .field("du", &self.du())
            .field("msk3", &self.msk3())
            .field("pm", &self.pm())
            .field("ht", &self.ht())
            .field("hu", &self.hu())
            .field("msk2", &self.msk2())
            .field("mnt", &self.mnt())
            .field("mnu", &self.mnu())
            .field("msk1", &self.msk1())
            .field("st", &self.st())
            .field("su", &self.su())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units in BCD format"]
    #[inline(always)]
    pub fn su(&mut self) -> SuW<AlrmbrSpec> {
        SuW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format"]
    #[inline(always)]
    pub fn st(&mut self) -> StW<AlrmbrSpec> {
        StW::new(self, 4)
    }
    #[doc = "Bit 7 - Alarm B seconds mask"]
    #[inline(always)]
    pub fn msk1(&mut self) -> Msk1W<AlrmbrSpec> {
        Msk1W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    pub fn mnu(&mut self) -> MnuW<AlrmbrSpec> {
        MnuW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    pub fn mnt(&mut self) -> MntW<AlrmbrSpec> {
        MntW::new(self, 12)
    }
    #[doc = "Bit 15 - Alarm B minutes mask"]
    #[inline(always)]
    pub fn msk2(&mut self) -> Msk2W<AlrmbrSpec> {
        Msk2W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn hu(&mut self) -> HuW<AlrmbrSpec> {
        HuW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    pub fn ht(&mut self) -> HtW<AlrmbrSpec> {
        HtW::new(self, 20)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<AlrmbrSpec> {
        PmW::new(self, 22)
    }
    #[doc = "Bit 23 - Alarm B hours mask"]
    #[inline(always)]
    pub fn msk3(&mut self) -> Msk3W<AlrmbrSpec> {
        Msk3W::new(self, 23)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format"]
    #[inline(always)]
    pub fn du(&mut self) -> DuW<AlrmbrSpec> {
        DuW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format"]
    #[inline(always)]
    pub fn dt(&mut self) -> DtW<AlrmbrSpec> {
        DtW::new(self, 28)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    pub fn wdsel(&mut self) -> WdselW<AlrmbrSpec> {
        WdselW::new(self, 30)
    }
    #[doc = "Bit 31 - Alarm B date mask"]
    #[inline(always)]
    pub fn msk4(&mut self) -> Msk4W<AlrmbrSpec> {
        Msk4W::new(self, 31)
    }
}
#[doc = "alarm B register\n\nYou can [`read`](crate::Reg::read) this register and get [`alrmbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlrmbrSpec;
impl crate::RegisterSpec for AlrmbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alrmbr::R`](R) reader structure"]
impl crate::Readable for AlrmbrSpec {}
#[doc = "`write(|w| ..)` method takes [`alrmbr::W`](W) writer structure"]
impl crate::Writable for AlrmbrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALRMBR to value 0"]
impl crate::Resettable for AlrmbrSpec {}

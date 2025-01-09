#[doc = "Register `SMPR1` reader"]
pub type R = crate::R<Smpr1Spec>;
#[doc = "Register `SMPR1` writer"]
pub type W = crate::W<Smpr1Spec>;
#[doc = "Field `SMP0` reader - SMP0"]
pub type Smp0R = crate::FieldReader;
#[doc = "Field `SMP0` writer - SMP0"]
pub type Smp0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP1` reader - SMP1"]
pub type Smp1R = crate::FieldReader;
#[doc = "Field `SMP1` writer - SMP1"]
pub type Smp1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP2` reader - SMP2"]
pub type Smp2R = crate::FieldReader;
#[doc = "Field `SMP2` writer - SMP2"]
pub type Smp2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP3` reader - SMP3"]
pub type Smp3R = crate::FieldReader;
#[doc = "Field `SMP3` writer - SMP3"]
pub type Smp3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP4` reader - SMP4"]
pub type Smp4R = crate::FieldReader;
#[doc = "Field `SMP4` writer - SMP4"]
pub type Smp4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP5` reader - SMP5"]
pub type Smp5R = crate::FieldReader;
#[doc = "Field `SMP5` writer - SMP5"]
pub type Smp5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP6` reader - SMP6"]
pub type Smp6R = crate::FieldReader;
#[doc = "Field `SMP6` writer - SMP6"]
pub type Smp6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP7` reader - SMP7"]
pub type Smp7R = crate::FieldReader;
#[doc = "Field `SMP7` writer - SMP7"]
pub type Smp7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP8` reader - SMP8"]
pub type Smp8R = crate::FieldReader;
#[doc = "Field `SMP8` writer - SMP8"]
pub type Smp8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP9` reader - SMP9"]
pub type Smp9R = crate::FieldReader;
#[doc = "Field `SMP9` writer - SMP9"]
pub type Smp9W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMPPLUS` reader - Addition of one clock cycle to the sampling time"]
pub type SmpplusR = crate::BitReader;
#[doc = "Field `SMPPLUS` writer - Addition of one clock cycle to the sampling time"]
pub type SmpplusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - SMP0"]
    #[inline(always)]
    pub fn smp0(&self) -> Smp0R {
        Smp0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - SMP1"]
    #[inline(always)]
    pub fn smp1(&self) -> Smp1R {
        Smp1R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - SMP2"]
    #[inline(always)]
    pub fn smp2(&self) -> Smp2R {
        Smp2R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - SMP3"]
    #[inline(always)]
    pub fn smp3(&self) -> Smp3R {
        Smp3R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - SMP4"]
    #[inline(always)]
    pub fn smp4(&self) -> Smp4R {
        Smp4R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - SMP5"]
    #[inline(always)]
    pub fn smp5(&self) -> Smp5R {
        Smp5R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - SMP6"]
    #[inline(always)]
    pub fn smp6(&self) -> Smp6R {
        Smp6R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - SMP7"]
    #[inline(always)]
    pub fn smp7(&self) -> Smp7R {
        Smp7R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - SMP8"]
    #[inline(always)]
    pub fn smp8(&self) -> Smp8R {
        Smp8R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - SMP9"]
    #[inline(always)]
    pub fn smp9(&self) -> Smp9R {
        Smp9R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bit 31 - Addition of one clock cycle to the sampling time"]
    #[inline(always)]
    pub fn smpplus(&self) -> SmpplusR {
        SmpplusR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR1")
            .field("smp9", &self.smp9())
            .field("smp8", &self.smp8())
            .field("smp7", &self.smp7())
            .field("smp6", &self.smp6())
            .field("smp5", &self.smp5())
            .field("smp4", &self.smp4())
            .field("smp3", &self.smp3())
            .field("smp2", &self.smp2())
            .field("smp1", &self.smp1())
            .field("smpplus", &self.smpplus())
            .field("smp0", &self.smp0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - SMP0"]
    #[inline(always)]
    pub fn smp0(&mut self) -> Smp0W<Smpr1Spec> {
        Smp0W::new(self, 0)
    }
    #[doc = "Bits 3:5 - SMP1"]
    #[inline(always)]
    pub fn smp1(&mut self) -> Smp1W<Smpr1Spec> {
        Smp1W::new(self, 3)
    }
    #[doc = "Bits 6:8 - SMP2"]
    #[inline(always)]
    pub fn smp2(&mut self) -> Smp2W<Smpr1Spec> {
        Smp2W::new(self, 6)
    }
    #[doc = "Bits 9:11 - SMP3"]
    #[inline(always)]
    pub fn smp3(&mut self) -> Smp3W<Smpr1Spec> {
        Smp3W::new(self, 9)
    }
    #[doc = "Bits 12:14 - SMP4"]
    #[inline(always)]
    pub fn smp4(&mut self) -> Smp4W<Smpr1Spec> {
        Smp4W::new(self, 12)
    }
    #[doc = "Bits 15:17 - SMP5"]
    #[inline(always)]
    pub fn smp5(&mut self) -> Smp5W<Smpr1Spec> {
        Smp5W::new(self, 15)
    }
    #[doc = "Bits 18:20 - SMP6"]
    #[inline(always)]
    pub fn smp6(&mut self) -> Smp6W<Smpr1Spec> {
        Smp6W::new(self, 18)
    }
    #[doc = "Bits 21:23 - SMP7"]
    #[inline(always)]
    pub fn smp7(&mut self) -> Smp7W<Smpr1Spec> {
        Smp7W::new(self, 21)
    }
    #[doc = "Bits 24:26 - SMP8"]
    #[inline(always)]
    pub fn smp8(&mut self) -> Smp8W<Smpr1Spec> {
        Smp8W::new(self, 24)
    }
    #[doc = "Bits 27:29 - SMP9"]
    #[inline(always)]
    pub fn smp9(&mut self) -> Smp9W<Smpr1Spec> {
        Smp9W::new(self, 27)
    }
    #[doc = "Bit 31 - Addition of one clock cycle to the sampling time"]
    #[inline(always)]
    pub fn smpplus(&mut self) -> SmpplusW<Smpr1Spec> {
        SmpplusW::new(self, 31)
    }
}
#[doc = "sample time register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`smpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Smpr1Spec;
impl crate::RegisterSpec for Smpr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr1::R`](R) reader structure"]
impl crate::Readable for Smpr1Spec {}
#[doc = "`write(|w| ..)` method takes [`smpr1::W`](W) writer structure"]
impl crate::Writable for Smpr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMPR1 to value 0"]
impl crate::Resettable for Smpr1Spec {
    const RESET_VALUE: u32 = 0;
}

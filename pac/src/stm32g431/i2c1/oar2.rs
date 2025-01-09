#[doc = "Register `OAR2` reader"]
pub type R = crate::R<Oar2Spec>;
#[doc = "Register `OAR2` writer"]
pub type W = crate::W<Oar2Spec>;
#[doc = "Field `OA2` reader - Interface address"]
pub type Oa2R = crate::FieldReader;
#[doc = "Field `OA2` writer - Interface address"]
pub type Oa2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `OA2MSK` reader - Own Address 2 masks"]
pub type Oa2mskR = crate::FieldReader;
#[doc = "Field `OA2MSK` writer - Own Address 2 masks"]
pub type Oa2mskW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OA2EN` reader - Own Address 2 enable"]
pub type Oa2enR = crate::BitReader;
#[doc = "Field `OA2EN` writer - Own Address 2 enable"]
pub type Oa2enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn oa2(&self) -> Oa2R {
        Oa2R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - Own Address 2 masks"]
    #[inline(always)]
    pub fn oa2msk(&self) -> Oa2mskR {
        Oa2mskR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Own Address 2 enable"]
    #[inline(always)]
    pub fn oa2en(&self) -> Oa2enR {
        Oa2enR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OAR2")
            .field("oa2", &self.oa2())
            .field("oa2msk", &self.oa2msk())
            .field("oa2en", &self.oa2en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn oa2(&mut self) -> Oa2W<Oar2Spec> {
        Oa2W::new(self, 1)
    }
    #[doc = "Bits 8:10 - Own Address 2 masks"]
    #[inline(always)]
    pub fn oa2msk(&mut self) -> Oa2mskW<Oar2Spec> {
        Oa2mskW::new(self, 8)
    }
    #[doc = "Bit 15 - Own Address 2 enable"]
    #[inline(always)]
    pub fn oa2en(&mut self) -> Oa2enW<Oar2Spec> {
        Oa2enW::new(self, 15)
    }
}
#[doc = "Own address register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`oar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oar2Spec;
impl crate::RegisterSpec for Oar2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oar2::R`](R) reader structure"]
impl crate::Readable for Oar2Spec {}
#[doc = "`write(|w| ..)` method takes [`oar2::W`](W) writer structure"]
impl crate::Writable for Oar2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OAR2 to value 0"]
impl crate::Resettable for Oar2Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DTR2` reader"]
pub type R = crate::R<Dtr2Spec>;
#[doc = "Register `DTR2` writer"]
pub type W = crate::W<Dtr2Spec>;
#[doc = "Field `DTGF` reader - Dead-time generator setup"]
pub type DtgfR = crate::FieldReader;
#[doc = "Field `DTGF` writer - Dead-time generator setup"]
pub type DtgfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DTAE` reader - Deadtime Asymmetric Enable"]
pub type DtaeR = crate::BitReader;
#[doc = "Field `DTAE` writer - Deadtime Asymmetric Enable"]
pub type DtaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTPE` reader - Deadtime Preload Enable"]
pub type DtpeR = crate::BitReader;
#[doc = "Field `DTPE` writer - Deadtime Preload Enable"]
pub type DtpeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    pub fn dtgf(&self) -> DtgfR {
        DtgfR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Deadtime Asymmetric Enable"]
    #[inline(always)]
    pub fn dtae(&self) -> DtaeR {
        DtaeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Deadtime Preload Enable"]
    #[inline(always)]
    pub fn dtpe(&self) -> DtpeR {
        DtpeR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTR2")
            .field("dtgf", &self.dtgf())
            .field("dtae", &self.dtae())
            .field("dtpe", &self.dtpe())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    pub fn dtgf(&mut self) -> DtgfW<'_, Dtr2Spec> {
        DtgfW::new(self, 0)
    }
    #[doc = "Bit 16 - Deadtime Asymmetric Enable"]
    #[inline(always)]
    pub fn dtae(&mut self) -> DtaeW<'_, Dtr2Spec> {
        DtaeW::new(self, 16)
    }
    #[doc = "Bit 17 - Deadtime Preload Enable"]
    #[inline(always)]
    pub fn dtpe(&mut self) -> DtpeW<'_, Dtr2Spec> {
        DtpeW::new(self, 17)
    }
}
#[doc = "timer Deadtime Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dtr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dtr2Spec;
impl crate::RegisterSpec for Dtr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtr2::R`](R) reader structure"]
impl crate::Readable for Dtr2Spec {}
#[doc = "`write(|w| ..)` method takes [`dtr2::W`](W) writer structure"]
impl crate::Writable for Dtr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTR2 to value 0"]
impl crate::Resettable for Dtr2Spec {}

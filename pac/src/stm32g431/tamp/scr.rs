#[doc = "Register `SCR` reader"]
pub type R = crate::R<ScrSpec>;
#[doc = "Register `SCR` writer"]
pub type W = crate::W<ScrSpec>;
#[doc = "Field `CTAMP1F` reader - CTAMP1F"]
pub type Ctamp1fR = crate::BitReader;
#[doc = "Field `CTAMP1F` writer - CTAMP1F"]
pub type Ctamp1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTAMP2F` reader - CTAMP2F"]
pub type Ctamp2fR = crate::BitReader;
#[doc = "Field `CTAMP2F` writer - CTAMP2F"]
pub type Ctamp2fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTAMP3F` reader - CTAMP3F"]
pub type Ctamp3fR = crate::BitReader;
#[doc = "Field `CTAMP3F` writer - CTAMP3F"]
pub type Ctamp3fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP3F` reader - CITAMP3F"]
pub type Citamp3fR = crate::BitReader;
#[doc = "Field `CITAMP3F` writer - CITAMP3F"]
pub type Citamp3fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP4F` reader - CITAMP4F"]
pub type Citamp4fR = crate::BitReader;
#[doc = "Field `CITAMP4F` writer - CITAMP4F"]
pub type Citamp4fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP5F` reader - CITAMP5F"]
pub type Citamp5fR = crate::BitReader;
#[doc = "Field `CITAMP5F` writer - CITAMP5F"]
pub type Citamp5fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP6F` reader - CITAMP6F"]
pub type Citamp6fR = crate::BitReader;
#[doc = "Field `CITAMP6F` writer - CITAMP6F"]
pub type Citamp6fW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CTAMP1F"]
    #[inline(always)]
    pub fn ctamp1f(&self) -> Ctamp1fR {
        Ctamp1fR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTAMP2F"]
    #[inline(always)]
    pub fn ctamp2f(&self) -> Ctamp2fR {
        Ctamp2fR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTAMP3F"]
    #[inline(always)]
    pub fn ctamp3f(&self) -> Ctamp3fR {
        Ctamp3fR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - CITAMP3F"]
    #[inline(always)]
    pub fn citamp3f(&self) -> Citamp3fR {
        Citamp3fR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CITAMP4F"]
    #[inline(always)]
    pub fn citamp4f(&self) -> Citamp4fR {
        Citamp4fR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CITAMP5F"]
    #[inline(always)]
    pub fn citamp5f(&self) -> Citamp5fR {
        Citamp5fR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CITAMP6F"]
    #[inline(always)]
    pub fn citamp6f(&self) -> Citamp6fR {
        Citamp6fR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR")
            .field("ctamp1f", &self.ctamp1f())
            .field("ctamp2f", &self.ctamp2f())
            .field("ctamp3f", &self.ctamp3f())
            .field("citamp3f", &self.citamp3f())
            .field("citamp4f", &self.citamp4f())
            .field("citamp5f", &self.citamp5f())
            .field("citamp6f", &self.citamp6f())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CTAMP1F"]
    #[inline(always)]
    pub fn ctamp1f(&mut self) -> Ctamp1fW<'_, ScrSpec> {
        Ctamp1fW::new(self, 0)
    }
    #[doc = "Bit 1 - CTAMP2F"]
    #[inline(always)]
    pub fn ctamp2f(&mut self) -> Ctamp2fW<'_, ScrSpec> {
        Ctamp2fW::new(self, 1)
    }
    #[doc = "Bit 2 - CTAMP3F"]
    #[inline(always)]
    pub fn ctamp3f(&mut self) -> Ctamp3fW<'_, ScrSpec> {
        Ctamp3fW::new(self, 2)
    }
    #[doc = "Bit 18 - CITAMP3F"]
    #[inline(always)]
    pub fn citamp3f(&mut self) -> Citamp3fW<'_, ScrSpec> {
        Citamp3fW::new(self, 18)
    }
    #[doc = "Bit 19 - CITAMP4F"]
    #[inline(always)]
    pub fn citamp4f(&mut self) -> Citamp4fW<'_, ScrSpec> {
        Citamp4fW::new(self, 19)
    }
    #[doc = "Bit 20 - CITAMP5F"]
    #[inline(always)]
    pub fn citamp5f(&mut self) -> Citamp5fW<'_, ScrSpec> {
        Citamp5fW::new(self, 20)
    }
    #[doc = "Bit 21 - CITAMP6F"]
    #[inline(always)]
    pub fn citamp6f(&mut self) -> Citamp6fW<'_, ScrSpec> {
        Citamp6fW::new(self, 21)
    }
}
#[doc = "TAMP status clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrSpec;
impl crate::RegisterSpec for ScrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for ScrSpec {}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for ScrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for ScrSpec {}

#[doc = "Register `AFRCR` reader"]
pub type R = crate::R<AfrcrSpec>;
#[doc = "Register `AFRCR` writer"]
pub type W = crate::W<AfrcrSpec>;
#[doc = "Field `FRL` reader - Frame length"]
pub type FrlR = crate::FieldReader;
#[doc = "Field `FRL` writer - Frame length"]
pub type FrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FSALL` reader - Frame synchronization active level length"]
pub type FsallR = crate::FieldReader;
#[doc = "Field `FSALL` writer - Frame synchronization active level length"]
pub type FsallW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `FSDEF` reader - Frame synchronization definition"]
pub type FsdefR = crate::BitReader;
#[doc = "Field `FSDEF` writer - Frame synchronization definition"]
pub type FsdefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSPOL` reader - Frame synchronization polarity"]
pub type FspolR = crate::BitReader;
#[doc = "Field `FSPOL` writer - Frame synchronization polarity"]
pub type FspolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSOFF` reader - Frame synchronization offset"]
pub type FsoffR = crate::BitReader;
#[doc = "Field `FSOFF` writer - Frame synchronization offset"]
pub type FsoffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Frame length"]
    #[inline(always)]
    pub fn frl(&self) -> FrlR {
        FrlR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Frame synchronization active level length"]
    #[inline(always)]
    pub fn fsall(&self) -> FsallR {
        FsallR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Frame synchronization definition"]
    #[inline(always)]
    pub fn fsdef(&self) -> FsdefR {
        FsdefR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Frame synchronization polarity"]
    #[inline(always)]
    pub fn fspol(&self) -> FspolR {
        FspolR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Frame synchronization offset"]
    #[inline(always)]
    pub fn fsoff(&self) -> FsoffR {
        FsoffR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFRCR")
            .field("fsoff", &self.fsoff())
            .field("fspol", &self.fspol())
            .field("fsdef", &self.fsdef())
            .field("fsall", &self.fsall())
            .field("frl", &self.frl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame length"]
    #[inline(always)]
    pub fn frl(&mut self) -> FrlW<AfrcrSpec> {
        FrlW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Frame synchronization active level length"]
    #[inline(always)]
    pub fn fsall(&mut self) -> FsallW<AfrcrSpec> {
        FsallW::new(self, 8)
    }
    #[doc = "Bit 16 - Frame synchronization definition"]
    #[inline(always)]
    pub fn fsdef(&mut self) -> FsdefW<AfrcrSpec> {
        FsdefW::new(self, 16)
    }
    #[doc = "Bit 17 - Frame synchronization polarity"]
    #[inline(always)]
    pub fn fspol(&mut self) -> FspolW<AfrcrSpec> {
        FspolW::new(self, 17)
    }
    #[doc = "Bit 18 - Frame synchronization offset"]
    #[inline(always)]
    pub fn fsoff(&mut self) -> FsoffW<AfrcrSpec> {
        FsoffW::new(self, 18)
    }
}
#[doc = "AFRCR\n\nYou can [`read`](crate::Reg::read) this register and get [`afrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfrcrSpec;
impl crate::RegisterSpec for AfrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrcr::R`](R) reader structure"]
impl crate::Readable for AfrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`afrcr::W`](W) writer structure"]
impl crate::Writable for AfrcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFRCR to value 0x07"]
impl crate::Resettable for AfrcrSpec {
    const RESET_VALUE: u32 = 0x07;
}

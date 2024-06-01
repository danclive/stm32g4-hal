#[doc = "Register `MISR` reader"]
pub type R = crate::R<MisrSpec>;
#[doc = "Field `MCMP1` reader - Master Compare 1 Interrupt Flag"]
pub type Mcmp1R = crate::BitReader;
#[doc = "Field `MCMP2` reader - Master Compare 2 Interrupt Flag"]
pub type Mcmp2R = crate::BitReader;
#[doc = "Field `MCMP3` reader - Master Compare 3 Interrupt Flag"]
pub type Mcmp3R = crate::BitReader;
#[doc = "Field `MCMP4` reader - Master Compare 4 Interrupt Flag"]
pub type Mcmp4R = crate::BitReader;
#[doc = "Field `MREP` reader - Master Repetition Interrupt Flag"]
pub type MrepR = crate::BitReader;
#[doc = "Field `SYNC` reader - Sync Input Interrupt Flag"]
pub type SyncR = crate::BitReader;
#[doc = "Field `MUPD` reader - Master Update Interrupt Flag"]
pub type MupdR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Master Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp1(&self) -> Mcmp1R {
        Mcmp1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp2(&self) -> Mcmp2R {
        Mcmp2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master Compare 3 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp3(&self) -> Mcmp3R {
        Mcmp3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Compare 4 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp4(&self) -> Mcmp4R {
        Mcmp4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master Repetition Interrupt Flag"]
    #[inline(always)]
    pub fn mrep(&self) -> MrepR {
        MrepR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sync Input Interrupt Flag"]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Master Update Interrupt Flag"]
    #[inline(always)]
    pub fn mupd(&self) -> MupdR {
        MupdR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISR")
            .field("mupd", &self.mupd())
            .field("sync", &self.sync())
            .field("mrep", &self.mrep())
            .field("mcmp4", &self.mcmp4())
            .field("mcmp3", &self.mcmp3())
            .field("mcmp2", &self.mcmp2())
            .field("mcmp1", &self.mcmp1())
            .finish()
    }
}
#[doc = "Master Timer Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisrSpec;
impl crate::RegisterSpec for MisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misr::R`](R) reader structure"]
impl crate::Readable for MisrSpec {}
#[doc = "`reset()` method sets MISR to value 0"]
impl crate::Resettable for MisrSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `BDMUPDR` reader"]
pub type R = crate::R<BdmupdrSpec>;
#[doc = "Register `BDMUPDR` writer"]
pub type W = crate::W<BdmupdrSpec>;
#[doc = "Field `MCR` reader - MCR"]
pub type McrR = crate::BitReader;
#[doc = "Field `MCR` writer - MCR"]
pub type McrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MICR` reader - MICR"]
pub type MicrR = crate::BitReader;
#[doc = "Field `MICR` writer - MICR"]
pub type MicrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDIER` reader - MDIER"]
pub type MdierR = crate::BitReader;
#[doc = "Field `MDIER` writer - MDIER"]
pub type MdierW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCNT` reader - MCNT"]
pub type McntR = crate::BitReader;
#[doc = "Field `MCNT` writer - MCNT"]
pub type McntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPER` reader - MPER"]
pub type MperR = crate::BitReader;
#[doc = "Field `MPER` writer - MPER"]
pub type MperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MREP` reader - MREP"]
pub type MrepR = crate::BitReader;
#[doc = "Field `MREP` writer - MREP"]
pub type MrepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP1` reader - MCMP1"]
pub type Mcmp1R = crate::BitReader;
#[doc = "Field `MCMP1` writer - MCMP1"]
pub type Mcmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP2` reader - MCMP2"]
pub type Mcmp2R = crate::BitReader;
#[doc = "Field `MCMP2` writer - MCMP2"]
pub type Mcmp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP3` reader - MCMP3"]
pub type Mcmp3R = crate::BitReader;
#[doc = "Field `MCMP3` writer - MCMP3"]
pub type Mcmp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP4` reader - MCMP4"]
pub type Mcmp4R = crate::BitReader;
#[doc = "Field `MCMP4` writer - MCMP4"]
pub type Mcmp4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MCR"]
    #[inline(always)]
    pub fn mcr(&self) -> McrR {
        McrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MICR"]
    #[inline(always)]
    pub fn micr(&self) -> MicrR {
        MicrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MDIER"]
    #[inline(always)]
    pub fn mdier(&self) -> MdierR {
        MdierR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MCNT"]
    #[inline(always)]
    pub fn mcnt(&self) -> McntR {
        McntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MPER"]
    #[inline(always)]
    pub fn mper(&self) -> MperR {
        MperR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MREP"]
    #[inline(always)]
    pub fn mrep(&self) -> MrepR {
        MrepR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MCMP1"]
    #[inline(always)]
    pub fn mcmp1(&self) -> Mcmp1R {
        Mcmp1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MCMP2"]
    #[inline(always)]
    pub fn mcmp2(&self) -> Mcmp2R {
        Mcmp2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MCMP3"]
    #[inline(always)]
    pub fn mcmp3(&self) -> Mcmp3R {
        Mcmp3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MCMP4"]
    #[inline(always)]
    pub fn mcmp4(&self) -> Mcmp4R {
        Mcmp4R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDMUPDR")
            .field("mcmp4", &self.mcmp4())
            .field("mcmp3", &self.mcmp3())
            .field("mcmp2", &self.mcmp2())
            .field("mcmp1", &self.mcmp1())
            .field("mrep", &self.mrep())
            .field("mper", &self.mper())
            .field("mcnt", &self.mcnt())
            .field("mdier", &self.mdier())
            .field("micr", &self.micr())
            .field("mcr", &self.mcr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - MCR"]
    #[inline(always)]
    #[must_use]
    pub fn mcr(&mut self) -> McrW<BdmupdrSpec> {
        McrW::new(self, 0)
    }
    #[doc = "Bit 1 - MICR"]
    #[inline(always)]
    #[must_use]
    pub fn micr(&mut self) -> MicrW<BdmupdrSpec> {
        MicrW::new(self, 1)
    }
    #[doc = "Bit 2 - MDIER"]
    #[inline(always)]
    #[must_use]
    pub fn mdier(&mut self) -> MdierW<BdmupdrSpec> {
        MdierW::new(self, 2)
    }
    #[doc = "Bit 3 - MCNT"]
    #[inline(always)]
    #[must_use]
    pub fn mcnt(&mut self) -> McntW<BdmupdrSpec> {
        McntW::new(self, 3)
    }
    #[doc = "Bit 4 - MPER"]
    #[inline(always)]
    #[must_use]
    pub fn mper(&mut self) -> MperW<BdmupdrSpec> {
        MperW::new(self, 4)
    }
    #[doc = "Bit 5 - MREP"]
    #[inline(always)]
    #[must_use]
    pub fn mrep(&mut self) -> MrepW<BdmupdrSpec> {
        MrepW::new(self, 5)
    }
    #[doc = "Bit 6 - MCMP1"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp1(&mut self) -> Mcmp1W<BdmupdrSpec> {
        Mcmp1W::new(self, 6)
    }
    #[doc = "Bit 7 - MCMP2"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp2(&mut self) -> Mcmp2W<BdmupdrSpec> {
        Mcmp2W::new(self, 7)
    }
    #[doc = "Bit 8 - MCMP3"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp3(&mut self) -> Mcmp3W<BdmupdrSpec> {
        Mcmp3W::new(self, 8)
    }
    #[doc = "Bit 9 - MCMP4"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp4(&mut self) -> Mcmp4W<BdmupdrSpec> {
        Mcmp4W::new(self, 9)
    }
}
#[doc = "BDMUPDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdmupdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdmupdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdmupdrSpec;
impl crate::RegisterSpec for BdmupdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdmupdr::R`](R) reader structure"]
impl crate::Readable for BdmupdrSpec {}
#[doc = "`write(|w| ..)` method takes [`bdmupdr::W`](W) writer structure"]
impl crate::Writable for BdmupdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BDMUPDR to value 0"]
impl crate::Resettable for BdmupdrSpec {
    const RESET_VALUE: u32 = 0;
}

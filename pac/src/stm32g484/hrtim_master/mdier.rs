#[doc = "Register `MDIER` reader"]
pub type R = crate::R<MdierSpec>;
#[doc = "Register `MDIER` writer"]
pub type W = crate::W<MdierSpec>;
#[doc = "Field `MCMP1IE` reader - MCMP1IE"]
pub type Mcmp1ieR = crate::BitReader;
#[doc = "Field `MCMP1IE` writer - MCMP1IE"]
pub type Mcmp1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP2IE` reader - MCMP2IE"]
pub type Mcmp2ieR = crate::BitReader;
#[doc = "Field `MCMP2IE` writer - MCMP2IE"]
pub type Mcmp2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP3IE` reader - MCMP3IE"]
pub type Mcmp3ieR = crate::BitReader;
#[doc = "Field `MCMP3IE` writer - MCMP3IE"]
pub type Mcmp3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP4IE` reader - MCMP4IE"]
pub type Mcmp4ieR = crate::BitReader;
#[doc = "Field `MCMP4IE` writer - MCMP4IE"]
pub type Mcmp4ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MREPIE` reader - MREPIE"]
pub type MrepieR = crate::BitReader;
#[doc = "Field `MREPIE` writer - MREPIE"]
pub type MrepieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCIE` reader - SYNCIE"]
pub type SyncieR = crate::BitReader;
#[doc = "Field `SYNCIE` writer - SYNCIE"]
pub type SyncieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUPDIE` reader - MUPDIE"]
pub type MupdieR = crate::BitReader;
#[doc = "Field `MUPDIE` writer - MUPDIE"]
pub type MupdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP1DE` reader - MCMP1DE"]
pub type Mcmp1deR = crate::BitReader;
#[doc = "Field `MCMP1DE` writer - MCMP1DE"]
pub type Mcmp1deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP2DE` reader - MCMP2DE"]
pub type Mcmp2deR = crate::BitReader;
#[doc = "Field `MCMP2DE` writer - MCMP2DE"]
pub type Mcmp2deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP3DE` reader - MCMP3DE"]
pub type Mcmp3deR = crate::BitReader;
#[doc = "Field `MCMP3DE` writer - MCMP3DE"]
pub type Mcmp3deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP4DE` reader - MCMP4DE"]
pub type Mcmp4deR = crate::BitReader;
#[doc = "Field `MCMP4DE` writer - MCMP4DE"]
pub type Mcmp4deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MREPDE` reader - MREPDE"]
pub type MrepdeR = crate::BitReader;
#[doc = "Field `MREPDE` writer - MREPDE"]
pub type MrepdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCDE` reader - SYNCDE"]
pub type SyncdeR = crate::BitReader;
#[doc = "Field `SYNCDE` writer - SYNCDE"]
pub type SyncdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUPDDE` reader - MUPDDE"]
pub type MupddeR = crate::BitReader;
#[doc = "Field `MUPDDE` writer - MUPDDE"]
pub type MupddeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MCMP1IE"]
    #[inline(always)]
    pub fn mcmp1ie(&self) -> Mcmp1ieR {
        Mcmp1ieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCMP2IE"]
    #[inline(always)]
    pub fn mcmp2ie(&self) -> Mcmp2ieR {
        Mcmp2ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MCMP3IE"]
    #[inline(always)]
    pub fn mcmp3ie(&self) -> Mcmp3ieR {
        Mcmp3ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MCMP4IE"]
    #[inline(always)]
    pub fn mcmp4ie(&self) -> Mcmp4ieR {
        Mcmp4ieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MREPIE"]
    #[inline(always)]
    pub fn mrepie(&self) -> MrepieR {
        MrepieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SYNCIE"]
    #[inline(always)]
    pub fn syncie(&self) -> SyncieR {
        SyncieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MUPDIE"]
    #[inline(always)]
    pub fn mupdie(&self) -> MupdieR {
        MupdieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - MCMP1DE"]
    #[inline(always)]
    pub fn mcmp1de(&self) -> Mcmp1deR {
        Mcmp1deR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MCMP2DE"]
    #[inline(always)]
    pub fn mcmp2de(&self) -> Mcmp2deR {
        Mcmp2deR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MCMP3DE"]
    #[inline(always)]
    pub fn mcmp3de(&self) -> Mcmp3deR {
        Mcmp3deR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MCMP4DE"]
    #[inline(always)]
    pub fn mcmp4de(&self) -> Mcmp4deR {
        Mcmp4deR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MREPDE"]
    #[inline(always)]
    pub fn mrepde(&self) -> MrepdeR {
        MrepdeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SYNCDE"]
    #[inline(always)]
    pub fn syncde(&self) -> SyncdeR {
        SyncdeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MUPDDE"]
    #[inline(always)]
    pub fn mupdde(&self) -> MupddeR {
        MupddeR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDIER")
            .field("mupdde", &self.mupdde())
            .field("syncde", &self.syncde())
            .field("mrepde", &self.mrepde())
            .field("mcmp4de", &self.mcmp4de())
            .field("mcmp3de", &self.mcmp3de())
            .field("mcmp2de", &self.mcmp2de())
            .field("mcmp1de", &self.mcmp1de())
            .field("mupdie", &self.mupdie())
            .field("syncie", &self.syncie())
            .field("mrepie", &self.mrepie())
            .field("mcmp4ie", &self.mcmp4ie())
            .field("mcmp3ie", &self.mcmp3ie())
            .field("mcmp2ie", &self.mcmp2ie())
            .field("mcmp1ie", &self.mcmp1ie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - MCMP1IE"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp1ie(&mut self) -> Mcmp1ieW<MdierSpec> {
        Mcmp1ieW::new(self, 0)
    }
    #[doc = "Bit 1 - MCMP2IE"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp2ie(&mut self) -> Mcmp2ieW<MdierSpec> {
        Mcmp2ieW::new(self, 1)
    }
    #[doc = "Bit 2 - MCMP3IE"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp3ie(&mut self) -> Mcmp3ieW<MdierSpec> {
        Mcmp3ieW::new(self, 2)
    }
    #[doc = "Bit 3 - MCMP4IE"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp4ie(&mut self) -> Mcmp4ieW<MdierSpec> {
        Mcmp4ieW::new(self, 3)
    }
    #[doc = "Bit 4 - MREPIE"]
    #[inline(always)]
    #[must_use]
    pub fn mrepie(&mut self) -> MrepieW<MdierSpec> {
        MrepieW::new(self, 4)
    }
    #[doc = "Bit 5 - SYNCIE"]
    #[inline(always)]
    #[must_use]
    pub fn syncie(&mut self) -> SyncieW<MdierSpec> {
        SyncieW::new(self, 5)
    }
    #[doc = "Bit 6 - MUPDIE"]
    #[inline(always)]
    #[must_use]
    pub fn mupdie(&mut self) -> MupdieW<MdierSpec> {
        MupdieW::new(self, 6)
    }
    #[doc = "Bit 16 - MCMP1DE"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp1de(&mut self) -> Mcmp1deW<MdierSpec> {
        Mcmp1deW::new(self, 16)
    }
    #[doc = "Bit 17 - MCMP2DE"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp2de(&mut self) -> Mcmp2deW<MdierSpec> {
        Mcmp2deW::new(self, 17)
    }
    #[doc = "Bit 18 - MCMP3DE"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp3de(&mut self) -> Mcmp3deW<MdierSpec> {
        Mcmp3deW::new(self, 18)
    }
    #[doc = "Bit 19 - MCMP4DE"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp4de(&mut self) -> Mcmp4deW<MdierSpec> {
        Mcmp4deW::new(self, 19)
    }
    #[doc = "Bit 20 - MREPDE"]
    #[inline(always)]
    #[must_use]
    pub fn mrepde(&mut self) -> MrepdeW<MdierSpec> {
        MrepdeW::new(self, 20)
    }
    #[doc = "Bit 21 - SYNCDE"]
    #[inline(always)]
    #[must_use]
    pub fn syncde(&mut self) -> SyncdeW<MdierSpec> {
        SyncdeW::new(self, 21)
    }
    #[doc = "Bit 22 - MUPDDE"]
    #[inline(always)]
    #[must_use]
    pub fn mupdde(&mut self) -> MupddeW<MdierSpec> {
        MupddeW::new(self, 22)
    }
}
#[doc = "HRTIM Master Timer DMA / Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdierSpec;
impl crate::RegisterSpec for MdierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdier::R`](R) reader structure"]
impl crate::Readable for MdierSpec {}
#[doc = "`write(|w| ..)` method takes [`mdier::W`](W) writer structure"]
impl crate::Writable for MdierSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIER to value 0"]
impl crate::Resettable for MdierSpec {
    const RESET_VALUE: u32 = 0;
}

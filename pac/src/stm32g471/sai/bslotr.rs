#[doc = "Register `BSLOTR` reader"]
pub type R = crate::R<BslotrSpec>;
#[doc = "Register `BSLOTR` writer"]
pub type W = crate::W<BslotrSpec>;
#[doc = "Field `FBOFF` reader - First bit offset"]
pub type FboffR = crate::FieldReader;
#[doc = "Field `FBOFF` writer - First bit offset"]
pub type FboffW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SLOTSZ` reader - Slot size"]
pub type SlotszR = crate::FieldReader;
#[doc = "Field `SLOTSZ` writer - Slot size"]
pub type SlotszW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NBSLOT` reader - Number of slots in an audio frame"]
pub type NbslotR = crate::FieldReader;
#[doc = "Field `NBSLOT` writer - Number of slots in an audio frame"]
pub type NbslotW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SLOTEN` reader - Slot enable"]
pub type SlotenR = crate::FieldReader<u16>;
#[doc = "Field `SLOTEN` writer - Slot enable"]
pub type SlotenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - First bit offset"]
    #[inline(always)]
    pub fn fboff(&self) -> FboffR {
        FboffR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - Slot size"]
    #[inline(always)]
    pub fn slotsz(&self) -> SlotszR {
        SlotszR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Number of slots in an audio frame"]
    #[inline(always)]
    pub fn nbslot(&self) -> NbslotR {
        NbslotR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Slot enable"]
    #[inline(always)]
    pub fn sloten(&self) -> SlotenR {
        SlotenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BSLOTR")
            .field("sloten", &self.sloten())
            .field("nbslot", &self.nbslot())
            .field("slotsz", &self.slotsz())
            .field("fboff", &self.fboff())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - First bit offset"]
    #[inline(always)]
    #[must_use]
    pub fn fboff(&mut self) -> FboffW<BslotrSpec> {
        FboffW::new(self, 0)
    }
    #[doc = "Bits 6:7 - Slot size"]
    #[inline(always)]
    #[must_use]
    pub fn slotsz(&mut self) -> SlotszW<BslotrSpec> {
        SlotszW::new(self, 6)
    }
    #[doc = "Bits 8:11 - Number of slots in an audio frame"]
    #[inline(always)]
    #[must_use]
    pub fn nbslot(&mut self) -> NbslotW<BslotrSpec> {
        NbslotW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Slot enable"]
    #[inline(always)]
    #[must_use]
    pub fn sloten(&mut self) -> SlotenW<BslotrSpec> {
        SlotenW::new(self, 16)
    }
}
#[doc = "BSlot register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bslotr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bslotr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BslotrSpec;
impl crate::RegisterSpec for BslotrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bslotr::R`](R) reader structure"]
impl crate::Readable for BslotrSpec {}
#[doc = "`write(|w| ..)` method takes [`bslotr::W`](W) writer structure"]
impl crate::Writable for BslotrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BSLOTR to value 0"]
impl crate::Resettable for BslotrSpec {
    const RESET_VALUE: u32 = 0;
}

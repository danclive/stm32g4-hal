#[doc = "Register `ASLOTR` reader"]
pub type R = crate::R<AslotrSpec>;
#[doc = "Register `ASLOTR` writer"]
pub type W = crate::W<AslotrSpec>;
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
        f.debug_struct("ASLOTR")
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
    pub fn fboff(&mut self) -> FboffW<'_, AslotrSpec> {
        FboffW::new(self, 0)
    }
    #[doc = "Bits 6:7 - Slot size"]
    #[inline(always)]
    pub fn slotsz(&mut self) -> SlotszW<'_, AslotrSpec> {
        SlotszW::new(self, 6)
    }
    #[doc = "Bits 8:11 - Number of slots in an audio frame"]
    #[inline(always)]
    pub fn nbslot(&mut self) -> NbslotW<'_, AslotrSpec> {
        NbslotW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Slot enable"]
    #[inline(always)]
    pub fn sloten(&mut self) -> SlotenW<'_, AslotrSpec> {
        SlotenW::new(self, 16)
    }
}
#[doc = "ASlot register\n\nYou can [`read`](crate::Reg::read) this register and get [`aslotr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aslotr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AslotrSpec;
impl crate::RegisterSpec for AslotrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aslotr::R`](R) reader structure"]
impl crate::Readable for AslotrSpec {}
#[doc = "`write(|w| ..)` method takes [`aslotr::W`](W) writer structure"]
impl crate::Writable for AslotrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ASLOTR to value 0"]
impl crate::Resettable for AslotrSpec {}

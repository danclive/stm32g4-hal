#[doc = "Register `C13CR` reader"]
pub type R = crate::R<C13crSpec>;
#[doc = "Register `C13CR` writer"]
pub type W = crate::W<C13crSpec>;
#[doc = "Field `DMAREQ_ID` reader - Input DMA request line selected"]
pub type DmareqIdR = crate::FieldReader;
#[doc = "Field `DMAREQ_ID` writer - Input DMA request line selected"]
pub type DmareqIdW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SOIE` reader - Interrupt enable at synchronization event overrun"]
pub type SoieR = crate::BitReader;
#[doc = "Field `SOIE` writer - Interrupt enable at synchronization event overrun"]
pub type SoieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EGE` reader - Event generation enable/disable"]
pub type EgeR = crate::BitReader;
#[doc = "Field `EGE` writer - Event generation enable/disable"]
pub type EgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE` reader - Synchronous operating mode enable/disable"]
pub type SeR = crate::BitReader;
#[doc = "Field `SE` writer - Synchronous operating mode enable/disable"]
pub type SeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPOL` reader - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
pub type SpolR = crate::FieldReader;
#[doc = "Field `SPOL` writer - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
pub type SpolW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NBREQ` reader - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
pub type NbreqR = crate::FieldReader;
#[doc = "Field `NBREQ` writer - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
pub type NbreqW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SYNC_ID` reader - Synchronization input selected"]
pub type SyncIdR = crate::FieldReader;
#[doc = "Field `SYNC_ID` writer - Synchronization input selected"]
pub type SyncIdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:6 - Input DMA request line selected"]
    #[inline(always)]
    pub fn dmareq_id(&self) -> DmareqIdR {
        DmareqIdR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Interrupt enable at synchronization event overrun"]
    #[inline(always)]
    pub fn soie(&self) -> SoieR {
        SoieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event generation enable/disable"]
    #[inline(always)]
    pub fn ege(&self) -> EgeR {
        EgeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Synchronous operating mode enable/disable"]
    #[inline(always)]
    pub fn se(&self) -> SeR {
        SeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
    #[inline(always)]
    pub fn spol(&self) -> SpolR {
        SpolR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
    #[inline(always)]
    pub fn nbreq(&self) -> NbreqR {
        NbreqR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Synchronization input selected"]
    #[inline(always)]
    pub fn sync_id(&self) -> SyncIdR {
        SyncIdR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C13CR")
            .field("dmareq_id", &self.dmareq_id())
            .field("soie", &self.soie())
            .field("ege", &self.ege())
            .field("se", &self.se())
            .field("spol", &self.spol())
            .field("nbreq", &self.nbreq())
            .field("sync_id", &self.sync_id())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Input DMA request line selected"]
    #[inline(always)]
    pub fn dmareq_id(&mut self) -> DmareqIdW<C13crSpec> {
        DmareqIdW::new(self, 0)
    }
    #[doc = "Bit 8 - Interrupt enable at synchronization event overrun"]
    #[inline(always)]
    pub fn soie(&mut self) -> SoieW<C13crSpec> {
        SoieW::new(self, 8)
    }
    #[doc = "Bit 9 - Event generation enable/disable"]
    #[inline(always)]
    pub fn ege(&mut self) -> EgeW<C13crSpec> {
        EgeW::new(self, 9)
    }
    #[doc = "Bit 16 - Synchronous operating mode enable/disable"]
    #[inline(always)]
    pub fn se(&mut self) -> SeW<C13crSpec> {
        SeW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
    #[inline(always)]
    pub fn spol(&mut self) -> SpolW<C13crSpec> {
        SpolW::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
    #[inline(always)]
    pub fn nbreq(&mut self) -> NbreqW<C13crSpec> {
        NbreqW::new(self, 19)
    }
    #[doc = "Bits 24:28 - Synchronization input selected"]
    #[inline(always)]
    pub fn sync_id(&mut self) -> SyncIdW<C13crSpec> {
        SyncIdW::new(self, 24)
    }
}
#[doc = "DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c13cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C13crSpec;
impl crate::RegisterSpec for C13crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c13cr::R`](R) reader structure"]
impl crate::Readable for C13crSpec {}
#[doc = "`write(|w| ..)` method takes [`c13cr::W`](W) writer structure"]
impl crate::Writable for C13crSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C13CR to value 0"]
impl crate::Resettable for C13crSpec {
    const RESET_VALUE: u32 = 0;
}

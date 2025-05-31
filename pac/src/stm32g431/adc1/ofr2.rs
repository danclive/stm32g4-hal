#[doc = "Register `OFR2` reader"]
pub type R = crate::R<Ofr2Spec>;
#[doc = "Register `OFR2` writer"]
pub type W = crate::W<Ofr2Spec>;
#[doc = "Field `OFFSET1` reader - OFFSET1"]
pub type Offset1R = crate::FieldReader<u16>;
#[doc = "Field `OFFSET1` writer - OFFSET1"]
pub type Offset1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `OFFSETPOS` reader - OFFSETPOS"]
pub type OffsetposR = crate::BitReader;
#[doc = "Field `OFFSETPOS` writer - OFFSETPOS"]
pub type OffsetposW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SATEN` reader - SATEN"]
pub type SatenR = crate::BitReader;
#[doc = "Field `SATEN` writer - SATEN"]
pub type SatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFFSET1_CH` reader - OFFSET1_CH"]
pub type Offset1ChR = crate::FieldReader;
#[doc = "Field `OFFSET1_CH` writer - OFFSET1_CH"]
pub type Offset1ChW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OFFSET1_EN` reader - OFFSET1_EN"]
pub type Offset1EnR = crate::BitReader;
#[doc = "Field `OFFSET1_EN` writer - OFFSET1_EN"]
pub type Offset1EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - OFFSET1"]
    #[inline(always)]
    pub fn offset1(&self) -> Offset1R {
        Offset1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 24 - OFFSETPOS"]
    #[inline(always)]
    pub fn offsetpos(&self) -> OffsetposR {
        OffsetposR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SATEN"]
    #[inline(always)]
    pub fn saten(&self) -> SatenR {
        SatenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - OFFSET1_CH"]
    #[inline(always)]
    pub fn offset1_ch(&self) -> Offset1ChR {
        Offset1ChR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - OFFSET1_EN"]
    #[inline(always)]
    pub fn offset1_en(&self) -> Offset1EnR {
        Offset1EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OFR2")
            .field("offset1_en", &self.offset1_en())
            .field("offset1_ch", &self.offset1_ch())
            .field("saten", &self.saten())
            .field("offsetpos", &self.offsetpos())
            .field("offset1", &self.offset1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - OFFSET1"]
    #[inline(always)]
    pub fn offset1(&mut self) -> Offset1W<Ofr2Spec> {
        Offset1W::new(self, 0)
    }
    #[doc = "Bit 24 - OFFSETPOS"]
    #[inline(always)]
    pub fn offsetpos(&mut self) -> OffsetposW<Ofr2Spec> {
        OffsetposW::new(self, 24)
    }
    #[doc = "Bit 25 - SATEN"]
    #[inline(always)]
    pub fn saten(&mut self) -> SatenW<Ofr2Spec> {
        SatenW::new(self, 25)
    }
    #[doc = "Bits 26:30 - OFFSET1_CH"]
    #[inline(always)]
    pub fn offset1_ch(&mut self) -> Offset1ChW<Ofr2Spec> {
        Offset1ChW::new(self, 26)
    }
    #[doc = "Bit 31 - OFFSET1_EN"]
    #[inline(always)]
    pub fn offset1_en(&mut self) -> Offset1EnW<Ofr2Spec> {
        Offset1EnW::new(self, 31)
    }
}
#[doc = "offset register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ofr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ofr2Spec;
impl crate::RegisterSpec for Ofr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ofr2::R`](R) reader structure"]
impl crate::Readable for Ofr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ofr2::W`](W) writer structure"]
impl crate::Writable for Ofr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OFR2 to value 0"]
impl crate::Resettable for Ofr2Spec {}

#[doc = "Register `NBTP` reader"]
pub type R = crate::R<NbtpSpec>;
#[doc = "Register `NBTP` writer"]
pub type W = crate::W<NbtpSpec>;
#[doc = "Field `NTSEG2` reader - NTSEG2"]
pub type Ntseg2R = crate::FieldReader;
#[doc = "Field `NTSEG2` writer - NTSEG2"]
pub type Ntseg2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NTSEG1` reader - NTSEG1"]
pub type Ntseg1R = crate::FieldReader;
#[doc = "Field `NTSEG1` writer - NTSEG1"]
pub type Ntseg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NBRP` reader - NBRP"]
pub type NbrpR = crate::FieldReader<u16>;
#[doc = "Field `NBRP` writer - NBRP"]
pub type NbrpW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `NSJW` reader - NSJW"]
pub type NsjwR = crate::FieldReader;
#[doc = "Field `NSJW` writer - NSJW"]
pub type NsjwW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - NTSEG2"]
    #[inline(always)]
    pub fn ntseg2(&self) -> Ntseg2R {
        Ntseg2R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - NTSEG1"]
    #[inline(always)]
    pub fn ntseg1(&self) -> Ntseg1R {
        Ntseg1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24 - NBRP"]
    #[inline(always)]
    pub fn nbrp(&self) -> NbrpR {
        NbrpR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - NSJW"]
    #[inline(always)]
    pub fn nsjw(&self) -> NsjwR {
        NsjwR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NBTP")
            .field("ntseg2", &self.ntseg2())
            .field("ntseg1", &self.ntseg1())
            .field("nbrp", &self.nbrp())
            .field("nsjw", &self.nsjw())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - NTSEG2"]
    #[inline(always)]
    #[must_use]
    pub fn ntseg2(&mut self) -> Ntseg2W<NbtpSpec> {
        Ntseg2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - NTSEG1"]
    #[inline(always)]
    #[must_use]
    pub fn ntseg1(&mut self) -> Ntseg1W<NbtpSpec> {
        Ntseg1W::new(self, 8)
    }
    #[doc = "Bits 16:24 - NBRP"]
    #[inline(always)]
    #[must_use]
    pub fn nbrp(&mut self) -> NbrpW<NbtpSpec> {
        NbrpW::new(self, 16)
    }
    #[doc = "Bits 25:31 - NSJW"]
    #[inline(always)]
    #[must_use]
    pub fn nsjw(&mut self) -> NsjwW<NbtpSpec> {
        NsjwW::new(self, 25)
    }
}
#[doc = "FDCAN_NBTP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nbtp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nbtp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NbtpSpec;
impl crate::RegisterSpec for NbtpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nbtp::R`](R) reader structure"]
impl crate::Readable for NbtpSpec {}
#[doc = "`write(|w| ..)` method takes [`nbtp::W`](W) writer structure"]
impl crate::Writable for NbtpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NBTP to value 0x0600_0a03"]
impl crate::Resettable for NbtpSpec {
    const RESET_VALUE: u32 = 0x0600_0a03;
}

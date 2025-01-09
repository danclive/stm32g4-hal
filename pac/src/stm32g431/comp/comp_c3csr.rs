#[doc = "Register `COMP_C3CSR` reader"]
pub type R = crate::R<CompC3csrSpec>;
#[doc = "Register `COMP_C3CSR` writer"]
pub type W = crate::W<CompC3csrSpec>;
#[doc = "Field `EN` reader - EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INMSEL` reader - INMSEL"]
pub type InmselR = crate::FieldReader;
#[doc = "Field `INMSEL` writer - INMSEL"]
pub type InmselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `INPSEL` reader - INPSEL"]
pub type InpselR = crate::BitReader;
#[doc = "Field `INPSEL` writer - INPSEL"]
pub type InpselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL` reader - POL"]
pub type PolR = crate::BitReader;
#[doc = "Field `POL` writer - POL"]
pub type PolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - HYST"]
pub type HystR = crate::FieldReader;
#[doc = "Field `HYST` writer - HYST"]
pub type HystW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BLANKSEL` reader - BLANKSEL"]
pub type BlankselR = crate::FieldReader;
#[doc = "Field `BLANKSEL` writer - BLANKSEL"]
pub type BlankselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BRGEN` reader - BRGEN"]
pub type BrgenR = crate::BitReader;
#[doc = "Field `BRGEN` writer - BRGEN"]
pub type BrgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCALEN` reader - SCALEN"]
pub type ScalenR = crate::BitReader;
#[doc = "Field `SCALEN` writer - SCALEN"]
pub type ScalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VALUE` reader - VALUE"]
pub type ValueR = crate::BitReader;
#[doc = "Field `LOCK` reader - LOCK"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - LOCK"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - INMSEL"]
    #[inline(always)]
    pub fn inmsel(&self) -> InmselR {
        InmselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - INPSEL"]
    #[inline(always)]
    pub fn inpsel(&self) -> InpselR {
        InpselR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - POL"]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - HYST"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - BLANKSEL"]
    #[inline(always)]
    pub fn blanksel(&self) -> BlankselR {
        BlankselR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bit 22 - BRGEN"]
    #[inline(always)]
    pub fn brgen(&self) -> BrgenR {
        BrgenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SCALEN"]
    #[inline(always)]
    pub fn scalen(&self) -> ScalenR {
        ScalenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - VALUE"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP_C3CSR")
            .field("en", &self.en())
            .field("inmsel", &self.inmsel())
            .field("inpsel", &self.inpsel())
            .field("pol", &self.pol())
            .field("hyst", &self.hyst())
            .field("blanksel", &self.blanksel())
            .field("brgen", &self.brgen())
            .field("scalen", &self.scalen())
            .field("value", &self.value())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CompC3csrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 4:6 - INMSEL"]
    #[inline(always)]
    pub fn inmsel(&mut self) -> InmselW<CompC3csrSpec> {
        InmselW::new(self, 4)
    }
    #[doc = "Bit 8 - INPSEL"]
    #[inline(always)]
    pub fn inpsel(&mut self) -> InpselW<CompC3csrSpec> {
        InpselW::new(self, 8)
    }
    #[doc = "Bit 15 - POL"]
    #[inline(always)]
    pub fn pol(&mut self) -> PolW<CompC3csrSpec> {
        PolW::new(self, 15)
    }
    #[doc = "Bits 16:18 - HYST"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HystW<CompC3csrSpec> {
        HystW::new(self, 16)
    }
    #[doc = "Bits 19:21 - BLANKSEL"]
    #[inline(always)]
    pub fn blanksel(&mut self) -> BlankselW<CompC3csrSpec> {
        BlankselW::new(self, 19)
    }
    #[doc = "Bit 22 - BRGEN"]
    #[inline(always)]
    pub fn brgen(&mut self) -> BrgenW<CompC3csrSpec> {
        BrgenW::new(self, 22)
    }
    #[doc = "Bit 23 - SCALEN"]
    #[inline(always)]
    pub fn scalen(&mut self) -> ScalenW<CompC3csrSpec> {
        ScalenW::new(self, 23)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<CompC3csrSpec> {
        LockW::new(self, 31)
    }
}
#[doc = "Comparator control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_c3csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_c3csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompC3csrSpec;
impl crate::RegisterSpec for CompC3csrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_c3csr::R`](R) reader structure"]
impl crate::Readable for CompC3csrSpec {}
#[doc = "`write(|w| ..)` method takes [`comp_c3csr::W`](W) writer structure"]
impl crate::Writable for CompC3csrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP_C3CSR to value 0"]
impl crate::Resettable for CompC3csrSpec {
    const RESET_VALUE: u32 = 0;
}

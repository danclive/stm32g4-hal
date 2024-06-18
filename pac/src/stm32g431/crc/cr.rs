#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `RESET` writer - RESET bit"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLYSIZE` reader - Polynomial size"]
pub type PolysizeR = crate::FieldReader;
#[doc = "Field `POLYSIZE` writer - Polynomial size"]
pub type PolysizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REV_IN` reader - Reverse input data"]
pub type RevInR = crate::FieldReader;
#[doc = "Field `REV_IN` writer - Reverse input data"]
pub type RevInW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REV_OUT` reader - Reverse output data"]
pub type RevOutR = crate::BitReader;
#[doc = "Field `REV_OUT` writer - Reverse output data"]
pub type RevOutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 3:4 - Polynomial size"]
    #[inline(always)]
    pub fn polysize(&self) -> PolysizeR {
        PolysizeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Reverse input data"]
    #[inline(always)]
    pub fn rev_in(&self) -> RevInR {
        RevInR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Reverse output data"]
    #[inline(always)]
    pub fn rev_out(&self) -> RevOutR {
        RevOutR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("rev_out", &self.rev_out())
            .field("rev_in", &self.rev_in())
            .field("polysize", &self.polysize())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RESET bit"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<CrSpec> {
        ResetW::new(self, 0)
    }
    #[doc = "Bits 3:4 - Polynomial size"]
    #[inline(always)]
    #[must_use]
    pub fn polysize(&mut self) -> PolysizeW<CrSpec> {
        PolysizeW::new(self, 3)
    }
    #[doc = "Bits 5:6 - Reverse input data"]
    #[inline(always)]
    #[must_use]
    pub fn rev_in(&mut self) -> RevInW<CrSpec> {
        RevInW::new(self, 5)
    }
    #[doc = "Bit 7 - Reverse output data"]
    #[inline(always)]
    #[must_use]
    pub fn rev_out(&mut self) -> RevOutW<CrSpec> {
        RevOutW::new(self, 7)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}

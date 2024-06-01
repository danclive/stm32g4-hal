#[doc = "Register `CMP1CER` reader"]
pub type R = crate::R<Cmp1cerSpec>;
#[doc = "Register `CMP1CER` writer"]
pub type W = crate::W<Cmp1cerSpec>;
#[doc = "Field `CMP1x` reader - Timerx Compare 1 value"]
pub type Cmp1xR = crate::FieldReader<u16>;
#[doc = "Field `CMP1x` writer - Timerx Compare 1 value"]
pub type Cmp1xW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REPx` reader - Timerx Repetition value (aliased from HRTIM_REPx register)"]
pub type RepxR = crate::FieldReader;
#[doc = "Field `REPx` writer - Timerx Repetition value (aliased from HRTIM_REPx register)"]
pub type RepxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 1 value"]
    #[inline(always)]
    pub fn cmp1x(&self) -> Cmp1xR {
        Cmp1xR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Timerx Repetition value (aliased from HRTIM_REPx register)"]
    #[inline(always)]
    pub fn repx(&self) -> RepxR {
        RepxR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMP1CER")
            .field("repx", &self.repx())
            .field("cmp1x", &self.cmp1x())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 1 value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1x(&mut self) -> Cmp1xW<Cmp1cerSpec> {
        Cmp1xW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Timerx Repetition value (aliased from HRTIM_REPx register)"]
    #[inline(always)]
    #[must_use]
    pub fn repx(&mut self) -> RepxW<Cmp1cerSpec> {
        RepxW::new(self, 16)
    }
}
#[doc = "Timerx Compare 1 Compound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1cer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1cer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp1cerSpec;
impl crate::RegisterSpec for Cmp1cerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp1cer::R`](R) reader structure"]
impl crate::Readable for Cmp1cerSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp1cer::W`](W) writer structure"]
impl crate::Writable for Cmp1cerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP1CER to value 0"]
impl crate::Resettable for Cmp1cerSpec {
    const RESET_VALUE: u32 = 0;
}

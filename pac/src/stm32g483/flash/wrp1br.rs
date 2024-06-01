#[doc = "Register `WRP1BR` reader"]
pub type R = crate::R<Wrp1brSpec>;
#[doc = "Register `WRP1BR` writer"]
pub type W = crate::W<Wrp1brSpec>;
#[doc = "Field `WRP1B_STRT` reader - Bank 1 WRP second area B end offset"]
pub type Wrp1bStrtR = crate::FieldReader;
#[doc = "Field `WRP1B_STRT` writer - Bank 1 WRP second area B end offset"]
pub type Wrp1bStrtW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRP1B_END` reader - Bank 1 WRP second area B start offset"]
pub type Wrp1bEndR = crate::FieldReader;
#[doc = "Field `WRP1B_END` writer - Bank 1 WRP second area B start offset"]
pub type Wrp1bEndW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Bank 1 WRP second area B end offset"]
    #[inline(always)]
    pub fn wrp1b_strt(&self) -> Wrp1bStrtR {
        Wrp1bStrtR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Bank 1 WRP second area B start offset"]
    #[inline(always)]
    pub fn wrp1b_end(&self) -> Wrp1bEndR {
        Wrp1bEndR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRP1BR")
            .field("wrp1b_strt", &self.wrp1b_strt())
            .field("wrp1b_end", &self.wrp1b_end())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Bank 1 WRP second area B end offset"]
    #[inline(always)]
    #[must_use]
    pub fn wrp1b_strt(&mut self) -> Wrp1bStrtW<Wrp1brSpec> {
        Wrp1bStrtW::new(self, 0)
    }
    #[doc = "Bits 16:22 - Bank 1 WRP second area B start offset"]
    #[inline(always)]
    #[must_use]
    pub fn wrp1b_end(&mut self) -> Wrp1bEndW<Wrp1brSpec> {
        Wrp1bEndW::new(self, 16)
    }
}
#[doc = "Flash Bank 1 WRP area B address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp1br::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp1br::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wrp1brSpec;
impl crate::RegisterSpec for Wrp1brSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrp1br::R`](R) reader structure"]
impl crate::Readable for Wrp1brSpec {}
#[doc = "`write(|w| ..)` method takes [`wrp1br::W`](W) writer structure"]
impl crate::Writable for Wrp1brSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRP1BR to value 0"]
impl crate::Resettable for Wrp1brSpec {
    const RESET_VALUE: u32 = 0;
}

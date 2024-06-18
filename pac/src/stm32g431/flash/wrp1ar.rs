#[doc = "Register `WRP1AR` reader"]
pub type R = crate::R<Wrp1arSpec>;
#[doc = "Register `WRP1AR` writer"]
pub type W = crate::W<Wrp1arSpec>;
#[doc = "Field `WRP1A_STRT` reader - Bank 1 WRP first area start offset"]
pub type Wrp1aStrtR = crate::FieldReader;
#[doc = "Field `WRP1A_STRT` writer - Bank 1 WRP first area start offset"]
pub type Wrp1aStrtW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRP1A_END` reader - Bank 1 WRP first area A end offset"]
pub type Wrp1aEndR = crate::FieldReader;
#[doc = "Field `WRP1A_END` writer - Bank 1 WRP first area A end offset"]
pub type Wrp1aEndW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Bank 1 WRP first area start offset"]
    #[inline(always)]
    pub fn wrp1a_strt(&self) -> Wrp1aStrtR {
        Wrp1aStrtR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Bank 1 WRP first area A end offset"]
    #[inline(always)]
    pub fn wrp1a_end(&self) -> Wrp1aEndR {
        Wrp1aEndR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRP1AR")
            .field("wrp1a_strt", &self.wrp1a_strt())
            .field("wrp1a_end", &self.wrp1a_end())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Bank 1 WRP first area start offset"]
    #[inline(always)]
    #[must_use]
    pub fn wrp1a_strt(&mut self) -> Wrp1aStrtW<Wrp1arSpec> {
        Wrp1aStrtW::new(self, 0)
    }
    #[doc = "Bits 16:22 - Bank 1 WRP first area A end offset"]
    #[inline(always)]
    #[must_use]
    pub fn wrp1a_end(&mut self) -> Wrp1aEndW<Wrp1arSpec> {
        Wrp1aEndW::new(self, 16)
    }
}
#[doc = "Flash Bank 1 WRP area A address register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrp1ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp1ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wrp1arSpec;
impl crate::RegisterSpec for Wrp1arSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrp1ar::R`](R) reader structure"]
impl crate::Readable for Wrp1arSpec {}
#[doc = "`write(|w| ..)` method takes [`wrp1ar::W`](W) writer structure"]
impl crate::Writable for Wrp1arSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRP1AR to value 0"]
impl crate::Resettable for Wrp1arSpec {
    const RESET_VALUE: u32 = 0;
}

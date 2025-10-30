#[doc = "Register `DADDR` reader"]
pub type R = crate::R<DaddrSpec>;
#[doc = "Register `DADDR` writer"]
pub type W = crate::W<DaddrSpec>;
#[doc = "Field `ADD` reader - ADD"]
pub type AddR = crate::FieldReader;
#[doc = "Field `ADD` writer - ADD"]
pub type AddW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EF` reader - EF"]
pub type EfR = crate::BitReader;
#[doc = "Field `EF` writer - EF"]
pub type EfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - ADD"]
    #[inline(always)]
    pub fn add(&self) -> AddR {
        AddR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - EF"]
    #[inline(always)]
    pub fn ef(&self) -> EfR {
        EfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DADDR")
            .field("add", &self.add())
            .field("ef", &self.ef())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - ADD"]
    #[inline(always)]
    pub fn add(&mut self) -> AddW<'_, DaddrSpec> {
        AddW::new(self, 0)
    }
    #[doc = "Bit 7 - EF"]
    #[inline(always)]
    pub fn ef(&mut self) -> EfW<'_, DaddrSpec> {
        EfW::new(self, 7)
    }
}
#[doc = "USB device address\n\nYou can [`read`](crate::Reg::read) this register and get [`daddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaddrSpec;
impl crate::RegisterSpec for DaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daddr::R`](R) reader structure"]
impl crate::Readable for DaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`daddr::W`](W) writer structure"]
impl crate::Writable for DaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DADDR to value 0"]
impl crate::Resettable for DaddrSpec {}

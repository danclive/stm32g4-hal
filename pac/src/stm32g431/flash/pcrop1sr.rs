#[doc = "Register `PCROP1SR` reader"]
pub type R = crate::R<Pcrop1srSpec>;
#[doc = "Register `PCROP1SR` writer"]
pub type W = crate::W<Pcrop1srSpec>;
#[doc = "Field `PCROP1_STRT` reader - Bank 1 PCROP area start offset"]
pub type Pcrop1StrtR = crate::FieldReader<u16>;
#[doc = "Field `PCROP1_STRT` writer - Bank 1 PCROP area start offset"]
pub type Pcrop1StrtW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Bank 1 PCROP area start offset"]
    #[inline(always)]
    pub fn pcrop1_strt(&self) -> Pcrop1StrtR {
        Pcrop1StrtR::new((self.bits & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCROP1SR")
            .field("pcrop1_strt", &self.pcrop1_strt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:14 - Bank 1 PCROP area start offset"]
    #[inline(always)]
    pub fn pcrop1_strt(&mut self) -> Pcrop1StrtW<'_, Pcrop1srSpec> {
        Pcrop1StrtW::new(self, 0)
    }
}
#[doc = "Flash Bank 1 PCROP Start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrop1sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcrop1srSpec;
impl crate::RegisterSpec for Pcrop1srSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrop1sr::R`](R) reader structure"]
impl crate::Readable for Pcrop1srSpec {}
#[doc = "`write(|w| ..)` method takes [`pcrop1sr::W`](W) writer structure"]
impl crate::Writable for Pcrop1srSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCROP1SR to value 0xffff_0000"]
impl crate::Resettable for Pcrop1srSpec {
    const RESET_VALUE: u32 = 0xffff_0000;
}

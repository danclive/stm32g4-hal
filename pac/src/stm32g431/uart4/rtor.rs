#[doc = "Register `RTOR` reader"]
pub type R = crate::R<RtorSpec>;
#[doc = "Register `RTOR` writer"]
pub type W = crate::W<RtorSpec>;
#[doc = "Field `RTO` reader - Receiver timeout value"]
pub type RtoR = crate::FieldReader<u32>;
#[doc = "Field `RTO` writer - Receiver timeout value"]
pub type RtoW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `BLEN` reader - Block Length"]
pub type BlenR = crate::FieldReader;
#[doc = "Field `BLEN` writer - Block Length"]
pub type BlenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - Receiver timeout value"]
    #[inline(always)]
    pub fn rto(&self) -> RtoR {
        RtoR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - Block Length"]
    #[inline(always)]
    pub fn blen(&self) -> BlenR {
        BlenR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTOR")
            .field("blen", &self.blen())
            .field("rto", &self.rto())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:23 - Receiver timeout value"]
    #[inline(always)]
    pub fn rto(&mut self) -> RtoW<RtorSpec> {
        RtoW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Block Length"]
    #[inline(always)]
    pub fn blen(&mut self) -> BlenW<RtorSpec> {
        BlenW::new(self, 24)
    }
}
#[doc = "Receiver timeout register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtorSpec;
impl crate::RegisterSpec for RtorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtor::R`](R) reader structure"]
impl crate::Readable for RtorSpec {}
#[doc = "`write(|w| ..)` method takes [`rtor::W`](W) writer structure"]
impl crate::Writable for RtorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTOR to value 0"]
impl crate::Resettable for RtorSpec {
    const RESET_VALUE: u32 = 0;
}

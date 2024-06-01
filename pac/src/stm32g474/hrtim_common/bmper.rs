#[doc = "Register `BMPER` reader"]
pub type R = crate::R<BmperSpec>;
#[doc = "Register `BMPER` writer"]
pub type W = crate::W<BmperSpec>;
#[doc = "Field `BMPER` reader - Burst mode Period"]
pub type BmperR = crate::FieldReader<u16>;
#[doc = "Field `BMPER` writer - Burst mode Period"]
pub type BmperW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Burst mode Period"]
    #[inline(always)]
    pub fn bmper(&self) -> BmperR {
        BmperR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BMPER")
            .field("bmper", &self.bmper())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Burst mode Period"]
    #[inline(always)]
    #[must_use]
    pub fn bmper(&mut self) -> BmperW<BmperSpec> {
        BmperW::new(self, 0)
    }
}
#[doc = "Burst Mode Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmperSpec;
impl crate::RegisterSpec for BmperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmper::R`](R) reader structure"]
impl crate::Readable for BmperSpec {}
#[doc = "`write(|w| ..)` method takes [`bmper::W`](W) writer structure"]
impl crate::Writable for BmperSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BMPER to value 0"]
impl crate::Resettable for BmperSpec {
    const RESET_VALUE: u32 = 0;
}

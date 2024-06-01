#[doc = "Register `CMP3BR` reader"]
pub type R = crate::R<Cmp3brSpec>;
#[doc = "Register `CMP3BR` writer"]
pub type W = crate::W<Cmp3brSpec>;
#[doc = "Field `CMP3x` reader - Timerx Compare 3 value"]
pub type Cmp3xR = crate::FieldReader<u16>;
#[doc = "Field `CMP3x` writer - Timerx Compare 3 value"]
pub type Cmp3xW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 3 value"]
    #[inline(always)]
    pub fn cmp3x(&self) -> Cmp3xR {
        Cmp3xR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMP3BR")
            .field("cmp3x", &self.cmp3x())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 3 value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3x(&mut self) -> Cmp3xW<Cmp3brSpec> {
        Cmp3xW::new(self, 0)
    }
}
#[doc = "Timerx Compare 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp3br::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp3br::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp3brSpec;
impl crate::RegisterSpec for Cmp3brSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp3br::R`](R) reader structure"]
impl crate::Readable for Cmp3brSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp3br::W`](W) writer structure"]
impl crate::Writable for Cmp3brSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP3BR to value 0"]
impl crate::Resettable for Cmp3brSpec {
    const RESET_VALUE: u32 = 0;
}

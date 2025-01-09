#[doc = "Register `WINR` reader"]
pub type R = crate::R<WinrSpec>;
#[doc = "Register `WINR` writer"]
pub type W = crate::W<WinrSpec>;
#[doc = "Field `WIN` reader - Watchdog counter window value"]
pub type WinR = crate::FieldReader<u16>;
#[doc = "Field `WIN` writer - Watchdog counter window value"]
pub type WinW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Watchdog counter window value"]
    #[inline(always)]
    pub fn win(&self) -> WinR {
        WinR::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WINR").field("win", &self.win()).finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter window value"]
    #[inline(always)]
    pub fn win(&mut self) -> WinW<WinrSpec> {
        WinW::new(self, 0)
    }
}
#[doc = "Window register\n\nYou can [`read`](crate::Reg::read) this register and get [`winr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WinrSpec;
impl crate::RegisterSpec for WinrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`winr::R`](R) reader structure"]
impl crate::Readable for WinrSpec {}
#[doc = "`write(|w| ..)` method takes [`winr::W`](W) writer structure"]
impl crate::Writable for WinrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WINR to value 0x0fff"]
impl crate::Resettable for WinrSpec {
    const RESET_VALUE: u32 = 0x0fff;
}

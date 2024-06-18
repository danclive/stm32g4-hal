#[doc = "Register `MEMRMP` reader"]
pub type R = crate::R<MemrmpSpec>;
#[doc = "Register `MEMRMP` writer"]
pub type W = crate::W<MemrmpSpec>;
#[doc = "Field `MEM_MODE` reader - Memory mapping selection"]
pub type MemModeR = crate::FieldReader;
#[doc = "Field `MEM_MODE` writer - Memory mapping selection"]
pub type MemModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FB_mode` reader - User Flash Bank mode"]
pub type FbModeR = crate::BitReader;
#[doc = "Field `FB_mode` writer - User Flash Bank mode"]
pub type FbModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Memory mapping selection"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MemModeR {
        MemModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - User Flash Bank mode"]
    #[inline(always)]
    pub fn fb_mode(&self) -> FbModeR {
        FbModeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEMRMP")
            .field("mem_mode", &self.mem_mode())
            .field("fb_mode", &self.fb_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Memory mapping selection"]
    #[inline(always)]
    #[must_use]
    pub fn mem_mode(&mut self) -> MemModeW<MemrmpSpec> {
        MemModeW::new(self, 0)
    }
    #[doc = "Bit 8 - User Flash Bank mode"]
    #[inline(always)]
    #[must_use]
    pub fn fb_mode(&mut self) -> FbModeW<MemrmpSpec> {
        FbModeW::new(self, 8)
    }
}
#[doc = "Remap Memory register\n\nYou can [`read`](crate::Reg::read) this register and get [`memrmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memrmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemrmpSpec;
impl crate::RegisterSpec for MemrmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memrmp::R`](R) reader structure"]
impl crate::Readable for MemrmpSpec {}
#[doc = "`write(|w| ..)` method takes [`memrmp::W`](W) writer structure"]
impl crate::Writable for MemrmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMRMP to value 0"]
impl crate::Resettable for MemrmpSpec {
    const RESET_VALUE: u32 = 0;
}

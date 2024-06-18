#[doc = "Register `ICSR` reader"]
pub type R = crate::R<IcsrSpec>;
#[doc = "Register `ICSR` writer"]
pub type W = crate::W<IcsrSpec>;
#[doc = "Field `ALRAWF` reader - Alarm A write flag"]
pub type AlrawfR = crate::BitReader;
#[doc = "Field `ALRBWF` reader - Alarm B write flag"]
pub type AlrbwfR = crate::BitReader;
#[doc = "Field `WUTWF` reader - Wakeup timer write flag"]
pub type WutwfR = crate::BitReader;
#[doc = "Field `SHPF` reader - Shift operation pending"]
pub type ShpfR = crate::BitReader;
#[doc = "Field `SHPF` writer - Shift operation pending"]
pub type ShpfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITS` reader - Initialization status flag"]
pub type InitsR = crate::BitReader;
#[doc = "Field `RSF` reader - Registers synchronization flag"]
pub type RsfR = crate::BitReader;
#[doc = "Field `RSF` writer - Registers synchronization flag"]
pub type RsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITF` reader - Initialization flag"]
pub type InitfR = crate::BitReader;
#[doc = "Field `INIT` reader - Initialization mode"]
pub type InitR = crate::BitReader;
#[doc = "Field `INIT` writer - Initialization mode"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECALPF` reader - Recalibration pending Flag"]
pub type RecalpfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Alarm A write flag"]
    #[inline(always)]
    pub fn alrawf(&self) -> AlrawfR {
        AlrawfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B write flag"]
    #[inline(always)]
    pub fn alrbwf(&self) -> AlrbwfR {
        AlrbwfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer write flag"]
    #[inline(always)]
    pub fn wutwf(&self) -> WutwfR {
        WutwfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shift operation pending"]
    #[inline(always)]
    pub fn shpf(&self) -> ShpfR {
        ShpfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Initialization status flag"]
    #[inline(always)]
    pub fn inits(&self) -> InitsR {
        InitsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    pub fn rsf(&self) -> RsfR {
        RsfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Initialization flag"]
    #[inline(always)]
    pub fn initf(&self) -> InitfR {
        InitfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Recalibration pending Flag"]
    #[inline(always)]
    pub fn recalpf(&self) -> RecalpfR {
        RecalpfR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICSR")
            .field("alrawf", &self.alrawf())
            .field("alrbwf", &self.alrbwf())
            .field("wutwf", &self.wutwf())
            .field("shpf", &self.shpf())
            .field("inits", &self.inits())
            .field("rsf", &self.rsf())
            .field("initf", &self.initf())
            .field("init", &self.init())
            .field("recalpf", &self.recalpf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - Shift operation pending"]
    #[inline(always)]
    #[must_use]
    pub fn shpf(&mut self) -> ShpfW<IcsrSpec> {
        ShpfW::new(self, 3)
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RsfW<IcsrSpec> {
        RsfW::new(self, 5)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> InitW<IcsrSpec> {
        InitW::new(self, 7)
    }
}
#[doc = "initialization and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`icsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcsrSpec;
impl crate::RegisterSpec for IcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icsr::R`](R) reader structure"]
impl crate::Readable for IcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`icsr::W`](W) writer structure"]
impl crate::Writable for IcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICSR to value 0x07"]
impl crate::Resettable for IcsrSpec {
    const RESET_VALUE: u32 = 0x07;
}

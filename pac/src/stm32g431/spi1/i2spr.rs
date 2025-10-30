#[doc = "Register `I2SPR` reader"]
pub type R = crate::R<I2sprSpec>;
#[doc = "Register `I2SPR` writer"]
pub type W = crate::W<I2sprSpec>;
#[doc = "Field `I2SDIV` reader - I2SDIV"]
pub type I2sdivR = crate::FieldReader;
#[doc = "Field `I2SDIV` writer - I2SDIV"]
pub type I2sdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ODD` reader - ODD"]
pub type OddR = crate::BitReader;
#[doc = "Field `ODD` writer - ODD"]
pub type OddW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCKOE` reader - MCKOE"]
pub type MckoeR = crate::BitReader;
#[doc = "Field `MCKOE` writer - MCKOE"]
pub type MckoeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - I2SDIV"]
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2sdivR {
        I2sdivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - ODD"]
    #[inline(always)]
    pub fn odd(&self) -> OddR {
        OddR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MCKOE"]
    #[inline(always)]
    pub fn mckoe(&self) -> MckoeR {
        MckoeR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SPR")
            .field("i2sdiv", &self.i2sdiv())
            .field("odd", &self.odd())
            .field("mckoe", &self.mckoe())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - I2SDIV"]
    #[inline(always)]
    pub fn i2sdiv(&mut self) -> I2sdivW<'_, I2sprSpec> {
        I2sdivW::new(self, 0)
    }
    #[doc = "Bit 8 - ODD"]
    #[inline(always)]
    pub fn odd(&mut self) -> OddW<'_, I2sprSpec> {
        OddW::new(self, 8)
    }
    #[doc = "Bit 9 - MCKOE"]
    #[inline(always)]
    pub fn mckoe(&mut self) -> MckoeW<'_, I2sprSpec> {
        MckoeW::new(self, 9)
    }
}
#[doc = "prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2spr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2spr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sprSpec;
impl crate::RegisterSpec for I2sprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2spr::R`](R) reader structure"]
impl crate::Readable for I2sprSpec {}
#[doc = "`write(|w| ..)` method takes [`i2spr::W`](W) writer structure"]
impl crate::Writable for I2sprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2SPR to value 0x02"]
impl crate::Resettable for I2sprSpec {
    const RESET_VALUE: u32 = 0x02;
}

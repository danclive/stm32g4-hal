#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<Cfgr1Spec>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<Cfgr1Spec>;
#[doc = "Field `BOOSTEN` reader - BOOSTEN"]
pub type BoostenR = crate::BitReader;
#[doc = "Field `BOOSTEN` writer - BOOSTEN"]
pub type BoostenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANASWVDD` reader - GPIO analog switch control voltage selection"]
pub type AnaswvddR = crate::BitReader;
#[doc = "Field `ANASWVDD` writer - GPIO analog switch control voltage selection"]
pub type AnaswvddW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB6_FMP` reader - FM+ drive capability on PB6"]
pub type I2cPb6FmpR = crate::BitReader;
#[doc = "Field `I2C_PB6_FMP` writer - FM+ drive capability on PB6"]
pub type I2cPb6FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB7_FMP` reader - FM+ drive capability on PB6"]
pub type I2cPb7FmpR = crate::BitReader;
#[doc = "Field `I2C_PB7_FMP` writer - FM+ drive capability on PB6"]
pub type I2cPb7FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB8_FMP` reader - FM+ drive capability on PB6"]
pub type I2cPb8FmpR = crate::BitReader;
#[doc = "Field `I2C_PB8_FMP` writer - FM+ drive capability on PB6"]
pub type I2cPb8FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB9_FMP` reader - FM+ drive capability on PB6"]
pub type I2cPb9FmpR = crate::BitReader;
#[doc = "Field `I2C_PB9_FMP` writer - FM+ drive capability on PB6"]
pub type I2cPb9FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_FMP` reader - I2C1 FM+ drive capability enable"]
pub type I2c1FmpR = crate::BitReader;
#[doc = "Field `I2C1_FMP` writer - I2C1 FM+ drive capability enable"]
pub type I2c1FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_FMP` reader - I2C1 FM+ drive capability enable"]
pub type I2c2FmpR = crate::BitReader;
#[doc = "Field `I2C2_FMP` writer - I2C1 FM+ drive capability enable"]
pub type I2c2FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3_FMP` reader - I2C1 FM+ drive capability enable"]
pub type I2c3FmpR = crate::BitReader;
#[doc = "Field `I2C3_FMP` writer - I2C1 FM+ drive capability enable"]
pub type I2c3FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4_FMP` reader - I2C1 FM+ drive capability enable"]
pub type I2c4FmpR = crate::BitReader;
#[doc = "Field `I2C4_FMP` writer - I2C1 FM+ drive capability enable"]
pub type I2c4FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPU_IE` reader - FPU Interrupts Enable"]
pub type FpuIeR = crate::FieldReader;
#[doc = "Field `FPU_IE` writer - FPU Interrupts Enable"]
pub type FpuIeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 8 - BOOSTEN"]
    #[inline(always)]
    pub fn boosten(&self) -> BoostenR {
        BoostenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO analog switch control voltage selection"]
    #[inline(always)]
    pub fn anaswvdd(&self) -> AnaswvddR {
        AnaswvddR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - FM+ drive capability on PB6"]
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2cPb6FmpR {
        I2cPb6FmpR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FM+ drive capability on PB6"]
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2cPb7FmpR {
        I2cPb7FmpR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FM+ drive capability on PB6"]
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2cPb8FmpR {
        I2cPb8FmpR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - FM+ drive capability on PB6"]
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2cPb9FmpR {
        I2cPb9FmpR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I2C1 FM+ drive capability enable"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2c1FmpR {
        I2c1FmpR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 FM+ drive capability enable"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2c2FmpR {
        I2c2FmpR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 FM+ drive capability enable"]
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2c3FmpR {
        I2c3FmpR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C1 FM+ drive capability enable"]
    #[inline(always)]
    pub fn i2c4_fmp(&self) -> I2c4FmpR {
        I2c4FmpR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 26:31 - FPU Interrupts Enable"]
    #[inline(always)]
    pub fn fpu_ie(&self) -> FpuIeR {
        FpuIeR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("boosten", &self.boosten())
            .field("anaswvdd", &self.anaswvdd())
            .field("i2c_pb6_fmp", &self.i2c_pb6_fmp())
            .field("i2c_pb7_fmp", &self.i2c_pb7_fmp())
            .field("i2c_pb8_fmp", &self.i2c_pb8_fmp())
            .field("i2c_pb9_fmp", &self.i2c_pb9_fmp())
            .field("i2c1_fmp", &self.i2c1_fmp())
            .field("i2c2_fmp", &self.i2c2_fmp())
            .field("i2c3_fmp", &self.i2c3_fmp())
            .field("i2c4_fmp", &self.i2c4_fmp())
            .field("fpu_ie", &self.fpu_ie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - BOOSTEN"]
    #[inline(always)]
    pub fn boosten(&mut self) -> BoostenW<'_, Cfgr1Spec> {
        BoostenW::new(self, 8)
    }
    #[doc = "Bit 9 - GPIO analog switch control voltage selection"]
    #[inline(always)]
    pub fn anaswvdd(&mut self) -> AnaswvddW<'_, Cfgr1Spec> {
        AnaswvddW::new(self, 9)
    }
    #[doc = "Bit 16 - FM+ drive capability on PB6"]
    #[inline(always)]
    pub fn i2c_pb6_fmp(&mut self) -> I2cPb6FmpW<'_, Cfgr1Spec> {
        I2cPb6FmpW::new(self, 16)
    }
    #[doc = "Bit 17 - FM+ drive capability on PB6"]
    #[inline(always)]
    pub fn i2c_pb7_fmp(&mut self) -> I2cPb7FmpW<'_, Cfgr1Spec> {
        I2cPb7FmpW::new(self, 17)
    }
    #[doc = "Bit 18 - FM+ drive capability on PB6"]
    #[inline(always)]
    pub fn i2c_pb8_fmp(&mut self) -> I2cPb8FmpW<'_, Cfgr1Spec> {
        I2cPb8FmpW::new(self, 18)
    }
    #[doc = "Bit 19 - FM+ drive capability on PB6"]
    #[inline(always)]
    pub fn i2c_pb9_fmp(&mut self) -> I2cPb9FmpW<'_, Cfgr1Spec> {
        I2cPb9FmpW::new(self, 19)
    }
    #[doc = "Bit 20 - I2C1 FM+ drive capability enable"]
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2c1FmpW<'_, Cfgr1Spec> {
        I2c1FmpW::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 FM+ drive capability enable"]
    #[inline(always)]
    pub fn i2c2_fmp(&mut self) -> I2c2FmpW<'_, Cfgr1Spec> {
        I2c2FmpW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C1 FM+ drive capability enable"]
    #[inline(always)]
    pub fn i2c3_fmp(&mut self) -> I2c3FmpW<'_, Cfgr1Spec> {
        I2c3FmpW::new(self, 22)
    }
    #[doc = "Bit 23 - I2C1 FM+ drive capability enable"]
    #[inline(always)]
    pub fn i2c4_fmp(&mut self) -> I2c4FmpW<'_, Cfgr1Spec> {
        I2c4FmpW::new(self, 23)
    }
    #[doc = "Bits 26:31 - FPU Interrupts Enable"]
    #[inline(always)]
    pub fn fpu_ie(&mut self) -> FpuIeW<'_, Cfgr1Spec> {
        FpuIeW::new(self, 26)
    }
}
#[doc = "peripheral mode configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr1Spec;
impl crate::RegisterSpec for Cfgr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for Cfgr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for Cfgr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR1 to value 0x7c00_0001"]
impl crate::Resettable for Cfgr1Spec {
    const RESET_VALUE: u32 = 0x7c00_0001;
}

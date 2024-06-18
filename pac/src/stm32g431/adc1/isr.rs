#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<IsrSpec>;
#[doc = "Field `ADRDY` reader - ADRDY"]
pub type AdrdyR = crate::BitReader;
#[doc = "Field `ADRDY` writer - ADRDY"]
pub type AdrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSMP` reader - EOSMP"]
pub type EosmpR = crate::BitReader;
#[doc = "Field `EOSMP` writer - EOSMP"]
pub type EosmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC` reader - EOC"]
pub type EocR = crate::BitReader;
#[doc = "Field `EOC` writer - EOC"]
pub type EocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOS` reader - EOS"]
pub type EosR = crate::BitReader;
#[doc = "Field `EOS` writer - EOS"]
pub type EosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR` reader - OVR"]
pub type OvrR = crate::BitReader;
#[doc = "Field `OVR` writer - OVR"]
pub type OvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOC` reader - JEOC"]
pub type JeocR = crate::BitReader;
#[doc = "Field `JEOC` writer - JEOC"]
pub type JeocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOS` reader - JEOS"]
pub type JeosR = crate::BitReader;
#[doc = "Field `JEOS` writer - JEOS"]
pub type JeosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1` reader - AWD1"]
pub type Awd1R = crate::BitReader;
#[doc = "Field `AWD1` writer - AWD1"]
pub type Awd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD2` reader - AWD2"]
pub type Awd2R = crate::BitReader;
#[doc = "Field `AWD2` writer - AWD2"]
pub type Awd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD3` reader - AWD3"]
pub type Awd3R = crate::BitReader;
#[doc = "Field `AWD3` writer - AWD3"]
pub type Awd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JQOVF` reader - JQOVF"]
pub type JqovfR = crate::BitReader;
#[doc = "Field `JQOVF` writer - JQOVF"]
pub type JqovfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADRDY"]
    #[inline(always)]
    pub fn adrdy(&self) -> AdrdyR {
        AdrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EOSMP"]
    #[inline(always)]
    pub fn eosmp(&self) -> EosmpR {
        EosmpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EOC"]
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOS"]
    #[inline(always)]
    pub fn eos(&self) -> EosR {
        EosR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OVR"]
    #[inline(always)]
    pub fn ovr(&self) -> OvrR {
        OvrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JEOC"]
    #[inline(always)]
    pub fn jeoc(&self) -> JeocR {
        JeocR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - JEOS"]
    #[inline(always)]
    pub fn jeos(&self) -> JeosR {
        JeosR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AWD1"]
    #[inline(always)]
    pub fn awd1(&self) -> Awd1R {
        Awd1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AWD2"]
    #[inline(always)]
    pub fn awd2(&self) -> Awd2R {
        Awd2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AWD3"]
    #[inline(always)]
    pub fn awd3(&self) -> Awd3R {
        Awd3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - JQOVF"]
    #[inline(always)]
    pub fn jqovf(&self) -> JqovfR {
        JqovfR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("jqovf", &self.jqovf())
            .field("awd3", &self.awd3())
            .field("awd2", &self.awd2())
            .field("awd1", &self.awd1())
            .field("jeos", &self.jeos())
            .field("jeoc", &self.jeoc())
            .field("ovr", &self.ovr())
            .field("eos", &self.eos())
            .field("eoc", &self.eoc())
            .field("eosmp", &self.eosmp())
            .field("adrdy", &self.adrdy())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ADRDY"]
    #[inline(always)]
    #[must_use]
    pub fn adrdy(&mut self) -> AdrdyW<IsrSpec> {
        AdrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - EOSMP"]
    #[inline(always)]
    #[must_use]
    pub fn eosmp(&mut self) -> EosmpW<IsrSpec> {
        EosmpW::new(self, 1)
    }
    #[doc = "Bit 2 - EOC"]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EocW<IsrSpec> {
        EocW::new(self, 2)
    }
    #[doc = "Bit 3 - EOS"]
    #[inline(always)]
    #[must_use]
    pub fn eos(&mut self) -> EosW<IsrSpec> {
        EosW::new(self, 3)
    }
    #[doc = "Bit 4 - OVR"]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OvrW<IsrSpec> {
        OvrW::new(self, 4)
    }
    #[doc = "Bit 5 - JEOC"]
    #[inline(always)]
    #[must_use]
    pub fn jeoc(&mut self) -> JeocW<IsrSpec> {
        JeocW::new(self, 5)
    }
    #[doc = "Bit 6 - JEOS"]
    #[inline(always)]
    #[must_use]
    pub fn jeos(&mut self) -> JeosW<IsrSpec> {
        JeosW::new(self, 6)
    }
    #[doc = "Bit 7 - AWD1"]
    #[inline(always)]
    #[must_use]
    pub fn awd1(&mut self) -> Awd1W<IsrSpec> {
        Awd1W::new(self, 7)
    }
    #[doc = "Bit 8 - AWD2"]
    #[inline(always)]
    #[must_use]
    pub fn awd2(&mut self) -> Awd2W<IsrSpec> {
        Awd2W::new(self, 8)
    }
    #[doc = "Bit 9 - AWD3"]
    #[inline(always)]
    #[must_use]
    pub fn awd3(&mut self) -> Awd3W<IsrSpec> {
        Awd3W::new(self, 9)
    }
    #[doc = "Bit 10 - JQOVF"]
    #[inline(always)]
    #[must_use]
    pub fn jqovf(&mut self) -> JqovfW<IsrSpec> {
        JqovfW::new(self, 10)
    }
}
#[doc = "interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for IsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0;
}

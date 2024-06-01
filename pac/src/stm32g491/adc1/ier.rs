#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `ADRDYIE` reader - ADRDYIE"]
pub type AdrdyieR = crate::BitReader;
#[doc = "Field `ADRDYIE` writer - ADRDYIE"]
pub type AdrdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSMPIE` reader - EOSMPIE"]
pub type EosmpieR = crate::BitReader;
#[doc = "Field `EOSMPIE` writer - EOSMPIE"]
pub type EosmpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCIE` reader - EOCIE"]
pub type EocieR = crate::BitReader;
#[doc = "Field `EOCIE` writer - EOCIE"]
pub type EocieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSIE` reader - EOSIE"]
pub type EosieR = crate::BitReader;
#[doc = "Field `EOSIE` writer - EOSIE"]
pub type EosieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRIE` reader - OVRIE"]
pub type OvrieR = crate::BitReader;
#[doc = "Field `OVRIE` writer - OVRIE"]
pub type OvrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOCIE` reader - JEOCIE"]
pub type JeocieR = crate::BitReader;
#[doc = "Field `JEOCIE` writer - JEOCIE"]
pub type JeocieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOSIE` reader - JEOSIE"]
pub type JeosieR = crate::BitReader;
#[doc = "Field `JEOSIE` writer - JEOSIE"]
pub type JeosieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1IE` reader - AWD1IE"]
pub type Awd1ieR = crate::BitReader;
#[doc = "Field `AWD1IE` writer - AWD1IE"]
pub type Awd1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD2IE` reader - AWD2IE"]
pub type Awd2ieR = crate::BitReader;
#[doc = "Field `AWD2IE` writer - AWD2IE"]
pub type Awd2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD3IE` reader - AWD3IE"]
pub type Awd3ieR = crate::BitReader;
#[doc = "Field `AWD3IE` writer - AWD3IE"]
pub type Awd3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JQOVFIE` reader - JQOVFIE"]
pub type JqovfieR = crate::BitReader;
#[doc = "Field `JQOVFIE` writer - JQOVFIE"]
pub type JqovfieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADRDYIE"]
    #[inline(always)]
    pub fn adrdyie(&self) -> AdrdyieR {
        AdrdyieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EOSMPIE"]
    #[inline(always)]
    pub fn eosmpie(&self) -> EosmpieR {
        EosmpieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EOCIE"]
    #[inline(always)]
    pub fn eocie(&self) -> EocieR {
        EocieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOSIE"]
    #[inline(always)]
    pub fn eosie(&self) -> EosieR {
        EosieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OVRIE"]
    #[inline(always)]
    pub fn ovrie(&self) -> OvrieR {
        OvrieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JEOCIE"]
    #[inline(always)]
    pub fn jeocie(&self) -> JeocieR {
        JeocieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - JEOSIE"]
    #[inline(always)]
    pub fn jeosie(&self) -> JeosieR {
        JeosieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AWD1IE"]
    #[inline(always)]
    pub fn awd1ie(&self) -> Awd1ieR {
        Awd1ieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AWD2IE"]
    #[inline(always)]
    pub fn awd2ie(&self) -> Awd2ieR {
        Awd2ieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AWD3IE"]
    #[inline(always)]
    pub fn awd3ie(&self) -> Awd3ieR {
        Awd3ieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - JQOVFIE"]
    #[inline(always)]
    pub fn jqovfie(&self) -> JqovfieR {
        JqovfieR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("jqovfie", &self.jqovfie())
            .field("awd3ie", &self.awd3ie())
            .field("awd2ie", &self.awd2ie())
            .field("awd1ie", &self.awd1ie())
            .field("jeosie", &self.jeosie())
            .field("jeocie", &self.jeocie())
            .field("ovrie", &self.ovrie())
            .field("eosie", &self.eosie())
            .field("eocie", &self.eocie())
            .field("eosmpie", &self.eosmpie())
            .field("adrdyie", &self.adrdyie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ADRDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn adrdyie(&mut self) -> AdrdyieW<IerSpec> {
        AdrdyieW::new(self, 0)
    }
    #[doc = "Bit 1 - EOSMPIE"]
    #[inline(always)]
    #[must_use]
    pub fn eosmpie(&mut self) -> EosmpieW<IerSpec> {
        EosmpieW::new(self, 1)
    }
    #[doc = "Bit 2 - EOCIE"]
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EocieW<IerSpec> {
        EocieW::new(self, 2)
    }
    #[doc = "Bit 3 - EOSIE"]
    #[inline(always)]
    #[must_use]
    pub fn eosie(&mut self) -> EosieW<IerSpec> {
        EosieW::new(self, 3)
    }
    #[doc = "Bit 4 - OVRIE"]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OvrieW<IerSpec> {
        OvrieW::new(self, 4)
    }
    #[doc = "Bit 5 - JEOCIE"]
    #[inline(always)]
    #[must_use]
    pub fn jeocie(&mut self) -> JeocieW<IerSpec> {
        JeocieW::new(self, 5)
    }
    #[doc = "Bit 6 - JEOSIE"]
    #[inline(always)]
    #[must_use]
    pub fn jeosie(&mut self) -> JeosieW<IerSpec> {
        JeosieW::new(self, 6)
    }
    #[doc = "Bit 7 - AWD1IE"]
    #[inline(always)]
    #[must_use]
    pub fn awd1ie(&mut self) -> Awd1ieW<IerSpec> {
        Awd1ieW::new(self, 7)
    }
    #[doc = "Bit 8 - AWD2IE"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ie(&mut self) -> Awd2ieW<IerSpec> {
        Awd2ieW::new(self, 8)
    }
    #[doc = "Bit 9 - AWD3IE"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ie(&mut self) -> Awd3ieW<IerSpec> {
        Awd3ieW::new(self, 9)
    }
    #[doc = "Bit 10 - JQOVFIE"]
    #[inline(always)]
    #[must_use]
    pub fn jqovfie(&mut self) -> JqovfieW<IerSpec> {
        JqovfieW::new(self, 10)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {
    const RESET_VALUE: u32 = 0;
}

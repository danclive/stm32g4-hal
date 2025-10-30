#[doc = "Register `CCMR3_Output` reader"]
pub type R = crate::R<Ccmr3OutputSpec>;
#[doc = "Register `CCMR3_Output` writer"]
pub type W = crate::W<Ccmr3OutputSpec>;
#[doc = "Field `OC5FE` reader - Output compare 5 fast enable"]
pub type Oc5feR = crate::BitReader;
#[doc = "Field `OC5FE` writer - Output compare 5 fast enable"]
pub type Oc5feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5PE` reader - Output compare 5 preload enable"]
pub type Oc5peR = crate::BitReader;
#[doc = "Field `OC5PE` writer - Output compare 5 preload enable"]
pub type Oc5peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5M` reader - Output compare 5 mode"]
pub type Oc5mR = crate::FieldReader;
#[doc = "Field `OC5M` writer - Output compare 5 mode"]
pub type Oc5mW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC5CE` reader - Output compare 5 clear enable"]
pub type Oc5ceR = crate::BitReader;
#[doc = "Field `OC5CE` writer - Output compare 5 clear enable"]
pub type Oc5ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6FE` reader - Output compare 6 fast enable"]
pub type Oc6feR = crate::BitReader;
#[doc = "Field `OC6FE` writer - Output compare 6 fast enable"]
pub type Oc6feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6PE` reader - Output compare 6 preload enable"]
pub type Oc6peR = crate::BitReader;
#[doc = "Field `OC6PE` writer - Output compare 6 preload enable"]
pub type Oc6peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6M` reader - Output compare 6 mode"]
pub type Oc6mR = crate::FieldReader;
#[doc = "Field `OC6M` writer - Output compare 6 mode"]
pub type Oc6mW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC6CE` reader - Output compare 6 clear enable"]
pub type Oc6ceR = crate::BitReader;
#[doc = "Field `OC6CE` writer - Output compare 6 clear enable"]
pub type Oc6ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5M_bit3` reader - Output Compare 5 mode bit 3"]
pub type Oc5mBit3R = crate::FieldReader;
#[doc = "Field `OC5M_bit3` writer - Output Compare 5 mode bit 3"]
pub type Oc5mBit3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC6M_bit3` reader - Output Compare 6 mode bit 3"]
pub type Oc6mBit3R = crate::BitReader;
#[doc = "Field `OC6M_bit3` writer - Output Compare 6 mode bit 3"]
pub type Oc6mBit3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Output compare 5 fast enable"]
    #[inline(always)]
    pub fn oc5fe(&self) -> Oc5feR {
        Oc5feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 5 preload enable"]
    #[inline(always)]
    pub fn oc5pe(&self) -> Oc5peR {
        Oc5peR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 5 mode"]
    #[inline(always)]
    pub fn oc5m(&self) -> Oc5mR {
        Oc5mR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output compare 5 clear enable"]
    #[inline(always)]
    pub fn oc5ce(&self) -> Oc5ceR {
        Oc5ceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Output compare 6 fast enable"]
    #[inline(always)]
    pub fn oc6fe(&self) -> Oc6feR {
        Oc6feR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 6 preload enable"]
    #[inline(always)]
    pub fn oc6pe(&self) -> Oc6peR {
        Oc6peR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Output compare 6 mode"]
    #[inline(always)]
    pub fn oc6m(&self) -> Oc6mR {
        Oc6mR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Output compare 6 clear enable"]
    #[inline(always)]
    pub fn oc6ce(&self) -> Oc6ceR {
        Oc6ceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Output Compare 5 mode bit 3"]
    #[inline(always)]
    pub fn oc5m_bit3(&self) -> Oc5mBit3R {
        Oc5mBit3R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - Output Compare 6 mode bit 3"]
    #[inline(always)]
    pub fn oc6m_bit3(&self) -> Oc6mBit3R {
        Oc6mBit3R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR3_Output")
            .field("oc6m_bit3", &self.oc6m_bit3())
            .field("oc5m_bit3", &self.oc5m_bit3())
            .field("oc6ce", &self.oc6ce())
            .field("oc6m", &self.oc6m())
            .field("oc6pe", &self.oc6pe())
            .field("oc6fe", &self.oc6fe())
            .field("oc5ce", &self.oc5ce())
            .field("oc5m", &self.oc5m())
            .field("oc5pe", &self.oc5pe())
            .field("oc5fe", &self.oc5fe())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Output compare 5 fast enable"]
    #[inline(always)]
    pub fn oc5fe(&mut self) -> Oc5feW<'_, Ccmr3OutputSpec> {
        Oc5feW::new(self, 2)
    }
    #[doc = "Bit 3 - Output compare 5 preload enable"]
    #[inline(always)]
    pub fn oc5pe(&mut self) -> Oc5peW<'_, Ccmr3OutputSpec> {
        Oc5peW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Output compare 5 mode"]
    #[inline(always)]
    pub fn oc5m(&mut self) -> Oc5mW<'_, Ccmr3OutputSpec> {
        Oc5mW::new(self, 4)
    }
    #[doc = "Bit 7 - Output compare 5 clear enable"]
    #[inline(always)]
    pub fn oc5ce(&mut self) -> Oc5ceW<'_, Ccmr3OutputSpec> {
        Oc5ceW::new(self, 7)
    }
    #[doc = "Bit 10 - Output compare 6 fast enable"]
    #[inline(always)]
    pub fn oc6fe(&mut self) -> Oc6feW<'_, Ccmr3OutputSpec> {
        Oc6feW::new(self, 10)
    }
    #[doc = "Bit 11 - Output compare 6 preload enable"]
    #[inline(always)]
    pub fn oc6pe(&mut self) -> Oc6peW<'_, Ccmr3OutputSpec> {
        Oc6peW::new(self, 11)
    }
    #[doc = "Bits 12:14 - Output compare 6 mode"]
    #[inline(always)]
    pub fn oc6m(&mut self) -> Oc6mW<'_, Ccmr3OutputSpec> {
        Oc6mW::new(self, 12)
    }
    #[doc = "Bit 15 - Output compare 6 clear enable"]
    #[inline(always)]
    pub fn oc6ce(&mut self) -> Oc6ceW<'_, Ccmr3OutputSpec> {
        Oc6ceW::new(self, 15)
    }
    #[doc = "Bits 16:18 - Output Compare 5 mode bit 3"]
    #[inline(always)]
    pub fn oc5m_bit3(&mut self) -> Oc5mBit3W<'_, Ccmr3OutputSpec> {
        Oc5mBit3W::new(self, 16)
    }
    #[doc = "Bit 24 - Output Compare 6 mode bit 3"]
    #[inline(always)]
    pub fn oc6m_bit3(&mut self) -> Oc6mBit3W<'_, Ccmr3OutputSpec> {
        Oc6mBit3W::new(self, 24)
    }
}
#[doc = "capture/compare mode register 2 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr3_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr3_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr3OutputSpec;
impl crate::RegisterSpec for Ccmr3OutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr3_output::R`](R) reader structure"]
impl crate::Readable for Ccmr3OutputSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmr3_output::W`](W) writer structure"]
impl crate::Writable for Ccmr3OutputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMR3_Output to value 0"]
impl crate::Resettable for Ccmr3OutputSpec {}

#[doc = "Register `CCMR2_Output` reader"]
pub type R = crate::R<Ccmr2OutputSpec>;
#[doc = "Register `CCMR2_Output` writer"]
pub type W = crate::W<Ccmr2OutputSpec>;
#[doc = "Field `CC3S` reader - Capture/Compare 3 selection"]
pub type Cc3sR = crate::FieldReader;
#[doc = "Field `CC3S` writer - Capture/Compare 3 selection"]
pub type Cc3sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC3FE` reader - Output compare 3 fast enable"]
pub type Oc3feR = crate::BitReader;
#[doc = "Field `OC3FE` writer - Output compare 3 fast enable"]
pub type Oc3feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3PE` reader - Output compare 3 preload enable"]
pub type Oc3peR = crate::BitReader;
#[doc = "Field `OC3PE` writer - Output compare 3 preload enable"]
pub type Oc3peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3M` reader - Output compare 3 mode"]
pub type Oc3mR = crate::FieldReader;
#[doc = "Field `OC3M` writer - Output compare 3 mode"]
pub type Oc3mW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC3CE` reader - Output compare 3 clear enable"]
pub type Oc3ceR = crate::BitReader;
#[doc = "Field `OC3CE` writer - Output compare 3 clear enable"]
pub type Oc3ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4S` reader - Capture/Compare 4 selection"]
pub type Cc4sR = crate::FieldReader;
#[doc = "Field `CC4S` writer - Capture/Compare 4 selection"]
pub type Cc4sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC4FE` reader - Output compare 4 fast enable"]
pub type Oc4feR = crate::BitReader;
#[doc = "Field `OC4FE` writer - Output compare 4 fast enable"]
pub type Oc4feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4PE` reader - Output compare 4 preload enable"]
pub type Oc4peR = crate::BitReader;
#[doc = "Field `OC4PE` writer - Output compare 4 preload enable"]
pub type Oc4peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4M` reader - Output compare 4 mode"]
pub type Oc4mR = crate::FieldReader;
#[doc = "Field `OC4M` writer - Output compare 4 mode"]
pub type Oc4mW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC4CE` reader - Output compare 4 clear enable"]
pub type Oc4ceR = crate::BitReader;
#[doc = "Field `OC4CE` writer - Output compare 4 clear enable"]
pub type Oc4ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3M_3` reader - Output Compare 3 mode - bit 3"]
pub type Oc3m3R = crate::BitReader;
#[doc = "Field `OC3M_3` writer - Output Compare 3 mode - bit 3"]
pub type Oc3m3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4M_3` reader - Output Compare 4 mode - bit 3"]
pub type Oc4m3R = crate::BitReader;
#[doc = "Field `OC4M_3` writer - Output Compare 4 mode - bit 3"]
pub type Oc4m3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 3 selection"]
    #[inline(always)]
    pub fn cc3s(&self) -> Cc3sR {
        Cc3sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output compare 3 fast enable"]
    #[inline(always)]
    pub fn oc3fe(&self) -> Oc3feR {
        Oc3feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 3 preload enable"]
    #[inline(always)]
    pub fn oc3pe(&self) -> Oc3peR {
        Oc3peR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 3 mode"]
    #[inline(always)]
    pub fn oc3m(&self) -> Oc3mR {
        Oc3mR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output compare 3 clear enable"]
    #[inline(always)]
    pub fn oc3ce(&self) -> Oc3ceR {
        Oc3ceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    pub fn cc4s(&self) -> Cc4sR {
        Cc4sR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline(always)]
    pub fn oc4fe(&self) -> Oc4feR {
        Oc4feR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 4 preload enable"]
    #[inline(always)]
    pub fn oc4pe(&self) -> Oc4peR {
        Oc4peR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Output compare 4 mode"]
    #[inline(always)]
    pub fn oc4m(&self) -> Oc4mR {
        Oc4mR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Output compare 4 clear enable"]
    #[inline(always)]
    pub fn oc4ce(&self) -> Oc4ceR {
        Oc4ceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output Compare 3 mode - bit 3"]
    #[inline(always)]
    pub fn oc3m_3(&self) -> Oc3m3R {
        Oc3m3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Output Compare 4 mode - bit 3"]
    #[inline(always)]
    pub fn oc4m_3(&self) -> Oc4m3R {
        Oc4m3R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR2_Output")
            .field("oc4m_3", &self.oc4m_3())
            .field("oc3m_3", &self.oc3m_3())
            .field("oc4ce", &self.oc4ce())
            .field("oc4m", &self.oc4m())
            .field("oc4pe", &self.oc4pe())
            .field("oc4fe", &self.oc4fe())
            .field("cc4s", &self.cc4s())
            .field("oc3ce", &self.oc3ce())
            .field("oc3m", &self.oc3m())
            .field("oc3pe", &self.oc3pe())
            .field("oc3fe", &self.oc3fe())
            .field("cc3s", &self.cc3s())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 3 selection"]
    #[inline(always)]
    pub fn cc3s(&mut self) -> Cc3sW<'_, Ccmr2OutputSpec> {
        Cc3sW::new(self, 0)
    }
    #[doc = "Bit 2 - Output compare 3 fast enable"]
    #[inline(always)]
    pub fn oc3fe(&mut self) -> Oc3feW<'_, Ccmr2OutputSpec> {
        Oc3feW::new(self, 2)
    }
    #[doc = "Bit 3 - Output compare 3 preload enable"]
    #[inline(always)]
    pub fn oc3pe(&mut self) -> Oc3peW<'_, Ccmr2OutputSpec> {
        Oc3peW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Output compare 3 mode"]
    #[inline(always)]
    pub fn oc3m(&mut self) -> Oc3mW<'_, Ccmr2OutputSpec> {
        Oc3mW::new(self, 4)
    }
    #[doc = "Bit 7 - Output compare 3 clear enable"]
    #[inline(always)]
    pub fn oc3ce(&mut self) -> Oc3ceW<'_, Ccmr2OutputSpec> {
        Oc3ceW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    pub fn cc4s(&mut self) -> Cc4sW<'_, Ccmr2OutputSpec> {
        Cc4sW::new(self, 8)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline(always)]
    pub fn oc4fe(&mut self) -> Oc4feW<'_, Ccmr2OutputSpec> {
        Oc4feW::new(self, 10)
    }
    #[doc = "Bit 11 - Output compare 4 preload enable"]
    #[inline(always)]
    pub fn oc4pe(&mut self) -> Oc4peW<'_, Ccmr2OutputSpec> {
        Oc4peW::new(self, 11)
    }
    #[doc = "Bits 12:14 - Output compare 4 mode"]
    #[inline(always)]
    pub fn oc4m(&mut self) -> Oc4mW<'_, Ccmr2OutputSpec> {
        Oc4mW::new(self, 12)
    }
    #[doc = "Bit 15 - Output compare 4 clear enable"]
    #[inline(always)]
    pub fn oc4ce(&mut self) -> Oc4ceW<'_, Ccmr2OutputSpec> {
        Oc4ceW::new(self, 15)
    }
    #[doc = "Bit 16 - Output Compare 3 mode - bit 3"]
    #[inline(always)]
    pub fn oc3m_3(&mut self) -> Oc3m3W<'_, Ccmr2OutputSpec> {
        Oc3m3W::new(self, 16)
    }
    #[doc = "Bit 24 - Output Compare 4 mode - bit 3"]
    #[inline(always)]
    pub fn oc4m_3(&mut self) -> Oc4m3W<'_, Ccmr2OutputSpec> {
        Oc4m3W::new(self, 24)
    }
}
#[doc = "capture/compare mode register 2 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr2OutputSpec;
impl crate::RegisterSpec for Ccmr2OutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr2_output::R`](R) reader structure"]
impl crate::Readable for Ccmr2OutputSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmr2_output::W`](W) writer structure"]
impl crate::Writable for Ccmr2OutputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMR2_Output to value 0"]
impl crate::Resettable for Ccmr2OutputSpec {}

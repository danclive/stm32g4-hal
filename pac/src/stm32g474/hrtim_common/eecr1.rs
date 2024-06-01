#[doc = "Register `EECR1` reader"]
pub type R = crate::R<Eecr1Spec>;
#[doc = "Register `EECR1` writer"]
pub type W = crate::W<Eecr1Spec>;
#[doc = "Field `EE1SRC` reader - External Event 1 Source"]
pub type Ee1srcR = crate::FieldReader;
#[doc = "Field `EE1SRC` writer - External Event 1 Source"]
pub type Ee1srcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE1POL` reader - External Event 1 Polarity"]
pub type Ee1polR = crate::BitReader;
#[doc = "Field `EE1POL` writer - External Event 1 Polarity"]
pub type Ee1polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE1SNS` reader - External Event 1 Sensitivity"]
pub type Ee1snsR = crate::FieldReader;
#[doc = "Field `EE1SNS` writer - External Event 1 Sensitivity"]
pub type Ee1snsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE1FAST` reader - External Event 1 Fast mode"]
pub type Ee1fastR = crate::BitReader;
#[doc = "Field `EE1FAST` writer - External Event 1 Fast mode"]
pub type Ee1fastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE2SRC` reader - External Event 2 Source"]
pub type Ee2srcR = crate::FieldReader;
#[doc = "Field `EE2SRC` writer - External Event 2 Source"]
pub type Ee2srcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE2POL` reader - External Event 2 Polarity"]
pub type Ee2polR = crate::BitReader;
#[doc = "Field `EE2POL` writer - External Event 2 Polarity"]
pub type Ee2polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE2SNS` reader - External Event 2 Sensitivity"]
pub type Ee2snsR = crate::FieldReader;
#[doc = "Field `EE2SNS` writer - External Event 2 Sensitivity"]
pub type Ee2snsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE2FAST` reader - External Event 2 Fast mode"]
pub type Ee2fastR = crate::BitReader;
#[doc = "Field `EE2FAST` writer - External Event 2 Fast mode"]
pub type Ee2fastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE3SRC` reader - External Event 3 Source"]
pub type Ee3srcR = crate::FieldReader;
#[doc = "Field `EE3SRC` writer - External Event 3 Source"]
pub type Ee3srcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE3POL` reader - External Event 3 Polarity"]
pub type Ee3polR = crate::BitReader;
#[doc = "Field `EE3POL` writer - External Event 3 Polarity"]
pub type Ee3polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE3SNS` reader - External Event 3 Sensitivity"]
pub type Ee3snsR = crate::FieldReader;
#[doc = "Field `EE3SNS` writer - External Event 3 Sensitivity"]
pub type Ee3snsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE3FAST` reader - External Event 3 Fast mode"]
pub type Ee3fastR = crate::BitReader;
#[doc = "Field `EE3FAST` writer - External Event 3 Fast mode"]
pub type Ee3fastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE4SRC` reader - External Event 4 Source"]
pub type Ee4srcR = crate::FieldReader;
#[doc = "Field `EE4SRC` writer - External Event 4 Source"]
pub type Ee4srcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE4POL` reader - External Event 4 Polarity"]
pub type Ee4polR = crate::BitReader;
#[doc = "Field `EE4POL` writer - External Event 4 Polarity"]
pub type Ee4polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE4SNS` reader - External Event 4 Sensitivity"]
pub type Ee4snsR = crate::FieldReader;
#[doc = "Field `EE4SNS` writer - External Event 4 Sensitivity"]
pub type Ee4snsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE4FAST` reader - External Event 4 Fast mode"]
pub type Ee4fastR = crate::BitReader;
#[doc = "Field `EE4FAST` writer - External Event 4 Fast mode"]
pub type Ee4fastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE5SRC` reader - External Event 5 Source"]
pub type Ee5srcR = crate::FieldReader;
#[doc = "Field `EE5SRC` writer - External Event 5 Source"]
pub type Ee5srcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE5POL` reader - External Event 5 Polarity"]
pub type Ee5polR = crate::BitReader;
#[doc = "Field `EE5POL` writer - External Event 5 Polarity"]
pub type Ee5polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE5SNS` reader - External Event 5 Sensitivity"]
pub type Ee5snsR = crate::FieldReader;
#[doc = "Field `EE5SNS` writer - External Event 5 Sensitivity"]
pub type Ee5snsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE5FAST` reader - External Event 5 Fast mode"]
pub type Ee5fastR = crate::BitReader;
#[doc = "Field `EE5FAST` writer - External Event 5 Fast mode"]
pub type Ee5fastW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - External Event 1 Source"]
    #[inline(always)]
    pub fn ee1src(&self) -> Ee1srcR {
        Ee1srcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - External Event 1 Polarity"]
    #[inline(always)]
    pub fn ee1pol(&self) -> Ee1polR {
        Ee1polR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - External Event 1 Sensitivity"]
    #[inline(always)]
    pub fn ee1sns(&self) -> Ee1snsR {
        Ee1snsR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - External Event 1 Fast mode"]
    #[inline(always)]
    pub fn ee1fast(&self) -> Ee1fastR {
        Ee1fastR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - External Event 2 Source"]
    #[inline(always)]
    pub fn ee2src(&self) -> Ee2srcR {
        Ee2srcR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - External Event 2 Polarity"]
    #[inline(always)]
    pub fn ee2pol(&self) -> Ee2polR {
        Ee2polR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - External Event 2 Sensitivity"]
    #[inline(always)]
    pub fn ee2sns(&self) -> Ee2snsR {
        Ee2snsR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - External Event 2 Fast mode"]
    #[inline(always)]
    pub fn ee2fast(&self) -> Ee2fastR {
        Ee2fastR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - External Event 3 Source"]
    #[inline(always)]
    pub fn ee3src(&self) -> Ee3srcR {
        Ee3srcR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External Event 3 Polarity"]
    #[inline(always)]
    pub fn ee3pol(&self) -> Ee3polR {
        Ee3polR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - External Event 3 Sensitivity"]
    #[inline(always)]
    pub fn ee3sns(&self) -> Ee3snsR {
        Ee3snsR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - External Event 3 Fast mode"]
    #[inline(always)]
    pub fn ee3fast(&self) -> Ee3fastR {
        Ee3fastR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - External Event 4 Source"]
    #[inline(always)]
    pub fn ee4src(&self) -> Ee4srcR {
        Ee4srcR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - External Event 4 Polarity"]
    #[inline(always)]
    pub fn ee4pol(&self) -> Ee4polR {
        Ee4polR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - External Event 4 Sensitivity"]
    #[inline(always)]
    pub fn ee4sns(&self) -> Ee4snsR {
        Ee4snsR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - External Event 4 Fast mode"]
    #[inline(always)]
    pub fn ee4fast(&self) -> Ee4fastR {
        Ee4fastR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - External Event 5 Source"]
    #[inline(always)]
    pub fn ee5src(&self) -> Ee5srcR {
        Ee5srcR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - External Event 5 Polarity"]
    #[inline(always)]
    pub fn ee5pol(&self) -> Ee5polR {
        Ee5polR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - External Event 5 Sensitivity"]
    #[inline(always)]
    pub fn ee5sns(&self) -> Ee5snsR {
        Ee5snsR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - External Event 5 Fast mode"]
    #[inline(always)]
    pub fn ee5fast(&self) -> Ee5fastR {
        Ee5fastR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EECR1")
            .field("ee5fast", &self.ee5fast())
            .field("ee5sns", &self.ee5sns())
            .field("ee5pol", &self.ee5pol())
            .field("ee5src", &self.ee5src())
            .field("ee4fast", &self.ee4fast())
            .field("ee4sns", &self.ee4sns())
            .field("ee4pol", &self.ee4pol())
            .field("ee4src", &self.ee4src())
            .field("ee3fast", &self.ee3fast())
            .field("ee3sns", &self.ee3sns())
            .field("ee3pol", &self.ee3pol())
            .field("ee3src", &self.ee3src())
            .field("ee2fast", &self.ee2fast())
            .field("ee2sns", &self.ee2sns())
            .field("ee2pol", &self.ee2pol())
            .field("ee2src", &self.ee2src())
            .field("ee1fast", &self.ee1fast())
            .field("ee1sns", &self.ee1sns())
            .field("ee1pol", &self.ee1pol())
            .field("ee1src", &self.ee1src())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - External Event 1 Source"]
    #[inline(always)]
    #[must_use]
    pub fn ee1src(&mut self) -> Ee1srcW<Eecr1Spec> {
        Ee1srcW::new(self, 0)
    }
    #[doc = "Bit 2 - External Event 1 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ee1pol(&mut self) -> Ee1polW<Eecr1Spec> {
        Ee1polW::new(self, 2)
    }
    #[doc = "Bits 3:4 - External Event 1 Sensitivity"]
    #[inline(always)]
    #[must_use]
    pub fn ee1sns(&mut self) -> Ee1snsW<Eecr1Spec> {
        Ee1snsW::new(self, 3)
    }
    #[doc = "Bit 5 - External Event 1 Fast mode"]
    #[inline(always)]
    #[must_use]
    pub fn ee1fast(&mut self) -> Ee1fastW<Eecr1Spec> {
        Ee1fastW::new(self, 5)
    }
    #[doc = "Bits 6:7 - External Event 2 Source"]
    #[inline(always)]
    #[must_use]
    pub fn ee2src(&mut self) -> Ee2srcW<Eecr1Spec> {
        Ee2srcW::new(self, 6)
    }
    #[doc = "Bit 8 - External Event 2 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ee2pol(&mut self) -> Ee2polW<Eecr1Spec> {
        Ee2polW::new(self, 8)
    }
    #[doc = "Bits 9:10 - External Event 2 Sensitivity"]
    #[inline(always)]
    #[must_use]
    pub fn ee2sns(&mut self) -> Ee2snsW<Eecr1Spec> {
        Ee2snsW::new(self, 9)
    }
    #[doc = "Bit 11 - External Event 2 Fast mode"]
    #[inline(always)]
    #[must_use]
    pub fn ee2fast(&mut self) -> Ee2fastW<Eecr1Spec> {
        Ee2fastW::new(self, 11)
    }
    #[doc = "Bits 12:13 - External Event 3 Source"]
    #[inline(always)]
    #[must_use]
    pub fn ee3src(&mut self) -> Ee3srcW<Eecr1Spec> {
        Ee3srcW::new(self, 12)
    }
    #[doc = "Bit 14 - External Event 3 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ee3pol(&mut self) -> Ee3polW<Eecr1Spec> {
        Ee3polW::new(self, 14)
    }
    #[doc = "Bits 15:16 - External Event 3 Sensitivity"]
    #[inline(always)]
    #[must_use]
    pub fn ee3sns(&mut self) -> Ee3snsW<Eecr1Spec> {
        Ee3snsW::new(self, 15)
    }
    #[doc = "Bit 17 - External Event 3 Fast mode"]
    #[inline(always)]
    #[must_use]
    pub fn ee3fast(&mut self) -> Ee3fastW<Eecr1Spec> {
        Ee3fastW::new(self, 17)
    }
    #[doc = "Bits 18:19 - External Event 4 Source"]
    #[inline(always)]
    #[must_use]
    pub fn ee4src(&mut self) -> Ee4srcW<Eecr1Spec> {
        Ee4srcW::new(self, 18)
    }
    #[doc = "Bit 20 - External Event 4 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ee4pol(&mut self) -> Ee4polW<Eecr1Spec> {
        Ee4polW::new(self, 20)
    }
    #[doc = "Bits 21:22 - External Event 4 Sensitivity"]
    #[inline(always)]
    #[must_use]
    pub fn ee4sns(&mut self) -> Ee4snsW<Eecr1Spec> {
        Ee4snsW::new(self, 21)
    }
    #[doc = "Bit 23 - External Event 4 Fast mode"]
    #[inline(always)]
    #[must_use]
    pub fn ee4fast(&mut self) -> Ee4fastW<Eecr1Spec> {
        Ee4fastW::new(self, 23)
    }
    #[doc = "Bits 24:25 - External Event 5 Source"]
    #[inline(always)]
    #[must_use]
    pub fn ee5src(&mut self) -> Ee5srcW<Eecr1Spec> {
        Ee5srcW::new(self, 24)
    }
    #[doc = "Bit 26 - External Event 5 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ee5pol(&mut self) -> Ee5polW<Eecr1Spec> {
        Ee5polW::new(self, 26)
    }
    #[doc = "Bits 27:28 - External Event 5 Sensitivity"]
    #[inline(always)]
    #[must_use]
    pub fn ee5sns(&mut self) -> Ee5snsW<Eecr1Spec> {
        Ee5snsW::new(self, 27)
    }
    #[doc = "Bit 29 - External Event 5 Fast mode"]
    #[inline(always)]
    #[must_use]
    pub fn ee5fast(&mut self) -> Ee5fastW<Eecr1Spec> {
        Ee5fastW::new(self, 29)
    }
}
#[doc = "Timer External Event Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eecr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eecr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eecr1Spec;
impl crate::RegisterSpec for Eecr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eecr1::R`](R) reader structure"]
impl crate::Readable for Eecr1Spec {}
#[doc = "`write(|w| ..)` method takes [`eecr1::W`](W) writer structure"]
impl crate::Writable for Eecr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EECR1 to value 0"]
impl crate::Resettable for Eecr1Spec {
    const RESET_VALUE: u32 = 0;
}

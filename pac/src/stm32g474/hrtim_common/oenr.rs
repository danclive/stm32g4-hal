#[doc = "Register `OENR` reader"]
pub type R = crate::R<OenrSpec>;
#[doc = "Register `OENR` writer"]
pub type W = crate::W<OenrSpec>;
#[doc = "Field `TA1OEN` reader - Timer A Output 1 Enable"]
pub type Ta1oenR = crate::BitReader;
#[doc = "Field `TA1OEN` writer - Timer A Output 1 Enable"]
pub type Ta1oenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TA2OEN` reader - Timer A Output 2 Enable"]
pub type Ta2oenR = crate::BitReader;
#[doc = "Field `TA2OEN` writer - Timer A Output 2 Enable"]
pub type Ta2oenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TB1OEN` reader - Timer B Output 1 Enable"]
pub type Tb1oenR = crate::BitReader;
#[doc = "Field `TB1OEN` writer - Timer B Output 1 Enable"]
pub type Tb1oenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TB2OEN` reader - Timer B Output 2 Enable"]
pub type Tb2oenR = crate::BitReader;
#[doc = "Field `TB2OEN` writer - Timer B Output 2 Enable"]
pub type Tb2oenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1OEN` reader - Timer C Output 1 Enable"]
pub type Tc1oenR = crate::BitReader;
#[doc = "Field `TC1OEN` writer - Timer C Output 1 Enable"]
pub type Tc1oenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2OEN` reader - Timer C Output 2 Enable"]
pub type Tc2oenR = crate::BitReader;
#[doc = "Field `TC2OEN` writer - Timer C Output 2 Enable"]
pub type Tc2oenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TD1OEN` reader - Timer D Output 1 Enable"]
pub type Td1oenR = crate::BitReader;
#[doc = "Field `TD1OEN` writer - Timer D Output 1 Enable"]
pub type Td1oenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TD2OEN` reader - Timer D Output 2 Enable"]
pub type Td2oenR = crate::BitReader;
#[doc = "Field `TD2OEN` writer - Timer D Output 2 Enable"]
pub type Td2oenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE1OEN` reader - Timer E Output 1 Enable"]
pub type Te1oenR = crate::BitReader;
#[doc = "Field `TE1OEN` writer - Timer E Output 1 Enable"]
pub type Te1oenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE2OEN` reader - Timer E Output 2 Enable"]
pub type Te2oenR = crate::BitReader;
#[doc = "Field `TE2OEN` writer - Timer E Output 2 Enable"]
pub type Te2oenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TF1ODS` reader - Timer F Output 1 disable status"]
pub type Tf1odsR = crate::BitReader;
#[doc = "Field `TF1ODS` writer - Timer F Output 1 disable status"]
pub type Tf1odsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TF2ODS` reader - Timer F Output 2 disable status"]
pub type Tf2odsR = crate::BitReader;
#[doc = "Field `TF2ODS` writer - Timer F Output 2 disable status"]
pub type Tf2odsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer A Output 1 Enable"]
    #[inline(always)]
    pub fn ta1oen(&self) -> Ta1oenR {
        Ta1oenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer A Output 2 Enable"]
    #[inline(always)]
    pub fn ta2oen(&self) -> Ta2oenR {
        Ta2oenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer B Output 1 Enable"]
    #[inline(always)]
    pub fn tb1oen(&self) -> Tb1oenR {
        Tb1oenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer B Output 2 Enable"]
    #[inline(always)]
    pub fn tb2oen(&self) -> Tb2oenR {
        Tb2oenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer C Output 1 Enable"]
    #[inline(always)]
    pub fn tc1oen(&self) -> Tc1oenR {
        Tc1oenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer C Output 2 Enable"]
    #[inline(always)]
    pub fn tc2oen(&self) -> Tc2oenR {
        Tc2oenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer D Output 1 Enable"]
    #[inline(always)]
    pub fn td1oen(&self) -> Td1oenR {
        Td1oenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer D Output 2 Enable"]
    #[inline(always)]
    pub fn td2oen(&self) -> Td2oenR {
        Td2oenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer E Output 1 Enable"]
    #[inline(always)]
    pub fn te1oen(&self) -> Te1oenR {
        Te1oenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timer E Output 2 Enable"]
    #[inline(always)]
    pub fn te2oen(&self) -> Te2oenR {
        Te2oenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Timer F Output 1 disable status"]
    #[inline(always)]
    pub fn tf1ods(&self) -> Tf1odsR {
        Tf1odsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer F Output 2 disable status"]
    #[inline(always)]
    pub fn tf2ods(&self) -> Tf2odsR {
        Tf2odsR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OENR")
            .field("tf2ods", &self.tf2ods())
            .field("tf1ods", &self.tf1ods())
            .field("te2oen", &self.te2oen())
            .field("te1oen", &self.te1oen())
            .field("td2oen", &self.td2oen())
            .field("td1oen", &self.td1oen())
            .field("tc2oen", &self.tc2oen())
            .field("tc1oen", &self.tc1oen())
            .field("tb2oen", &self.tb2oen())
            .field("tb1oen", &self.tb1oen())
            .field("ta2oen", &self.ta2oen())
            .field("ta1oen", &self.ta1oen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timer A Output 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ta1oen(&mut self) -> Ta1oenW<OenrSpec> {
        Ta1oenW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer A Output 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ta2oen(&mut self) -> Ta2oenW<OenrSpec> {
        Ta2oenW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer B Output 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tb1oen(&mut self) -> Tb1oenW<OenrSpec> {
        Tb1oenW::new(self, 2)
    }
    #[doc = "Bit 3 - Timer B Output 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tb2oen(&mut self) -> Tb2oenW<OenrSpec> {
        Tb2oenW::new(self, 3)
    }
    #[doc = "Bit 4 - Timer C Output 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc1oen(&mut self) -> Tc1oenW<OenrSpec> {
        Tc1oenW::new(self, 4)
    }
    #[doc = "Bit 5 - Timer C Output 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc2oen(&mut self) -> Tc2oenW<OenrSpec> {
        Tc2oenW::new(self, 5)
    }
    #[doc = "Bit 6 - Timer D Output 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn td1oen(&mut self) -> Td1oenW<OenrSpec> {
        Td1oenW::new(self, 6)
    }
    #[doc = "Bit 7 - Timer D Output 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn td2oen(&mut self) -> Td2oenW<OenrSpec> {
        Td2oenW::new(self, 7)
    }
    #[doc = "Bit 8 - Timer E Output 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn te1oen(&mut self) -> Te1oenW<OenrSpec> {
        Te1oenW::new(self, 8)
    }
    #[doc = "Bit 9 - Timer E Output 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn te2oen(&mut self) -> Te2oenW<OenrSpec> {
        Te2oenW::new(self, 9)
    }
    #[doc = "Bit 10 - Timer F Output 1 disable status"]
    #[inline(always)]
    #[must_use]
    pub fn tf1ods(&mut self) -> Tf1odsW<OenrSpec> {
        Tf1odsW::new(self, 10)
    }
    #[doc = "Bit 11 - Timer F Output 2 disable status"]
    #[inline(always)]
    #[must_use]
    pub fn tf2ods(&mut self) -> Tf2odsW<OenrSpec> {
        Tf2odsW::new(self, 11)
    }
}
#[doc = "Output Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OenrSpec;
impl crate::RegisterSpec for OenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oenr::R`](R) reader structure"]
impl crate::Readable for OenrSpec {}
#[doc = "`write(|w| ..)` method takes [`oenr::W`](W) writer structure"]
impl crate::Writable for OenrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OENR to value 0"]
impl crate::Resettable for OenrSpec {
    const RESET_VALUE: u32 = 0;
}

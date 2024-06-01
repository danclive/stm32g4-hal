#[doc = "Register `CPT2ACR` reader"]
pub type R = crate::R<Cpt2acrSpec>;
#[doc = "Register `CPT2ACR` writer"]
pub type W = crate::W<Cpt2acrSpec>;
#[doc = "Field `SWCPT` reader - Software Capture"]
pub type SwcptR = crate::BitReader;
#[doc = "Field `SWCPT` writer - Software Capture"]
pub type SwcptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDPCPT` reader - Update Capture"]
pub type UdpcptR = crate::BitReader;
#[doc = "Field `UDPCPT` writer - Update Capture"]
pub type UdpcptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXEV1CPT` reader - External Event 1 Capture"]
pub type Exev1cptR = crate::BitReader;
#[doc = "Field `EXEV1CPT` writer - External Event 1 Capture"]
pub type Exev1cptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXEV2CPT` reader - External Event 2 Capture"]
pub type Exev2cptR = crate::BitReader;
#[doc = "Field `EXEV2CPT` writer - External Event 2 Capture"]
pub type Exev2cptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXEV3CPT` reader - External Event 3 Capture"]
pub type Exev3cptR = crate::BitReader;
#[doc = "Field `EXEV3CPT` writer - External Event 3 Capture"]
pub type Exev3cptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXEV4CPT` reader - External Event 4 Capture"]
pub type Exev4cptR = crate::BitReader;
#[doc = "Field `EXEV4CPT` writer - External Event 4 Capture"]
pub type Exev4cptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXEV5CPT` reader - External Event 5 Capture"]
pub type Exev5cptR = crate::BitReader;
#[doc = "Field `EXEV5CPT` writer - External Event 5 Capture"]
pub type Exev5cptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXEV6CPT` reader - External Event 6 Capture"]
pub type Exev6cptR = crate::BitReader;
#[doc = "Field `EXEV6CPT` writer - External Event 6 Capture"]
pub type Exev6cptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXEV7CPT` reader - External Event 7 Capture"]
pub type Exev7cptR = crate::BitReader;
#[doc = "Field `EXEV7CPT` writer - External Event 7 Capture"]
pub type Exev7cptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXEV8CPT` reader - External Event 8 Capture"]
pub type Exev8cptR = crate::BitReader;
#[doc = "Field `EXEV8CPT` writer - External Event 8 Capture"]
pub type Exev8cptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXEV9CPT` reader - External Event 9 Capture"]
pub type Exev9cptR = crate::BitReader;
#[doc = "Field `EXEV9CPT` writer - External Event 9 Capture"]
pub type Exev9cptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXEV10CPT` reader - External Event 10 Capture"]
pub type Exev10cptR = crate::BitReader;
#[doc = "Field `EXEV10CPT` writer - External Event 10 Capture"]
pub type Exev10cptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TF1SET` reader - TF1SET"]
pub type Tf1setR = crate::BitReader;
#[doc = "Field `TF1SET` writer - TF1SET"]
pub type Tf1setW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TF1RST` reader - TF1RST"]
pub type Tf1rstR = crate::BitReader;
#[doc = "Field `TF1RST` writer - TF1RST"]
pub type Tf1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFCMP1` reader - TFCMP1"]
pub type Tfcmp1R = crate::BitReader;
#[doc = "Field `TFCMP1` writer - TFCMP1"]
pub type Tfcmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFCMP2` reader - TFCMP2"]
pub type Tfcmp2R = crate::BitReader;
#[doc = "Field `TFCMP2` writer - TFCMP2"]
pub type Tfcmp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TB1SET` reader - Timer B output 1 Set"]
pub type Tb1setR = crate::BitReader;
#[doc = "Field `TB1SET` writer - Timer B output 1 Set"]
pub type Tb1setW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TB1RST` reader - Timer B output 1 Reset"]
pub type Tb1rstR = crate::BitReader;
#[doc = "Field `TB1RST` writer - Timer B output 1 Reset"]
pub type Tb1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBCMP1` reader - Timer B Compare 1"]
pub type Tbcmp1R = crate::BitReader;
#[doc = "Field `TBCMP1` writer - Timer B Compare 1"]
pub type Tbcmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBCMP2` reader - Timer B Compare 2"]
pub type Tbcmp2R = crate::BitReader;
#[doc = "Field `TBCMP2` writer - Timer B Compare 2"]
pub type Tbcmp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1SET` reader - Timer C output 1 Set"]
pub type Tc1setR = crate::BitReader;
#[doc = "Field `TC1SET` writer - Timer C output 1 Set"]
pub type Tc1setW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1RST` reader - Timer C output 1 Reset"]
pub type Tc1rstR = crate::BitReader;
#[doc = "Field `TC1RST` writer - Timer C output 1 Reset"]
pub type Tc1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCMP1` reader - Timer C Compare 1"]
pub type Tccmp1R = crate::BitReader;
#[doc = "Field `TCCMP1` writer - Timer C Compare 1"]
pub type Tccmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCMP2` reader - Timer C Compare 2"]
pub type Tccmp2R = crate::BitReader;
#[doc = "Field `TCCMP2` writer - Timer C Compare 2"]
pub type Tccmp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TD1SET` reader - Timer D output 1 Set"]
pub type Td1setR = crate::BitReader;
#[doc = "Field `TD1SET` writer - Timer D output 1 Set"]
pub type Td1setW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TD1RST` reader - Timer D output 1 Reset"]
pub type Td1rstR = crate::BitReader;
#[doc = "Field `TD1RST` writer - Timer D output 1 Reset"]
pub type Td1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDCMP1` reader - Timer D Compare 1"]
pub type Tdcmp1R = crate::BitReader;
#[doc = "Field `TDCMP1` writer - Timer D Compare 1"]
pub type Tdcmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDCMP2` reader - Timer D Compare 2"]
pub type Tdcmp2R = crate::BitReader;
#[doc = "Field `TDCMP2` writer - Timer D Compare 2"]
pub type Tdcmp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE1SET` reader - Timer E output 1 Set"]
pub type Te1setR = crate::BitReader;
#[doc = "Field `TE1SET` writer - Timer E output 1 Set"]
pub type Te1setW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE1RST` reader - Timer E output 1 Reset"]
pub type Te1rstR = crate::BitReader;
#[doc = "Field `TE1RST` writer - Timer E output 1 Reset"]
pub type Te1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TECMP1` reader - Timer E Compare 1"]
pub type Tecmp1R = crate::BitReader;
#[doc = "Field `TECMP1` writer - Timer E Compare 1"]
pub type Tecmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TECMP2` reader - Timer E Compare 2"]
pub type Tecmp2R = crate::BitReader;
#[doc = "Field `TECMP2` writer - Timer E Compare 2"]
pub type Tecmp2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Capture"]
    #[inline(always)]
    pub fn swcpt(&self) -> SwcptR {
        SwcptR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update Capture"]
    #[inline(always)]
    pub fn udpcpt(&self) -> UdpcptR {
        UdpcptR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Event 1 Capture"]
    #[inline(always)]
    pub fn exev1cpt(&self) -> Exev1cptR {
        Exev1cptR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Event 2 Capture"]
    #[inline(always)]
    pub fn exev2cpt(&self) -> Exev2cptR {
        Exev2cptR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Event 3 Capture"]
    #[inline(always)]
    pub fn exev3cpt(&self) -> Exev3cptR {
        Exev3cptR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Event 4 Capture"]
    #[inline(always)]
    pub fn exev4cpt(&self) -> Exev4cptR {
        Exev4cptR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Event 5 Capture"]
    #[inline(always)]
    pub fn exev5cpt(&self) -> Exev5cptR {
        Exev5cptR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Event 6 Capture"]
    #[inline(always)]
    pub fn exev6cpt(&self) -> Exev6cptR {
        Exev6cptR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - External Event 7 Capture"]
    #[inline(always)]
    pub fn exev7cpt(&self) -> Exev7cptR {
        Exev7cptR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External Event 8 Capture"]
    #[inline(always)]
    pub fn exev8cpt(&self) -> Exev8cptR {
        Exev8cptR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External Event 9 Capture"]
    #[inline(always)]
    pub fn exev9cpt(&self) -> Exev9cptR {
        Exev9cptR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External Event 10 Capture"]
    #[inline(always)]
    pub fn exev10cpt(&self) -> Exev10cptR {
        Exev10cptR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TF1SET"]
    #[inline(always)]
    pub fn tf1set(&self) -> Tf1setR {
        Tf1setR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TF1RST"]
    #[inline(always)]
    pub fn tf1rst(&self) -> Tf1rstR {
        Tf1rstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TFCMP1"]
    #[inline(always)]
    pub fn tfcmp1(&self) -> Tfcmp1R {
        Tfcmp1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TFCMP2"]
    #[inline(always)]
    pub fn tfcmp2(&self) -> Tfcmp2R {
        Tfcmp2R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer B output 1 Set"]
    #[inline(always)]
    pub fn tb1set(&self) -> Tb1setR {
        Tb1setR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer B output 1 Reset"]
    #[inline(always)]
    pub fn tb1rst(&self) -> Tb1rstR {
        Tb1rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer B Compare 1"]
    #[inline(always)]
    pub fn tbcmp1(&self) -> Tbcmp1R {
        Tbcmp1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer B Compare 2"]
    #[inline(always)]
    pub fn tbcmp2(&self) -> Tbcmp2R {
        Tbcmp2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer C output 1 Set"]
    #[inline(always)]
    pub fn tc1set(&self) -> Tc1setR {
        Tc1setR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer C output 1 Reset"]
    #[inline(always)]
    pub fn tc1rst(&self) -> Tc1rstR {
        Tc1rstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timer C Compare 1"]
    #[inline(always)]
    pub fn tccmp1(&self) -> Tccmp1R {
        Tccmp1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Timer C Compare 2"]
    #[inline(always)]
    pub fn tccmp2(&self) -> Tccmp2R {
        Tccmp2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Timer D output 1 Set"]
    #[inline(always)]
    pub fn td1set(&self) -> Td1setR {
        Td1setR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Timer D output 1 Reset"]
    #[inline(always)]
    pub fn td1rst(&self) -> Td1rstR {
        Td1rstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Timer D Compare 1"]
    #[inline(always)]
    pub fn tdcmp1(&self) -> Tdcmp1R {
        Tdcmp1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Timer D Compare 2"]
    #[inline(always)]
    pub fn tdcmp2(&self) -> Tdcmp2R {
        Tdcmp2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Timer E output 1 Set"]
    #[inline(always)]
    pub fn te1set(&self) -> Te1setR {
        Te1setR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Timer E output 1 Reset"]
    #[inline(always)]
    pub fn te1rst(&self) -> Te1rstR {
        Te1rstR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Timer E Compare 1"]
    #[inline(always)]
    pub fn tecmp1(&self) -> Tecmp1R {
        Tecmp1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Timer E Compare 2"]
    #[inline(always)]
    pub fn tecmp2(&self) -> Tecmp2R {
        Tecmp2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPT2ACR")
            .field("tecmp2", &self.tecmp2())
            .field("tecmp1", &self.tecmp1())
            .field("te1rst", &self.te1rst())
            .field("te1set", &self.te1set())
            .field("tdcmp2", &self.tdcmp2())
            .field("tdcmp1", &self.tdcmp1())
            .field("td1rst", &self.td1rst())
            .field("td1set", &self.td1set())
            .field("tccmp2", &self.tccmp2())
            .field("tccmp1", &self.tccmp1())
            .field("tc1rst", &self.tc1rst())
            .field("tc1set", &self.tc1set())
            .field("tbcmp2", &self.tbcmp2())
            .field("tbcmp1", &self.tbcmp1())
            .field("tb1rst", &self.tb1rst())
            .field("tb1set", &self.tb1set())
            .field("tfcmp2", &self.tfcmp2())
            .field("tfcmp1", &self.tfcmp1())
            .field("tf1rst", &self.tf1rst())
            .field("tf1set", &self.tf1set())
            .field("exev10cpt", &self.exev10cpt())
            .field("exev9cpt", &self.exev9cpt())
            .field("exev8cpt", &self.exev8cpt())
            .field("exev7cpt", &self.exev7cpt())
            .field("exev6cpt", &self.exev6cpt())
            .field("exev5cpt", &self.exev5cpt())
            .field("exev4cpt", &self.exev4cpt())
            .field("exev3cpt", &self.exev3cpt())
            .field("exev2cpt", &self.exev2cpt())
            .field("exev1cpt", &self.exev1cpt())
            .field("udpcpt", &self.udpcpt())
            .field("swcpt", &self.swcpt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Software Capture"]
    #[inline(always)]
    #[must_use]
    pub fn swcpt(&mut self) -> SwcptW<Cpt2acrSpec> {
        SwcptW::new(self, 0)
    }
    #[doc = "Bit 1 - Update Capture"]
    #[inline(always)]
    #[must_use]
    pub fn udpcpt(&mut self) -> UdpcptW<Cpt2acrSpec> {
        UdpcptW::new(self, 1)
    }
    #[doc = "Bit 2 - External Event 1 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev1cpt(&mut self) -> Exev1cptW<Cpt2acrSpec> {
        Exev1cptW::new(self, 2)
    }
    #[doc = "Bit 3 - External Event 2 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev2cpt(&mut self) -> Exev2cptW<Cpt2acrSpec> {
        Exev2cptW::new(self, 3)
    }
    #[doc = "Bit 4 - External Event 3 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev3cpt(&mut self) -> Exev3cptW<Cpt2acrSpec> {
        Exev3cptW::new(self, 4)
    }
    #[doc = "Bit 5 - External Event 4 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev4cpt(&mut self) -> Exev4cptW<Cpt2acrSpec> {
        Exev4cptW::new(self, 5)
    }
    #[doc = "Bit 6 - External Event 5 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev5cpt(&mut self) -> Exev5cptW<Cpt2acrSpec> {
        Exev5cptW::new(self, 6)
    }
    #[doc = "Bit 7 - External Event 6 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev6cpt(&mut self) -> Exev6cptW<Cpt2acrSpec> {
        Exev6cptW::new(self, 7)
    }
    #[doc = "Bit 8 - External Event 7 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev7cpt(&mut self) -> Exev7cptW<Cpt2acrSpec> {
        Exev7cptW::new(self, 8)
    }
    #[doc = "Bit 9 - External Event 8 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev8cpt(&mut self) -> Exev8cptW<Cpt2acrSpec> {
        Exev8cptW::new(self, 9)
    }
    #[doc = "Bit 10 - External Event 9 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev9cpt(&mut self) -> Exev9cptW<Cpt2acrSpec> {
        Exev9cptW::new(self, 10)
    }
    #[doc = "Bit 11 - External Event 10 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev10cpt(&mut self) -> Exev10cptW<Cpt2acrSpec> {
        Exev10cptW::new(self, 11)
    }
    #[doc = "Bit 12 - TF1SET"]
    #[inline(always)]
    #[must_use]
    pub fn tf1set(&mut self) -> Tf1setW<Cpt2acrSpec> {
        Tf1setW::new(self, 12)
    }
    #[doc = "Bit 13 - TF1RST"]
    #[inline(always)]
    #[must_use]
    pub fn tf1rst(&mut self) -> Tf1rstW<Cpt2acrSpec> {
        Tf1rstW::new(self, 13)
    }
    #[doc = "Bit 14 - TFCMP1"]
    #[inline(always)]
    #[must_use]
    pub fn tfcmp1(&mut self) -> Tfcmp1W<Cpt2acrSpec> {
        Tfcmp1W::new(self, 14)
    }
    #[doc = "Bit 15 - TFCMP2"]
    #[inline(always)]
    #[must_use]
    pub fn tfcmp2(&mut self) -> Tfcmp2W<Cpt2acrSpec> {
        Tfcmp2W::new(self, 15)
    }
    #[doc = "Bit 16 - Timer B output 1 Set"]
    #[inline(always)]
    #[must_use]
    pub fn tb1set(&mut self) -> Tb1setW<Cpt2acrSpec> {
        Tb1setW::new(self, 16)
    }
    #[doc = "Bit 17 - Timer B output 1 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn tb1rst(&mut self) -> Tb1rstW<Cpt2acrSpec> {
        Tb1rstW::new(self, 17)
    }
    #[doc = "Bit 18 - Timer B Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn tbcmp1(&mut self) -> Tbcmp1W<Cpt2acrSpec> {
        Tbcmp1W::new(self, 18)
    }
    #[doc = "Bit 19 - Timer B Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn tbcmp2(&mut self) -> Tbcmp2W<Cpt2acrSpec> {
        Tbcmp2W::new(self, 19)
    }
    #[doc = "Bit 20 - Timer C output 1 Set"]
    #[inline(always)]
    #[must_use]
    pub fn tc1set(&mut self) -> Tc1setW<Cpt2acrSpec> {
        Tc1setW::new(self, 20)
    }
    #[doc = "Bit 21 - Timer C output 1 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn tc1rst(&mut self) -> Tc1rstW<Cpt2acrSpec> {
        Tc1rstW::new(self, 21)
    }
    #[doc = "Bit 22 - Timer C Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn tccmp1(&mut self) -> Tccmp1W<Cpt2acrSpec> {
        Tccmp1W::new(self, 22)
    }
    #[doc = "Bit 23 - Timer C Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn tccmp2(&mut self) -> Tccmp2W<Cpt2acrSpec> {
        Tccmp2W::new(self, 23)
    }
    #[doc = "Bit 24 - Timer D output 1 Set"]
    #[inline(always)]
    #[must_use]
    pub fn td1set(&mut self) -> Td1setW<Cpt2acrSpec> {
        Td1setW::new(self, 24)
    }
    #[doc = "Bit 25 - Timer D output 1 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn td1rst(&mut self) -> Td1rstW<Cpt2acrSpec> {
        Td1rstW::new(self, 25)
    }
    #[doc = "Bit 26 - Timer D Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn tdcmp1(&mut self) -> Tdcmp1W<Cpt2acrSpec> {
        Tdcmp1W::new(self, 26)
    }
    #[doc = "Bit 27 - Timer D Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn tdcmp2(&mut self) -> Tdcmp2W<Cpt2acrSpec> {
        Tdcmp2W::new(self, 27)
    }
    #[doc = "Bit 28 - Timer E output 1 Set"]
    #[inline(always)]
    #[must_use]
    pub fn te1set(&mut self) -> Te1setW<Cpt2acrSpec> {
        Te1setW::new(self, 28)
    }
    #[doc = "Bit 29 - Timer E output 1 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn te1rst(&mut self) -> Te1rstW<Cpt2acrSpec> {
        Te1rstW::new(self, 29)
    }
    #[doc = "Bit 30 - Timer E Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn tecmp1(&mut self) -> Tecmp1W<Cpt2acrSpec> {
        Tecmp1W::new(self, 30)
    }
    #[doc = "Bit 31 - Timer E Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn tecmp2(&mut self) -> Tecmp2W<Cpt2acrSpec> {
        Tecmp2W::new(self, 31)
    }
}
#[doc = "CPT2xCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt2acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpt2acrSpec;
impl crate::RegisterSpec for Cpt2acrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpt2acr::R`](R) reader structure"]
impl crate::Readable for Cpt2acrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpt2acr::W`](W) writer structure"]
impl crate::Writable for Cpt2acrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPT2ACR to value 0"]
impl crate::Resettable for Cpt2acrSpec {
    const RESET_VALUE: u32 = 0;
}

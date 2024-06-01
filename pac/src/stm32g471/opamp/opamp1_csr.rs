#[doc = "Register `OPAMP1_CSR` reader"]
pub type R = crate::R<Opamp1CsrSpec>;
#[doc = "Register `OPAMP1_CSR` writer"]
pub type W = crate::W<Opamp1CsrSpec>;
#[doc = "Field `OPAEN` reader - Operational amplifier Enable"]
pub type OpaenR = crate::BitReader;
#[doc = "Field `OPAEN` writer - Operational amplifier Enable"]
pub type OpaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_VP` reader - FORCE_VP"]
pub type ForceVpR = crate::BitReader;
#[doc = "Field `FORCE_VP` writer - FORCE_VP"]
pub type ForceVpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VP_SEL` reader - VP_SEL"]
pub type VpSelR = crate::FieldReader;
#[doc = "Field `VP_SEL` writer - VP_SEL"]
pub type VpSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USERTRIM` reader - USERTRIM"]
pub type UsertrimR = crate::BitReader;
#[doc = "Field `USERTRIM` writer - USERTRIM"]
pub type UsertrimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VM_SEL` reader - VM_SEL"]
pub type VmSelR = crate::FieldReader;
#[doc = "Field `VM_SEL` writer - VM_SEL"]
pub type VmSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OPAHSM` reader - OPAHSM"]
pub type OpahsmR = crate::BitReader;
#[doc = "Field `OPAHSM` writer - OPAHSM"]
pub type OpahsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAINTOEN` reader - OPAINTOEN"]
pub type OpaintoenR = crate::BitReader;
#[doc = "Field `OPAINTOEN` writer - OPAINTOEN"]
pub type OpaintoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALON` reader - CALON"]
pub type CalonR = crate::BitReader;
#[doc = "Field `CALON` writer - CALON"]
pub type CalonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALSEL` reader - CALSEL"]
pub type CalselR = crate::FieldReader;
#[doc = "Field `CALSEL` writer - CALSEL"]
pub type CalselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PGA_GAIN` reader - PGA_GAIN"]
pub type PgaGainR = crate::FieldReader;
#[doc = "Field `PGA_GAIN` writer - PGA_GAIN"]
pub type PgaGainW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIMOFFSETP` reader - TRIMOFFSETP"]
pub type TrimoffsetpR = crate::FieldReader;
#[doc = "Field `TRIMOFFSETP` writer - TRIMOFFSETP"]
pub type TrimoffsetpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIMOFFSETN` reader - TRIMOFFSETN"]
pub type TrimoffsetnR = crate::FieldReader;
#[doc = "Field `TRIMOFFSETN` writer - TRIMOFFSETN"]
pub type TrimoffsetnW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CALOUT` reader - CALOUT"]
pub type CaloutR = crate::BitReader;
#[doc = "Field `CALOUT` writer - CALOUT"]
pub type CaloutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - LOCK"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - LOCK"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    pub fn opaen(&self) -> OpaenR {
        OpaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    pub fn force_vp(&self) -> ForceVpR {
        ForceVpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - VP_SEL"]
    #[inline(always)]
    pub fn vp_sel(&self) -> VpSelR {
        VpSelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - USERTRIM"]
    #[inline(always)]
    pub fn usertrim(&self) -> UsertrimR {
        UsertrimR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - VM_SEL"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VmSelR {
        VmSelR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - OPAHSM"]
    #[inline(always)]
    pub fn opahsm(&self) -> OpahsmR {
        OpahsmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OPAINTOEN"]
    #[inline(always)]
    pub fn opaintoen(&self) -> OpaintoenR {
        OpaintoenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - CALON"]
    #[inline(always)]
    pub fn calon(&self) -> CalonR {
        CalonR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - CALSEL"]
    #[inline(always)]
    pub fn calsel(&self) -> CalselR {
        CalselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:18 - PGA_GAIN"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PgaGainR {
        PgaGainR::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - TRIMOFFSETP"]
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TrimoffsetpR {
        TrimoffsetpR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - TRIMOFFSETN"]
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TrimoffsetnR {
        TrimoffsetnR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - CALOUT"]
    #[inline(always)]
    pub fn calout(&self) -> CaloutR {
        CaloutR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPAMP1_CSR")
            .field("opaen", &self.opaen())
            .field("force_vp", &self.force_vp())
            .field("vp_sel", &self.vp_sel())
            .field("usertrim", &self.usertrim())
            .field("vm_sel", &self.vm_sel())
            .field("opahsm", &self.opahsm())
            .field("opaintoen", &self.opaintoen())
            .field("calon", &self.calon())
            .field("calsel", &self.calsel())
            .field("pga_gain", &self.pga_gain())
            .field("trimoffsetp", &self.trimoffsetp())
            .field("trimoffsetn", &self.trimoffsetn())
            .field("calout", &self.calout())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opaen(&mut self) -> OpaenW<Opamp1CsrSpec> {
        OpaenW::new(self, 0)
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    #[must_use]
    pub fn force_vp(&mut self) -> ForceVpW<Opamp1CsrSpec> {
        ForceVpW::new(self, 1)
    }
    #[doc = "Bits 2:3 - VP_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn vp_sel(&mut self) -> VpSelW<Opamp1CsrSpec> {
        VpSelW::new(self, 2)
    }
    #[doc = "Bit 4 - USERTRIM"]
    #[inline(always)]
    #[must_use]
    pub fn usertrim(&mut self) -> UsertrimW<Opamp1CsrSpec> {
        UsertrimW::new(self, 4)
    }
    #[doc = "Bits 5:6 - VM_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn vm_sel(&mut self) -> VmSelW<Opamp1CsrSpec> {
        VmSelW::new(self, 5)
    }
    #[doc = "Bit 7 - OPAHSM"]
    #[inline(always)]
    #[must_use]
    pub fn opahsm(&mut self) -> OpahsmW<Opamp1CsrSpec> {
        OpahsmW::new(self, 7)
    }
    #[doc = "Bit 8 - OPAINTOEN"]
    #[inline(always)]
    #[must_use]
    pub fn opaintoen(&mut self) -> OpaintoenW<Opamp1CsrSpec> {
        OpaintoenW::new(self, 8)
    }
    #[doc = "Bit 11 - CALON"]
    #[inline(always)]
    #[must_use]
    pub fn calon(&mut self) -> CalonW<Opamp1CsrSpec> {
        CalonW::new(self, 11)
    }
    #[doc = "Bits 12:13 - CALSEL"]
    #[inline(always)]
    #[must_use]
    pub fn calsel(&mut self) -> CalselW<Opamp1CsrSpec> {
        CalselW::new(self, 12)
    }
    #[doc = "Bits 14:18 - PGA_GAIN"]
    #[inline(always)]
    #[must_use]
    pub fn pga_gain(&mut self) -> PgaGainW<Opamp1CsrSpec> {
        PgaGainW::new(self, 14)
    }
    #[doc = "Bits 19:23 - TRIMOFFSETP"]
    #[inline(always)]
    #[must_use]
    pub fn trimoffsetp(&mut self) -> TrimoffsetpW<Opamp1CsrSpec> {
        TrimoffsetpW::new(self, 19)
    }
    #[doc = "Bits 24:28 - TRIMOFFSETN"]
    #[inline(always)]
    #[must_use]
    pub fn trimoffsetn(&mut self) -> TrimoffsetnW<Opamp1CsrSpec> {
        TrimoffsetnW::new(self, 24)
    }
    #[doc = "Bit 30 - CALOUT"]
    #[inline(always)]
    #[must_use]
    pub fn calout(&mut self) -> CaloutW<Opamp1CsrSpec> {
        CaloutW::new(self, 30)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<Opamp1CsrSpec> {
        LockW::new(self, 31)
    }
}
#[doc = "OPAMP1 control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp1_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp1_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Opamp1CsrSpec;
impl crate::RegisterSpec for Opamp1CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp1_csr::R`](R) reader structure"]
impl crate::Readable for Opamp1CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`opamp1_csr::W`](W) writer structure"]
impl crate::Writable for Opamp1CsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPAMP1_CSR to value 0"]
impl crate::Resettable for Opamp1CsrSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `OPAMP2_CSR` reader"]
pub type R = crate::R<Opamp2CsrSpec>;
#[doc = "Register `OPAMP2_CSR` writer"]
pub type W = crate::W<Opamp2CsrSpec>;
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
        f.debug_struct("OPAMP2_CSR")
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
    pub fn opaen(&mut self) -> OpaenW<'_, Opamp2CsrSpec> {
        OpaenW::new(self, 0)
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    pub fn force_vp(&mut self) -> ForceVpW<'_, Opamp2CsrSpec> {
        ForceVpW::new(self, 1)
    }
    #[doc = "Bits 2:3 - VP_SEL"]
    #[inline(always)]
    pub fn vp_sel(&mut self) -> VpSelW<'_, Opamp2CsrSpec> {
        VpSelW::new(self, 2)
    }
    #[doc = "Bit 4 - USERTRIM"]
    #[inline(always)]
    pub fn usertrim(&mut self) -> UsertrimW<'_, Opamp2CsrSpec> {
        UsertrimW::new(self, 4)
    }
    #[doc = "Bits 5:6 - VM_SEL"]
    #[inline(always)]
    pub fn vm_sel(&mut self) -> VmSelW<'_, Opamp2CsrSpec> {
        VmSelW::new(self, 5)
    }
    #[doc = "Bit 7 - OPAHSM"]
    #[inline(always)]
    pub fn opahsm(&mut self) -> OpahsmW<'_, Opamp2CsrSpec> {
        OpahsmW::new(self, 7)
    }
    #[doc = "Bit 8 - OPAINTOEN"]
    #[inline(always)]
    pub fn opaintoen(&mut self) -> OpaintoenW<'_, Opamp2CsrSpec> {
        OpaintoenW::new(self, 8)
    }
    #[doc = "Bit 11 - CALON"]
    #[inline(always)]
    pub fn calon(&mut self) -> CalonW<'_, Opamp2CsrSpec> {
        CalonW::new(self, 11)
    }
    #[doc = "Bits 12:13 - CALSEL"]
    #[inline(always)]
    pub fn calsel(&mut self) -> CalselW<'_, Opamp2CsrSpec> {
        CalselW::new(self, 12)
    }
    #[doc = "Bits 14:18 - PGA_GAIN"]
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PgaGainW<'_, Opamp2CsrSpec> {
        PgaGainW::new(self, 14)
    }
    #[doc = "Bits 19:23 - TRIMOFFSETP"]
    #[inline(always)]
    pub fn trimoffsetp(&mut self) -> TrimoffsetpW<'_, Opamp2CsrSpec> {
        TrimoffsetpW::new(self, 19)
    }
    #[doc = "Bits 24:28 - TRIMOFFSETN"]
    #[inline(always)]
    pub fn trimoffsetn(&mut self) -> TrimoffsetnW<'_, Opamp2CsrSpec> {
        TrimoffsetnW::new(self, 24)
    }
    #[doc = "Bit 30 - CALOUT"]
    #[inline(always)]
    pub fn calout(&mut self) -> CaloutW<'_, Opamp2CsrSpec> {
        CaloutW::new(self, 30)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, Opamp2CsrSpec> {
        LockW::new(self, 31)
    }
}
#[doc = "OPAMP2 control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp2_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Opamp2CsrSpec;
impl crate::RegisterSpec for Opamp2CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp2_csr::R`](R) reader structure"]
impl crate::Readable for Opamp2CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`opamp2_csr::W`](W) writer structure"]
impl crate::Writable for Opamp2CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPAMP2_CSR to value 0"]
impl crate::Resettable for Opamp2CsrSpec {}

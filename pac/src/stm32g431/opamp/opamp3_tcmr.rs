#[doc = "Register `OPAMP3_TCMR` reader"]
pub type R = crate::R<Opamp3TcmrSpec>;
#[doc = "Register `OPAMP3_TCMR` writer"]
pub type W = crate::W<Opamp3TcmrSpec>;
#[doc = "Field `VMS_SEL` reader - VMS_SEL"]
pub type VmsSelR = crate::BitReader;
#[doc = "Field `VMS_SEL` writer - VMS_SEL"]
pub type VmsSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPS_SEL` reader - VPS_SEL"]
pub type VpsSelR = crate::FieldReader;
#[doc = "Field `VPS_SEL` writer - VPS_SEL"]
pub type VpsSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `T1CM_EN` reader - T1CM_EN"]
pub type T1cmEnR = crate::BitReader;
#[doc = "Field `T1CM_EN` writer - T1CM_EN"]
pub type T1cmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T8CM_EN` reader - T8CM_EN"]
pub type T8cmEnR = crate::BitReader;
#[doc = "Field `T8CM_EN` writer - T8CM_EN"]
pub type T8cmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T20CM_EN` reader - T20CM_EN"]
pub type T20cmEnR = crate::BitReader;
#[doc = "Field `T20CM_EN` writer - T20CM_EN"]
pub type T20cmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - LOCK"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - LOCK"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VMS_SEL"]
    #[inline(always)]
    pub fn vms_sel(&self) -> VmsSelR {
        VmsSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - VPS_SEL"]
    #[inline(always)]
    pub fn vps_sel(&self) -> VpsSelR {
        VpsSelR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - T1CM_EN"]
    #[inline(always)]
    pub fn t1cm_en(&self) -> T1cmEnR {
        T1cmEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - T8CM_EN"]
    #[inline(always)]
    pub fn t8cm_en(&self) -> T8cmEnR {
        T8cmEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - T20CM_EN"]
    #[inline(always)]
    pub fn t20cm_en(&self) -> T20cmEnR {
        T20cmEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPAMP3_TCMR")
            .field("vms_sel", &self.vms_sel())
            .field("vps_sel", &self.vps_sel())
            .field("t1cm_en", &self.t1cm_en())
            .field("t8cm_en", &self.t8cm_en())
            .field("t20cm_en", &self.t20cm_en())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - VMS_SEL"]
    #[inline(always)]
    pub fn vms_sel(&mut self) -> VmsSelW<'_, Opamp3TcmrSpec> {
        VmsSelW::new(self, 0)
    }
    #[doc = "Bits 1:2 - VPS_SEL"]
    #[inline(always)]
    pub fn vps_sel(&mut self) -> VpsSelW<'_, Opamp3TcmrSpec> {
        VpsSelW::new(self, 1)
    }
    #[doc = "Bit 3 - T1CM_EN"]
    #[inline(always)]
    pub fn t1cm_en(&mut self) -> T1cmEnW<'_, Opamp3TcmrSpec> {
        T1cmEnW::new(self, 3)
    }
    #[doc = "Bit 4 - T8CM_EN"]
    #[inline(always)]
    pub fn t8cm_en(&mut self) -> T8cmEnW<'_, Opamp3TcmrSpec> {
        T8cmEnW::new(self, 4)
    }
    #[doc = "Bit 5 - T20CM_EN"]
    #[inline(always)]
    pub fn t20cm_en(&mut self) -> T20cmEnW<'_, Opamp3TcmrSpec> {
        T20cmEnW::new(self, 5)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, Opamp3TcmrSpec> {
        LockW::new(self, 31)
    }
}
#[doc = "OPAMP3 control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp3_tcmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp3_tcmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Opamp3TcmrSpec;
impl crate::RegisterSpec for Opamp3TcmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp3_tcmr::R`](R) reader structure"]
impl crate::Readable for Opamp3TcmrSpec {}
#[doc = "`write(|w| ..)` method takes [`opamp3_tcmr::W`](W) writer structure"]
impl crate::Writable for Opamp3TcmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPAMP3_TCMR to value 0"]
impl crate::Resettable for Opamp3TcmrSpec {}

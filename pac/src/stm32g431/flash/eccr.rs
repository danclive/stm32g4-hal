#[doc = "Register `ECCR` reader"]
pub type R = crate::R<EccrSpec>;
#[doc = "Register `ECCR` writer"]
pub type W = crate::W<EccrSpec>;
#[doc = "Field `ADDR_ECC` reader - ECC fail address"]
pub type AddrEccR = crate::FieldReader<u32>;
#[doc = "Field `BK_ECC` reader - BK_ECC"]
pub type BkEccR = crate::BitReader;
#[doc = "Field `SYSF_ECC` reader - SYSF_ECC"]
pub type SysfEccR = crate::BitReader;
#[doc = "Field `ECCIE` reader - ECCIE"]
pub type EccieR = crate::BitReader;
#[doc = "Field `ECCIE` writer - ECCIE"]
pub type EccieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCC2` reader - ECC correction"]
pub type Eccc2R = crate::BitReader;
#[doc = "Field `ECCC2` writer - ECC correction"]
pub type Eccc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCD2` reader - ECC2 detection"]
pub type Eccd2R = crate::BitReader;
#[doc = "Field `ECCD2` writer - ECC2 detection"]
pub type Eccd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCC` reader - ECC correction"]
pub type EcccR = crate::BitReader;
#[doc = "Field `ECCC` writer - ECC correction"]
pub type EcccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCD` reader - ECC detection"]
pub type EccdR = crate::BitReader;
#[doc = "Field `ECCD` writer - ECC detection"]
pub type EccdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18 - ECC fail address"]
    #[inline(always)]
    pub fn addr_ecc(&self) -> AddrEccR {
        AddrEccR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bit 21 - BK_ECC"]
    #[inline(always)]
    pub fn bk_ecc(&self) -> BkEccR {
        BkEccR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SYSF_ECC"]
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SysfEccR {
        SysfEccR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - ECCIE"]
    #[inline(always)]
    pub fn eccie(&self) -> EccieR {
        EccieR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - ECC correction"]
    #[inline(always)]
    pub fn eccc2(&self) -> Eccc2R {
        Eccc2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ECC2 detection"]
    #[inline(always)]
    pub fn eccd2(&self) -> Eccd2R {
        Eccd2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ECC correction"]
    #[inline(always)]
    pub fn eccc(&self) -> EcccR {
        EcccR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ECC detection"]
    #[inline(always)]
    pub fn eccd(&self) -> EccdR {
        EccdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECCR")
            .field("addr_ecc", &self.addr_ecc())
            .field("bk_ecc", &self.bk_ecc())
            .field("sysf_ecc", &self.sysf_ecc())
            .field("eccie", &self.eccie())
            .field("eccc2", &self.eccc2())
            .field("eccd2", &self.eccd2())
            .field("eccc", &self.eccc())
            .field("eccd", &self.eccd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 24 - ECCIE"]
    #[inline(always)]
    pub fn eccie(&mut self) -> EccieW<EccrSpec> {
        EccieW::new(self, 24)
    }
    #[doc = "Bit 28 - ECC correction"]
    #[inline(always)]
    pub fn eccc2(&mut self) -> Eccc2W<EccrSpec> {
        Eccc2W::new(self, 28)
    }
    #[doc = "Bit 29 - ECC2 detection"]
    #[inline(always)]
    pub fn eccd2(&mut self) -> Eccd2W<EccrSpec> {
        Eccd2W::new(self, 29)
    }
    #[doc = "Bit 30 - ECC correction"]
    #[inline(always)]
    pub fn eccc(&mut self) -> EcccW<EccrSpec> {
        EcccW::new(self, 30)
    }
    #[doc = "Bit 31 - ECC detection"]
    #[inline(always)]
    pub fn eccd(&mut self) -> EccdW<EccrSpec> {
        EccdW::new(self, 31)
    }
}
#[doc = "Flash ECC register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccrSpec;
impl crate::RegisterSpec for EccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccr::R`](R) reader structure"]
impl crate::Readable for EccrSpec {}
#[doc = "`write(|w| ..)` method takes [`eccr::W`](W) writer structure"]
impl crate::Writable for EccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCR to value 0"]
impl crate::Resettable for EccrSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `BDTR` reader"]
pub type R = crate::R<BdtrSpec>;
#[doc = "Register `BDTR` writer"]
pub type W = crate::W<BdtrSpec>;
#[doc = "Field `DTG` reader - Dead-time generator setup"]
pub type DtgR = crate::FieldReader;
#[doc = "Field `DTG` writer - Dead-time generator setup"]
pub type DtgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LOCK` reader - Lock configuration"]
pub type LockR = crate::FieldReader;
#[doc = "Field `LOCK` writer - Lock configuration"]
pub type LockW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSSI` reader - Off-state selection for Idle mode"]
pub type OssiR = crate::BitReader;
#[doc = "Field `OSSI` writer - Off-state selection for Idle mode"]
pub type OssiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSR` reader - Off-state selection for Run mode"]
pub type OssrR = crate::BitReader;
#[doc = "Field `OSSR` writer - Off-state selection for Run mode"]
pub type OssrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKE` reader - Break enable"]
pub type BkeR = crate::BitReader;
#[doc = "Field `BKE` writer - Break enable"]
pub type BkeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKP` reader - Break polarity"]
pub type BkpR = crate::BitReader;
#[doc = "Field `BKP` writer - Break polarity"]
pub type BkpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AOE` reader - Automatic output enable"]
pub type AoeR = crate::BitReader;
#[doc = "Field `AOE` writer - Automatic output enable"]
pub type AoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOE` reader - Main output enable"]
pub type MoeR = crate::BitReader;
#[doc = "Field `MOE` writer - Main output enable"]
pub type MoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKF` reader - Break filter"]
pub type BkfR = crate::FieldReader;
#[doc = "Field `BKF` writer - Break filter"]
pub type BkfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BKDSRM` reader - BKDSRM"]
pub type BkdsrmR = crate::BitReader;
#[doc = "Field `BKDSRM` writer - BKDSRM"]
pub type BkdsrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKBID` reader - BKBID"]
pub type BkbidR = crate::BitReader;
#[doc = "Field `BKBID` writer - BKBID"]
pub type BkbidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    pub fn dtg(&self) -> DtgR {
        DtgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Lock configuration"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode"]
    #[inline(always)]
    pub fn ossi(&self) -> OssiR {
        OssiR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode"]
    #[inline(always)]
    pub fn ossr(&self) -> OssrR {
        OssrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    pub fn bke(&self) -> BkeR {
        BkeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    pub fn bkp(&self) -> BkpR {
        BkpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    pub fn aoe(&self) -> AoeR {
        AoeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Main output enable"]
    #[inline(always)]
    pub fn moe(&self) -> MoeR {
        MoeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Break filter"]
    #[inline(always)]
    pub fn bkf(&self) -> BkfR {
        BkfR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 26 - BKDSRM"]
    #[inline(always)]
    pub fn bkdsrm(&self) -> BkdsrmR {
        BkdsrmR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - BKBID"]
    #[inline(always)]
    pub fn bkbid(&self) -> BkbidR {
        BkbidR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDTR")
            .field("dtg", &self.dtg())
            .field("lock", &self.lock())
            .field("ossi", &self.ossi())
            .field("ossr", &self.ossr())
            .field("bke", &self.bke())
            .field("bkp", &self.bkp())
            .field("aoe", &self.aoe())
            .field("moe", &self.moe())
            .field("bkf", &self.bkf())
            .field("bkdsrm", &self.bkdsrm())
            .field("bkbid", &self.bkbid())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    pub fn dtg(&mut self) -> DtgW<BdtrSpec> {
        DtgW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Lock configuration"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<BdtrSpec> {
        LockW::new(self, 8)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode"]
    #[inline(always)]
    pub fn ossi(&mut self) -> OssiW<BdtrSpec> {
        OssiW::new(self, 10)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode"]
    #[inline(always)]
    pub fn ossr(&mut self) -> OssrW<BdtrSpec> {
        OssrW::new(self, 11)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    pub fn bke(&mut self) -> BkeW<BdtrSpec> {
        BkeW::new(self, 12)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BkpW<BdtrSpec> {
        BkpW::new(self, 13)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    pub fn aoe(&mut self) -> AoeW<BdtrSpec> {
        AoeW::new(self, 14)
    }
    #[doc = "Bit 15 - Main output enable"]
    #[inline(always)]
    pub fn moe(&mut self) -> MoeW<BdtrSpec> {
        MoeW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Break filter"]
    #[inline(always)]
    pub fn bkf(&mut self) -> BkfW<BdtrSpec> {
        BkfW::new(self, 16)
    }
    #[doc = "Bit 26 - BKDSRM"]
    #[inline(always)]
    pub fn bkdsrm(&mut self) -> BkdsrmW<BdtrSpec> {
        BkdsrmW::new(self, 26)
    }
    #[doc = "Bit 28 - BKBID"]
    #[inline(always)]
    pub fn bkbid(&mut self) -> BkbidW<BdtrSpec> {
        BkbidW::new(self, 28)
    }
}
#[doc = "break and dead-time register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdtrSpec;
impl crate::RegisterSpec for BdtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdtr::R`](R) reader structure"]
impl crate::Readable for BdtrSpec {}
#[doc = "`write(|w| ..)` method takes [`bdtr::W`](W) writer structure"]
impl crate::Writable for BdtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BDTR to value 0"]
impl crate::Resettable for BdtrSpec {
    const RESET_VALUE: u32 = 0;
}

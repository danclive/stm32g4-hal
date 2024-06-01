#[doc = "Register `BWTR1` reader"]
pub type R = crate::R<Bwtr1Spec>;
#[doc = "Register `BWTR1` writer"]
pub type W = crate::W<Bwtr1Spec>;
#[doc = "Field `ADDSET` reader - ADDSET"]
pub type AddsetR = crate::FieldReader;
#[doc = "Field `ADDSET` writer - ADDSET"]
pub type AddsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDHLD` reader - ADDHLD"]
pub type AddhldR = crate::FieldReader;
#[doc = "Field `ADDHLD` writer - ADDHLD"]
pub type AddhldW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATAST` reader - DATAST"]
pub type DatastR = crate::FieldReader;
#[doc = "Field `DATAST` writer - DATAST"]
pub type DatastW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BUSTURN` reader - BUSTURN"]
pub type BusturnR = crate::FieldReader;
#[doc = "Field `BUSTURN` writer - BUSTURN"]
pub type BusturnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ACCMOD` reader - ACCMOD"]
pub type AccmodR = crate::FieldReader;
#[doc = "Field `ACCMOD` writer - ACCMOD"]
pub type AccmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATAHLD` reader - DATAHLD"]
pub type DatahldR = crate::FieldReader;
#[doc = "Field `DATAHLD` writer - DATAHLD"]
pub type DatahldW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - ADDSET"]
    #[inline(always)]
    pub fn addset(&self) -> AddsetR {
        AddsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ADDHLD"]
    #[inline(always)]
    pub fn addhld(&self) -> AddhldR {
        AddhldR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - DATAST"]
    #[inline(always)]
    pub fn datast(&self) -> DatastR {
        DatastR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - BUSTURN"]
    #[inline(always)]
    pub fn busturn(&self) -> BusturnR {
        BusturnR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - ACCMOD"]
    #[inline(always)]
    pub fn accmod(&self) -> AccmodR {
        AccmodR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - DATAHLD"]
    #[inline(always)]
    pub fn datahld(&self) -> DatahldR {
        DatahldR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BWTR1")
            .field("datahld", &self.datahld())
            .field("accmod", &self.accmod())
            .field("busturn", &self.busturn())
            .field("datast", &self.datast())
            .field("addhld", &self.addhld())
            .field("addset", &self.addset())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDSET"]
    #[inline(always)]
    #[must_use]
    pub fn addset(&mut self) -> AddsetW<Bwtr1Spec> {
        AddsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - ADDHLD"]
    #[inline(always)]
    #[must_use]
    pub fn addhld(&mut self) -> AddhldW<Bwtr1Spec> {
        AddhldW::new(self, 4)
    }
    #[doc = "Bits 8:15 - DATAST"]
    #[inline(always)]
    #[must_use]
    pub fn datast(&mut self) -> DatastW<Bwtr1Spec> {
        DatastW::new(self, 8)
    }
    #[doc = "Bits 16:19 - BUSTURN"]
    #[inline(always)]
    #[must_use]
    pub fn busturn(&mut self) -> BusturnW<Bwtr1Spec> {
        BusturnW::new(self, 16)
    }
    #[doc = "Bits 28:29 - ACCMOD"]
    #[inline(always)]
    #[must_use]
    pub fn accmod(&mut self) -> AccmodW<Bwtr1Spec> {
        AccmodW::new(self, 28)
    }
    #[doc = "Bits 30:31 - DATAHLD"]
    #[inline(always)]
    #[must_use]
    pub fn datahld(&mut self) -> DatahldW<Bwtr1Spec> {
        DatahldW::new(self, 30)
    }
}
#[doc = "SRAM/NOR-Flash write timing registers 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bwtr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bwtr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bwtr1Spec;
impl crate::RegisterSpec for Bwtr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bwtr1::R`](R) reader structure"]
impl crate::Readable for Bwtr1Spec {}
#[doc = "`write(|w| ..)` method takes [`bwtr1::W`](W) writer structure"]
impl crate::Writable for Bwtr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BWTR1 to value 0x0fff_ffff"]
impl crate::Resettable for Bwtr1Spec {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}

#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `ADDRCF` writer - Address Matched flag clear"]
pub type AddrcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACKCF` writer - Not Acknowledge flag clear"]
pub type NackcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPCF` writer - Stop detection flag clear"]
pub type StopcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERRCF` writer - Bus error flag clear"]
pub type BerrcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARLOCF` writer - Arbitration lost flag clear"]
pub type ArlocfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRCF` writer - Overrun/Underrun flag clear"]
pub type OvrcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECCF` writer - PEC Error flag clear"]
pub type PeccfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMOUTCF` writer - Timeout detection flag clear"]
pub type TimoutcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALERTCF` writer - Alert flag clear"]
pub type AlertcfW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IcrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 3 - Address Matched flag clear"]
    #[inline(always)]
    pub fn addrcf(&mut self) -> AddrcfW<IcrSpec> {
        AddrcfW::new(self, 3)
    }
    #[doc = "Bit 4 - Not Acknowledge flag clear"]
    #[inline(always)]
    pub fn nackcf(&mut self) -> NackcfW<IcrSpec> {
        NackcfW::new(self, 4)
    }
    #[doc = "Bit 5 - Stop detection flag clear"]
    #[inline(always)]
    pub fn stopcf(&mut self) -> StopcfW<IcrSpec> {
        StopcfW::new(self, 5)
    }
    #[doc = "Bit 8 - Bus error flag clear"]
    #[inline(always)]
    pub fn berrcf(&mut self) -> BerrcfW<IcrSpec> {
        BerrcfW::new(self, 8)
    }
    #[doc = "Bit 9 - Arbitration lost flag clear"]
    #[inline(always)]
    pub fn arlocf(&mut self) -> ArlocfW<IcrSpec> {
        ArlocfW::new(self, 9)
    }
    #[doc = "Bit 10 - Overrun/Underrun flag clear"]
    #[inline(always)]
    pub fn ovrcf(&mut self) -> OvrcfW<IcrSpec> {
        OvrcfW::new(self, 10)
    }
    #[doc = "Bit 11 - PEC Error flag clear"]
    #[inline(always)]
    pub fn peccf(&mut self) -> PeccfW<IcrSpec> {
        PeccfW::new(self, 11)
    }
    #[doc = "Bit 12 - Timeout detection flag clear"]
    #[inline(always)]
    pub fn timoutcf(&mut self) -> TimoutcfW<IcrSpec> {
        TimoutcfW::new(self, 12)
    }
    #[doc = "Bit 13 - Alert flag clear"]
    #[inline(always)]
    pub fn alertcf(&mut self) -> AlertcfW<IcrSpec> {
        AlertcfW::new(self, 13)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}

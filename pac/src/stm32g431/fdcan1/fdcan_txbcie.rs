#[doc = "Register `FDCAN_TXBCIE` reader"]
pub type R = crate::R<FdcanTxbcieSpec>;
#[doc = "Register `FDCAN_TXBCIE` writer"]
pub type W = crate::W<FdcanTxbcieSpec>;
#[doc = "Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfie {
    #[doc = "0: Cancellation finished interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Cancellation finished interrupt enabled"]
    B0x1 = 1,
}
impl From<Cfie> for u8 {
    #[inline(always)]
    fn from(variant: Cfie) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfie {
    type Ux = u8;
}
impl crate::IsEnum for Cfie {}
#[doc = "Field `CFIE` reader - Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit."]
pub type CfieR = crate::FieldReader<Cfie>;
impl CfieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfie> {
        match self.bits {
            0 => Some(Cfie::B0x0),
            1 => Some(Cfie::B0x1),
            _ => None,
        }
    }
    #[doc = "Cancellation finished interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cfie::B0x0
    }
    #[doc = "Cancellation finished interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cfie::B0x1
    }
}
#[doc = "Field `CFIE` writer - Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit."]
pub type CfieW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cfie>;
impl<'a, REG> CfieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Cancellation finished interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfie::B0x0)
    }
    #[doc = "Cancellation finished interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfie::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit."]
    #[inline(always)]
    pub fn cfie(&self) -> CfieR {
        CfieR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBCIE")
            .field("cfie", &self.cfie())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit."]
    #[inline(always)]
    pub fn cfie(&mut self) -> CfieW<FdcanTxbcieSpec> {
        CfieW::new(self, 0)
    }
}
#[doc = "FDCAN Tx buffer cancellation finished interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txbcie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbcie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTxbcieSpec;
impl crate::RegisterSpec for FdcanTxbcieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbcie::R`](R) reader structure"]
impl crate::Readable for FdcanTxbcieSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txbcie::W`](W) writer structure"]
impl crate::Writable for FdcanTxbcieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TXBCIE to value 0"]
impl crate::Resettable for FdcanTxbcieSpec {
    const RESET_VALUE: u32 = 0;
}

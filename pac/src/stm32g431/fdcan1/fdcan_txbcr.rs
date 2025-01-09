#[doc = "Register `FDCAN_TXBCR` reader"]
pub type R = crate::R<FdcanTxbcrSpec>;
#[doc = "Register `FDCAN_TXBCR` writer"]
pub type W = crate::W<FdcanTxbcrSpec>;
#[doc = "Cancellation request Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cr {
    #[doc = "0: No cancellation pending"]
    B0x0 = 0,
    #[doc = "1: Cancellation pending"]
    B0x1 = 1,
}
impl From<Cr> for u8 {
    #[inline(always)]
    fn from(variant: Cr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cr {
    type Ux = u8;
}
impl crate::IsEnum for Cr {}
#[doc = "Field `CR` reader - Cancellation request Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset."]
pub type CrR = crate::FieldReader<Cr>;
impl CrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cr> {
        match self.bits {
            0 => Some(Cr::B0x0),
            1 => Some(Cr::B0x1),
            _ => None,
        }
    }
    #[doc = "No cancellation pending"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cr::B0x0
    }
    #[doc = "Cancellation pending"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cr::B0x1
    }
}
#[doc = "Field `CR` writer - Cancellation request Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset."]
pub type CrW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cr>;
impl<'a, REG> CrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No cancellation pending"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cr::B0x0)
    }
    #[doc = "Cancellation pending"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cr::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Cancellation request Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset."]
    #[inline(always)]
    pub fn cr(&self) -> CrR {
        CrR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBCR")
            .field("cr", &self.cr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Cancellation request Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset."]
    #[inline(always)]
    pub fn cr(&mut self) -> CrW<FdcanTxbcrSpec> {
        CrW::new(self, 0)
    }
}
#[doc = "FDCAN Tx buffer cancellation request register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txbcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTxbcrSpec;
impl crate::RegisterSpec for FdcanTxbcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbcr::R`](R) reader structure"]
impl crate::Readable for FdcanTxbcrSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txbcr::W`](W) writer structure"]
impl crate::Writable for FdcanTxbcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TXBCR to value 0"]
impl crate::Resettable for FdcanTxbcrSpec {
    const RESET_VALUE: u32 = 0;
}

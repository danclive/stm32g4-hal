#[doc = "Register `FDCAN_TXBC` reader"]
pub type R = crate::R<FdcanTxbcSpec>;
#[doc = "Register `FDCAN_TXBC` writer"]
pub type W = crate::W<FdcanTxbcSpec>;
#[doc = "Tx FIFO/queue mode This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfqm {
    #[doc = "0: Tx FIFO operation"]
    B0x0 = 0,
    #[doc = "1: Tx queue operation."]
    B0x1 = 1,
}
impl From<Tfqm> for bool {
    #[inline(always)]
    fn from(variant: Tfqm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFQM` reader - Tx FIFO/queue mode This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type TfqmR = crate::BitReader<Tfqm>;
impl TfqmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfqm {
        match self.bits {
            false => Tfqm::B0x0,
            true => Tfqm::B0x1,
        }
    }
    #[doc = "Tx FIFO operation"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tfqm::B0x0
    }
    #[doc = "Tx queue operation."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tfqm::B0x1
    }
}
#[doc = "Field `TFQM` writer - Tx FIFO/queue mode This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type TfqmW<'a, REG> = crate::BitWriter<'a, REG, Tfqm>;
impl<'a, REG> TfqmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx FIFO operation"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tfqm::B0x0)
    }
    #[doc = "Tx queue operation."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tfqm::B0x1)
    }
}
impl R {
    #[doc = "Bit 24 - Tx FIFO/queue mode This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn tfqm(&self) -> TfqmR {
        TfqmR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBC")
            .field("tfqm", &self.tfqm())
            .finish()
    }
}
impl W {
    #[doc = "Bit 24 - Tx FIFO/queue mode This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn tfqm(&mut self) -> TfqmW<FdcanTxbcSpec> {
        TfqmW::new(self, 24)
    }
}
#[doc = "FDCAN Tx buffer configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTxbcSpec;
impl crate::RegisterSpec for FdcanTxbcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbc::R`](R) reader structure"]
impl crate::Readable for FdcanTxbcSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txbc::W`](W) writer structure"]
impl crate::Writable for FdcanTxbcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TXBC to value 0"]
impl crate::Resettable for FdcanTxbcSpec {
    const RESET_VALUE: u32 = 0;
}

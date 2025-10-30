#[doc = "Register `FDCAN_TEST` reader"]
pub type R = crate::R<FdcanTestSpec>;
#[doc = "Register `FDCAN_TEST` writer"]
pub type W = crate::W<FdcanTestSpec>;
#[doc = "Loop back mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbck {
    #[doc = "0: Reset value, Loop Back mode is disabled"]
    B0x0 = 0,
    #[doc = "1: Loop Back mode is enabled (see Power down (Sleep mode))"]
    B0x1 = 1,
}
impl From<Lbck> for bool {
    #[inline(always)]
    fn from(variant: Lbck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBCK` reader - Loop back mode"]
pub type LbckR = crate::BitReader<Lbck>;
impl LbckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbck {
        match self.bits {
            false => Lbck::B0x0,
            true => Lbck::B0x1,
        }
    }
    #[doc = "Reset value, Loop Back mode is disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lbck::B0x0
    }
    #[doc = "Loop Back mode is enabled (see Power down (Sleep mode))"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lbck::B0x1
    }
}
#[doc = "Field `LBCK` writer - Loop back mode"]
pub type LbckW<'a, REG> = crate::BitWriter<'a, REG, Lbck>;
impl<'a, REG> LbckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset value, Loop Back mode is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lbck::B0x0)
    }
    #[doc = "Loop Back mode is enabled (see Power down (Sleep mode))"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lbck::B0x1)
    }
}
#[doc = "Control of transmit pin\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tx {
    #[doc = "0: Reset value, FDCANx_TX TX is controlled by the CAN core, updated at the end of the CAN bit time"]
    B0x0 = 0,
    #[doc = "1: Sample point can be monitored at pin FDCANx_TX"]
    B0x1 = 1,
    #[doc = "2: Dominant (0) level at pin FDCANx_TX"]
    B0x2 = 2,
    #[doc = "3: Recessive (1) at pin FDCANx_TX"]
    B0x3 = 3,
}
impl From<Tx> for u8 {
    #[inline(always)]
    fn from(variant: Tx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tx {
    type Ux = u8;
}
impl crate::IsEnum for Tx {}
#[doc = "Field `TX` reader - Control of transmit pin"]
pub type TxR = crate::FieldReader<Tx>;
impl TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tx {
        match self.bits {
            0 => Tx::B0x0,
            1 => Tx::B0x1,
            2 => Tx::B0x2,
            3 => Tx::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Reset value, FDCANx_TX TX is controlled by the CAN core, updated at the end of the CAN bit time"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tx::B0x0
    }
    #[doc = "Sample point can be monitored at pin FDCANx_TX"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tx::B0x1
    }
    #[doc = "Dominant (0) level at pin FDCANx_TX"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Tx::B0x2
    }
    #[doc = "Recessive (1) at pin FDCANx_TX"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Tx::B0x3
    }
}
#[doc = "Field `TX` writer - Control of transmit pin"]
pub type TxW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tx, crate::Safe>;
impl<'a, REG> TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset value, FDCANx_TX TX is controlled by the CAN core, updated at the end of the CAN bit time"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::B0x0)
    }
    #[doc = "Sample point can be monitored at pin FDCANx_TX"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::B0x1)
    }
    #[doc = "Dominant (0) level at pin FDCANx_TX"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::B0x2)
    }
    #[doc = "Recessive (1) at pin FDCANx_TX"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::B0x3)
    }
}
#[doc = "Receive pin Monitors the actual value of pin FDCANx_RX\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rx {
    #[doc = "0: The CAN bus is dominant (FDCANx_RX = 0)"]
    B0x0 = 0,
    #[doc = "1: The CAN bus is recessive (FDCANx_RX = 1)"]
    B0x1 = 1,
}
impl From<Rx> for bool {
    #[inline(always)]
    fn from(variant: Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX` reader - Receive pin Monitors the actual value of pin FDCANx_RX"]
pub type RxR = crate::BitReader<Rx>;
impl RxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rx {
        match self.bits {
            false => Rx::B0x0,
            true => Rx::B0x1,
        }
    }
    #[doc = "The CAN bus is dominant (FDCANx_RX = 0)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rx::B0x0
    }
    #[doc = "The CAN bus is recessive (FDCANx_RX = 1)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rx::B0x1
    }
}
impl R {
    #[doc = "Bit 4 - Loop back mode"]
    #[inline(always)]
    pub fn lbck(&self) -> LbckR {
        LbckR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Control of transmit pin"]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Receive pin Monitors the actual value of pin FDCANx_RX"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TEST")
            .field("lbck", &self.lbck())
            .field("tx", &self.tx())
            .field("rx", &self.rx())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - Loop back mode"]
    #[inline(always)]
    pub fn lbck(&mut self) -> LbckW<'_, FdcanTestSpec> {
        LbckW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Control of transmit pin"]
    #[inline(always)]
    pub fn tx(&mut self) -> TxW<'_, FdcanTestSpec> {
        TxW::new(self, 5)
    }
}
#[doc = "FDCAN test register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTestSpec;
impl crate::RegisterSpec for FdcanTestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_test::R`](R) reader structure"]
impl crate::Readable for FdcanTestSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_test::W`](W) writer structure"]
impl crate::Writable for FdcanTestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TEST to value 0"]
impl crate::Resettable for FdcanTestSpec {}

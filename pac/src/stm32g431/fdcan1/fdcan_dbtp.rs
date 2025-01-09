#[doc = "Register `FDCAN_DBTP` reader"]
pub type R = crate::R<FdcanDbtpSpec>;
#[doc = "Register `FDCAN_DBTP` writer"]
pub type W = crate::W<FdcanDbtpSpec>;
#[doc = "Field `DSJW` reader - Synchronization jump width Must always be smaller than DTSEG2, valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1: t<sub>SJW</sub> = (DSJW + 1) x tq."]
pub type DsjwR = crate::FieldReader;
#[doc = "Field `DSJW` writer - Synchronization jump width Must always be smaller than DTSEG2, valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1: t<sub>SJW</sub> = (DSJW + 1) x tq."]
pub type DsjwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTSEG2` reader - Data time segment after sample point Valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1, i.e. t<sub>BS2</sub> = (DTSEG2 + 1) x tq."]
pub type Dtseg2R = crate::FieldReader;
#[doc = "Field `DTSEG2` writer - Data time segment after sample point Valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1, i.e. t<sub>BS2</sub> = (DTSEG2 + 1) x tq."]
pub type Dtseg2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTSEG1` reader - Data time segment before sample point Valid values are 0 to 31. The value used by the hardware is the one programmed, incremented by 1, i.e. t<sub>BS1</sub> = (DTSEG1 + 1) x tq."]
pub type Dtseg1R = crate::FieldReader;
#[doc = "Field `DTSEG1` writer - Data time segment before sample point Valid values are 0 to 31. The value used by the hardware is the one programmed, incremented by 1, i.e. t<sub>BS1</sub> = (DTSEG1 + 1) x tq."]
pub type Dtseg1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DBRP` reader - Data bit rate prescaler The value by which the oscillator frequency is divided to generate the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are 0 to 31. The hardware interpreters this value as the value programmed plus 1."]
pub type DbrpR = crate::FieldReader;
#[doc = "Field `DBRP` writer - Data bit rate prescaler The value by which the oscillator frequency is divided to generate the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are 0 to 31. The hardware interpreters this value as the value programmed plus 1."]
pub type DbrpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Transceiver delay compensation\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdc {
    #[doc = "0: Transceiver delay compensation disabled"]
    B0x0 = 0,
    #[doc = "1: Transceiver delay compensation enabled"]
    B0x1 = 1,
}
impl From<Tdc> for bool {
    #[inline(always)]
    fn from(variant: Tdc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDC` reader - Transceiver delay compensation"]
pub type TdcR = crate::BitReader<Tdc>;
impl TdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdc {
        match self.bits {
            false => Tdc::B0x0,
            true => Tdc::B0x1,
        }
    }
    #[doc = "Transceiver delay compensation disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tdc::B0x0
    }
    #[doc = "Transceiver delay compensation enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tdc::B0x1
    }
}
#[doc = "Field `TDC` writer - Transceiver delay compensation"]
pub type TdcW<'a, REG> = crate::BitWriter<'a, REG, Tdc>;
impl<'a, REG> TdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transceiver delay compensation disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tdc::B0x0)
    }
    #[doc = "Transceiver delay compensation enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tdc::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Synchronization jump width Must always be smaller than DTSEG2, valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1: t<sub>SJW</sub> = (DSJW + 1) x tq."]
    #[inline(always)]
    pub fn dsjw(&self) -> DsjwR {
        DsjwR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Data time segment after sample point Valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1, i.e. t<sub>BS2</sub> = (DTSEG2 + 1) x tq."]
    #[inline(always)]
    pub fn dtseg2(&self) -> Dtseg2R {
        Dtseg2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Data time segment before sample point Valid values are 0 to 31. The value used by the hardware is the one programmed, incremented by 1, i.e. t<sub>BS1</sub> = (DTSEG1 + 1) x tq."]
    #[inline(always)]
    pub fn dtseg1(&self) -> Dtseg1R {
        Dtseg1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Data bit rate prescaler The value by which the oscillator frequency is divided to generate the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are 0 to 31. The hardware interpreters this value as the value programmed plus 1."]
    #[inline(always)]
    pub fn dbrp(&self) -> DbrpR {
        DbrpR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Transceiver delay compensation"]
    #[inline(always)]
    pub fn tdc(&self) -> TdcR {
        TdcR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_DBTP")
            .field("dsjw", &self.dsjw())
            .field("dtseg2", &self.dtseg2())
            .field("dtseg1", &self.dtseg1())
            .field("dbrp", &self.dbrp())
            .field("tdc", &self.tdc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Synchronization jump width Must always be smaller than DTSEG2, valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1: t<sub>SJW</sub> = (DSJW + 1) x tq."]
    #[inline(always)]
    pub fn dsjw(&mut self) -> DsjwW<FdcanDbtpSpec> {
        DsjwW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Data time segment after sample point Valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1, i.e. t<sub>BS2</sub> = (DTSEG2 + 1) x tq."]
    #[inline(always)]
    pub fn dtseg2(&mut self) -> Dtseg2W<FdcanDbtpSpec> {
        Dtseg2W::new(self, 4)
    }
    #[doc = "Bits 8:12 - Data time segment before sample point Valid values are 0 to 31. The value used by the hardware is the one programmed, incremented by 1, i.e. t<sub>BS1</sub> = (DTSEG1 + 1) x tq."]
    #[inline(always)]
    pub fn dtseg1(&mut self) -> Dtseg1W<FdcanDbtpSpec> {
        Dtseg1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Data bit rate prescaler The value by which the oscillator frequency is divided to generate the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are 0 to 31. The hardware interpreters this value as the value programmed plus 1."]
    #[inline(always)]
    pub fn dbrp(&mut self) -> DbrpW<FdcanDbtpSpec> {
        DbrpW::new(self, 16)
    }
    #[doc = "Bit 23 - Transceiver delay compensation"]
    #[inline(always)]
    pub fn tdc(&mut self) -> TdcW<FdcanDbtpSpec> {
        TdcW::new(self, 23)
    }
}
#[doc = "FDCAN data bit timing and prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_dbtp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_dbtp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanDbtpSpec;
impl crate::RegisterSpec for FdcanDbtpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_dbtp::R`](R) reader structure"]
impl crate::Readable for FdcanDbtpSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_dbtp::W`](W) writer structure"]
impl crate::Writable for FdcanDbtpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_DBTP to value 0x0a33"]
impl crate::Resettable for FdcanDbtpSpec {
    const RESET_VALUE: u32 = 0x0a33;
}

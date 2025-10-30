#[doc = "Register `FDCAN_TSCC` reader"]
pub type R = crate::R<FdcanTsccSpec>;
#[doc = "Register `FDCAN_TSCC` writer"]
pub type W = crate::W<FdcanTsccSpec>;
#[doc = "Timestamp select These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tss {
    #[doc = "0: Timestamp counter value always 0x0000"]
    B0x0 = 0,
    #[doc = "1: Timestamp counter value incremented according to TCP"]
    B0x1 = 1,
    #[doc = "2: External timestamp counter from TIM3 value (tim3_cnt\\[0:15\\])"]
    B0x2 = 2,
    #[doc = "3: Same as 00."]
    B0x3 = 3,
}
impl From<Tss> for u8 {
    #[inline(always)]
    fn from(variant: Tss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tss {
    type Ux = u8;
}
impl crate::IsEnum for Tss {}
#[doc = "Field `TSS` reader - Timestamp select These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type TssR = crate::FieldReader<Tss>;
impl TssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tss {
        match self.bits {
            0 => Tss::B0x0,
            1 => Tss::B0x1,
            2 => Tss::B0x2,
            3 => Tss::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Timestamp counter value always 0x0000"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tss::B0x0
    }
    #[doc = "Timestamp counter value incremented according to TCP"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tss::B0x1
    }
    #[doc = "External timestamp counter from TIM3 value (tim3_cnt\\[0:15\\])"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Tss::B0x2
    }
    #[doc = "Same as 00."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Tss::B0x3
    }
}
#[doc = "Field `TSS` writer - Timestamp select These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type TssW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tss, crate::Safe>;
impl<'a, REG> TssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timestamp counter value always 0x0000"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tss::B0x0)
    }
    #[doc = "Timestamp counter value incremented according to TCP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tss::B0x1)
    }
    #[doc = "External timestamp counter from TIM3 value (tim3_cnt\\[0:15\\])"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Tss::B0x2)
    }
    #[doc = "Same as 00."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Tss::B0x3)
    }
}
#[doc = "Field `TCP` reader - Timestamp counter prescaler"]
pub type TcpR = crate::FieldReader;
#[doc = "Field `TCP` writer - Timestamp counter prescaler"]
pub type TcpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Timestamp select These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn tss(&self) -> TssR {
        TssR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - Timestamp counter prescaler"]
    #[inline(always)]
    pub fn tcp(&self) -> TcpR {
        TcpR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TSCC")
            .field("tss", &self.tss())
            .field("tcp", &self.tcp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Timestamp select These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn tss(&mut self) -> TssW<'_, FdcanTsccSpec> {
        TssW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Timestamp counter prescaler"]
    #[inline(always)]
    pub fn tcp(&mut self) -> TcpW<'_, FdcanTsccSpec> {
        TcpW::new(self, 16)
    }
}
#[doc = "FDCAN timestamp counter configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_tscc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tscc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTsccSpec;
impl crate::RegisterSpec for FdcanTsccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_tscc::R`](R) reader structure"]
impl crate::Readable for FdcanTsccSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_tscc::W`](W) writer structure"]
impl crate::Writable for FdcanTsccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TSCC to value 0"]
impl crate::Resettable for FdcanTsccSpec {}

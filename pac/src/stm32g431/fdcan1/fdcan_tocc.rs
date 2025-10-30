#[doc = "Register `FDCAN_TOCC` reader"]
pub type R = crate::R<FdcanToccSpec>;
#[doc = "Register `FDCAN_TOCC` writer"]
pub type W = crate::W<FdcanToccSpec>;
#[doc = "Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Etoc {
    #[doc = "0: Timeout counter disabled"]
    B0x0 = 0,
    #[doc = "1: Timeout counter enabled"]
    B0x1 = 1,
}
impl From<Etoc> for bool {
    #[inline(always)]
    fn from(variant: Etoc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETOC` reader - Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type EtocR = crate::BitReader<Etoc>;
impl EtocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etoc {
        match self.bits {
            false => Etoc::B0x0,
            true => Etoc::B0x1,
        }
    }
    #[doc = "Timeout counter disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Etoc::B0x0
    }
    #[doc = "Timeout counter enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Etoc::B0x1
    }
}
#[doc = "Field `ETOC` writer - Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type EtocW<'a, REG> = crate::BitWriter<'a, REG, Etoc>;
impl<'a, REG> EtocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timeout counter disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Etoc::B0x0)
    }
    #[doc = "Timeout counter enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Etoc::B0x1)
    }
}
#[doc = "Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\] and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tos {
    #[doc = "0: Continuous operation"]
    B0x0 = 0,
    #[doc = "1: Timeout controlled by Tx event FIFO"]
    B0x1 = 1,
    #[doc = "2: Timeout controlled by Rx FIFO 0"]
    B0x2 = 2,
    #[doc = "3: Timeout controlled by Rx FIFO 1"]
    B0x3 = 3,
}
impl From<Tos> for u8 {
    #[inline(always)]
    fn from(variant: Tos) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tos {
    type Ux = u8;
}
impl crate::IsEnum for Tos {}
#[doc = "Field `TOS` reader - Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\] and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type TosR = crate::FieldReader<Tos>;
impl TosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tos {
        match self.bits {
            0 => Tos::B0x0,
            1 => Tos::B0x1,
            2 => Tos::B0x2,
            3 => Tos::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Continuous operation"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tos::B0x0
    }
    #[doc = "Timeout controlled by Tx event FIFO"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tos::B0x1
    }
    #[doc = "Timeout controlled by Rx FIFO 0"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Tos::B0x2
    }
    #[doc = "Timeout controlled by Rx FIFO 1"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Tos::B0x3
    }
}
#[doc = "Field `TOS` writer - Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\] and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type TosW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tos, crate::Safe>;
impl<'a, REG> TosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Continuous operation"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tos::B0x0)
    }
    #[doc = "Timeout controlled by Tx event FIFO"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tos::B0x1)
    }
    #[doc = "Timeout controlled by Rx FIFO 0"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Tos::B0x2)
    }
    #[doc = "Timeout controlled by Rx FIFO 1"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Tos::B0x3)
    }
}
#[doc = "Field `TOP` reader - Timeout period Start value of the timeout counter (down-counter). Configures the timeout period."]
pub type TopR = crate::FieldReader<u16>;
#[doc = "Field `TOP` writer - Timeout period Start value of the timeout counter (down-counter). Configures the timeout period."]
pub type TopW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn etoc(&self) -> EtocR {
        EtocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\] and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn tos(&self) -> TosR {
        TosR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 16:31 - Timeout period Start value of the timeout counter (down-counter). Configures the timeout period."]
    #[inline(always)]
    pub fn top(&self) -> TopR {
        TopR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TOCC")
            .field("etoc", &self.etoc())
            .field("tos", &self.tos())
            .field("top", &self.top())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn etoc(&mut self) -> EtocW<'_, FdcanToccSpec> {
        EtocW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\] and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn tos(&mut self) -> TosW<'_, FdcanToccSpec> {
        TosW::new(self, 1)
    }
    #[doc = "Bits 16:31 - Timeout period Start value of the timeout counter (down-counter). Configures the timeout period."]
    #[inline(always)]
    pub fn top(&mut self) -> TopW<'_, FdcanToccSpec> {
        TopW::new(self, 16)
    }
}
#[doc = "FDCAN timeout counter configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_tocc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tocc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanToccSpec;
impl crate::RegisterSpec for FdcanToccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_tocc::R`](R) reader structure"]
impl crate::Readable for FdcanToccSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_tocc::W`](W) writer structure"]
impl crate::Writable for FdcanToccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TOCC to value 0xffff_0000"]
impl crate::Resettable for FdcanToccSpec {
    const RESET_VALUE: u32 = 0xffff_0000;
}

#[doc = "Register `FDCAN_CKDIV` reader"]
pub type R = crate::R<FdcanCkdivSpec>;
#[doc = "Register `FDCAN_CKDIV` writer"]
pub type W = crate::W<FdcanCkdivSpec>;
#[doc = "input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pdiv {
    #[doc = "0: Divide by 1"]
    B0x0 = 0,
    #[doc = "1: Divide by 2"]
    B0x1 = 1,
    #[doc = "2: Divide by 4"]
    B0x2 = 2,
    #[doc = "3: Divide by 6"]
    B0x3 = 3,
    #[doc = "4: Divide by 8"]
    B0x4 = 4,
    #[doc = "5: Divide by 10"]
    B0x5 = 5,
    #[doc = "6: Divide by 12"]
    B0x6 = 6,
    #[doc = "7: Divide by 14"]
    B0x7 = 7,
    #[doc = "8: Divide by 16"]
    B0x8 = 8,
    #[doc = "9: Divide by 18"]
    B0x9 = 9,
    #[doc = "10: Divide by 20"]
    B0xA = 10,
    #[doc = "11: Divide by 22"]
    B0xB = 11,
    #[doc = "12: Divide by 24"]
    B0xC = 12,
    #[doc = "13: Divide by 26"]
    B0xD = 13,
    #[doc = "14: Divide by 28"]
    B0xE = 14,
    #[doc = "15: Divide by 30"]
    B0xF = 15,
}
impl From<Pdiv> for u8 {
    #[inline(always)]
    fn from(variant: Pdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pdiv {
    type Ux = u8;
}
impl crate::IsEnum for Pdiv {}
#[doc = "Field `PDIV` reader - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type PdivR = crate::FieldReader<Pdiv>;
impl PdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdiv {
        match self.bits {
            0 => Pdiv::B0x0,
            1 => Pdiv::B0x1,
            2 => Pdiv::B0x2,
            3 => Pdiv::B0x3,
            4 => Pdiv::B0x4,
            5 => Pdiv::B0x5,
            6 => Pdiv::B0x6,
            7 => Pdiv::B0x7,
            8 => Pdiv::B0x8,
            9 => Pdiv::B0x9,
            10 => Pdiv::B0xA,
            11 => Pdiv::B0xB,
            12 => Pdiv::B0xC,
            13 => Pdiv::B0xD,
            14 => Pdiv::B0xE,
            15 => Pdiv::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pdiv::B0x0
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pdiv::B0x1
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pdiv::B0x2
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Pdiv::B0x3
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Pdiv::B0x4
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Pdiv::B0x5
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Pdiv::B0x6
    }
    #[doc = "Divide by 14"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Pdiv::B0x7
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Pdiv::B0x8
    }
    #[doc = "Divide by 18"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Pdiv::B0x9
    }
    #[doc = "Divide by 20"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Pdiv::B0xA
    }
    #[doc = "Divide by 22"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Pdiv::B0xB
    }
    #[doc = "Divide by 24"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Pdiv::B0xC
    }
    #[doc = "Divide by 26"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Pdiv::B0xD
    }
    #[doc = "Divide by 28"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Pdiv::B0xE
    }
    #[doc = "Divide by 30"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Pdiv::B0xF
    }
}
#[doc = "Field `PDIV` writer - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type PdivW<'a, REG> = crate::FieldWriter<'a, REG, 4, Pdiv, crate::Safe>;
impl<'a, REG> PdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdiv::B0x0)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdiv::B0x1)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pdiv::B0x2)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Pdiv::B0x3)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Pdiv::B0x4)
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Pdiv::B0x5)
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Pdiv::B0x6)
    }
    #[doc = "Divide by 14"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Pdiv::B0x7)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Pdiv::B0x8)
    }
    #[doc = "Divide by 18"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Pdiv::B0x9)
    }
    #[doc = "Divide by 20"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Pdiv::B0xA)
    }
    #[doc = "Divide by 22"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Pdiv::B0xB)
    }
    #[doc = "Divide by 24"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Pdiv::B0xC)
    }
    #[doc = "Divide by 26"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Pdiv::B0xD)
    }
    #[doc = "Divide by 28"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Pdiv::B0xE)
    }
    #[doc = "Divide by 30"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Pdiv::B0xF)
    }
}
impl R {
    #[doc = "Bits 0:3 - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn pdiv(&self) -> PdivR {
        PdivR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_CKDIV")
            .field("pdiv", &self.pdiv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn pdiv(&mut self) -> PdivW<FdcanCkdivSpec> {
        PdivW::new(self, 0)
    }
}
#[doc = "FDCAN CFG clock divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ckdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ckdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanCkdivSpec;
impl crate::RegisterSpec for FdcanCkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ckdiv::R`](R) reader structure"]
impl crate::Readable for FdcanCkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ckdiv::W`](W) writer structure"]
impl crate::Writable for FdcanCkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_CKDIV to value 0"]
impl crate::Resettable for FdcanCkdivSpec {
    const RESET_VALUE: u32 = 0;
}

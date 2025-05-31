#[doc = "Register `RCC_PLLCFGR` reader"]
pub type R = crate::R<RccPllcfgrSpec>;
#[doc = "Register `RCC_PLLCFGR` writer"]
pub type W = crate::W<RccPllcfgrSpec>;
#[doc = "Main PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC should be 00.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pllsrc {
    #[doc = "0: No clock sent to PLL"]
    B0x0 = 0,
    #[doc = "1: No clock sent to PLL"]
    B0x1 = 1,
    #[doc = "2: HSI16 clock selected as PLL clock entry"]
    B0x2 = 2,
    #[doc = "3: HSE clock selected as PLL clock entry"]
    B0x3 = 3,
}
impl From<Pllsrc> for u8 {
    #[inline(always)]
    fn from(variant: Pllsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pllsrc {
    type Ux = u8;
}
impl crate::IsEnum for Pllsrc {}
#[doc = "Field `PLLSRC` reader - Main PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC should be 00."]
pub type PllsrcR = crate::FieldReader<Pllsrc>;
impl PllsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllsrc {
        match self.bits {
            0 => Pllsrc::B0x0,
            1 => Pllsrc::B0x1,
            2 => Pllsrc::B0x2,
            3 => Pllsrc::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock sent to PLL"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pllsrc::B0x0
    }
    #[doc = "No clock sent to PLL"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllsrc::B0x1
    }
    #[doc = "HSI16 clock selected as PLL clock entry"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pllsrc::B0x2
    }
    #[doc = "HSE clock selected as PLL clock entry"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Pllsrc::B0x3
    }
}
#[doc = "Field `PLLSRC` writer - Main PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC should be 00."]
pub type PllsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pllsrc, crate::Safe>;
impl<'a, REG> PllsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock sent to PLL"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsrc::B0x0)
    }
    #[doc = "No clock sent to PLL"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsrc::B0x1)
    }
    #[doc = "HSI16 clock selected as PLL clock entry"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsrc::B0x2)
    }
    #[doc = "HSE clock selected as PLL clock entry"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsrc::B0x3)
    }
}
#[doc = "Division factor for the main PLL input clock Set and cleared by software to divide the PLL input clock before the VCO. These bits can be written only when all PLLs are disabled. VCO input frequency = PLL input clock frequency / PLLM with 1 <= PLLM <= 16 ... Note: The software has to set these bits correctly to ensure that the VCO input frequency is within the range defined in the device datasheet.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pllm {
    #[doc = "0: PLLM = 1"]
    B0x0 = 0,
    #[doc = "1: PLLM = 2"]
    B0x1 = 1,
    #[doc = "2: PLLM = 3"]
    B0x2 = 2,
    #[doc = "3: PLLM = 4"]
    B0x3 = 3,
    #[doc = "4: PLLM = 5"]
    B0x4 = 4,
    #[doc = "5: PLLM = 6"]
    B0x5 = 5,
    #[doc = "6: PLLM = 7"]
    B0x6 = 6,
    #[doc = "7: PLLM = 8"]
    B0x7 = 7,
    #[doc = "8: PLLSYSM = 9"]
    B0x8 = 8,
    #[doc = "15: PLLSYSM= 16"]
    B0xF = 15,
}
impl From<Pllm> for u8 {
    #[inline(always)]
    fn from(variant: Pllm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pllm {
    type Ux = u8;
}
impl crate::IsEnum for Pllm {}
#[doc = "Field `PLLM` reader - Division factor for the main PLL input clock Set and cleared by software to divide the PLL input clock before the VCO. These bits can be written only when all PLLs are disabled. VCO input frequency = PLL input clock frequency / PLLM with 1 <= PLLM <= 16 ... Note: The software has to set these bits correctly to ensure that the VCO input frequency is within the range defined in the device datasheet."]
pub type PllmR = crate::FieldReader<Pllm>;
impl PllmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pllm> {
        match self.bits {
            0 => Some(Pllm::B0x0),
            1 => Some(Pllm::B0x1),
            2 => Some(Pllm::B0x2),
            3 => Some(Pllm::B0x3),
            4 => Some(Pllm::B0x4),
            5 => Some(Pllm::B0x5),
            6 => Some(Pllm::B0x6),
            7 => Some(Pllm::B0x7),
            8 => Some(Pllm::B0x8),
            15 => Some(Pllm::B0xF),
            _ => None,
        }
    }
    #[doc = "PLLM = 1"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pllm::B0x0
    }
    #[doc = "PLLM = 2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllm::B0x1
    }
    #[doc = "PLLM = 3"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pllm::B0x2
    }
    #[doc = "PLLM = 4"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Pllm::B0x3
    }
    #[doc = "PLLM = 5"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Pllm::B0x4
    }
    #[doc = "PLLM = 6"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Pllm::B0x5
    }
    #[doc = "PLLM = 7"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Pllm::B0x6
    }
    #[doc = "PLLM = 8"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Pllm::B0x7
    }
    #[doc = "PLLSYSM = 9"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Pllm::B0x8
    }
    #[doc = "PLLSYSM= 16"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Pllm::B0xF
    }
}
#[doc = "Field `PLLM` writer - Division factor for the main PLL input clock Set and cleared by software to divide the PLL input clock before the VCO. These bits can be written only when all PLLs are disabled. VCO input frequency = PLL input clock frequency / PLLM with 1 <= PLLM <= 16 ... Note: The software has to set these bits correctly to ensure that the VCO input frequency is within the range defined in the device datasheet."]
pub type PllmW<'a, REG> = crate::FieldWriter<'a, REG, 4, Pllm>;
impl<'a, REG> PllmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLLM = 1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::B0x0)
    }
    #[doc = "PLLM = 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::B0x1)
    }
    #[doc = "PLLM = 3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::B0x2)
    }
    #[doc = "PLLM = 4"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::B0x3)
    }
    #[doc = "PLLM = 5"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::B0x4)
    }
    #[doc = "PLLM = 6"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::B0x5)
    }
    #[doc = "PLLM = 7"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::B0x6)
    }
    #[doc = "PLLM = 8"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::B0x7)
    }
    #[doc = "PLLSYSM = 9"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::B0x8)
    }
    #[doc = "PLLSYSM= 16"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::B0xF)
    }
}
#[doc = "Main PLL multiplication factor for VCO Set and cleared by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled. VCO output frequency = VCO input frequency x PLLN with 8 =< PLLN =< 127 ... ... Note: The software has to set correctly these bits to assure that the VCO output frequency is within the range defined in the device datasheet.\n\nValue on reset: 16"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Plln {
    #[doc = "0: PLLN = 0 wrong configuration"]
    B0x0 = 0,
    #[doc = "1: PLLN = 1 wrong configuration"]
    B0x1 = 1,
    #[doc = "7: PLLN = 7 wrong configuration"]
    B0x7 = 7,
    #[doc = "8: PLLN = 8"]
    B0x8 = 8,
    #[doc = "9: PLLN = 9"]
    B0x9 = 9,
    #[doc = "127: PLLN = 127"]
    B0x7f = 127,
}
impl From<Plln> for u8 {
    #[inline(always)]
    fn from(variant: Plln) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Plln {
    type Ux = u8;
}
impl crate::IsEnum for Plln {}
#[doc = "Field `PLLN` reader - Main PLL multiplication factor for VCO Set and cleared by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled. VCO output frequency = VCO input frequency x PLLN with 8 =< PLLN =< 127 ... ... Note: The software has to set correctly these bits to assure that the VCO output frequency is within the range defined in the device datasheet."]
pub type PllnR = crate::FieldReader<Plln>;
impl PllnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Plln> {
        match self.bits {
            0 => Some(Plln::B0x0),
            1 => Some(Plln::B0x1),
            7 => Some(Plln::B0x7),
            8 => Some(Plln::B0x8),
            9 => Some(Plln::B0x9),
            127 => Some(Plln::B0x7f),
            _ => None,
        }
    }
    #[doc = "PLLN = 0 wrong configuration"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Plln::B0x0
    }
    #[doc = "PLLN = 1 wrong configuration"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Plln::B0x1
    }
    #[doc = "PLLN = 7 wrong configuration"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Plln::B0x7
    }
    #[doc = "PLLN = 8"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Plln::B0x8
    }
    #[doc = "PLLN = 9"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Plln::B0x9
    }
    #[doc = "PLLN = 127"]
    #[inline(always)]
    pub fn is_b_0x7f(&self) -> bool {
        *self == Plln::B0x7f
    }
}
#[doc = "Field `PLLN` writer - Main PLL multiplication factor for VCO Set and cleared by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled. VCO output frequency = VCO input frequency x PLLN with 8 =< PLLN =< 127 ... ... Note: The software has to set correctly these bits to assure that the VCO output frequency is within the range defined in the device datasheet."]
pub type PllnW<'a, REG> = crate::FieldWriter<'a, REG, 7, Plln>;
impl<'a, REG> PllnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLLN = 0 wrong configuration"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Plln::B0x0)
    }
    #[doc = "PLLN = 1 wrong configuration"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Plln::B0x1)
    }
    #[doc = "PLLN = 7 wrong configuration"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Plln::B0x7)
    }
    #[doc = "PLLN = 8"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Plln::B0x8)
    }
    #[doc = "PLLN = 9"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Plln::B0x9)
    }
    #[doc = "PLLN = 127"]
    #[inline(always)]
    pub fn b_0x7f(self) -> &'a mut crate::W<REG> {
        self.variant(Plln::B0x7f)
    }
}
#[doc = "Main PLL PLL P clock output enable Set and reset by software to enable the PLL P clock output of the PLL. In order to save power, when the PLL P clock output of the PLL is not used, the value of PLLPEN should be 0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllpen {
    #[doc = "0: PLL P clock output disable"]
    B0x0 = 0,
    #[doc = "1: PLL P clock output enable"]
    B0x1 = 1,
}
impl From<Pllpen> for bool {
    #[inline(always)]
    fn from(variant: Pllpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLPEN` reader - Main PLL PLL P clock output enable Set and reset by software to enable the PLL P clock output of the PLL. In order to save power, when the PLL P clock output of the PLL is not used, the value of PLLPEN should be 0."]
pub type PllpenR = crate::BitReader<Pllpen>;
impl PllpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllpen {
        match self.bits {
            false => Pllpen::B0x0,
            true => Pllpen::B0x1,
        }
    }
    #[doc = "PLL P clock output disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pllpen::B0x0
    }
    #[doc = "PLL P clock output enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllpen::B0x1
    }
}
#[doc = "Field `PLLPEN` writer - Main PLL PLL P clock output enable Set and reset by software to enable the PLL P clock output of the PLL. In order to save power, when the PLL P clock output of the PLL is not used, the value of PLLPEN should be 0."]
pub type PllpenW<'a, REG> = crate::BitWriter<'a, REG, Pllpen>;
impl<'a, REG> PllpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL P clock output disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllpen::B0x0)
    }
    #[doc = "PLL P clock output enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllpen::B0x1)
    }
}
#[doc = "Main PLL division factor for PLL P clock. Set and cleared by software to control the frequency of the main PLL output clock PLL P clock. These bits can be written only if PLL is disabled. When the PLLPDIV\\[4:0\\] is set to 00000PLL P output clock frequency = VCO frequency / PLLP with PLLP =7, or 17 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllp {
    #[doc = "0: PLLP = 7"]
    B0x0 = 0,
    #[doc = "1: PLLP = 17"]
    B0x1 = 1,
}
impl From<Pllp> for bool {
    #[inline(always)]
    fn from(variant: Pllp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLP` reader - Main PLL division factor for PLL P clock. Set and cleared by software to control the frequency of the main PLL output clock PLL P clock. These bits can be written only if PLL is disabled. When the PLLPDIV\\[4:0\\] is set to 00000PLL P output clock frequency = VCO frequency / PLLP with PLLP =7, or 17 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
pub type PllpR = crate::BitReader<Pllp>;
impl PllpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllp {
        match self.bits {
            false => Pllp::B0x0,
            true => Pllp::B0x1,
        }
    }
    #[doc = "PLLP = 7"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pllp::B0x0
    }
    #[doc = "PLLP = 17"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllp::B0x1
    }
}
#[doc = "Field `PLLP` writer - Main PLL division factor for PLL P clock. Set and cleared by software to control the frequency of the main PLL output clock PLL P clock. These bits can be written only if PLL is disabled. When the PLLPDIV\\[4:0\\] is set to 00000PLL P output clock frequency = VCO frequency / PLLP with PLLP =7, or 17 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
pub type PllpW<'a, REG> = crate::BitWriter<'a, REG, Pllp>;
impl<'a, REG> PllpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLLP = 7"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllp::B0x0)
    }
    #[doc = "PLLP = 17"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllp::B0x1)
    }
}
#[doc = "Main PLL Q clock output enable Set and reset by software to enable the PLL Q clock output of the PLL. In order to save power, when the PLL Q clock output of the PLL is not used, the value of PLLQEN should be 0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllqen {
    #[doc = "0: PLL Q clock output disable"]
    B0x0 = 0,
    #[doc = "1: PLL Q clock output enable"]
    B0x1 = 1,
}
impl From<Pllqen> for bool {
    #[inline(always)]
    fn from(variant: Pllqen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLQEN` reader - Main PLL Q clock output enable Set and reset by software to enable the PLL Q clock output of the PLL. In order to save power, when the PLL Q clock output of the PLL is not used, the value of PLLQEN should be 0."]
pub type PllqenR = crate::BitReader<Pllqen>;
impl PllqenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllqen {
        match self.bits {
            false => Pllqen::B0x0,
            true => Pllqen::B0x1,
        }
    }
    #[doc = "PLL Q clock output disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pllqen::B0x0
    }
    #[doc = "PLL Q clock output enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllqen::B0x1
    }
}
#[doc = "Field `PLLQEN` writer - Main PLL Q clock output enable Set and reset by software to enable the PLL Q clock output of the PLL. In order to save power, when the PLL Q clock output of the PLL is not used, the value of PLLQEN should be 0."]
pub type PllqenW<'a, REG> = crate::BitWriter<'a, REG, Pllqen>;
impl<'a, REG> PllqenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL Q clock output disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllqen::B0x0)
    }
    #[doc = "PLL Q clock output enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllqen::B0x1)
    }
}
#[doc = "Main PLL division factor for PLL Q clock. Set and cleared by software to control the frequency of the main PLL output clock PLL Q clock. This output can be selected for USB, RNG, SAI (48 MHz clock). These bits can be written only if PLL is disabled. PLL Q output clock frequency = VCO frequency / PLLQ with PLLQ = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pllq {
    #[doc = "0: PLLQ = 2"]
    B0x0 = 0,
    #[doc = "1: PLLQ = 4"]
    B0x1 = 1,
    #[doc = "2: PLLQ = 6"]
    B0x2 = 2,
    #[doc = "3: PLLQ = 8"]
    B0x3 = 3,
}
impl From<Pllq> for u8 {
    #[inline(always)]
    fn from(variant: Pllq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pllq {
    type Ux = u8;
}
impl crate::IsEnum for Pllq {}
#[doc = "Field `PLLQ` reader - Main PLL division factor for PLL Q clock. Set and cleared by software to control the frequency of the main PLL output clock PLL Q clock. This output can be selected for USB, RNG, SAI (48 MHz clock). These bits can be written only if PLL is disabled. PLL Q output clock frequency = VCO frequency / PLLQ with PLLQ = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
pub type PllqR = crate::FieldReader<Pllq>;
impl PllqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllq {
        match self.bits {
            0 => Pllq::B0x0,
            1 => Pllq::B0x1,
            2 => Pllq::B0x2,
            3 => Pllq::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "PLLQ = 2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pllq::B0x0
    }
    #[doc = "PLLQ = 4"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllq::B0x1
    }
    #[doc = "PLLQ = 6"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pllq::B0x2
    }
    #[doc = "PLLQ = 8"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Pllq::B0x3
    }
}
#[doc = "Field `PLLQ` writer - Main PLL division factor for PLL Q clock. Set and cleared by software to control the frequency of the main PLL output clock PLL Q clock. This output can be selected for USB, RNG, SAI (48 MHz clock). These bits can be written only if PLL is disabled. PLL Q output clock frequency = VCO frequency / PLLQ with PLLQ = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
pub type PllqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pllq, crate::Safe>;
impl<'a, REG> PllqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLLQ = 2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::B0x0)
    }
    #[doc = "PLLQ = 4"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::B0x1)
    }
    #[doc = "PLLQ = 6"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::B0x2)
    }
    #[doc = "PLLQ = 8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::B0x3)
    }
}
#[doc = "PLL R clock output enable Set and reset by software to enable the PLL R clock output of the PLL (used as system clock). This bit cannot be written when PLL R clock output of the PLL is used as System Clock. In order to save power, when the PLL R clock output of the PLL is not used, the value of PLLREN should be 0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllren {
    #[doc = "0: PLL R clock output disable"]
    B0x0 = 0,
    #[doc = "1: PLL R clock output enable"]
    B0x1 = 1,
}
impl From<Pllren> for bool {
    #[inline(always)]
    fn from(variant: Pllren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLREN` reader - PLL R clock output enable Set and reset by software to enable the PLL R clock output of the PLL (used as system clock). This bit cannot be written when PLL R clock output of the PLL is used as System Clock. In order to save power, when the PLL R clock output of the PLL is not used, the value of PLLREN should be 0."]
pub type PllrenR = crate::BitReader<Pllren>;
impl PllrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllren {
        match self.bits {
            false => Pllren::B0x0,
            true => Pllren::B0x1,
        }
    }
    #[doc = "PLL R clock output disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pllren::B0x0
    }
    #[doc = "PLL R clock output enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllren::B0x1
    }
}
#[doc = "Field `PLLREN` writer - PLL R clock output enable Set and reset by software to enable the PLL R clock output of the PLL (used as system clock). This bit cannot be written when PLL R clock output of the PLL is used as System Clock. In order to save power, when the PLL R clock output of the PLL is not used, the value of PLLREN should be 0."]
pub type PllrenW<'a, REG> = crate::BitWriter<'a, REG, Pllren>;
impl<'a, REG> PllrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL R clock output disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllren::B0x0)
    }
    #[doc = "PLL R clock output enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllren::B0x1)
    }
}
#[doc = "Main PLL division factor for PLL R clock (system clock) Set and cleared by software to control the frequency of the main PLL output clock PLLCLK. This output can be selected as system clock. These bits can be written only if PLL is disabled. PLL R output clock frequency = VCO frequency / PLLR with PLLR = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pllr {
    #[doc = "0: PLLR = 2"]
    B0x0 = 0,
    #[doc = "1: PLLR = 4"]
    B0x1 = 1,
    #[doc = "2: PLLR = 6"]
    B0x2 = 2,
    #[doc = "3: PLLR = 8"]
    B0x3 = 3,
}
impl From<Pllr> for u8 {
    #[inline(always)]
    fn from(variant: Pllr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pllr {
    type Ux = u8;
}
impl crate::IsEnum for Pllr {}
#[doc = "Field `PLLR` reader - Main PLL division factor for PLL R clock (system clock) Set and cleared by software to control the frequency of the main PLL output clock PLLCLK. This output can be selected as system clock. These bits can be written only if PLL is disabled. PLL R output clock frequency = VCO frequency / PLLR with PLLR = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
pub type PllrR = crate::FieldReader<Pllr>;
impl PllrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllr {
        match self.bits {
            0 => Pllr::B0x0,
            1 => Pllr::B0x1,
            2 => Pllr::B0x2,
            3 => Pllr::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "PLLR = 2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pllr::B0x0
    }
    #[doc = "PLLR = 4"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllr::B0x1
    }
    #[doc = "PLLR = 6"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pllr::B0x2
    }
    #[doc = "PLLR = 8"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Pllr::B0x3
    }
}
#[doc = "Field `PLLR` writer - Main PLL division factor for PLL R clock (system clock) Set and cleared by software to control the frequency of the main PLL output clock PLLCLK. This output can be selected as system clock. These bits can be written only if PLL is disabled. PLL R output clock frequency = VCO frequency / PLLR with PLLR = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
pub type PllrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pllr, crate::Safe>;
impl<'a, REG> PllrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLLR = 2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllr::B0x0)
    }
    #[doc = "PLLR = 4"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllr::B0x1)
    }
    #[doc = "PLLR = 6"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pllr::B0x2)
    }
    #[doc = "PLLR = 8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Pllr::B0x3)
    }
}
#[doc = "Main PLLP division factor Set and cleared by software to control the PLL P frequency. PLL P output clock frequency = VCO frequency / PLLPDIV. ....\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pllpdiv {
    #[doc = "0: PLL P clock is controlled by the bit PLLP"]
    B0x0 = 0,
    #[doc = "1: Reserved."]
    B0x1 = 1,
    #[doc = "2: PLL P clock = VCO / 2"]
    B0x2 = 2,
    #[doc = "31: PLL P clock = VCO / 31"]
    B0x1f = 31,
}
impl From<Pllpdiv> for u8 {
    #[inline(always)]
    fn from(variant: Pllpdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pllpdiv {
    type Ux = u8;
}
impl crate::IsEnum for Pllpdiv {}
#[doc = "Field `PLLPDIV` reader - Main PLLP division factor Set and cleared by software to control the PLL P frequency. PLL P output clock frequency = VCO frequency / PLLPDIV. ...."]
pub type PllpdivR = crate::FieldReader<Pllpdiv>;
impl PllpdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pllpdiv> {
        match self.bits {
            0 => Some(Pllpdiv::B0x0),
            1 => Some(Pllpdiv::B0x1),
            2 => Some(Pllpdiv::B0x2),
            31 => Some(Pllpdiv::B0x1f),
            _ => None,
        }
    }
    #[doc = "PLL P clock is controlled by the bit PLLP"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pllpdiv::B0x0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllpdiv::B0x1
    }
    #[doc = "PLL P clock = VCO / 2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pllpdiv::B0x2
    }
    #[doc = "PLL P clock = VCO / 31"]
    #[inline(always)]
    pub fn is_b_0x1f(&self) -> bool {
        *self == Pllpdiv::B0x1f
    }
}
#[doc = "Field `PLLPDIV` writer - Main PLLP division factor Set and cleared by software to control the PLL P frequency. PLL P output clock frequency = VCO frequency / PLLPDIV. ...."]
pub type PllpdivW<'a, REG> = crate::FieldWriter<'a, REG, 5, Pllpdiv>;
impl<'a, REG> PllpdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLL P clock is controlled by the bit PLLP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllpdiv::B0x0)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllpdiv::B0x1)
    }
    #[doc = "PLL P clock = VCO / 2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pllpdiv::B0x2)
    }
    #[doc = "PLL P clock = VCO / 31"]
    #[inline(always)]
    pub fn b_0x1f(self) -> &'a mut crate::W<REG> {
        self.variant(Pllpdiv::B0x1f)
    }
}
impl R {
    #[doc = "Bits 0:1 - Main PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC should be 00."]
    #[inline(always)]
    pub fn pllsrc(&self) -> PllsrcR {
        PllsrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:7 - Division factor for the main PLL input clock Set and cleared by software to divide the PLL input clock before the VCO. These bits can be written only when all PLLs are disabled. VCO input frequency = PLL input clock frequency / PLLM with 1 <= PLLM <= 16 ... Note: The software has to set these bits correctly to ensure that the VCO input frequency is within the range defined in the device datasheet."]
    #[inline(always)]
    pub fn pllm(&self) -> PllmR {
        PllmR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - Main PLL multiplication factor for VCO Set and cleared by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled. VCO output frequency = VCO input frequency x PLLN with 8 =< PLLN =< 127 ... ... Note: The software has to set correctly these bits to assure that the VCO output frequency is within the range defined in the device datasheet."]
    #[inline(always)]
    pub fn plln(&self) -> PllnR {
        PllnR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Main PLL PLL P clock output enable Set and reset by software to enable the PLL P clock output of the PLL. In order to save power, when the PLL P clock output of the PLL is not used, the value of PLLPEN should be 0."]
    #[inline(always)]
    pub fn pllpen(&self) -> PllpenR {
        PllpenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Main PLL division factor for PLL P clock. Set and cleared by software to control the frequency of the main PLL output clock PLL P clock. These bits can be written only if PLL is disabled. When the PLLPDIV\\[4:0\\] is set to 00000PLL P output clock frequency = VCO frequency / PLLP with PLLP =7, or 17 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
    #[inline(always)]
    pub fn pllp(&self) -> PllpR {
        PllpR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Main PLL Q clock output enable Set and reset by software to enable the PLL Q clock output of the PLL. In order to save power, when the PLL Q clock output of the PLL is not used, the value of PLLQEN should be 0."]
    #[inline(always)]
    pub fn pllqen(&self) -> PllqenR {
        PllqenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Main PLL division factor for PLL Q clock. Set and cleared by software to control the frequency of the main PLL output clock PLL Q clock. This output can be selected for USB, RNG, SAI (48 MHz clock). These bits can be written only if PLL is disabled. PLL Q output clock frequency = VCO frequency / PLLQ with PLLQ = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
    #[inline(always)]
    pub fn pllq(&self) -> PllqR {
        PllqR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 24 - PLL R clock output enable Set and reset by software to enable the PLL R clock output of the PLL (used as system clock). This bit cannot be written when PLL R clock output of the PLL is used as System Clock. In order to save power, when the PLL R clock output of the PLL is not used, the value of PLLREN should be 0."]
    #[inline(always)]
    pub fn pllren(&self) -> PllrenR {
        PllrenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Main PLL division factor for PLL R clock (system clock) Set and cleared by software to control the frequency of the main PLL output clock PLLCLK. This output can be selected as system clock. These bits can be written only if PLL is disabled. PLL R output clock frequency = VCO frequency / PLLR with PLLR = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
    #[inline(always)]
    pub fn pllr(&self) -> PllrR {
        PllrR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:31 - Main PLLP division factor Set and cleared by software to control the PLL P frequency. PLL P output clock frequency = VCO frequency / PLLPDIV. ...."]
    #[inline(always)]
    pub fn pllpdiv(&self) -> PllpdivR {
        PllpdivR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLLCFGR")
            .field("pllsrc", &self.pllsrc())
            .field("pllm", &self.pllm())
            .field("plln", &self.plln())
            .field("pllpen", &self.pllpen())
            .field("pllp", &self.pllp())
            .field("pllqen", &self.pllqen())
            .field("pllq", &self.pllq())
            .field("pllren", &self.pllren())
            .field("pllr", &self.pllr())
            .field("pllpdiv", &self.pllpdiv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Main PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC should be 00."]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PllsrcW<RccPllcfgrSpec> {
        PllsrcW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Division factor for the main PLL input clock Set and cleared by software to divide the PLL input clock before the VCO. These bits can be written only when all PLLs are disabled. VCO input frequency = PLL input clock frequency / PLLM with 1 <= PLLM <= 16 ... Note: The software has to set these bits correctly to ensure that the VCO input frequency is within the range defined in the device datasheet."]
    #[inline(always)]
    pub fn pllm(&mut self) -> PllmW<RccPllcfgrSpec> {
        PllmW::new(self, 4)
    }
    #[doc = "Bits 8:14 - Main PLL multiplication factor for VCO Set and cleared by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled. VCO output frequency = VCO input frequency x PLLN with 8 =< PLLN =< 127 ... ... Note: The software has to set correctly these bits to assure that the VCO output frequency is within the range defined in the device datasheet."]
    #[inline(always)]
    pub fn plln(&mut self) -> PllnW<RccPllcfgrSpec> {
        PllnW::new(self, 8)
    }
    #[doc = "Bit 16 - Main PLL PLL P clock output enable Set and reset by software to enable the PLL P clock output of the PLL. In order to save power, when the PLL P clock output of the PLL is not used, the value of PLLPEN should be 0."]
    #[inline(always)]
    pub fn pllpen(&mut self) -> PllpenW<RccPllcfgrSpec> {
        PllpenW::new(self, 16)
    }
    #[doc = "Bit 17 - Main PLL division factor for PLL P clock. Set and cleared by software to control the frequency of the main PLL output clock PLL P clock. These bits can be written only if PLL is disabled. When the PLLPDIV\\[4:0\\] is set to 00000PLL P output clock frequency = VCO frequency / PLLP with PLLP =7, or 17 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
    #[inline(always)]
    pub fn pllp(&mut self) -> PllpW<RccPllcfgrSpec> {
        PllpW::new(self, 17)
    }
    #[doc = "Bit 20 - Main PLL Q clock output enable Set and reset by software to enable the PLL Q clock output of the PLL. In order to save power, when the PLL Q clock output of the PLL is not used, the value of PLLQEN should be 0."]
    #[inline(always)]
    pub fn pllqen(&mut self) -> PllqenW<RccPllcfgrSpec> {
        PllqenW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Main PLL division factor for PLL Q clock. Set and cleared by software to control the frequency of the main PLL output clock PLL Q clock. This output can be selected for USB, RNG, SAI (48 MHz clock). These bits can be written only if PLL is disabled. PLL Q output clock frequency = VCO frequency / PLLQ with PLLQ = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
    #[inline(always)]
    pub fn pllq(&mut self) -> PllqW<RccPllcfgrSpec> {
        PllqW::new(self, 21)
    }
    #[doc = "Bit 24 - PLL R clock output enable Set and reset by software to enable the PLL R clock output of the PLL (used as system clock). This bit cannot be written when PLL R clock output of the PLL is used as System Clock. In order to save power, when the PLL R clock output of the PLL is not used, the value of PLLREN should be 0."]
    #[inline(always)]
    pub fn pllren(&mut self) -> PllrenW<RccPllcfgrSpec> {
        PllrenW::new(self, 24)
    }
    #[doc = "Bits 25:26 - Main PLL division factor for PLL R clock (system clock) Set and cleared by software to control the frequency of the main PLL output clock PLLCLK. This output can be selected as system clock. These bits can be written only if PLL is disabled. PLL R output clock frequency = VCO frequency / PLLR with PLLR = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
    #[inline(always)]
    pub fn pllr(&mut self) -> PllrW<RccPllcfgrSpec> {
        PllrW::new(self, 25)
    }
    #[doc = "Bits 27:31 - Main PLLP division factor Set and cleared by software to control the PLL P frequency. PLL P output clock frequency = VCO frequency / PLLPDIV. ...."]
    #[inline(always)]
    pub fn pllpdiv(&mut self) -> PllpdivW<RccPllcfgrSpec> {
        PllpdivW::new(self, 27)
    }
}
#[doc = "PLL configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_pllcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_pllcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccPllcfgrSpec;
impl crate::RegisterSpec for RccPllcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_pllcfgr::R`](R) reader structure"]
impl crate::Readable for RccPllcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_pllcfgr::W`](W) writer structure"]
impl crate::Writable for RccPllcfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCC_PLLCFGR to value 0x1000"]
impl crate::Resettable for RccPllcfgrSpec {
    const RESET_VALUE: u32 = 0x1000;
}

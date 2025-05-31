#[doc = "Register `PWR_CR2` reader"]
pub type R = crate::R<PwrCr2Spec>;
#[doc = "Register `PWR_CR2` writer"]
pub type W = crate::W<PwrCr2Spec>;
#[doc = "Programmable voltage detector enable Note: This bit is write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pvde {
    #[doc = "0: Programmable voltage detector disable."]
    B0x0 = 0,
    #[doc = "1: Programmable voltage detector enable."]
    B0x1 = 1,
}
impl From<Pvde> for bool {
    #[inline(always)]
    fn from(variant: Pvde) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDE` reader - Programmable voltage detector enable Note: This bit is write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset."]
pub type PvdeR = crate::BitReader<Pvde>;
impl PvdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pvde {
        match self.bits {
            false => Pvde::B0x0,
            true => Pvde::B0x1,
        }
    }
    #[doc = "Programmable voltage detector disable."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pvde::B0x0
    }
    #[doc = "Programmable voltage detector enable."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pvde::B0x1
    }
}
#[doc = "Field `PVDE` writer - Programmable voltage detector enable Note: This bit is write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset."]
pub type PvdeW<'a, REG> = crate::BitWriter<'a, REG, Pvde>;
impl<'a, REG> PvdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Programmable voltage detector disable."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pvde::B0x0)
    }
    #[doc = "Programmable voltage detector enable."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pvde::B0x1)
    }
}
#[doc = "Programmable voltage detector level selection. These bits select the PVD falling threshold: Note: These bits are write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pvdls {
    #[doc = "0: V<sub>PVD0</sub> PVD threshold 0"]
    B0x0 = 0,
    #[doc = "1: V<sub>PVD1</sub> PVD threshold 1"]
    B0x1 = 1,
    #[doc = "2: V<sub>PVD2</sub> PVD threshold 2"]
    B0x2 = 2,
    #[doc = "3: V<sub>PVD3</sub> PVD threshold 3"]
    B0x3 = 3,
    #[doc = "4: V<sub>PVD4</sub> PVD threshold 4"]
    B0x4 = 4,
    #[doc = "5: V<sub>PVD5</sub> PVD threshold 5"]
    B0x5 = 5,
    #[doc = "6: V<sub>PVD6</sub> PVD threshold 6"]
    B0x6 = 6,
    #[doc = "7: External input analog voltage PVD_IN (compared internally to V<sub>REFINT</sub>)"]
    B0x7 = 7,
}
impl From<Pvdls> for u8 {
    #[inline(always)]
    fn from(variant: Pvdls) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pvdls {
    type Ux = u8;
}
impl crate::IsEnum for Pvdls {}
#[doc = "Field `PVDLS` reader - Programmable voltage detector level selection. These bits select the PVD falling threshold: Note: These bits are write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset."]
pub type PvdlsR = crate::FieldReader<Pvdls>;
impl PvdlsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pvdls {
        match self.bits {
            0 => Pvdls::B0x0,
            1 => Pvdls::B0x1,
            2 => Pvdls::B0x2,
            3 => Pvdls::B0x3,
            4 => Pvdls::B0x4,
            5 => Pvdls::B0x5,
            6 => Pvdls::B0x6,
            7 => Pvdls::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "V<sub>PVD0</sub> PVD threshold 0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pvdls::B0x0
    }
    #[doc = "V<sub>PVD1</sub> PVD threshold 1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pvdls::B0x1
    }
    #[doc = "V<sub>PVD2</sub> PVD threshold 2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pvdls::B0x2
    }
    #[doc = "V<sub>PVD3</sub> PVD threshold 3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Pvdls::B0x3
    }
    #[doc = "V<sub>PVD4</sub> PVD threshold 4"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Pvdls::B0x4
    }
    #[doc = "V<sub>PVD5</sub> PVD threshold 5"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Pvdls::B0x5
    }
    #[doc = "V<sub>PVD6</sub> PVD threshold 6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Pvdls::B0x6
    }
    #[doc = "External input analog voltage PVD_IN (compared internally to V<sub>REFINT</sub>)"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Pvdls::B0x7
    }
}
#[doc = "Field `PVDLS` writer - Programmable voltage detector level selection. These bits select the PVD falling threshold: Note: These bits are write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset."]
pub type PvdlsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pvdls, crate::Safe>;
impl<'a, REG> PvdlsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "V<sub>PVD0</sub> PVD threshold 0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pvdls::B0x0)
    }
    #[doc = "V<sub>PVD1</sub> PVD threshold 1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pvdls::B0x1)
    }
    #[doc = "V<sub>PVD2</sub> PVD threshold 2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pvdls::B0x2)
    }
    #[doc = "V<sub>PVD3</sub> PVD threshold 3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Pvdls::B0x3)
    }
    #[doc = "V<sub>PVD4</sub> PVD threshold 4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Pvdls::B0x4)
    }
    #[doc = "V<sub>PVD5</sub> PVD threshold 5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Pvdls::B0x5)
    }
    #[doc = "V<sub>PVD6</sub> PVD threshold 6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Pvdls::B0x6)
    }
    #[doc = "External input analog voltage PVD_IN (compared internally to V<sub>REFINT</sub>)"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Pvdls::B0x7)
    }
}
#[doc = "Peripheral voltage monitoring 3 enable: V<sub>DDA</sub> vs. ADC/COMP min voltage 1.62V\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pvmen1 {
    #[doc = "0: PVM1 (V<sub>DDA</sub> monitoring vs. 1.62V threshold) disable."]
    B0x0 = 0,
    #[doc = "1: PVM1 (V<sub>DDA</sub> monitoring vs. 1.62V threshold) enable."]
    B0x1 = 1,
}
impl From<Pvmen1> for bool {
    #[inline(always)]
    fn from(variant: Pvmen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVMEN1` reader - Peripheral voltage monitoring 3 enable: V<sub>DDA</sub> vs. ADC/COMP min voltage 1.62V"]
pub type Pvmen1R = crate::BitReader<Pvmen1>;
impl Pvmen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pvmen1 {
        match self.bits {
            false => Pvmen1::B0x0,
            true => Pvmen1::B0x1,
        }
    }
    #[doc = "PVM1 (V<sub>DDA</sub> monitoring vs. 1.62V threshold) disable."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pvmen1::B0x0
    }
    #[doc = "PVM1 (V<sub>DDA</sub> monitoring vs. 1.62V threshold) enable."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pvmen1::B0x1
    }
}
#[doc = "Field `PVMEN1` writer - Peripheral voltage monitoring 3 enable: V<sub>DDA</sub> vs. ADC/COMP min voltage 1.62V"]
pub type Pvmen1W<'a, REG> = crate::BitWriter<'a, REG, Pvmen1>;
impl<'a, REG> Pvmen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PVM1 (V<sub>DDA</sub> monitoring vs. 1.62V threshold) disable."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pvmen1::B0x0)
    }
    #[doc = "PVM1 (V<sub>DDA</sub> monitoring vs. 1.62V threshold) enable."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pvmen1::B0x1)
    }
}
#[doc = "Peripheral voltage monitoring 4 enable: V<sub>DDA</sub> vs. DAC 1MSPS /DAC 15MSPS min voltage.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pvmen2 {
    #[doc = "0: PVM2 (V<sub>DDA</sub> monitoring vs. 1.8 V threshold) disable."]
    B0x0 = 0,
    #[doc = "1: PVM2 (V<sub>DDA</sub> monitoring vs. 1.8 V threshold) enable."]
    B0x1 = 1,
}
impl From<Pvmen2> for bool {
    #[inline(always)]
    fn from(variant: Pvmen2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVMEN2` reader - Peripheral voltage monitoring 4 enable: V<sub>DDA</sub> vs. DAC 1MSPS /DAC 15MSPS min voltage."]
pub type Pvmen2R = crate::BitReader<Pvmen2>;
impl Pvmen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pvmen2 {
        match self.bits {
            false => Pvmen2::B0x0,
            true => Pvmen2::B0x1,
        }
    }
    #[doc = "PVM2 (V<sub>DDA</sub> monitoring vs. 1.8 V threshold) disable."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pvmen2::B0x0
    }
    #[doc = "PVM2 (V<sub>DDA</sub> monitoring vs. 1.8 V threshold) enable."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pvmen2::B0x1
    }
}
#[doc = "Field `PVMEN2` writer - Peripheral voltage monitoring 4 enable: V<sub>DDA</sub> vs. DAC 1MSPS /DAC 15MSPS min voltage."]
pub type Pvmen2W<'a, REG> = crate::BitWriter<'a, REG, Pvmen2>;
impl<'a, REG> Pvmen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PVM2 (V<sub>DDA</sub> monitoring vs. 1.8 V threshold) disable."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pvmen2::B0x0)
    }
    #[doc = "PVM2 (V<sub>DDA</sub> monitoring vs. 1.8 V threshold) enable."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pvmen2::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Programmable voltage detector enable Note: This bit is write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset."]
    #[inline(always)]
    pub fn pvde(&self) -> PvdeR {
        PvdeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Programmable voltage detector level selection. These bits select the PVD falling threshold: Note: These bits are write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset."]
    #[inline(always)]
    pub fn pvdls(&self) -> PvdlsR {
        PvdlsR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: V<sub>DDA</sub> vs. ADC/COMP min voltage 1.62V"]
    #[inline(always)]
    pub fn pvmen1(&self) -> Pvmen1R {
        Pvmen1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral voltage monitoring 4 enable: V<sub>DDA</sub> vs. DAC 1MSPS /DAC 15MSPS min voltage."]
    #[inline(always)]
    pub fn pvmen2(&self) -> Pvmen2R {
        Pvmen2R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_CR2")
            .field("pvde", &self.pvde())
            .field("pvdls", &self.pvdls())
            .field("pvmen1", &self.pvmen1())
            .field("pvmen2", &self.pvmen2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Programmable voltage detector enable Note: This bit is write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset."]
    #[inline(always)]
    pub fn pvde(&mut self) -> PvdeW<PwrCr2Spec> {
        PvdeW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Programmable voltage detector level selection. These bits select the PVD falling threshold: Note: These bits are write-protected when the PVDL bit is set in the SYSCFG_CFGR2 register. The protection can be reset only by a system reset."]
    #[inline(always)]
    pub fn pvdls(&mut self) -> PvdlsW<PwrCr2Spec> {
        PvdlsW::new(self, 1)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: V<sub>DDA</sub> vs. ADC/COMP min voltage 1.62V"]
    #[inline(always)]
    pub fn pvmen1(&mut self) -> Pvmen1W<PwrCr2Spec> {
        Pvmen1W::new(self, 6)
    }
    #[doc = "Bit 7 - Peripheral voltage monitoring 4 enable: V<sub>DDA</sub> vs. DAC 1MSPS /DAC 15MSPS min voltage."]
    #[inline(always)]
    pub fn pvmen2(&mut self) -> Pvmen2W<PwrCr2Spec> {
        Pvmen2W::new(self, 7)
    }
}
#[doc = "Power control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrCr2Spec;
impl crate::RegisterSpec for PwrCr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_cr2::R`](R) reader structure"]
impl crate::Readable for PwrCr2Spec {}
#[doc = "`write(|w| ..)` method takes [`pwr_cr2::W`](W) writer structure"]
impl crate::Writable for PwrCr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWR_CR2 to value 0"]
impl crate::Resettable for PwrCr2Spec {}

#[doc = "Register `ODISR` writer"]
pub type W = crate::W<OdisrSpec>;
#[doc = "Field `TA1ODIS` writer - TA1ODIS"]
pub type Ta1odisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TA2ODIS` writer - TA2ODIS"]
pub type Ta2odisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TB1ODIS` writer - TB1ODIS"]
pub type Tb1odisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TB2ODIS` writer - TB2ODIS"]
pub type Tb2odisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1ODIS` writer - TC1ODIS"]
pub type Tc1odisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2ODIS` writer - TC2ODIS"]
pub type Tc2odisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TD1ODIS` writer - TD1ODIS"]
pub type Td1odisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TD2ODIS` writer - TD2ODIS"]
pub type Td2odisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE1ODIS` writer - TE1ODIS"]
pub type Te1odisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE2ODIS` writer - TE2ODIS"]
pub type Te2odisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TF1ODIS` writer - TF1ODIS"]
pub type Tf1odisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TF2ODIS` writer - TF2ODIS"]
pub type Tf2odisW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<OdisrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - TA1ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn ta1odis(&mut self) -> Ta1odisW<OdisrSpec> {
        Ta1odisW::new(self, 0)
    }
    #[doc = "Bit 1 - TA2ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn ta2odis(&mut self) -> Ta2odisW<OdisrSpec> {
        Ta2odisW::new(self, 1)
    }
    #[doc = "Bit 2 - TB1ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn tb1odis(&mut self) -> Tb1odisW<OdisrSpec> {
        Tb1odisW::new(self, 2)
    }
    #[doc = "Bit 3 - TB2ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn tb2odis(&mut self) -> Tb2odisW<OdisrSpec> {
        Tb2odisW::new(self, 3)
    }
    #[doc = "Bit 4 - TC1ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn tc1odis(&mut self) -> Tc1odisW<OdisrSpec> {
        Tc1odisW::new(self, 4)
    }
    #[doc = "Bit 5 - TC2ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn tc2odis(&mut self) -> Tc2odisW<OdisrSpec> {
        Tc2odisW::new(self, 5)
    }
    #[doc = "Bit 6 - TD1ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn td1odis(&mut self) -> Td1odisW<OdisrSpec> {
        Td1odisW::new(self, 6)
    }
    #[doc = "Bit 7 - TD2ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn td2odis(&mut self) -> Td2odisW<OdisrSpec> {
        Td2odisW::new(self, 7)
    }
    #[doc = "Bit 8 - TE1ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn te1odis(&mut self) -> Te1odisW<OdisrSpec> {
        Te1odisW::new(self, 8)
    }
    #[doc = "Bit 9 - TE2ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn te2odis(&mut self) -> Te2odisW<OdisrSpec> {
        Te2odisW::new(self, 9)
    }
    #[doc = "Bit 10 - TF1ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn tf1odis(&mut self) -> Tf1odisW<OdisrSpec> {
        Tf1odisW::new(self, 10)
    }
    #[doc = "Bit 11 - TF2ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn tf2odis(&mut self) -> Tf2odisW<OdisrSpec> {
        Tf2odisW::new(self, 11)
    }
}
#[doc = "ODISR\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odisr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OdisrSpec;
impl crate::RegisterSpec for OdisrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`odisr::W`](W) writer structure"]
impl crate::Writable for OdisrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ODISR to value 0"]
impl crate::Resettable for OdisrSpec {
    const RESET_VALUE: u32 = 0;
}

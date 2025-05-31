#[doc = "Register `FDCAN_TXBTIE` reader"]
pub type R = crate::R<FdcanTxbtieSpec>;
#[doc = "Register `FDCAN_TXBTIE` writer"]
pub type W = crate::W<FdcanTxbtieSpec>;
#[doc = "Transmission interrupt enable Each Tx buffer has its own TIE bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tie {
    #[doc = "0: Transmission interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Transmission interrupt enable"]
    B0x1 = 1,
}
impl From<Tie> for u8 {
    #[inline(always)]
    fn from(variant: Tie) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tie {
    type Ux = u8;
}
impl crate::IsEnum for Tie {}
#[doc = "Field `TIE` reader - Transmission interrupt enable Each Tx buffer has its own TIE bit."]
pub type TieR = crate::FieldReader<Tie>;
impl TieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tie> {
        match self.bits {
            0 => Some(Tie::B0x0),
            1 => Some(Tie::B0x1),
            _ => None,
        }
    }
    #[doc = "Transmission interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tie::B0x0
    }
    #[doc = "Transmission interrupt enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tie::B0x1
    }
}
#[doc = "Field `TIE` writer - Transmission interrupt enable Each Tx buffer has its own TIE bit."]
pub type TieW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tie>;
impl<'a, REG> TieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transmission interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::B0x0)
    }
    #[doc = "Transmission interrupt enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Transmission interrupt enable Each Tx buffer has its own TIE bit."]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBTIE")
            .field("tie", &self.tie())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmission interrupt enable Each Tx buffer has its own TIE bit."]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<FdcanTxbtieSpec> {
        TieW::new(self, 0)
    }
}
#[doc = "FDCAN Tx buffer transmission interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txbtie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbtie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTxbtieSpec;
impl crate::RegisterSpec for FdcanTxbtieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbtie::R`](R) reader structure"]
impl crate::Readable for FdcanTxbtieSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txbtie::W`](W) writer structure"]
impl crate::Writable for FdcanTxbtieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TXBTIE to value 0"]
impl crate::Resettable for FdcanTxbtieSpec {}

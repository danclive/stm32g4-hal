#[doc = "Register `FDCAN_TXBAR` reader"]
pub type R = crate::R<FdcanTxbarSpec>;
#[doc = "Register `FDCAN_TXBAR` writer"]
pub type W = crate::W<FdcanTxbarSpec>;
#[doc = "Add request Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ar {
    #[doc = "0: No transmission request added"]
    B0x0 = 0,
    #[doc = "1: Transmission requested added."]
    B0x1 = 1,
}
impl From<Ar> for u8 {
    #[inline(always)]
    fn from(variant: Ar) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ar {
    type Ux = u8;
}
impl crate::IsEnum for Ar {}
#[doc = "Field `AR` reader - Add request Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed."]
pub type ArR = crate::FieldReader<Ar>;
impl ArR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ar> {
        match self.bits {
            0 => Some(Ar::B0x0),
            1 => Some(Ar::B0x1),
            _ => None,
        }
    }
    #[doc = "No transmission request added"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ar::B0x0
    }
    #[doc = "Transmission requested added."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ar::B0x1
    }
}
#[doc = "Field `AR` writer - Add request Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed."]
pub type ArW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ar>;
impl<'a, REG> ArW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No transmission request added"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ar::B0x0)
    }
    #[doc = "Transmission requested added."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ar::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Add request Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed."]
    #[inline(always)]
    pub fn ar(&self) -> ArR {
        ArR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBAR")
            .field("ar", &self.ar())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Add request Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed."]
    #[inline(always)]
    pub fn ar(&mut self) -> ArW<FdcanTxbarSpec> {
        ArW::new(self, 0)
    }
}
#[doc = "FDCAN Tx buffer add request register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTxbarSpec;
impl crate::RegisterSpec for FdcanTxbarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbar::R`](R) reader structure"]
impl crate::Readable for FdcanTxbarSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txbar::W`](W) writer structure"]
impl crate::Writable for FdcanTxbarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TXBAR to value 0"]
impl crate::Resettable for FdcanTxbarSpec {
    const RESET_VALUE: u32 = 0;
}

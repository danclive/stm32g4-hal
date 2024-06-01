#[doc = "Register `DIFSEL` reader"]
pub type R = crate::R<DifselSpec>;
#[doc = "Register `DIFSEL` writer"]
pub type W = crate::W<DifselSpec>;
#[doc = "Field `DIFSEL_0` reader - Differential mode for channels 0"]
pub type Difsel0R = crate::BitReader;
#[doc = "Field `DIFSEL_1_18` reader - Differential mode for channels 15 to 1"]
pub type Difsel1_18R = crate::FieldReader<u32>;
#[doc = "Field `DIFSEL_1_18` writer - Differential mode for channels 15 to 1"]
pub type Difsel1_18W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bit 0 - Differential mode for channels 0"]
    #[inline(always)]
    pub fn difsel_0(&self) -> Difsel0R {
        Difsel0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:18 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_1_18(&self) -> Difsel1_18R {
        Difsel1_18R::new((self.bits >> 1) & 0x0003_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIFSEL")
            .field("difsel_0", &self.difsel_0())
            .field("difsel_1_18", &self.difsel_1_18())
            .finish()
    }
}
impl W {
    #[doc = "Bits 1:18 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_1_18(&mut self) -> Difsel1_18W<DifselSpec> {
        Difsel1_18W::new(self, 1)
    }
}
#[doc = "Differential Mode Selection Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`difsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`difsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DifselSpec;
impl crate::RegisterSpec for DifselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`difsel::R`](R) reader structure"]
impl crate::Readable for DifselSpec {}
#[doc = "`write(|w| ..)` method takes [`difsel::W`](W) writer structure"]
impl crate::Writable for DifselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIFSEL to value 0"]
impl crate::Resettable for DifselSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `TIMEICR` writer"]
pub type W = crate::W<TimeicrSpec>;
#[doc = "Field `CMP1C` writer - Compare 1 Interrupt flag Clear"]
pub type Cmp1cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2C` writer - Compare 2 Interrupt flag Clear"]
pub type Cmp2cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP3C` writer - Compare 3 Interrupt flag Clear"]
pub type Cmp3cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP4C` writer - Compare 4 Interrupt flag Clear"]
pub type Cmp4cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPC` writer - Repetition Interrupt flag Clear"]
pub type RepcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDC` writer - Update Interrupt flag Clear"]
pub type UpdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPT1C` writer - Capture1 Interrupt flag Clear"]
pub type Cpt1cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPT2C` writer - Capture2 Interrupt flag Clear"]
pub type Cpt2cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET1xC` writer - Output 1 Set flag Clear"]
pub type Set1xCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTx1C` writer - Output 1 Reset flag Clear"]
pub type Rstx1cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET2xC` writer - Output 2 Set flag Clear"]
pub type Set2xCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTx2C` writer - Output 2 Reset flag Clear"]
pub type Rstx2cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTC` writer - Reset Interrupt flag Clear"]
pub type RstcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLYPRTC` writer - Delayed Protection Flag Clear"]
pub type DlyprtcW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<TimeicrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Compare 1 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1c(&mut self) -> Cmp1cW<TimeicrSpec> {
        Cmp1cW::new(self, 0)
    }
    #[doc = "Bit 1 - Compare 2 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2c(&mut self) -> Cmp2cW<TimeicrSpec> {
        Cmp2cW::new(self, 1)
    }
    #[doc = "Bit 2 - Compare 3 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3c(&mut self) -> Cmp3cW<TimeicrSpec> {
        Cmp3cW::new(self, 2)
    }
    #[doc = "Bit 3 - Compare 4 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmp4c(&mut self) -> Cmp4cW<TimeicrSpec> {
        Cmp4cW::new(self, 3)
    }
    #[doc = "Bit 4 - Repetition Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn repc(&mut self) -> RepcW<TimeicrSpec> {
        RepcW::new(self, 4)
    }
    #[doc = "Bit 6 - Update Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn updc(&mut self) -> UpdcW<TimeicrSpec> {
        UpdcW::new(self, 6)
    }
    #[doc = "Bit 7 - Capture1 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cpt1c(&mut self) -> Cpt1cW<TimeicrSpec> {
        Cpt1cW::new(self, 7)
    }
    #[doc = "Bit 8 - Capture2 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cpt2c(&mut self) -> Cpt2cW<TimeicrSpec> {
        Cpt2cW::new(self, 8)
    }
    #[doc = "Bit 9 - Output 1 Set flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn set1x_c(&mut self) -> Set1xCW<TimeicrSpec> {
        Set1xCW::new(self, 9)
    }
    #[doc = "Bit 10 - Output 1 Reset flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstx1c(&mut self) -> Rstx1cW<TimeicrSpec> {
        Rstx1cW::new(self, 10)
    }
    #[doc = "Bit 11 - Output 2 Set flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn set2x_c(&mut self) -> Set2xCW<TimeicrSpec> {
        Set2xCW::new(self, 11)
    }
    #[doc = "Bit 12 - Output 2 Reset flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstx2c(&mut self) -> Rstx2cW<TimeicrSpec> {
        Rstx2cW::new(self, 12)
    }
    #[doc = "Bit 13 - Reset Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstc(&mut self) -> RstcW<TimeicrSpec> {
        RstcW::new(self, 13)
    }
    #[doc = "Bit 14 - Delayed Protection Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dlyprtc(&mut self) -> DlyprtcW<TimeicrSpec> {
        DlyprtcW::new(self, 14)
    }
}
#[doc = "Timerx Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeicrSpec;
impl crate::RegisterSpec for TimeicrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`timeicr::W`](W) writer structure"]
impl crate::Writable for TimeicrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMEICR to value 0"]
impl crate::Resettable for TimeicrSpec {
    const RESET_VALUE: u32 = 0;
}

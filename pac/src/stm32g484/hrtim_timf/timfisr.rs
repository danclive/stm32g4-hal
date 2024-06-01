#[doc = "Register `TIMFISR` reader"]
pub type R = crate::R<TimfisrSpec>;
#[doc = "Field `CMP1` reader - Compare 1 Interrupt Flag"]
pub type Cmp1R = crate::BitReader;
#[doc = "Field `CMP2` reader - Compare 2 Interrupt Flag"]
pub type Cmp2R = crate::BitReader;
#[doc = "Field `CMP3` reader - Compare 3 Interrupt Flag"]
pub type Cmp3R = crate::BitReader;
#[doc = "Field `CMP4` reader - Compare 4 Interrupt Flag"]
pub type Cmp4R = crate::BitReader;
#[doc = "Field `REP` reader - Repetition Interrupt Flag"]
pub type RepR = crate::BitReader;
#[doc = "Field `UPD` reader - Update Interrupt Flag"]
pub type UpdR = crate::BitReader;
#[doc = "Field `CPT1` reader - Capture1 Interrupt Flag"]
pub type Cpt1R = crate::BitReader;
#[doc = "Field `CPT2` reader - Capture2 Interrupt Flag"]
pub type Cpt2R = crate::BitReader;
#[doc = "Field `SETx1` reader - Output 1 Set Interrupt Flag"]
pub type Setx1R = crate::BitReader;
#[doc = "Field `RSTx1` reader - Output 1 Reset Interrupt Flag"]
pub type Rstx1R = crate::BitReader;
#[doc = "Field `SETx2` reader - Output 2 Set Interrupt Flag"]
pub type Setx2R = crate::BitReader;
#[doc = "Field `RSTx2` reader - Output 2 Reset Interrupt Flag"]
pub type Rstx2R = crate::BitReader;
#[doc = "Field `RST` reader - Reset Interrupt Flag"]
pub type RstR = crate::BitReader;
#[doc = "Field `DLYPRT` reader - Delayed Protection Flag"]
pub type DlyprtR = crate::BitReader;
#[doc = "Field `CPPSTAT` reader - Current Push Pull Status"]
pub type CppstatR = crate::BitReader;
#[doc = "Field `IPPSTAT` reader - Idle Push Pull Status"]
pub type IppstatR = crate::BitReader;
#[doc = "Field `O1STAT` reader - Output 1 State"]
pub type O1statR = crate::BitReader;
#[doc = "Field `O2STAT` reader - Output 2 State"]
pub type O2statR = crate::BitReader;
#[doc = "Field `O1CPY` reader - Output 1 Copy"]
pub type O1cpyR = crate::BitReader;
#[doc = "Field `O2CPY` reader - Output 2 Copy"]
pub type O2cpyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp1(&self) -> Cmp1R {
        Cmp1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp2(&self) -> Cmp2R {
        Cmp2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare 3 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp3(&self) -> Cmp3R {
        Cmp3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare 4 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp4(&self) -> Cmp4R {
        Cmp4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Repetition Interrupt Flag"]
    #[inline(always)]
    pub fn rep(&self) -> RepR {
        RepR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Update Interrupt Flag"]
    #[inline(always)]
    pub fn upd(&self) -> UpdR {
        UpdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture1 Interrupt Flag"]
    #[inline(always)]
    pub fn cpt1(&self) -> Cpt1R {
        Cpt1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture2 Interrupt Flag"]
    #[inline(always)]
    pub fn cpt2(&self) -> Cpt2R {
        Cpt2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output 1 Set Interrupt Flag"]
    #[inline(always)]
    pub fn setx1(&self) -> Setx1R {
        Setx1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output 1 Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rstx1(&self) -> Rstx1R {
        Rstx1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output 2 Set Interrupt Flag"]
    #[inline(always)]
    pub fn setx2(&self) -> Setx2R {
        Setx2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output 2 Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rstx2(&self) -> Rstx2R {
        Rstx2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Delayed Protection Flag"]
    #[inline(always)]
    pub fn dlyprt(&self) -> DlyprtR {
        DlyprtR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Current Push Pull Status"]
    #[inline(always)]
    pub fn cppstat(&self) -> CppstatR {
        CppstatR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Idle Push Pull Status"]
    #[inline(always)]
    pub fn ippstat(&self) -> IppstatR {
        IppstatR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output 1 State"]
    #[inline(always)]
    pub fn o1stat(&self) -> O1statR {
        O1statR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output 2 State"]
    #[inline(always)]
    pub fn o2stat(&self) -> O2statR {
        O2statR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output 1 Copy"]
    #[inline(always)]
    pub fn o1cpy(&self) -> O1cpyR {
        O1cpyR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Output 2 Copy"]
    #[inline(always)]
    pub fn o2cpy(&self) -> O2cpyR {
        O2cpyR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMFISR")
            .field("o2cpy", &self.o2cpy())
            .field("o1cpy", &self.o1cpy())
            .field("o2stat", &self.o2stat())
            .field("o1stat", &self.o1stat())
            .field("ippstat", &self.ippstat())
            .field("cppstat", &self.cppstat())
            .field("dlyprt", &self.dlyprt())
            .field("rst", &self.rst())
            .field("rstx2", &self.rstx2())
            .field("setx2", &self.setx2())
            .field("rstx1", &self.rstx1())
            .field("setx1", &self.setx1())
            .field("cpt2", &self.cpt2())
            .field("cpt1", &self.cpt1())
            .field("upd", &self.upd())
            .field("rep", &self.rep())
            .field("cmp4", &self.cmp4())
            .field("cmp3", &self.cmp3())
            .field("cmp2", &self.cmp2())
            .field("cmp1", &self.cmp1())
            .finish()
    }
}
#[doc = "Timerx Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timfisr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimfisrSpec;
impl crate::RegisterSpec for TimfisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timfisr::R`](R) reader structure"]
impl crate::Readable for TimfisrSpec {}
#[doc = "`reset()` method sets TIMFISR to value 0"]
impl crate::Resettable for TimfisrSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `OR` reader"]
pub type R = crate::R<OrSpec>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<OrSpec>;
#[doc = "Field `IN1` reader - IN1"]
pub type In1R = crate::BitReader;
#[doc = "Field `IN1` writer - IN1"]
pub type In1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN2` reader - IN2"]
pub type In2R = crate::BitReader;
#[doc = "Field `IN2` writer - IN2"]
pub type In2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN1_2_1` reader - IN1_2_1"]
pub type In1_2_1R = crate::FieldReader;
#[doc = "Field `IN1_2_1` writer - IN1_2_1"]
pub type In1_2_1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IN2_2_1` reader - IN2_2_1"]
pub type In2_2_1R = crate::FieldReader;
#[doc = "Field `IN2_2_1` writer - IN2_2_1"]
pub type In2_2_1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - IN1"]
    #[inline(always)]
    pub fn in1(&self) -> In1R {
        In1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IN2"]
    #[inline(always)]
    pub fn in2(&self) -> In2R {
        In2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - IN1_2_1"]
    #[inline(always)]
    pub fn in1_2_1(&self) -> In1_2_1R {
        In1_2_1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - IN2_2_1"]
    #[inline(always)]
    pub fn in2_2_1(&self) -> In2_2_1R {
        In2_2_1R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("in1", &self.in1())
            .field("in2", &self.in2())
            .field("in1_2_1", &self.in1_2_1())
            .field("in2_2_1", &self.in2_2_1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - IN1"]
    #[inline(always)]
    #[must_use]
    pub fn in1(&mut self) -> In1W<OrSpec> {
        In1W::new(self, 0)
    }
    #[doc = "Bit 1 - IN2"]
    #[inline(always)]
    #[must_use]
    pub fn in2(&mut self) -> In2W<OrSpec> {
        In2W::new(self, 1)
    }
    #[doc = "Bits 2:3 - IN1_2_1"]
    #[inline(always)]
    #[must_use]
    pub fn in1_2_1(&mut self) -> In1_2_1W<OrSpec> {
        In1_2_1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - IN2_2_1"]
    #[inline(always)]
    #[must_use]
    pub fn in2_2_1(&mut self) -> In2_2_1W<OrSpec> {
        In2_2_1W::new(self, 4)
    }
}
#[doc = "option register\n\nYou can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OrSpec;
impl crate::RegisterSpec for OrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or::R`](R) reader structure"]
impl crate::Readable for OrSpec {}
#[doc = "`write(|w| ..)` method takes [`or::W`](W) writer structure"]
impl crate::Writable for OrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for OrSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `TSCC` reader"]
pub type R = crate::R<TsccSpec>;
#[doc = "Register `TSCC` writer"]
pub type W = crate::W<TsccSpec>;
#[doc = "Field `TSS` reader - TSS"]
pub type TssR = crate::FieldReader;
#[doc = "Field `TSS` writer - TSS"]
pub type TssW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TCP` reader - TCP"]
pub type TcpR = crate::FieldReader;
#[doc = "Field `TCP` writer - TCP"]
pub type TcpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - TSS"]
    #[inline(always)]
    pub fn tss(&self) -> TssR {
        TssR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - TCP"]
    #[inline(always)]
    pub fn tcp(&self) -> TcpR {
        TcpR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCC")
            .field("tss", &self.tss())
            .field("tcp", &self.tcp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - TSS"]
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TssW<TsccSpec> {
        TssW::new(self, 0)
    }
    #[doc = "Bits 16:19 - TCP"]
    #[inline(always)]
    #[must_use]
    pub fn tcp(&mut self) -> TcpW<TsccSpec> {
        TcpW::new(self, 16)
    }
}
#[doc = "FDCAN Timestamp Counter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsccSpec;
impl crate::RegisterSpec for TsccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscc::R`](R) reader structure"]
impl crate::Readable for TsccSpec {}
#[doc = "`write(|w| ..)` method takes [`tscc::W`](W) writer structure"]
impl crate::Writable for TsccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSCC to value 0"]
impl crate::Resettable for TsccSpec {
    const RESET_VALUE: u32 = 0;
}

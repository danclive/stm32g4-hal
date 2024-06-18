#[doc = "Register `PCROP1ER` reader"]
pub type R = crate::R<Pcrop1erSpec>;
#[doc = "Register `PCROP1ER` writer"]
pub type W = crate::W<Pcrop1erSpec>;
#[doc = "Field `PCROP1_END` reader - Bank 1 PCROP area end offset"]
pub type Pcrop1EndR = crate::FieldReader<u16>;
#[doc = "Field `PCROP1_END` writer - Bank 1 PCROP area end offset"]
pub type Pcrop1EndW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `PCROP_RDP` reader - PCROP area preserved when RDP level decreased"]
pub type PcropRdpR = crate::BitReader;
#[doc = "Field `PCROP_RDP` writer - PCROP area preserved when RDP level decreased"]
pub type PcropRdpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - Bank 1 PCROP area end offset"]
    #[inline(always)]
    pub fn pcrop1_end(&self) -> Pcrop1EndR {
        Pcrop1EndR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - PCROP area preserved when RDP level decreased"]
    #[inline(always)]
    pub fn pcrop_rdp(&self) -> PcropRdpR {
        PcropRdpR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCROP1ER")
            .field("pcrop1_end", &self.pcrop1_end())
            .field("pcrop_rdp", &self.pcrop_rdp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:14 - Bank 1 PCROP area end offset"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop1_end(&mut self) -> Pcrop1EndW<Pcrop1erSpec> {
        Pcrop1EndW::new(self, 0)
    }
    #[doc = "Bit 31 - PCROP area preserved when RDP level decreased"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop_rdp(&mut self) -> PcropRdpW<Pcrop1erSpec> {
        PcropRdpW::new(self, 31)
    }
}
#[doc = "Flash Bank 1 PCROP End address register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrop1er::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1er::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcrop1erSpec;
impl crate::RegisterSpec for Pcrop1erSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrop1er::R`](R) reader structure"]
impl crate::Readable for Pcrop1erSpec {}
#[doc = "`write(|w| ..)` method takes [`pcrop1er::W`](W) writer structure"]
impl crate::Writable for Pcrop1erSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCROP1ER to value 0x0fff_0000"]
impl crate::Resettable for Pcrop1erSpec {
    const RESET_VALUE: u32 = 0x0fff_0000;
}

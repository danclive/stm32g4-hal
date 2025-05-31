#[doc = "Register `BTABLE` reader"]
pub type R = crate::R<BtableSpec>;
#[doc = "Register `BTABLE` writer"]
pub type W = crate::W<BtableSpec>;
#[doc = "Field `BTABLE` reader - BTABLE"]
pub type BtableR = crate::FieldReader<u16>;
#[doc = "Field `BTABLE` writer - BTABLE"]
pub type BtableW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 3:15 - BTABLE"]
    #[inline(always)]
    pub fn btable(&self) -> BtableR {
        BtableR::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BTABLE")
            .field("btable", &self.btable())
            .finish()
    }
}
impl W {
    #[doc = "Bits 3:15 - BTABLE"]
    #[inline(always)]
    pub fn btable(&mut self) -> BtableW<BtableSpec> {
        BtableW::new(self, 3)
    }
}
#[doc = "Buffer table address\n\nYou can [`read`](crate::Reg::read) this register and get [`btable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BtableSpec;
impl crate::RegisterSpec for BtableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btable::R`](R) reader structure"]
impl crate::Readable for BtableSpec {}
#[doc = "`write(|w| ..)` method takes [`btable::W`](W) writer structure"]
impl crate::Writable for BtableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BTABLE to value 0"]
impl crate::Resettable for BtableSpec {}

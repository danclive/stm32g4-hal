#[doc = "Register `APB1H_FZ` reader"]
pub type R = crate::R<Apb1hFzSpec>;
#[doc = "Register `APB1H_FZ` writer"]
pub type W = crate::W<Apb1hFzSpec>;
#[doc = "Field `DBG_I2C4_STOP` reader - DBG_I2C4_STOP"]
pub type DbgI2c4StopR = crate::BitReader;
#[doc = "Field `DBG_I2C4_STOP` writer - DBG_I2C4_STOP"]
pub type DbgI2c4StopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - DBG_I2C4_STOP"]
    #[inline(always)]
    pub fn dbg_i2c4_stop(&self) -> DbgI2c4StopR {
        DbgI2c4StopR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1H_FZ")
            .field("dbg_i2c4_stop", &self.dbg_i2c4_stop())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - DBG_I2C4_STOP"]
    #[inline(always)]
    pub fn dbg_i2c4_stop(&mut self) -> DbgI2c4StopW<Apb1hFzSpec> {
        DbgI2c4StopW::new(self, 1)
    }
}
#[doc = "APB Low Freeze Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1h_fz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1h_fz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1hFzSpec;
impl crate::RegisterSpec for Apb1hFzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1h_fz::R`](R) reader structure"]
impl crate::Readable for Apb1hFzSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1h_fz::W`](W) writer structure"]
impl crate::Writable for Apb1hFzSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1H_FZ to value 0"]
impl crate::Resettable for Apb1hFzSpec {
    const RESET_VALUE: u32 = 0;
}

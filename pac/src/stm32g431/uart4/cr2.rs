#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `SLVEN` reader - SLVEN"]
pub type SlvenR = crate::BitReader;
#[doc = "Field `SLVEN` writer - SLVEN"]
pub type SlvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_NSS` reader - DIS_NSS"]
pub type DisNssR = crate::BitReader;
#[doc = "Field `DIS_NSS` writer - DIS_NSS"]
pub type DisNssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDM7` reader - 7-bit Address Detection/4-bit Address Detection"]
pub type Addm7R = crate::BitReader;
#[doc = "Field `ADDM7` writer - 7-bit Address Detection/4-bit Address Detection"]
pub type Addm7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDL` reader - LIN break detection length"]
pub type LbdlR = crate::BitReader;
#[doc = "Field `LBDL` writer - LIN break detection length"]
pub type LbdlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDIE` reader - LIN break detection interrupt enable"]
pub type LbdieR = crate::BitReader;
#[doc = "Field `LBDIE` writer - LIN break detection interrupt enable"]
pub type LbdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBCL` reader - Last bit clock pulse"]
pub type LbclR = crate::BitReader;
#[doc = "Field `LBCL` writer - Last bit clock pulse"]
pub type LbclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHA` reader - Clock phase"]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - Clock phase"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Clock polarity"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKEN` reader - Clock enable"]
pub type ClkenR = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock enable"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - STOP bits"]
pub type StopR = crate::FieldReader;
#[doc = "Field `STOP` writer - STOP bits"]
pub type StopW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LINEN` reader - LIN mode enable"]
pub type LinenR = crate::BitReader;
#[doc = "Field `LINEN` writer - LIN mode enable"]
pub type LinenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP` reader - Swap TX/RX pins"]
pub type SwapR = crate::BitReader;
#[doc = "Field `SWAP` writer - Swap TX/RX pins"]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINV` reader - RX pin active level inversion"]
pub type RxinvR = crate::BitReader;
#[doc = "Field `RXINV` writer - RX pin active level inversion"]
pub type RxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXINV` reader - TX pin active level inversion"]
pub type TxinvR = crate::BitReader;
#[doc = "Field `TXINV` writer - TX pin active level inversion"]
pub type TxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAINV` reader - Binary data inversion"]
pub type TainvR = crate::BitReader;
#[doc = "Field `TAINV` writer - Binary data inversion"]
pub type TainvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSBFIRST` reader - Most significant bit first"]
pub type MsbfirstR = crate::BitReader;
#[doc = "Field `MSBFIRST` writer - Most significant bit first"]
pub type MsbfirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABREN` reader - Auto baud rate enable"]
pub type AbrenR = crate::BitReader;
#[doc = "Field `ABREN` writer - Auto baud rate enable"]
pub type AbrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABRMOD0` reader - ABRMOD0"]
pub type Abrmod0R = crate::BitReader;
#[doc = "Field `ABRMOD0` writer - ABRMOD0"]
pub type Abrmod0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABRMOD1` reader - Auto baud rate mode"]
pub type Abrmod1R = crate::BitReader;
#[doc = "Field `ABRMOD1` writer - Auto baud rate mode"]
pub type Abrmod1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTOEN` reader - Receiver timeout enable"]
pub type RtoenR = crate::BitReader;
#[doc = "Field `RTOEN` writer - Receiver timeout enable"]
pub type RtoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD0_3` reader - Address of the USART node"]
pub type Add0_3R = crate::FieldReader;
#[doc = "Field `ADD0_3` writer - Address of the USART node"]
pub type Add0_3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADD4_7` reader - Address of the USART node"]
pub type Add4_7R = crate::FieldReader;
#[doc = "Field `ADD4_7` writer - Address of the USART node"]
pub type Add4_7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - SLVEN"]
    #[inline(always)]
    pub fn slven(&self) -> SlvenR {
        SlvenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - DIS_NSS"]
    #[inline(always)]
    pub fn dis_nss(&self) -> DisNssR {
        DisNssR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    pub fn addm7(&self) -> Addm7R {
        Addm7R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LIN break detection length"]
    #[inline(always)]
    pub fn lbdl(&self) -> LbdlR {
        LbdlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&self) -> LbdieR {
        LbdieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    pub fn lbcl(&self) -> LbclR {
        LbclR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn linen(&self) -> LinenR {
        LinenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    pub fn rxinv(&self) -> RxinvR {
        RxinvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    pub fn txinv(&self) -> TxinvR {
        TxinvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    pub fn tainv(&self) -> TainvR {
        TainvR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    pub fn msbfirst(&self) -> MsbfirstR {
        MsbfirstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Auto baud rate enable"]
    #[inline(always)]
    pub fn abren(&self) -> AbrenR {
        AbrenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ABRMOD0"]
    #[inline(always)]
    pub fn abrmod0(&self) -> Abrmod0R {
        Abrmod0R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Auto baud rate mode"]
    #[inline(always)]
    pub fn abrmod1(&self) -> Abrmod1R {
        Abrmod1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Receiver timeout enable"]
    #[inline(always)]
    pub fn rtoen(&self) -> RtoenR {
        RtoenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Address of the USART node"]
    #[inline(always)]
    pub fn add0_3(&self) -> Add0_3R {
        Add0_3R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Address of the USART node"]
    #[inline(always)]
    pub fn add4_7(&self) -> Add4_7R {
        Add4_7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("add4_7", &self.add4_7())
            .field("add0_3", &self.add0_3())
            .field("rtoen", &self.rtoen())
            .field("abrmod1", &self.abrmod1())
            .field("abrmod0", &self.abrmod0())
            .field("abren", &self.abren())
            .field("msbfirst", &self.msbfirst())
            .field("tainv", &self.tainv())
            .field("txinv", &self.txinv())
            .field("rxinv", &self.rxinv())
            .field("swap", &self.swap())
            .field("linen", &self.linen())
            .field("stop", &self.stop())
            .field("clken", &self.clken())
            .field("cpol", &self.cpol())
            .field("cpha", &self.cpha())
            .field("lbcl", &self.lbcl())
            .field("lbdie", &self.lbdie())
            .field("lbdl", &self.lbdl())
            .field("addm7", &self.addm7())
            .field("dis_nss", &self.dis_nss())
            .field("slven", &self.slven())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SLVEN"]
    #[inline(always)]
    #[must_use]
    pub fn slven(&mut self) -> SlvenW<Cr2Spec> {
        SlvenW::new(self, 0)
    }
    #[doc = "Bit 3 - DIS_NSS"]
    #[inline(always)]
    #[must_use]
    pub fn dis_nss(&mut self) -> DisNssW<Cr2Spec> {
        DisNssW::new(self, 3)
    }
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    #[must_use]
    pub fn addm7(&mut self) -> Addm7W<Cr2Spec> {
        Addm7W::new(self, 4)
    }
    #[doc = "Bit 5 - LIN break detection length"]
    #[inline(always)]
    #[must_use]
    pub fn lbdl(&mut self) -> LbdlW<Cr2Spec> {
        LbdlW::new(self, 5)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lbdie(&mut self) -> LbdieW<Cr2Spec> {
        LbdieW::new(self, 6)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    #[must_use]
    pub fn lbcl(&mut self) -> LbclW<Cr2Spec> {
        LbclW::new(self, 8)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CphaW<Cr2Spec> {
        CphaW::new(self, 9)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CpolW<Cr2Spec> {
        CpolW::new(self, 10)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> ClkenW<Cr2Spec> {
        ClkenW::new(self, 11)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<Cr2Spec> {
        StopW::new(self, 12)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn linen(&mut self) -> LinenW<Cr2Spec> {
        LinenW::new(self, 14)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SwapW<Cr2Spec> {
        SwapW::new(self, 15)
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RxinvW<Cr2Spec> {
        RxinvW::new(self, 16)
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TxinvW<Cr2Spec> {
        TxinvW::new(self, 17)
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    #[must_use]
    pub fn tainv(&mut self) -> TainvW<Cr2Spec> {
        TainvW::new(self, 18)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    #[must_use]
    pub fn msbfirst(&mut self) -> MsbfirstW<Cr2Spec> {
        MsbfirstW::new(self, 19)
    }
    #[doc = "Bit 20 - Auto baud rate enable"]
    #[inline(always)]
    #[must_use]
    pub fn abren(&mut self) -> AbrenW<Cr2Spec> {
        AbrenW::new(self, 20)
    }
    #[doc = "Bit 21 - ABRMOD0"]
    #[inline(always)]
    #[must_use]
    pub fn abrmod0(&mut self) -> Abrmod0W<Cr2Spec> {
        Abrmod0W::new(self, 21)
    }
    #[doc = "Bit 22 - Auto baud rate mode"]
    #[inline(always)]
    #[must_use]
    pub fn abrmod1(&mut self) -> Abrmod1W<Cr2Spec> {
        Abrmod1W::new(self, 22)
    }
    #[doc = "Bit 23 - Receiver timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtoen(&mut self) -> RtoenW<Cr2Spec> {
        RtoenW::new(self, 23)
    }
    #[doc = "Bits 24:27 - Address of the USART node"]
    #[inline(always)]
    #[must_use]
    pub fn add0_3(&mut self) -> Add0_3W<Cr2Spec> {
        Add0_3W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Address of the USART node"]
    #[inline(always)]
    #[must_use]
    pub fn add4_7(&mut self) -> Add4_7W<Cr2Spec> {
        Add4_7W::new(self, 28)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {
    const RESET_VALUE: u32 = 0;
}

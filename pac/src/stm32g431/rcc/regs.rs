#[doc = "AHB1 peripheral clock enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccAhb1enr(pub u32);
impl RccAhb1enr {
    #[doc = "DMA1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn dma1en(&self) -> super::vals::Dma1en {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dma1en::from_bits(val as u8)
    }
    #[doc = "DMA1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_dma1en(&mut self, val: super::vals::Dma1en) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn dma2en(&self) -> super::vals::Dma2en {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dma2en::from_bits(val as u8)
    }
    #[doc = "DMA2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_dma2en(&mut self, val: super::vals::Dma2en) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "DMAMUX1 clock enable Set and reset by software."]
    #[inline(always)]
    pub const fn dmamux1en(&self) -> super::vals::Dmamux1en {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dmamux1en::from_bits(val as u8)
    }
    #[doc = "DMAMUX1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn set_dmamux1en(&mut self, val: super::vals::Dmamux1en) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "CORDIC clock enable Set and reset by software."]
    #[inline(always)]
    pub const fn cordicen(&self) -> super::vals::Cordicen {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Cordicen::from_bits(val as u8)
    }
    #[doc = "CORDIC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn set_cordicen(&mut self, val: super::vals::Cordicen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "FMAC enable Set and reset by software."]
    #[inline(always)]
    pub const fn fmacen(&self) -> super::vals::Fmacen {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Fmacen::from_bits(val as u8)
    }
    #[doc = "FMAC enable Set and reset by software."]
    #[inline(always)]
    pub fn set_fmacen(&mut self, val: super::vals::Fmacen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Flash memory interface clock enable Set and cleared by software. This bit can be disabled only when the Flash is in power down mode."]
    #[inline(always)]
    pub const fn flashen(&self) -> super::vals::Flashen {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Flashen::from_bits(val as u8)
    }
    #[doc = "Flash memory interface clock enable Set and cleared by software. This bit can be disabled only when the Flash is in power down mode."]
    #[inline(always)]
    pub fn set_flashen(&mut self, val: super::vals::Flashen) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "CRC clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn crcen(&self) -> super::vals::Crcen {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Crcen::from_bits(val as u8)
    }
    #[doc = "CRC clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_crcen(&mut self, val: super::vals::Crcen) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
}
impl Default for RccAhb1enr {
    #[inline(always)]
    fn default() -> RccAhb1enr {
        RccAhb1enr(0)
    }
}
#[doc = "AHB1 peripheral reset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccAhb1rstr(pub u32);
impl RccAhb1rstr {
    #[doc = "DMA1 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn dma1rst(&self) -> super::vals::Dma1rst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dma1rst::from_bits(val as u8)
    }
    #[doc = "DMA1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_dma1rst(&mut self, val: super::vals::Dma1rst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA2 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn dma2rst(&self) -> super::vals::Dma2rst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dma2rst::from_bits(val as u8)
    }
    #[doc = "DMA2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_dma2rst(&mut self, val: super::vals::Dma2rst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Set and cleared by software."]
    #[inline(always)]
    pub const fn dmamux1rst(&self) -> super::vals::Dmamux1rst {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dmamux1rst::from_bits(val as u8)
    }
    #[doc = "Set and cleared by software."]
    #[inline(always)]
    pub fn set_dmamux1rst(&mut self, val: super::vals::Dmamux1rst) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Set and cleared by software"]
    #[inline(always)]
    pub const fn cordicrst(&self) -> super::vals::Cordicrst {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Cordicrst::from_bits(val as u8)
    }
    #[doc = "Set and cleared by software"]
    #[inline(always)]
    pub fn set_cordicrst(&mut self, val: super::vals::Cordicrst) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Set and cleared by software"]
    #[inline(always)]
    pub const fn fmacrst(&self) -> super::vals::Fmacrst {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Fmacrst::from_bits(val as u8)
    }
    #[doc = "Set and cleared by software"]
    #[inline(always)]
    pub fn set_fmacrst(&mut self, val: super::vals::Fmacrst) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Flash memory interface reset Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode."]
    #[inline(always)]
    pub const fn flashrst(&self) -> super::vals::Flashrst {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Flashrst::from_bits(val as u8)
    }
    #[doc = "Flash memory interface reset Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode."]
    #[inline(always)]
    pub fn set_flashrst(&mut self, val: super::vals::Flashrst) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "CRC reset Set and cleared by software."]
    #[inline(always)]
    pub const fn crcrst(&self) -> super::vals::Crcrst {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Crcrst::from_bits(val as u8)
    }
    #[doc = "CRC reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_crcrst(&mut self, val: super::vals::Crcrst) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
}
impl Default for RccAhb1rstr {
    #[inline(always)]
    fn default() -> RccAhb1rstr {
        RccAhb1rstr(0)
    }
}
#[doc = "AHB1 peripheral clocks enable in Sleep and Stop modes register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccAhb1smenr(pub u32);
impl RccAhb1smenr {
    #[doc = "DMA1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn dma1smen(&self) -> super::vals::Dma1smen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dma1smen::from_bits(val as u8)
    }
    #[doc = "DMA1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_dma1smen(&mut self, val: super::vals::Dma1smen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA2 clocks enable during Sleep and Stop modes Set and cleared by software during Sleep mode."]
    #[inline(always)]
    pub const fn dma2smen(&self) -> super::vals::Dma2smen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dma2smen::from_bits(val as u8)
    }
    #[doc = "DMA2 clocks enable during Sleep and Stop modes Set and cleared by software during Sleep mode."]
    #[inline(always)]
    pub fn set_dma2smen(&mut self, val: super::vals::Dma2smen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "DMAMUX1 clock enable during Sleep and Stop modes. Set and cleared by software."]
    #[inline(always)]
    pub const fn dmamux1smen(&self) -> super::vals::Dmamux1smen {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dmamux1smen::from_bits(val as u8)
    }
    #[doc = "DMAMUX1 clock enable during Sleep and Stop modes. Set and cleared by software."]
    #[inline(always)]
    pub fn set_dmamux1smen(&mut self, val: super::vals::Dmamux1smen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "CORDICSM clock enable. Set and cleared by software."]
    #[inline(always)]
    pub const fn cordicsmen(&self) -> super::vals::Cordicsmen {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Cordicsmen::from_bits(val as u8)
    }
    #[doc = "CORDICSM clock enable. Set and cleared by software."]
    #[inline(always)]
    pub fn set_cordicsmen(&mut self, val: super::vals::Cordicsmen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "FMACSM clock enable. Set and cleared by software."]
    #[inline(always)]
    pub const fn fmacsmen(&self) -> super::vals::Fmacsmen {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Fmacsmen::from_bits(val as u8)
    }
    #[doc = "FMACSM clock enable. Set and cleared by software."]
    #[inline(always)]
    pub fn set_fmacsmen(&mut self, val: super::vals::Fmacsmen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Flash memory interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn flashsmen(&self) -> super::vals::Flashsmen {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Flashsmen::from_bits(val as u8)
    }
    #[doc = "Flash memory interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_flashsmen(&mut self, val: super::vals::Flashsmen) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "SRAM1 interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn sram1smen(&self) -> super::vals::Sram1smen {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Sram1smen::from_bits(val as u8)
    }
    #[doc = "SRAM1 interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_sram1smen(&mut self, val: super::vals::Sram1smen) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "CRC clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn crcsmen(&self) -> super::vals::Crcsmen {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Crcsmen::from_bits(val as u8)
    }
    #[doc = "CRC clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_crcsmen(&mut self, val: super::vals::Crcsmen) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
}
impl Default for RccAhb1smenr {
    #[inline(always)]
    fn default() -> RccAhb1smenr {
        RccAhb1smenr(0)
    }
}
#[doc = "AHB2 peripheral clock enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccAhb2enr(pub u32);
impl RccAhb2enr {
    #[doc = "IO port A clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn gpioaen(&self) -> super::vals::Gpioaen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gpioaen::from_bits(val as u8)
    }
    #[doc = "IO port A clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_gpioaen(&mut self, val: super::vals::Gpioaen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IO port B clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn gpioben(&self) -> super::vals::Gpioben {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Gpioben::from_bits(val as u8)
    }
    #[doc = "IO port B clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_gpioben(&mut self, val: super::vals::Gpioben) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "IO port C clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn gpiocen(&self) -> super::vals::Gpiocen {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Gpiocen::from_bits(val as u8)
    }
    #[doc = "IO port C clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_gpiocen(&mut self, val: super::vals::Gpiocen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "IO port D clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn gpioden(&self) -> super::vals::Gpioden {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Gpioden::from_bits(val as u8)
    }
    #[doc = "IO port D clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_gpioden(&mut self, val: super::vals::Gpioden) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "IO port E clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn gpioeen(&self) -> super::vals::Gpioeen {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Gpioeen::from_bits(val as u8)
    }
    #[doc = "IO port E clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_gpioeen(&mut self, val: super::vals::Gpioeen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "IO port F clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn gpiofen(&self) -> super::vals::Gpiofen {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Gpiofen::from_bits(val as u8)
    }
    #[doc = "IO port F clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_gpiofen(&mut self, val: super::vals::Gpiofen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "IO port G clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn gpiogen(&self) -> super::vals::Gpiogen {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Gpiogen::from_bits(val as u8)
    }
    #[doc = "IO port G clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_gpiogen(&mut self, val: super::vals::Gpiogen) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "ADC12 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn adc12en(&self) -> super::vals::Adc12en {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Adc12en::from_bits(val as u8)
    }
    #[doc = "ADC12 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_adc12en(&mut self, val: super::vals::Adc12en) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "ADC345 clock enable Set and cleared by software"]
    #[inline(always)]
    pub const fn adc345en(&self) -> super::vals::Adc345en {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Adc345en::from_bits(val as u8)
    }
    #[doc = "ADC345 clock enable Set and cleared by software"]
    #[inline(always)]
    pub fn set_adc345en(&mut self, val: super::vals::Adc345en) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "DAC1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn dac1en(&self) -> super::vals::Dac1en {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Dac1en::from_bits(val as u8)
    }
    #[doc = "DAC1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_dac1en(&mut self, val: super::vals::Dac1en) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DAC2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn dac2en(&self) -> super::vals::Dac2en {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Dac2en::from_bits(val as u8)
    }
    #[doc = "DAC2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_dac2en(&mut self, val: super::vals::Dac2en) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "DAC3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn dac3en(&self) -> super::vals::Dac3en {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Dac3en::from_bits(val as u8)
    }
    #[doc = "DAC3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_dac3en(&mut self, val: super::vals::Dac3en) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "DAC4 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn dac4en(&self) -> super::vals::Dac4en {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Dac4en::from_bits(val as u8)
    }
    #[doc = "DAC4 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_dac4en(&mut self, val: super::vals::Dac4en) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "AES clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn aesen(&self) -> super::vals::Aesen {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Aesen::from_bits(val as u8)
    }
    #[doc = "AES clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_aesen(&mut self, val: super::vals::Aesen) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "RNG enable Set and cleared by software."]
    #[inline(always)]
    pub const fn rngen(&self) -> super::vals::RccAhb2enrRngen {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::RccAhb2enrRngen::from_bits(val as u8)
    }
    #[doc = "RNG enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_rngen(&mut self, val: super::vals::RccAhb2enrRngen) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for RccAhb2enr {
    #[inline(always)]
    fn default() -> RccAhb2enr {
        RccAhb2enr(0)
    }
}
#[doc = "AHB2 peripheral reset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccAhb2rstr(pub u32);
impl RccAhb2rstr {
    #[doc = "IO port A reset Set and cleared by software."]
    #[inline(always)]
    pub const fn gpioarst(&self) -> super::vals::Gpioarst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gpioarst::from_bits(val as u8)
    }
    #[doc = "IO port A reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_gpioarst(&mut self, val: super::vals::Gpioarst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IO port B reset Set and cleared by software."]
    #[inline(always)]
    pub const fn gpiobrst(&self) -> super::vals::Gpiobrst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Gpiobrst::from_bits(val as u8)
    }
    #[doc = "IO port B reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_gpiobrst(&mut self, val: super::vals::Gpiobrst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "IO port C reset Set and cleared by software."]
    #[inline(always)]
    pub const fn gpiocrst(&self) -> super::vals::Gpiocrst {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Gpiocrst::from_bits(val as u8)
    }
    #[doc = "IO port C reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_gpiocrst(&mut self, val: super::vals::Gpiocrst) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "IO port D reset Set and cleared by software."]
    #[inline(always)]
    pub const fn gpiodrst(&self) -> super::vals::Gpiodrst {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Gpiodrst::from_bits(val as u8)
    }
    #[doc = "IO port D reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_gpiodrst(&mut self, val: super::vals::Gpiodrst) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "IO port E reset Set and cleared by software."]
    #[inline(always)]
    pub const fn gpioerst(&self) -> super::vals::Gpioerst {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Gpioerst::from_bits(val as u8)
    }
    #[doc = "IO port E reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_gpioerst(&mut self, val: super::vals::Gpioerst) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "IO port F reset Set and cleared by software."]
    #[inline(always)]
    pub const fn gpiofrst(&self) -> super::vals::Gpiofrst {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Gpiofrst::from_bits(val as u8)
    }
    #[doc = "IO port F reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_gpiofrst(&mut self, val: super::vals::Gpiofrst) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "IO port G reset Set and cleared by software."]
    #[inline(always)]
    pub const fn gpiogrst(&self) -> super::vals::Gpiogrst {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Gpiogrst::from_bits(val as u8)
    }
    #[doc = "IO port G reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_gpiogrst(&mut self, val: super::vals::Gpiogrst) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "ADC12 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn adc12rst(&self) -> super::vals::Adc12rst {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Adc12rst::from_bits(val as u8)
    }
    #[doc = "ADC12 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_adc12rst(&mut self, val: super::vals::Adc12rst) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "ADC345 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn adc345rst(&self) -> super::vals::Adc345rst {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Adc345rst::from_bits(val as u8)
    }
    #[doc = "ADC345 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_adc345rst(&mut self, val: super::vals::Adc345rst) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "DAC1 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn dac1rst(&self) -> super::vals::Dac1rst {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Dac1rst::from_bits(val as u8)
    }
    #[doc = "DAC1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_dac1rst(&mut self, val: super::vals::Dac1rst) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DAC2 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn dac2rst(&self) -> super::vals::Dac2rst {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Dac2rst::from_bits(val as u8)
    }
    #[doc = "DAC2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_dac2rst(&mut self, val: super::vals::Dac2rst) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "DAC3 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn dac3rst(&self) -> super::vals::Dac3rst {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Dac3rst::from_bits(val as u8)
    }
    #[doc = "DAC3 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_dac3rst(&mut self, val: super::vals::Dac3rst) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "DAC4 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn dac4rst(&self) -> super::vals::Dac4rst {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Dac4rst::from_bits(val as u8)
    }
    #[doc = "DAC4 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_dac4rst(&mut self, val: super::vals::Dac4rst) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "AESRST reset Set and cleared by software."]
    #[inline(always)]
    pub const fn aesrst(&self) -> super::vals::Aesrst {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Aesrst::from_bits(val as u8)
    }
    #[doc = "AESRST reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_aesrst(&mut self, val: super::vals::Aesrst) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "RNG reset Set and cleared by software."]
    #[inline(always)]
    pub const fn rngrst(&self) -> super::vals::Rngrst {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Rngrst::from_bits(val as u8)
    }
    #[doc = "RNG reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_rngrst(&mut self, val: super::vals::Rngrst) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for RccAhb2rstr {
    #[inline(always)]
    fn default() -> RccAhb2rstr {
        RccAhb2rstr(0)
    }
}
#[doc = "AHB2 peripheral clocks enable in Sleep and Stop modes register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccAhb2smenr(pub u32);
impl RccAhb2smenr {
    #[doc = "IO port A clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn gpioasmen(&self) -> super::vals::Gpioasmen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gpioasmen::from_bits(val as u8)
    }
    #[doc = "IO port A clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_gpioasmen(&mut self, val: super::vals::Gpioasmen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IO port B clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn gpiobsmen(&self) -> super::vals::Gpiobsmen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Gpiobsmen::from_bits(val as u8)
    }
    #[doc = "IO port B clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_gpiobsmen(&mut self, val: super::vals::Gpiobsmen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "IO port C clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn gpiocsmen(&self) -> super::vals::Gpiocsmen {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Gpiocsmen::from_bits(val as u8)
    }
    #[doc = "IO port C clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_gpiocsmen(&mut self, val: super::vals::Gpiocsmen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "IO port D clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn gpiodsmen(&self) -> super::vals::Gpiodsmen {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Gpiodsmen::from_bits(val as u8)
    }
    #[doc = "IO port D clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_gpiodsmen(&mut self, val: super::vals::Gpiodsmen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "IO port E clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn gpioesmen(&self) -> super::vals::Gpioesmen {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Gpioesmen::from_bits(val as u8)
    }
    #[doc = "IO port E clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_gpioesmen(&mut self, val: super::vals::Gpioesmen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "IO port F clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn gpiofsmen(&self) -> super::vals::Gpiofsmen {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Gpiofsmen::from_bits(val as u8)
    }
    #[doc = "IO port F clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_gpiofsmen(&mut self, val: super::vals::Gpiofsmen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "IO port G clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn gpiogsmen(&self) -> super::vals::Gpiogsmen {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Gpiogsmen::from_bits(val as u8)
    }
    #[doc = "IO port G clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_gpiogsmen(&mut self, val: super::vals::Gpiogsmen) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "CCM SRAM interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn ccmsramsmen(&self) -> super::vals::Ccmsramsmen {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Ccmsramsmen::from_bits(val as u8)
    }
    #[doc = "CCM SRAM interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_ccmsramsmen(&mut self, val: super::vals::Ccmsramsmen) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "SRAM2 interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn sram2smen(&self) -> super::vals::Sram2smen {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Sram2smen::from_bits(val as u8)
    }
    #[doc = "SRAM2 interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_sram2smen(&mut self, val: super::vals::Sram2smen) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "ADC12 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn adc12smen(&self) -> super::vals::Adc12smen {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Adc12smen::from_bits(val as u8)
    }
    #[doc = "ADC12 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_adc12smen(&mut self, val: super::vals::Adc12smen) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "ADC345 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn adc345smen(&self) -> super::vals::Adc345smen {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Adc345smen::from_bits(val as u8)
    }
    #[doc = "ADC345 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_adc345smen(&mut self, val: super::vals::Adc345smen) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "DAC1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn dac1smen(&self) -> super::vals::Dac1smen {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Dac1smen::from_bits(val as u8)
    }
    #[doc = "DAC1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_dac1smen(&mut self, val: super::vals::Dac1smen) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DAC2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn dac2smen(&self) -> super::vals::Dac2smen {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Dac2smen::from_bits(val as u8)
    }
    #[doc = "DAC2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_dac2smen(&mut self, val: super::vals::Dac2smen) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "DAC3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn dac3smen(&self) -> super::vals::Dac3smen {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Dac3smen::from_bits(val as u8)
    }
    #[doc = "DAC3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_dac3smen(&mut self, val: super::vals::Dac3smen) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "DAC4 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn dac4smen(&self) -> super::vals::Dac4smen {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Dac4smen::from_bits(val as u8)
    }
    #[doc = "DAC4 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_dac4smen(&mut self, val: super::vals::Dac4smen) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "AESM clocks enable Set and cleared by software."]
    #[inline(always)]
    pub const fn aessmen(&self) -> super::vals::Aessmen {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Aessmen::from_bits(val as u8)
    }
    #[doc = "AESM clocks enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_aessmen(&mut self, val: super::vals::Aessmen) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "RNG enable Set and cleared by software."]
    #[inline(always)]
    pub const fn rngen(&self) -> super::vals::RccAhb2smenrRngen {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::RccAhb2smenrRngen::from_bits(val as u8)
    }
    #[doc = "RNG enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_rngen(&mut self, val: super::vals::RccAhb2smenrRngen) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for RccAhb2smenr {
    #[inline(always)]
    fn default() -> RccAhb2smenr {
        RccAhb2smenr(0)
    }
}
#[doc = "AHB3 peripheral clock enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccAhb3enr(pub u32);
impl RccAhb3enr {
    #[doc = "Flexible static memory controller clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn fmcen(&self) -> super::vals::Fmcen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Fmcen::from_bits(val as u8)
    }
    #[doc = "Flexible static memory controller clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_fmcen(&mut self, val: super::vals::Fmcen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "QUADSPI memory interface clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn qspien(&self) -> super::vals::Qspien {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Qspien::from_bits(val as u8)
    }
    #[doc = "QUADSPI memory interface clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_qspien(&mut self, val: super::vals::Qspien) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for RccAhb3enr {
    #[inline(always)]
    fn default() -> RccAhb3enr {
        RccAhb3enr(0)
    }
}
#[doc = "AHB3 peripheral reset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccAhb3rstr(pub u32);
impl RccAhb3rstr {
    #[doc = "Flexible static memory controller reset Set and cleared by software."]
    #[inline(always)]
    pub const fn fmcrst(&self) -> super::vals::Fmcrst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Fmcrst::from_bits(val as u8)
    }
    #[doc = "Flexible static memory controller reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_fmcrst(&mut self, val: super::vals::Fmcrst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "QUADSPI reset Set and cleared by software."]
    #[inline(always)]
    pub const fn qspirst(&self) -> super::vals::Qspirst {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Qspirst::from_bits(val as u8)
    }
    #[doc = "QUADSPI reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_qspirst(&mut self, val: super::vals::Qspirst) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for RccAhb3rstr {
    #[inline(always)]
    fn default() -> RccAhb3rstr {
        RccAhb3rstr(0)
    }
}
#[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccAhb3smenr(pub u32);
impl RccAhb3smenr {
    #[doc = "Flexible static memory controller clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn fmcsmen(&self) -> super::vals::Fmcsmen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Fmcsmen::from_bits(val as u8)
    }
    #[doc = "Flexible static memory controller clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_fmcsmen(&mut self, val: super::vals::Fmcsmen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "QUADSPI memory interface clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn qspismen(&self) -> super::vals::Qspismen {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Qspismen::from_bits(val as u8)
    }
    #[doc = "QUADSPI memory interface clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_qspismen(&mut self, val: super::vals::Qspismen) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for RccAhb3smenr {
    #[inline(always)]
    fn default() -> RccAhb3smenr {
        RccAhb3smenr(0)
    }
}
#[doc = "APB1 peripheral clock enable register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccApb1enr1(pub u32);
impl RccApb1enr1 {
    #[doc = "TIM2 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn tim2en(&self) -> super::vals::Tim2en {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tim2en::from_bits(val as u8)
    }
    #[doc = "TIM2 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim2en(&mut self, val: super::vals::Tim2en) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM3 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn tim3en(&self) -> super::vals::Tim3en {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tim3en::from_bits(val as u8)
    }
    #[doc = "TIM3 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim3en(&mut self, val: super::vals::Tim3en) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "TIM4 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn tim4en(&self) -> super::vals::Tim4en {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Tim4en::from_bits(val as u8)
    }
    #[doc = "TIM4 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim4en(&mut self, val: super::vals::Tim4en) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "TIM5 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn tim5en(&self) -> super::vals::Tim5en {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Tim5en::from_bits(val as u8)
    }
    #[doc = "TIM5 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim5en(&mut self, val: super::vals::Tim5en) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "TIM6 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn tim6en(&self) -> super::vals::Tim6en {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tim6en::from_bits(val as u8)
    }
    #[doc = "TIM6 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim6en(&mut self, val: super::vals::Tim6en) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "TIM7 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn tim7en(&self) -> super::vals::Tim7en {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Tim7en::from_bits(val as u8)
    }
    #[doc = "TIM7 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim7en(&mut self, val: super::vals::Tim7en) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "CRS Recovery System clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn crsen(&self) -> super::vals::Crsen {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Crsen::from_bits(val as u8)
    }
    #[doc = "CRS Recovery System clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_crsen(&mut self, val: super::vals::Crsen) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "RTC APB clock enable Set and cleared by software"]
    #[inline(always)]
    pub const fn rtcapben(&self) -> super::vals::Rtcapben {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Rtcapben::from_bits(val as u8)
    }
    #[doc = "RTC APB clock enable Set and cleared by software"]
    #[inline(always)]
    pub fn set_rtcapben(&mut self, val: super::vals::Rtcapben) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Window watchdog clock enable Set by software to enable the window watchdog clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset."]
    #[inline(always)]
    pub const fn wwdgen(&self) -> super::vals::Wwdgen {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Wwdgen::from_bits(val as u8)
    }
    #[doc = "Window watchdog clock enable Set by software to enable the window watchdog clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset."]
    #[inline(always)]
    pub fn set_wwdgen(&mut self, val: super::vals::Wwdgen) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "SPI2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn spi2en(&self) -> super::vals::Spi2en {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Spi2en::from_bits(val as u8)
    }
    #[doc = "SPI2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_spi2en(&mut self, val: super::vals::Spi2en) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "SPI3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn spi3en(&self) -> super::vals::Spi3en {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Spi3en::from_bits(val as u8)
    }
    #[doc = "SPI3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_spi3en(&mut self, val: super::vals::Spi3en) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "USART2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn usart2en(&self) -> super::vals::Usart2en {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Usart2en::from_bits(val as u8)
    }
    #[doc = "USART2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_usart2en(&mut self, val: super::vals::Usart2en) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "USART3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn usart3en(&self) -> super::vals::Usart3en {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Usart3en::from_bits(val as u8)
    }
    #[doc = "USART3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_usart3en(&mut self, val: super::vals::Usart3en) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "UART4 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn uart4en(&self) -> super::vals::Uart4en {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Uart4en::from_bits(val as u8)
    }
    #[doc = "UART4 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_uart4en(&mut self, val: super::vals::Uart4en) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "UART5 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn uart5en(&self) -> super::vals::Uart5en {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Uart5en::from_bits(val as u8)
    }
    #[doc = "UART5 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_uart5en(&mut self, val: super::vals::Uart5en) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "I2C1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn i2c1en(&self) -> super::vals::I2c1en {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::I2c1en::from_bits(val as u8)
    }
    #[doc = "I2C1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_i2c1en(&mut self, val: super::vals::I2c1en) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "I2C2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn i2c2en(&self) -> super::vals::I2c2en {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::I2c2en::from_bits(val as u8)
    }
    #[doc = "I2C2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_i2c2en(&mut self, val: super::vals::I2c2en) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "USB device clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn usben(&self) -> super::vals::Usben {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Usben::from_bits(val as u8)
    }
    #[doc = "USB device clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_usben(&mut self, val: super::vals::Usben) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "FDCAN clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn fdcanen(&self) -> super::vals::Fdcanen {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Fdcanen::from_bits(val as u8)
    }
    #[doc = "FDCAN clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_fdcanen(&mut self, val: super::vals::Fdcanen) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Power interface clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn pwren(&self) -> super::vals::Pwren {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pwren::from_bits(val as u8)
    }
    #[doc = "Power interface clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_pwren(&mut self, val: super::vals::Pwren) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "I2C3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn i2c3en(&self) -> super::vals::I2c3en {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::I2c3en::from_bits(val as u8)
    }
    #[doc = "I2C3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_i2c3en(&mut self, val: super::vals::I2c3en) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Low power timer 1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn lptim1en(&self) -> super::vals::Lptim1en {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Lptim1en::from_bits(val as u8)
    }
    #[doc = "Low power timer 1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_lptim1en(&mut self, val: super::vals::Lptim1en) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for RccApb1enr1 {
    #[inline(always)]
    fn default() -> RccApb1enr1 {
        RccApb1enr1(0)
    }
}
#[doc = "APB1 peripheral clock enable register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccApb1enr2(pub u32);
impl RccApb1enr2 {
    #[doc = "Low power UART 1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn lpuart1en(&self) -> super::vals::Lpuart1en {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart1en::from_bits(val as u8)
    }
    #[doc = "Low power UART 1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_lpuart1en(&mut self, val: super::vals::Lpuart1en) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "I2C4 clock enable Set and cleared by software"]
    #[inline(always)]
    pub const fn i2c4en(&self) -> super::vals::I2c4en {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::I2c4en::from_bits(val as u8)
    }
    #[doc = "I2C4 clock enable Set and cleared by software"]
    #[inline(always)]
    pub fn set_i2c4en(&mut self, val: super::vals::I2c4en) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "UCPD1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn ucpd1en(&self) -> super::vals::Ucpd1en {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Ucpd1en::from_bits(val as u8)
    }
    #[doc = "UCPD1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_ucpd1en(&mut self, val: super::vals::Ucpd1en) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for RccApb1enr2 {
    #[inline(always)]
    fn default() -> RccApb1enr2 {
        RccApb1enr2(0)
    }
}
#[doc = "APB1 peripheral reset register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccApb1rstr1(pub u32);
impl RccApb1rstr1 {
    #[doc = "TIM2 timer reset Set and cleared by software."]
    #[inline(always)]
    pub const fn tim2rst(&self) -> super::vals::Tim2rst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tim2rst::from_bits(val as u8)
    }
    #[doc = "TIM2 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim2rst(&mut self, val: super::vals::Tim2rst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM3 timer reset Set and cleared by software."]
    #[inline(always)]
    pub const fn tim3rst(&self) -> super::vals::Tim3rst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tim3rst::from_bits(val as u8)
    }
    #[doc = "TIM3 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim3rst(&mut self, val: super::vals::Tim3rst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "TIM3 timer reset Set and cleared by software."]
    #[inline(always)]
    pub const fn tim4rst(&self) -> super::vals::Tim4rst {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Tim4rst::from_bits(val as u8)
    }
    #[doc = "TIM3 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim4rst(&mut self, val: super::vals::Tim4rst) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "TIM5 timer reset Set and cleared by software."]
    #[inline(always)]
    pub const fn tim5rst(&self) -> super::vals::Tim5rst {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Tim5rst::from_bits(val as u8)
    }
    #[doc = "TIM5 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim5rst(&mut self, val: super::vals::Tim5rst) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "TIM6 timer reset Set and cleared by software."]
    #[inline(always)]
    pub const fn tim6rst(&self) -> super::vals::Tim6rst {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tim6rst::from_bits(val as u8)
    }
    #[doc = "TIM6 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim6rst(&mut self, val: super::vals::Tim6rst) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "TIM7 timer reset Set and cleared by software."]
    #[inline(always)]
    pub const fn tim7rst(&self) -> super::vals::Tim7rst {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Tim7rst::from_bits(val as u8)
    }
    #[doc = "TIM7 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim7rst(&mut self, val: super::vals::Tim7rst) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "CRS reset Set and cleared by software."]
    #[inline(always)]
    pub const fn crsrst(&self) -> super::vals::Crsrst {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Crsrst::from_bits(val as u8)
    }
    #[doc = "CRS reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_crsrst(&mut self, val: super::vals::Crsrst) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "SPI2 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn spi2rst(&self) -> super::vals::Spi2rst {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Spi2rst::from_bits(val as u8)
    }
    #[doc = "SPI2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_spi2rst(&mut self, val: super::vals::Spi2rst) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "SPI3 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn spi3rst(&self) -> super::vals::Spi3rst {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Spi3rst::from_bits(val as u8)
    }
    #[doc = "SPI3 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_spi3rst(&mut self, val: super::vals::Spi3rst) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "USART2 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn usart2rst(&self) -> super::vals::Usart2rst {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Usart2rst::from_bits(val as u8)
    }
    #[doc = "USART2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_usart2rst(&mut self, val: super::vals::Usart2rst) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "USART3 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn usart3rst(&self) -> super::vals::Usart3rst {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Usart3rst::from_bits(val as u8)
    }
    #[doc = "USART3 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_usart3rst(&mut self, val: super::vals::Usart3rst) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "UART4 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn uart4rst(&self) -> super::vals::Uart4rst {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Uart4rst::from_bits(val as u8)
    }
    #[doc = "UART4 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_uart4rst(&mut self, val: super::vals::Uart4rst) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "UART5 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn uart5rst(&self) -> super::vals::Uart5rst {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Uart5rst::from_bits(val as u8)
    }
    #[doc = "UART5 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_uart5rst(&mut self, val: super::vals::Uart5rst) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "I2C1 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn i2c1rst(&self) -> super::vals::I2c1rst {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::I2c1rst::from_bits(val as u8)
    }
    #[doc = "I2C1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_i2c1rst(&mut self, val: super::vals::I2c1rst) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "I2C2 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn i2c2rst(&self) -> super::vals::I2c2rst {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::I2c2rst::from_bits(val as u8)
    }
    #[doc = "I2C2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_i2c2rst(&mut self, val: super::vals::I2c2rst) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "USB device reset Set and reset by software."]
    #[inline(always)]
    pub const fn usbrst(&self) -> super::vals::Usbrst {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Usbrst::from_bits(val as u8)
    }
    #[doc = "USB device reset Set and reset by software."]
    #[inline(always)]
    pub fn set_usbrst(&mut self, val: super::vals::Usbrst) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "FDCAN reset Set and reset by software."]
    #[inline(always)]
    pub const fn fdcanrst(&self) -> super::vals::Fdcanrst {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Fdcanrst::from_bits(val as u8)
    }
    #[doc = "FDCAN reset Set and reset by software."]
    #[inline(always)]
    pub fn set_fdcanrst(&mut self, val: super::vals::Fdcanrst) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Power interface reset Set and cleared by software."]
    #[inline(always)]
    pub const fn pwrrst(&self) -> super::vals::Pwrrst {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pwrrst::from_bits(val as u8)
    }
    #[doc = "Power interface reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_pwrrst(&mut self, val: super::vals::Pwrrst) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "I2C3 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn i2c3rst(&self) -> super::vals::I2c3rst {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::I2c3rst::from_bits(val as u8)
    }
    #[doc = "I2C3 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_i2c3rst(&mut self, val: super::vals::I2c3rst) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Low Power Timer 1 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn lptim1rst(&self) -> super::vals::Lptim1rst {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Lptim1rst::from_bits(val as u8)
    }
    #[doc = "Low Power Timer 1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_lptim1rst(&mut self, val: super::vals::Lptim1rst) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for RccApb1rstr1 {
    #[inline(always)]
    fn default() -> RccApb1rstr1 {
        RccApb1rstr1(0)
    }
}
#[doc = "APB1 peripheral reset register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccApb1rstr2(pub u32);
impl RccApb1rstr2 {
    #[doc = "Low-power UART 1 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn lpuart1rst(&self) -> super::vals::Lpuart1rst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart1rst::from_bits(val as u8)
    }
    #[doc = "Low-power UART 1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_lpuart1rst(&mut self, val: super::vals::Lpuart1rst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "I2C4 reset Set and cleared by software"]
    #[inline(always)]
    pub const fn i2c4rst(&self) -> super::vals::I2c4rst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::I2c4rst::from_bits(val as u8)
    }
    #[doc = "I2C4 reset Set and cleared by software"]
    #[inline(always)]
    pub fn set_i2c4rst(&mut self, val: super::vals::I2c4rst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "UCPD1 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn ucpd1rst(&self) -> super::vals::Ucpd1rst {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Ucpd1rst::from_bits(val as u8)
    }
    #[doc = "UCPD1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_ucpd1rst(&mut self, val: super::vals::Ucpd1rst) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for RccApb1rstr2 {
    #[inline(always)]
    fn default() -> RccApb1rstr2 {
        RccApb1rstr2(0)
    }
}
#[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccApb1smenr1(pub u32);
impl RccApb1smenr1 {
    #[doc = "TIM2 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn tim2smen(&self) -> super::vals::Tim2smen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tim2smen::from_bits(val as u8)
    }
    #[doc = "TIM2 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim2smen(&mut self, val: super::vals::Tim2smen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM3 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn tim3smen(&self) -> super::vals::Tim3smen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tim3smen::from_bits(val as u8)
    }
    #[doc = "TIM3 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim3smen(&mut self, val: super::vals::Tim3smen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "TIM4 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn tim4smen(&self) -> super::vals::Tim4smen {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Tim4smen::from_bits(val as u8)
    }
    #[doc = "TIM4 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim4smen(&mut self, val: super::vals::Tim4smen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "TIM5 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn tim5smen(&self) -> super::vals::Tim5smen {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Tim5smen::from_bits(val as u8)
    }
    #[doc = "TIM5 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim5smen(&mut self, val: super::vals::Tim5smen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "TIM6 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn tim6smen(&self) -> super::vals::Tim6smen {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tim6smen::from_bits(val as u8)
    }
    #[doc = "TIM6 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim6smen(&mut self, val: super::vals::Tim6smen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "TIM7 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn tim7smen(&self) -> super::vals::Tim7smen {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Tim7smen::from_bits(val as u8)
    }
    #[doc = "TIM7 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim7smen(&mut self, val: super::vals::Tim7smen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "CRS timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn crssmen(&self) -> super::vals::Crssmen {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Crssmen::from_bits(val as u8)
    }
    #[doc = "CRS timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_crssmen(&mut self, val: super::vals::Crssmen) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "RTC APB clock enable during Sleep and Stop modes Set and cleared by software"]
    #[inline(always)]
    pub const fn rtcapbsmen(&self) -> super::vals::Rtcapbsmen {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Rtcapbsmen::from_bits(val as u8)
    }
    #[doc = "RTC APB clock enable during Sleep and Stop modes Set and cleared by software"]
    #[inline(always)]
    pub fn set_rtcapbsmen(&mut self, val: super::vals::Rtcapbsmen) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Window watchdog clocks enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated."]
    #[inline(always)]
    pub const fn wwdgsmen(&self) -> super::vals::Wwdgsmen {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Wwdgsmen::from_bits(val as u8)
    }
    #[doc = "Window watchdog clocks enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated."]
    #[inline(always)]
    pub fn set_wwdgsmen(&mut self, val: super::vals::Wwdgsmen) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "SPI2 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn spi2smen(&self) -> super::vals::Spi2smen {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Spi2smen::from_bits(val as u8)
    }
    #[doc = "SPI2 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_spi2smen(&mut self, val: super::vals::Spi2smen) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "SPI3 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn spi3smen(&self) -> super::vals::Spi3smen {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Spi3smen::from_bits(val as u8)
    }
    #[doc = "SPI3 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_spi3smen(&mut self, val: super::vals::Spi3smen) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "USART2 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn usart2smen(&self) -> super::vals::Usart2smen {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Usart2smen::from_bits(val as u8)
    }
    #[doc = "USART2 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_usart2smen(&mut self, val: super::vals::Usart2smen) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "USART3 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn usart3smen(&self) -> super::vals::Usart3smen {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Usart3smen::from_bits(val as u8)
    }
    #[doc = "USART3 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_usart3smen(&mut self, val: super::vals::Usart3smen) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "UART4 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn uart4smen(&self) -> super::vals::Uart4smen {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Uart4smen::from_bits(val as u8)
    }
    #[doc = "UART4 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_uart4smen(&mut self, val: super::vals::Uart4smen) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "UART5 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn uart5smen(&self) -> super::vals::Uart5smen {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Uart5smen::from_bits(val as u8)
    }
    #[doc = "UART5 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_uart5smen(&mut self, val: super::vals::Uart5smen) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "I2C1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn i2c1smen(&self) -> super::vals::I2c1smen {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::I2c1smen::from_bits(val as u8)
    }
    #[doc = "I2C1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_i2c1smen(&mut self, val: super::vals::I2c1smen) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "I2C2 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn i2c2smen(&self) -> super::vals::I2c2smen {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::I2c2smen::from_bits(val as u8)
    }
    #[doc = "I2C2 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_i2c2smen(&mut self, val: super::vals::I2c2smen) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "USB device clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn usbsmen(&self) -> super::vals::Usbsmen {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Usbsmen::from_bits(val as u8)
    }
    #[doc = "USB device clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_usbsmen(&mut self, val: super::vals::Usbsmen) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "FDCAN clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn fdcansmen(&self) -> super::vals::Fdcansmen {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Fdcansmen::from_bits(val as u8)
    }
    #[doc = "FDCAN clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_fdcansmen(&mut self, val: super::vals::Fdcansmen) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Power interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn pwrsmen(&self) -> super::vals::Pwrsmen {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pwrsmen::from_bits(val as u8)
    }
    #[doc = "Power interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_pwrsmen(&mut self, val: super::vals::Pwrsmen) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "I2C3 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn i2c3smen(&self) -> super::vals::I2c3smen {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::I2c3smen::from_bits(val as u8)
    }
    #[doc = "I2C3 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_i2c3smen(&mut self, val: super::vals::I2c3smen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Low power timer 1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn lptim1smen(&self) -> super::vals::Lptim1smen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Lptim1smen::from_bits(val as u8)
    }
    #[doc = "Low power timer 1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_lptim1smen(&mut self, val: super::vals::Lptim1smen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for RccApb1smenr1 {
    #[inline(always)]
    fn default() -> RccApb1smenr1 {
        RccApb1smenr1(0)
    }
}
#[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccApb1smenr2(pub u32);
impl RccApb1smenr2 {
    #[doc = "Low power UART 1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn lpuart1smen(&self) -> super::vals::Lpuart1smen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart1smen::from_bits(val as u8)
    }
    #[doc = "Low power UART 1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_lpuart1smen(&mut self, val: super::vals::Lpuart1smen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "I2C4 clocks enable during Sleep and Stop modes Set and cleared by software"]
    #[inline(always)]
    pub const fn i2c4smen(&self) -> super::vals::I2c4smen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::I2c4smen::from_bits(val as u8)
    }
    #[doc = "I2C4 clocks enable during Sleep and Stop modes Set and cleared by software"]
    #[inline(always)]
    pub fn set_i2c4smen(&mut self, val: super::vals::I2c4smen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn ucpd1smen(&self) -> super::vals::Ucpd1smen {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Ucpd1smen::from_bits(val as u8)
    }
    #[doc = "UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_ucpd1smen(&mut self, val: super::vals::Ucpd1smen) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for RccApb1smenr2 {
    #[inline(always)]
    fn default() -> RccApb1smenr2 {
        RccApb1smenr2(0)
    }
}
#[doc = "APB2 peripheral clock enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccApb2enr(pub u32);
impl RccApb2enr {
    #[doc = "SYSCFG + COMP + VREFBUF + OPAMP clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn syscfgen(&self) -> super::vals::Syscfgen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Syscfgen::from_bits(val as u8)
    }
    #[doc = "SYSCFG + COMP + VREFBUF + OPAMP clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_syscfgen(&mut self, val: super::vals::Syscfgen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM1 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn tim1en(&self) -> super::vals::Tim1en {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Tim1en::from_bits(val as u8)
    }
    #[doc = "TIM1 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim1en(&mut self, val: super::vals::Tim1en) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "SPI1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn spi1en(&self) -> super::vals::Spi1en {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Spi1en::from_bits(val as u8)
    }
    #[doc = "SPI1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_spi1en(&mut self, val: super::vals::Spi1en) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "TIM8 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn tim8en(&self) -> super::vals::Tim8en {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Tim8en::from_bits(val as u8)
    }
    #[doc = "TIM8 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim8en(&mut self, val: super::vals::Tim8en) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "USART1clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn usart1en(&self) -> super::vals::Usart1en {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Usart1en::from_bits(val as u8)
    }
    #[doc = "USART1clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_usart1en(&mut self, val: super::vals::Usart1en) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "SPI4 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn spi4en(&self) -> super::vals::Spi4en {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Spi4en::from_bits(val as u8)
    }
    #[doc = "SPI4 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_spi4en(&mut self, val: super::vals::Spi4en) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "TIM15 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn tim15en(&self) -> super::vals::Tim15en {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Tim15en::from_bits(val as u8)
    }
    #[doc = "TIM15 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim15en(&mut self, val: super::vals::Tim15en) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "TIM16 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn tim16en(&self) -> super::vals::Tim16en {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Tim16en::from_bits(val as u8)
    }
    #[doc = "TIM16 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim16en(&mut self, val: super::vals::Tim16en) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "TIM17 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn tim17en(&self) -> super::vals::Tim17en {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Tim17en::from_bits(val as u8)
    }
    #[doc = "TIM17 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim17en(&mut self, val: super::vals::Tim17en) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "TIM20 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn tim20en(&self) -> super::vals::Tim20en {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Tim20en::from_bits(val as u8)
    }
    #[doc = "TIM20 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim20en(&mut self, val: super::vals::Tim20en) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "SAI1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn sai1en(&self) -> super::vals::Sai1en {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Sai1en::from_bits(val as u8)
    }
    #[doc = "SAI1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_sai1en(&mut self, val: super::vals::Sai1en) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "HRTIM1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn hrtim1en(&self) -> super::vals::Hrtim1en {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Hrtim1en::from_bits(val as u8)
    }
    #[doc = "HRTIM1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_hrtim1en(&mut self, val: super::vals::Hrtim1en) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for RccApb2enr {
    #[inline(always)]
    fn default() -> RccApb2enr {
        RccApb2enr(0)
    }
}
#[doc = "APB2 peripheral reset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccApb2rstr(pub u32);
impl RccApb2rstr {
    #[doc = "SYSCFG + COMP + OPAMP + VREFBUF reset"]
    #[inline(always)]
    pub const fn syscfgrst(&self) -> super::vals::Syscfgrst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Syscfgrst::from_bits(val as u8)
    }
    #[doc = "SYSCFG + COMP + OPAMP + VREFBUF reset"]
    #[inline(always)]
    pub fn set_syscfgrst(&mut self, val: super::vals::Syscfgrst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM1 timer reset Set and cleared by software."]
    #[inline(always)]
    pub const fn tim1rst(&self) -> super::vals::Tim1rst {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Tim1rst::from_bits(val as u8)
    }
    #[doc = "TIM1 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim1rst(&mut self, val: super::vals::Tim1rst) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "SPI1 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn spi1rst(&self) -> super::vals::Spi1rst {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Spi1rst::from_bits(val as u8)
    }
    #[doc = "SPI1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_spi1rst(&mut self, val: super::vals::Spi1rst) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "TIM8 timer reset Set and cleared by software."]
    #[inline(always)]
    pub const fn tim8rst(&self) -> super::vals::Tim8rst {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Tim8rst::from_bits(val as u8)
    }
    #[doc = "TIM8 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim8rst(&mut self, val: super::vals::Tim8rst) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "USART1 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn usart1rst(&self) -> super::vals::Usart1rst {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Usart1rst::from_bits(val as u8)
    }
    #[doc = "USART1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_usart1rst(&mut self, val: super::vals::Usart1rst) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "SPI4 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn spi4rst(&self) -> super::vals::Spi4rst {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Spi4rst::from_bits(val as u8)
    }
    #[doc = "SPI4 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_spi4rst(&mut self, val: super::vals::Spi4rst) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "TIM15 timer reset Set and cleared by software."]
    #[inline(always)]
    pub const fn tim15rst(&self) -> super::vals::Tim15rst {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Tim15rst::from_bits(val as u8)
    }
    #[doc = "TIM15 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim15rst(&mut self, val: super::vals::Tim15rst) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "TIM16 timer reset Set and cleared by software."]
    #[inline(always)]
    pub const fn tim16rst(&self) -> super::vals::Tim16rst {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Tim16rst::from_bits(val as u8)
    }
    #[doc = "TIM16 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim16rst(&mut self, val: super::vals::Tim16rst) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "TIM17 timer reset Set and cleared by software."]
    #[inline(always)]
    pub const fn tim17rst(&self) -> super::vals::Tim17rst {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Tim17rst::from_bits(val as u8)
    }
    #[doc = "TIM17 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim17rst(&mut self, val: super::vals::Tim17rst) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "TIM20 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn tim20rst(&self) -> super::vals::Tim20rst {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Tim20rst::from_bits(val as u8)
    }
    #[doc = "TIM20 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim20rst(&mut self, val: super::vals::Tim20rst) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Serial audio interface 1 (SAI1) reset Set and cleared by software."]
    #[inline(always)]
    pub const fn sai1rst(&self) -> super::vals::Sai1rst {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Sai1rst::from_bits(val as u8)
    }
    #[doc = "Serial audio interface 1 (SAI1) reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_sai1rst(&mut self, val: super::vals::Sai1rst) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "HRTIM1 reset Set and cleared by software."]
    #[inline(always)]
    pub const fn hrtim1rst(&self) -> super::vals::Hrtim1rst {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Hrtim1rst::from_bits(val as u8)
    }
    #[doc = "HRTIM1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_hrtim1rst(&mut self, val: super::vals::Hrtim1rst) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for RccApb2rstr {
    #[inline(always)]
    fn default() -> RccApb2rstr {
        RccApb2rstr(0)
    }
}
#[doc = "APB2 peripheral clocks enable in Sleep and Stop modes register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccApb2smenr(pub u32);
impl RccApb2smenr {
    #[doc = "SYSCFG + COMP + VREFBUF + OPAMP clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn syscfgsmen(&self) -> super::vals::Syscfgsmen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Syscfgsmen::from_bits(val as u8)
    }
    #[doc = "SYSCFG + COMP + VREFBUF + OPAMP clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_syscfgsmen(&mut self, val: super::vals::Syscfgsmen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn tim1smen(&self) -> super::vals::Tim1smen {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Tim1smen::from_bits(val as u8)
    }
    #[doc = "TIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim1smen(&mut self, val: super::vals::Tim1smen) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "SPI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn spi1smen(&self) -> super::vals::Spi1smen {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Spi1smen::from_bits(val as u8)
    }
    #[doc = "SPI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_spi1smen(&mut self, val: super::vals::Spi1smen) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "TIM8 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn tim8smen(&self) -> super::vals::Tim8smen {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Tim8smen::from_bits(val as u8)
    }
    #[doc = "TIM8 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim8smen(&mut self, val: super::vals::Tim8smen) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "USART1clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn usart1smen(&self) -> super::vals::Usart1smen {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Usart1smen::from_bits(val as u8)
    }
    #[doc = "USART1clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_usart1smen(&mut self, val: super::vals::Usart1smen) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "SPI4 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn spi4smen(&self) -> super::vals::Spi4smen {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Spi4smen::from_bits(val as u8)
    }
    #[doc = "SPI4 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_spi4smen(&mut self, val: super::vals::Spi4smen) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "TIM15 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn tim15smen(&self) -> super::vals::Tim15smen {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Tim15smen::from_bits(val as u8)
    }
    #[doc = "TIM15 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim15smen(&mut self, val: super::vals::Tim15smen) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "TIM16 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn tim16smen(&self) -> super::vals::Tim16smen {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Tim16smen::from_bits(val as u8)
    }
    #[doc = "TIM16 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim16smen(&mut self, val: super::vals::Tim16smen) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "TIM17 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn tim17smen(&self) -> super::vals::Tim17smen {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Tim17smen::from_bits(val as u8)
    }
    #[doc = "TIM17 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim17smen(&mut self, val: super::vals::Tim17smen) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "TIM20 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn tim20smen(&self) -> super::vals::Tim20smen {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Tim20smen::from_bits(val as u8)
    }
    #[doc = "TIM20 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_tim20smen(&mut self, val: super::vals::Tim20smen) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "SAI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn sai1smen(&self) -> super::vals::Sai1smen {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Sai1smen::from_bits(val as u8)
    }
    #[doc = "SAI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_sai1smen(&mut self, val: super::vals::Sai1smen) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "HRTIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub const fn hrtim1smen(&self) -> super::vals::Hrtim1smen {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Hrtim1smen::from_bits(val as u8)
    }
    #[doc = "HRTIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn set_hrtim1smen(&mut self, val: super::vals::Hrtim1smen) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for RccApb2smenr {
    #[inline(always)]
    fn default() -> RccApb2smenr {
        RccApb2smenr(0)
    }
}
#[doc = "RTC domain control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccBdcr(pub u32);
impl RccBdcr {
    #[doc = "LSE oscillator enable Set and cleared by software."]
    #[inline(always)]
    pub const fn lseon(&self) -> super::vals::Lseon {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lseon::from_bits(val as u8)
    }
    #[doc = "LSE oscillator enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_lseon(&mut self, val: super::vals::Lseon) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LSE oscillator ready Set and cleared by hardware to indicate when the external 32 kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after 6 external low-speed oscillator clock cycles."]
    #[inline(always)]
    pub const fn lserdy(&self) -> super::vals::Lserdy {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Lserdy::from_bits(val as u8)
    }
    #[doc = "LSE oscillator ready Set and cleared by hardware to indicate when the external 32 kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after 6 external low-speed oscillator clock cycles."]
    #[inline(always)]
    pub fn set_lserdy(&mut self, val: super::vals::Lserdy) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32 kHz oscillator is disabled (LSEON=0 and LSERDY=0)."]
    #[inline(always)]
    pub const fn lsebyp(&self) -> super::vals::Lsebyp {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Lsebyp::from_bits(val as u8)
    }
    #[doc = "LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32 kHz oscillator is disabled (LSEON=0 and LSERDY=0)."]
    #[inline(always)]
    pub fn set_lsebyp(&mut self, val: super::vals::Lsebyp) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "LSE oscillator drive capability Set by software to modulate the LSE oscillators drive capability. The oscillator is in Xtal mode when it is not in bypass mode."]
    #[inline(always)]
    pub const fn lsedrv(&self) -> super::vals::Lsedrv {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::Lsedrv::from_bits(val as u8)
    }
    #[doc = "LSE oscillator drive capability Set by software to modulate the LSE oscillators drive capability. The oscillator is in Xtal mode when it is not in bypass mode."]
    #[inline(always)]
    pub fn set_lsedrv(&mut self, val: super::vals::Lsedrv) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "CSS on LSE enable Set by software to enable the Clock Security System on LSE (32 kHz oscillator). LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software MUST disable the LSECSSON bit."]
    #[inline(always)]
    pub const fn lsecsson(&self) -> super::vals::Lsecsson {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Lsecsson::from_bits(val as u8)
    }
    #[doc = "CSS on LSE enable Set by software to enable the Clock Security System on LSE (32 kHz oscillator). LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software MUST disable the LSECSSON bit."]
    #[inline(always)]
    pub fn set_lsecsson(&mut self, val: super::vals::Lsecsson) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "CSS on LSE failure Detection Set by hardware to indicate when a failure has been detected by the Clock Security System on the external 32 kHz oscillator (LSE)."]
    #[inline(always)]
    pub const fn lsecssd(&self) -> super::vals::Lsecssd {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Lsecssd::from_bits(val as u8)
    }
    #[doc = "CSS on LSE failure Detection Set by hardware to indicate when a failure has been detected by the Clock Security System on the external 32 kHz oscillator (LSE)."]
    #[inline(always)]
    pub fn set_lsecssd(&mut self, val: super::vals::Lsecssd) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "RTC clock source selection Set by software to select the clock source for the RTC. Once the RTC clock source has been selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset them."]
    #[inline(always)]
    pub const fn rtcsel(&self) -> super::vals::Rtcsel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Rtcsel::from_bits(val as u8)
    }
    #[doc = "RTC clock source selection Set by software to select the clock source for the RTC. Once the RTC clock source has been selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset them."]
    #[inline(always)]
    pub fn set_rtcsel(&mut self, val: super::vals::Rtcsel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "RTC clock enable Set and cleared by software."]
    #[inline(always)]
    pub const fn rtcen(&self) -> super::vals::Rtcen {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Rtcen::from_bits(val as u8)
    }
    #[doc = "RTC clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_rtcen(&mut self, val: super::vals::Rtcen) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "RTC domain software reset Set and cleared by software."]
    #[inline(always)]
    pub const fn bdrst(&self) -> super::vals::Bdrst {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Bdrst::from_bits(val as u8)
    }
    #[doc = "RTC domain software reset Set and cleared by software."]
    #[inline(always)]
    pub fn set_bdrst(&mut self, val: super::vals::Bdrst) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Low speed clock output enable Set and cleared by software."]
    #[inline(always)]
    pub const fn lscoen(&self) -> super::vals::Lscoen {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Lscoen::from_bits(val as u8)
    }
    #[doc = "Low speed clock output enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_lscoen(&mut self, val: super::vals::Lscoen) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Low speed clock output selection Set and cleared by software."]
    #[inline(always)]
    pub const fn lscosel(&self) -> super::vals::Lscosel {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Lscosel::from_bits(val as u8)
    }
    #[doc = "Low speed clock output selection Set and cleared by software."]
    #[inline(always)]
    pub fn set_lscosel(&mut self, val: super::vals::Lscosel) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for RccBdcr {
    #[inline(always)]
    fn default() -> RccBdcr {
        RccBdcr(0)
    }
}
#[doc = "Peripherals independent clock configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccCcipr(pub u32);
impl RccCcipr {
    #[doc = "USART1 clock source selection This bit is set and cleared by software to select the USART1 clock source."]
    #[inline(always)]
    pub const fn usart1sel(&self) -> super::vals::Usart1sel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Usart1sel::from_bits(val as u8)
    }
    #[doc = "USART1 clock source selection This bit is set and cleared by software to select the USART1 clock source."]
    #[inline(always)]
    pub fn set_usart1sel(&mut self, val: super::vals::Usart1sel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "USART2 clock source selection This bit is set and cleared by software to select the USART2 clock source."]
    #[inline(always)]
    pub const fn usart2sel(&self) -> super::vals::Usart2sel {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Usart2sel::from_bits(val as u8)
    }
    #[doc = "USART2 clock source selection This bit is set and cleared by software to select the USART2 clock source."]
    #[inline(always)]
    pub fn set_usart2sel(&mut self, val: super::vals::Usart2sel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "USART3 clock source selection This bit is set and cleared by software to select the USART3 clock source."]
    #[inline(always)]
    pub const fn usart3sel(&self) -> super::vals::Usart3sel {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Usart3sel::from_bits(val as u8)
    }
    #[doc = "USART3 clock source selection This bit is set and cleared by software to select the USART3 clock source."]
    #[inline(always)]
    pub fn set_usart3sel(&mut self, val: super::vals::Usart3sel) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "UART4 clock source selection This bit is set and cleared by software to select the UART4 clock source."]
    #[inline(always)]
    pub const fn uart4sel(&self) -> super::vals::Uart4sel {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Uart4sel::from_bits(val as u8)
    }
    #[doc = "UART4 clock source selection This bit is set and cleared by software to select the UART4 clock source."]
    #[inline(always)]
    pub fn set_uart4sel(&mut self, val: super::vals::Uart4sel) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "UART5 clock source selection These bits are set and cleared by software to select the UART5 clock source."]
    #[inline(always)]
    pub const fn uart5sel(&self) -> super::vals::Uart5sel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Uart5sel::from_bits(val as u8)
    }
    #[doc = "UART5 clock source selection These bits are set and cleared by software to select the UART5 clock source."]
    #[inline(always)]
    pub fn set_uart5sel(&mut self, val: super::vals::Uart5sel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LPUART1 clock source selection These bits are set and cleared by software to select the LPUART1 clock source."]
    #[inline(always)]
    pub const fn lpuart1sel(&self) -> super::vals::Lpuart1sel {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Lpuart1sel::from_bits(val as u8)
    }
    #[doc = "LPUART1 clock source selection These bits are set and cleared by software to select the LPUART1 clock source."]
    #[inline(always)]
    pub fn set_lpuart1sel(&mut self, val: super::vals::Lpuart1sel) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "I2C1 clock source selection These bits are set and cleared by software to select the I2C1 clock source."]
    #[inline(always)]
    pub const fn i2c1sel(&self) -> super::vals::I2c1sel {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::I2c1sel::from_bits(val as u8)
    }
    #[doc = "I2C1 clock source selection These bits are set and cleared by software to select the I2C1 clock source."]
    #[inline(always)]
    pub fn set_i2c1sel(&mut self, val: super::vals::I2c1sel) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "I2C2 clock source selection These bits are set and cleared by software to select the I2C2 clock source."]
    #[inline(always)]
    pub const fn i2c2sel(&self) -> super::vals::I2c2sel {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::I2c2sel::from_bits(val as u8)
    }
    #[doc = "I2C2 clock source selection These bits are set and cleared by software to select the I2C2 clock source."]
    #[inline(always)]
    pub fn set_i2c2sel(&mut self, val: super::vals::I2c2sel) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "I2C3 clock source selection These bits are set and cleared by software to select the I2C3 clock source."]
    #[inline(always)]
    pub const fn i2c3sel(&self) -> super::vals::I2c3sel {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::I2c3sel::from_bits(val as u8)
    }
    #[doc = "I2C3 clock source selection These bits are set and cleared by software to select the I2C3 clock source."]
    #[inline(always)]
    pub fn set_i2c3sel(&mut self, val: super::vals::I2c3sel) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Low power timer 1 clock source selection These bits are set and cleared by software to select the LPTIM1 clock source."]
    #[inline(always)]
    pub const fn lptim1sel(&self) -> super::vals::Lptim1sel {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Lptim1sel::from_bits(val as u8)
    }
    #[doc = "Low power timer 1 clock source selection These bits are set and cleared by software to select the LPTIM1 clock source."]
    #[inline(always)]
    pub fn set_lptim1sel(&mut self, val: super::vals::Lptim1sel) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "clock source selection These bits are set and cleared by software to select the SAI clock source."]
    #[inline(always)]
    pub const fn sai1sel(&self) -> super::vals::Sai1sel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Sai1sel::from_bits(val as u8)
    }
    #[doc = "clock source selection These bits are set and cleared by software to select the SAI clock source."]
    #[inline(always)]
    pub fn set_sai1sel(&mut self, val: super::vals::Sai1sel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "clock source selection These bits are set and cleared by software to select the I2S23 clock source."]
    #[inline(always)]
    pub const fn i2s23sel(&self) -> super::vals::I2s23sel {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::I2s23sel::from_bits(val as u8)
    }
    #[doc = "clock source selection These bits are set and cleared by software to select the I2S23 clock source."]
    #[inline(always)]
    pub fn set_i2s23sel(&mut self, val: super::vals::I2s23sel) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "None"]
    #[inline(always)]
    pub const fn fdcansel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn set_fdcansel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "48 MHz clock source selection These bits are set and cleared by software to select the 48 MHz clock source used by USB device FS and RNG."]
    #[inline(always)]
    pub const fn clk48sel(&self) -> super::vals::Clk48sel {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Clk48sel::from_bits(val as u8)
    }
    #[doc = "48 MHz clock source selection These bits are set and cleared by software to select the 48 MHz clock source used by USB device FS and RNG."]
    #[inline(always)]
    pub fn set_clk48sel(&mut self, val: super::vals::Clk48sel) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "ADC1/2 clock source selection These bits are set and cleared by software to select the clock source used by the ADC interface."]
    #[inline(always)]
    pub const fn adc12sel(&self) -> super::vals::Adc12sel {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Adc12sel::from_bits(val as u8)
    }
    #[doc = "ADC1/2 clock source selection These bits are set and cleared by software to select the clock source used by the ADC interface."]
    #[inline(always)]
    pub fn set_adc12sel(&mut self, val: super::vals::Adc12sel) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "ADC3/4/5 clock source selection These bits are set and cleared by software to select the clock source used by the ADC345 interface."]
    #[inline(always)]
    pub const fn adc345sel(&self) -> super::vals::Adc345sel {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Adc345sel::from_bits(val as u8)
    }
    #[doc = "ADC3/4/5 clock source selection These bits are set and cleared by software to select the clock source used by the ADC345 interface."]
    #[inline(always)]
    pub fn set_adc345sel(&mut self, val: super::vals::Adc345sel) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for RccCcipr {
    #[inline(always)]
    fn default() -> RccCcipr {
        RccCcipr(0)
    }
}
#[doc = "Peripherals independent clock configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccCcipr2(pub u32);
impl RccCcipr2 {
    #[doc = "I2C4 clock source selection These bits are set and cleared by software to select the I2C4 clock source."]
    #[inline(always)]
    pub const fn i2c4sel(&self) -> super::vals::I2c4sel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::I2c4sel::from_bits(val as u8)
    }
    #[doc = "I2C4 clock source selection These bits are set and cleared by software to select the I2C4 clock source."]
    #[inline(always)]
    pub fn set_i2c4sel(&mut self, val: super::vals::I2c4sel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "QUADSPI clock source selection Set and reset by software."]
    #[inline(always)]
    pub const fn qspisel(&self) -> super::vals::Qspisel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Qspisel::from_bits(val as u8)
    }
    #[doc = "QUADSPI clock source selection Set and reset by software."]
    #[inline(always)]
    pub fn set_qspisel(&mut self, val: super::vals::Qspisel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
}
impl Default for RccCcipr2 {
    #[inline(always)]
    fn default() -> RccCcipr2 {
        RccCcipr2(0)
    }
}
#[doc = "Clock configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccCfgr(pub u32);
impl RccCfgr {
    #[doc = "System clock switch Set and cleared by software to select system clock source (SYSCLK). Configured by hardware to force HSI16 oscillator selection when exiting stop and standby modes or in case of failure of the HSE oscillator."]
    #[inline(always)]
    pub const fn sw(&self) -> super::vals::Sw {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sw::from_bits(val as u8)
    }
    #[doc = "System clock switch Set and cleared by software to select system clock source (SYSCLK). Configured by hardware to force HSI16 oscillator selection when exiting stop and standby modes or in case of failure of the HSE oscillator."]
    #[inline(always)]
    pub fn set_sw(&mut self, val: super::vals::Sw) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "System clock switch status Set and cleared by hardware to indicate which clock source is used as system clock."]
    #[inline(always)]
    pub const fn sws(&self) -> super::vals::Sws {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sws::from_bits(val as u8)
    }
    #[doc = "System clock switch status Set and cleared by hardware to indicate which clock source is used as system clock."]
    #[inline(always)]
    pub fn set_sws(&mut self, val: super::vals::Sws) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "AHB prescaler Set and cleared by software to control the division factor of the AHB clock. Note: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details please refer to Section 6.1.5: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account. 0xxx: SYSCLK not divided"]
    #[inline(always)]
    pub const fn hpre(&self) -> super::vals::Hpre {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Hpre::from_bits(val as u8)
    }
    #[doc = "AHB prescaler Set and cleared by software to control the division factor of the AHB clock. Note: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details please refer to Section 6.1.5: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account. 0xxx: SYSCLK not divided"]
    #[inline(always)]
    pub fn set_hpre(&mut self, val: super::vals::Hpre) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (PCLK1). 0xx: HCLK not divided"]
    #[inline(always)]
    pub const fn ppre1(&self) -> super::vals::Ppre1 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Ppre1::from_bits(val as u8)
    }
    #[doc = "APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (PCLK1). 0xx: HCLK not divided"]
    #[inline(always)]
    pub fn set_ppre1(&mut self, val: super::vals::Ppre1) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (PCLK2). 0xx: HCLK not divided"]
    #[inline(always)]
    pub const fn ppre2(&self) -> super::vals::Ppre2 {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::Ppre2::from_bits(val as u8)
    }
    #[doc = "APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (PCLK2). 0xx: HCLK not divided"]
    #[inline(always)]
    pub fn set_ppre2(&mut self, val: super::vals::Ppre2) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Microcontroller clock output Set and cleared by software. Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching."]
    #[inline(always)]
    pub const fn mcosel(&self) -> super::vals::Mcosel {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Mcosel::from_bits(val as u8)
    }
    #[doc = "Microcontroller clock output Set and cleared by software. Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching."]
    #[inline(always)]
    pub fn set_mcosel(&mut self, val: super::vals::Mcosel) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "Microcontroller clock output prescaler These bits are set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed"]
    #[inline(always)]
    pub const fn mcopre(&self) -> super::vals::Mcopre {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mcopre::from_bits(val as u8)
    }
    #[doc = "Microcontroller clock output prescaler These bits are set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed"]
    #[inline(always)]
    pub fn set_mcopre(&mut self, val: super::vals::Mcopre) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for RccCfgr {
    #[inline(always)]
    fn default() -> RccCfgr {
        RccCfgr(0)
    }
}
#[doc = "Clock interrupt clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccCicr(pub u32);
impl RccCicr {
    #[doc = "LSI ready interrupt clear This bit is set by software to clear the LSIRDYF flag."]
    #[inline(always)]
    pub const fn lsirdyc(&self) -> super::vals::Lsirdyc {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lsirdyc::from_bits(val as u8)
    }
    #[doc = "LSI ready interrupt clear This bit is set by software to clear the LSIRDYF flag."]
    #[inline(always)]
    pub fn set_lsirdyc(&mut self, val: super::vals::Lsirdyc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LSE ready interrupt clear This bit is set by software to clear the LSERDYF flag."]
    #[inline(always)]
    pub const fn lserdyc(&self) -> super::vals::Lserdyc {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Lserdyc::from_bits(val as u8)
    }
    #[doc = "LSE ready interrupt clear This bit is set by software to clear the LSERDYF flag."]
    #[inline(always)]
    pub fn set_lserdyc(&mut self, val: super::vals::Lserdyc) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "HSI16 ready interrupt clear This bit is set software to clear the HSIRDYF flag."]
    #[inline(always)]
    pub const fn hsirdyc(&self) -> super::vals::Hsirdyc {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Hsirdyc::from_bits(val as u8)
    }
    #[doc = "HSI16 ready interrupt clear This bit is set software to clear the HSIRDYF flag."]
    #[inline(always)]
    pub fn set_hsirdyc(&mut self, val: super::vals::Hsirdyc) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "HSE ready interrupt clear This bit is set by software to clear the HSERDYF flag."]
    #[inline(always)]
    pub const fn hserdyc(&self) -> super::vals::Hserdyc {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Hserdyc::from_bits(val as u8)
    }
    #[doc = "HSE ready interrupt clear This bit is set by software to clear the HSERDYF flag."]
    #[inline(always)]
    pub fn set_hserdyc(&mut self, val: super::vals::Hserdyc) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "PLL ready interrupt clear This bit is set by software to clear the PLLRDYF flag."]
    #[inline(always)]
    pub const fn pllrdyc(&self) -> super::vals::Pllrdyc {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pllrdyc::from_bits(val as u8)
    }
    #[doc = "PLL ready interrupt clear This bit is set by software to clear the PLLRDYF flag."]
    #[inline(always)]
    pub fn set_pllrdyc(&mut self, val: super::vals::Pllrdyc) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Clock security system interrupt clear This bit is set by software to clear the CSSF flag."]
    #[inline(always)]
    pub const fn cssc(&self) -> super::vals::Cssc {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Cssc::from_bits(val as u8)
    }
    #[doc = "Clock security system interrupt clear This bit is set by software to clear the CSSF flag."]
    #[inline(always)]
    pub fn set_cssc(&mut self, val: super::vals::Cssc) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "LSE Clock security system interrupt clear This bit is set by software to clear the LSECSSF flag."]
    #[inline(always)]
    pub const fn lsecssc(&self) -> super::vals::Lsecssc {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Lsecssc::from_bits(val as u8)
    }
    #[doc = "LSE Clock security system interrupt clear This bit is set by software to clear the LSECSSF flag."]
    #[inline(always)]
    pub fn set_lsecssc(&mut self, val: super::vals::Lsecssc) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "HSI48 oscillator ready interrupt clear This bit is set by software to clear the HSI48RDYF flag."]
    #[inline(always)]
    pub const fn hsi48rdyc(&self) -> super::vals::Hsi48rdyc {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Hsi48rdyc::from_bits(val as u8)
    }
    #[doc = "HSI48 oscillator ready interrupt clear This bit is set by software to clear the HSI48RDYF flag."]
    #[inline(always)]
    pub fn set_hsi48rdyc(&mut self, val: super::vals::Hsi48rdyc) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for RccCicr {
    #[inline(always)]
    fn default() -> RccCicr {
        RccCicr(0)
    }
}
#[doc = "Clock interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccCier(pub u32);
impl RccCier {
    #[doc = "LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
    #[inline(always)]
    pub const fn lsirdyie(&self) -> super::vals::Lsirdyie {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lsirdyie::from_bits(val as u8)
    }
    #[doc = "LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
    #[inline(always)]
    pub fn set_lsirdyie(&mut self, val: super::vals::Lsirdyie) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
    #[inline(always)]
    pub const fn lserdyie(&self) -> super::vals::Lserdyie {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Lserdyie::from_bits(val as u8)
    }
    #[doc = "LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
    #[inline(always)]
    pub fn set_lserdyie(&mut self, val: super::vals::Lserdyie) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization."]
    #[inline(always)]
    pub const fn hsirdyie(&self) -> super::vals::Hsirdyie {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Hsirdyie::from_bits(val as u8)
    }
    #[doc = "HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization."]
    #[inline(always)]
    pub fn set_hsirdyie(&mut self, val: super::vals::Hsirdyie) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
    #[inline(always)]
    pub const fn hserdyie(&self) -> super::vals::Hserdyie {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Hserdyie::from_bits(val as u8)
    }
    #[doc = "HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
    #[inline(always)]
    pub fn set_hserdyie(&mut self, val: super::vals::Hserdyie) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL lock."]
    #[inline(always)]
    pub const fn pllrdyie(&self) -> super::vals::Pllrdyie {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pllrdyie::from_bits(val as u8)
    }
    #[doc = "PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL lock."]
    #[inline(always)]
    pub fn set_pllrdyie(&mut self, val: super::vals::Pllrdyie) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "LSE clock security system interrupt enable Set and cleared by software to enable/disable interrupt caused by the clock security system on LSE."]
    #[inline(always)]
    pub const fn lsecssie(&self) -> super::vals::Lsecssie {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Lsecssie::from_bits(val as u8)
    }
    #[doc = "LSE clock security system interrupt enable Set and cleared by software to enable/disable interrupt caused by the clock security system on LSE."]
    #[inline(always)]
    pub fn set_lsecssie(&mut self, val: super::vals::Lsecssie) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the internal HSI48 oscillator."]
    #[inline(always)]
    pub const fn hsi48rdyie(&self) -> super::vals::Hsi48rdyie {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Hsi48rdyie::from_bits(val as u8)
    }
    #[doc = "HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the internal HSI48 oscillator."]
    #[inline(always)]
    pub fn set_hsi48rdyie(&mut self, val: super::vals::Hsi48rdyie) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for RccCier {
    #[inline(always)]
    fn default() -> RccCier {
        RccCier(0)
    }
}
#[doc = "Clock interrupt flag register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccCifr(pub u32);
impl RccCifr {
    #[doc = "LSI ready interrupt flag Set by hardware when the LSI clock becomes stable and LSIRDYDIE is set. Cleared by software setting the LSIRDYC bit."]
    #[inline(always)]
    pub const fn lsirdyf(&self) -> super::vals::Lsirdyf {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lsirdyf::from_bits(val as u8)
    }
    #[doc = "LSI ready interrupt flag Set by hardware when the LSI clock becomes stable and LSIRDYDIE is set. Cleared by software setting the LSIRDYC bit."]
    #[inline(always)]
    pub fn set_lsirdyf(&mut self, val: super::vals::Lsirdyf) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LSE ready interrupt flag Set by hardware when the LSE clock becomes stable and LSERDYDIE is set. Cleared by software setting the LSERDYC bit."]
    #[inline(always)]
    pub const fn lserdyf(&self) -> super::vals::Lserdyf {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Lserdyf::from_bits(val as u8)
    }
    #[doc = "LSE ready interrupt flag Set by hardware when the LSE clock becomes stable and LSERDYDIE is set. Cleared by software setting the LSERDYC bit."]
    #[inline(always)]
    pub fn set_lserdyf(&mut self, val: super::vals::Lserdyf) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "HSI16 ready interrupt flag Set by hardware when the HSI16 clock becomes stable and HSIRDYDIE is set in a response to setting the HSION (refer to Clock control register (RCC_CR)). When HSION is not set but the HSI16 oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit."]
    #[inline(always)]
    pub const fn hsirdyf(&self) -> super::vals::Hsirdyf {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Hsirdyf::from_bits(val as u8)
    }
    #[doc = "HSI16 ready interrupt flag Set by hardware when the HSI16 clock becomes stable and HSIRDYDIE is set in a response to setting the HSION (refer to Clock control register (RCC_CR)). When HSION is not set but the HSI16 oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit."]
    #[inline(always)]
    pub fn set_hsirdyf(&mut self, val: super::vals::Hsirdyf) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "HSE ready interrupt flag Set by hardware when the HSE clock becomes stable and HSERDYDIE is set. Cleared by software setting the HSERDYC bit."]
    #[inline(always)]
    pub const fn hserdyf(&self) -> super::vals::Hserdyf {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Hserdyf::from_bits(val as u8)
    }
    #[doc = "HSE ready interrupt flag Set by hardware when the HSE clock becomes stable and HSERDYDIE is set. Cleared by software setting the HSERDYC bit."]
    #[inline(always)]
    pub fn set_hserdyf(&mut self, val: super::vals::Hserdyf) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "PLL ready interrupt flag Set by hardware when the PLL locks and PLLRDYDIE is set. Cleared by software setting the PLLRDYC bit."]
    #[inline(always)]
    pub const fn pllrdyf(&self) -> super::vals::Pllrdyf {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pllrdyf::from_bits(val as u8)
    }
    #[doc = "PLL ready interrupt flag Set by hardware when the PLL locks and PLLRDYDIE is set. Cleared by software setting the PLLRDYC bit."]
    #[inline(always)]
    pub fn set_pllrdyf(&mut self, val: super::vals::Pllrdyf) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Clock security system interrupt flag Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit."]
    #[inline(always)]
    pub const fn cssf(&self) -> super::vals::Cssf {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Cssf::from_bits(val as u8)
    }
    #[doc = "Clock security system interrupt flag Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit."]
    #[inline(always)]
    pub fn set_cssf(&mut self, val: super::vals::Cssf) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "LSE Clock security system interrupt flag Set by hardware when a failure is detected in the LSE oscillator. Cleared by software setting the LSECSSC bit."]
    #[inline(always)]
    pub const fn lsecssf(&self) -> super::vals::Lsecssf {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Lsecssf::from_bits(val as u8)
    }
    #[doc = "LSE Clock security system interrupt flag Set by hardware when a failure is detected in the LSE oscillator. Cleared by software setting the LSECSSC bit."]
    #[inline(always)]
    pub fn set_lsecssf(&mut self, val: super::vals::Lsecssf) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "HSI48 ready interrupt flag Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set in a response to setting the HSI48ON (refer to Clock recovery RC register (RCC_CRRCR)). Cleared by software setting the HSI48RDYC bit."]
    #[inline(always)]
    pub const fn hsi48rdyf(&self) -> super::vals::Hsi48rdyf {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Hsi48rdyf::from_bits(val as u8)
    }
    #[doc = "HSI48 ready interrupt flag Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set in a response to setting the HSI48ON (refer to Clock recovery RC register (RCC_CRRCR)). Cleared by software setting the HSI48RDYC bit."]
    #[inline(always)]
    pub fn set_hsi48rdyf(&mut self, val: super::vals::Hsi48rdyf) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for RccCifr {
    #[inline(always)]
    fn default() -> RccCifr {
        RccCifr(0)
    }
}
#[doc = "Clock control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccCr(pub u32);
impl RccCr {
    #[doc = "HSI16 clock enable Set and cleared by software. Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the HSI16 oscillator ON when STOPWUCK=1 or HSIASFS = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock."]
    #[inline(always)]
    pub const fn hsion(&self) -> super::vals::Hsion {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Hsion::from_bits(val as u8)
    }
    #[doc = "HSI16 clock enable Set and cleared by software. Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the HSI16 oscillator ON when STOPWUCK=1 or HSIASFS = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock."]
    #[inline(always)]
    pub fn set_hsion(&mut self, val: super::vals::Hsion) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "HSI16 always enable for peripheral kernels. Set and cleared by software to force HSI16 ON even in Stop modes. The HSI16 can only feed USARTs and I<sup>2</sup>Cs peripherals configured with HSI16 as kernel clock. Keeping the HSI16 ON in Stop mode allows to avoid slowing down the communication speed because of the HSI16 startup time. This bit has no effect on HSION value."]
    #[inline(always)]
    pub const fn hsikeron(&self) -> super::vals::Hsikeron {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Hsikeron::from_bits(val as u8)
    }
    #[doc = "HSI16 always enable for peripheral kernels. Set and cleared by software to force HSI16 ON even in Stop modes. The HSI16 can only feed USARTs and I<sup>2</sup>Cs peripherals configured with HSI16 as kernel clock. Keeping the HSI16 ON in Stop mode allows to avoid slowing down the communication speed because of the HSI16 startup time. This bit has no effect on HSION value."]
    #[inline(always)]
    pub fn set_hsikeron(&mut self, val: super::vals::Hsikeron) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "HSI16 clock ready flag Set by hardware to indicate that HSI16 oscillator is stable. This bit is set only when HSI16 is enabled by software by setting HSION. Note: Once the HSION bit is cleared, HSIRDY goes low after 6 HSI16 clock cycles."]
    #[inline(always)]
    pub const fn hsirdy(&self) -> super::vals::Hsirdy {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Hsirdy::from_bits(val as u8)
    }
    #[doc = "HSI16 clock ready flag Set by hardware to indicate that HSI16 oscillator is stable. This bit is set only when HSI16 is enabled by software by setting HSION. Note: Once the HSION bit is cleared, HSIRDY goes low after 6 HSI16 clock cycles."]
    #[inline(always)]
    pub fn set_hsirdy(&mut self, val: super::vals::Hsirdy) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock."]
    #[inline(always)]
    pub const fn hseon(&self) -> super::vals::Hseon {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Hseon::from_bits(val as u8)
    }
    #[doc = "HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock."]
    #[inline(always)]
    pub fn set_hseon(&mut self, val: super::vals::Hseon) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable. Note: Once the HSEON bit is cleared, HSERDY goes low after 6 HSE clock cycles."]
    #[inline(always)]
    pub const fn hserdy(&self) -> super::vals::Hserdy {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Hserdy::from_bits(val as u8)
    }
    #[doc = "HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable. Note: Once the HSEON bit is cleared, HSERDY goes low after 6 HSE clock cycles."]
    #[inline(always)]
    pub fn set_hserdy(&mut self, val: super::vals::Hserdy) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
    #[inline(always)]
    pub const fn hsebyp(&self) -> super::vals::Hsebyp {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Hsebyp::from_bits(val as u8)
    }
    #[doc = "HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
    #[inline(always)]
    pub fn set_hsebyp(&mut self, val: super::vals::Hsebyp) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset."]
    #[inline(always)]
    pub const fn csson(&self) -> super::vals::Csson {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Csson::from_bits(val as u8)
    }
    #[doc = "Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset."]
    #[inline(always)]
    pub fn set_csson(&mut self, val: super::vals::Csson) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Main PLL enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL clock is used as the system clock."]
    #[inline(always)]
    pub const fn pllon(&self) -> super::vals::Pllon {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pllon::from_bits(val as u8)
    }
    #[doc = "Main PLL enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL clock is used as the system clock."]
    #[inline(always)]
    pub fn set_pllon(&mut self, val: super::vals::Pllon) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Main PLL clock ready flag Set by hardware to indicate that the main PLL is locked."]
    #[inline(always)]
    pub const fn pllrdy(&self) -> super::vals::Pllrdy {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pllrdy::from_bits(val as u8)
    }
    #[doc = "Main PLL clock ready flag Set by hardware to indicate that the main PLL is locked."]
    #[inline(always)]
    pub fn set_pllrdy(&mut self, val: super::vals::Pllrdy) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for RccCr {
    #[inline(always)]
    fn default() -> RccCr {
        RccCr(0)
    }
}
#[doc = "Clock recovery RC register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccCrrcr(pub u32);
impl RccCrrcr {
    #[doc = "HSI48 clock enable Set and cleared by software. Cleared by hardware to stop the HSI48 when entering in Stop, Standby or Shutdown modes."]
    #[inline(always)]
    pub const fn hsi48on(&self) -> super::vals::Hsi48on {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Hsi48on::from_bits(val as u8)
    }
    #[doc = "HSI48 clock enable Set and cleared by software. Cleared by hardware to stop the HSI48 when entering in Stop, Standby or Shutdown modes."]
    #[inline(always)]
    pub fn set_hsi48on(&mut self, val: super::vals::Hsi48on) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "HSI48 clock ready flag Set by hardware to indicate that HSI48 oscillator is stable. This bit is set only when HSI48 is enabled by software by setting HSI48ON."]
    #[inline(always)]
    pub const fn hsi48rdy(&self) -> super::vals::Hsi48rdy {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Hsi48rdy::from_bits(val as u8)
    }
    #[doc = "HSI48 clock ready flag Set by hardware to indicate that HSI48 oscillator is stable. This bit is set only when HSI48 is enabled by software by setting HSI48ON."]
    #[inline(always)]
    pub fn set_hsi48rdy(&mut self, val: super::vals::Hsi48rdy) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "HSI48 clock calibration These bits are initialized at startup with the factory-programmed HSI48 calibration trim value. They are ready only."]
    #[inline(always)]
    pub const fn hsi48cal(&self) -> u16 {
        let val = (self.0 >> 7usize) & 0x01ff;
        val as u16
    }
    #[doc = "HSI48 clock calibration These bits are initialized at startup with the factory-programmed HSI48 calibration trim value. They are ready only."]
    #[inline(always)]
    pub fn set_hsi48cal(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 7usize)) | (((val as u32) & 0x01ff) << 7usize);
    }
}
impl Default for RccCrrcr {
    #[inline(always)]
    fn default() -> RccCrrcr {
        RccCrrcr(0)
    }
}
#[doc = "Control/status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccCsr(pub u32);
impl RccCsr {
    #[doc = "LSI oscillator enable Set and cleared by software."]
    #[inline(always)]
    pub const fn lsion(&self) -> super::vals::Lsion {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lsion::from_bits(val as u8)
    }
    #[doc = "LSI oscillator enable Set and cleared by software."]
    #[inline(always)]
    pub fn set_lsion(&mut self, val: super::vals::Lsion) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after 3 LSI oscillator clock cycles. This bit can be set even if LSION = 0 if the LSI is requested by the Clock Security System on LSE, by the Independent Watchdog or by the RTC."]
    #[inline(always)]
    pub const fn lsirdy(&self) -> super::vals::Lsirdy {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Lsirdy::from_bits(val as u8)
    }
    #[doc = "LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after 3 LSI oscillator clock cycles. This bit can be set even if LSION = 0 if the LSI is requested by the Clock Security System on LSE, by the Independent Watchdog or by the RTC."]
    #[inline(always)]
    pub fn set_lsirdy(&mut self, val: super::vals::Lsirdy) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Remove reset flag Set by software to clear the reset flags."]
    #[inline(always)]
    pub const fn rmvf(&self) -> super::vals::Rmvf {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Rmvf::from_bits(val as u8)
    }
    #[doc = "Remove reset flag Set by software to clear the reset flags."]
    #[inline(always)]
    pub fn set_rmvf(&mut self, val: super::vals::Rmvf) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Option byte loader reset flag Set by hardware when a reset from the Option Byte loading occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub const fn oblrstf(&self) -> super::vals::Oblrstf {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Oblrstf::from_bits(val as u8)
    }
    #[doc = "Option byte loader reset flag Set by hardware when a reset from the Option Byte loading occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn set_oblrstf(&mut self, val: super::vals::Oblrstf) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub const fn pinrstf(&self) -> super::vals::Pinrstf {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pinrstf::from_bits(val as u8)
    }
    #[doc = "Pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn set_pinrstf(&mut self, val: super::vals::Pinrstf) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "BOR flag Set by hardware when a BOR occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub const fn borrstf(&self) -> super::vals::Borrstf {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Borrstf::from_bits(val as u8)
    }
    #[doc = "BOR flag Set by hardware when a BOR occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn set_borrstf(&mut self, val: super::vals::Borrstf) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Software reset flag Set by hardware when a software reset occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub const fn sftrstf(&self) -> super::vals::Sftrstf {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Sftrstf::from_bits(val as u8)
    }
    #[doc = "Software reset flag Set by hardware when a software reset occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn set_sftrstf(&mut self, val: super::vals::Sftrstf) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Independent window watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub const fn iwdgrstf(&self) -> super::vals::Iwdgrstf {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Iwdgrstf::from_bits(val as u8)
    }
    #[doc = "Independent window watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn set_iwdgrstf(&mut self, val: super::vals::Iwdgrstf) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub const fn wwdgrstf(&self) -> super::vals::Wwdgrstf {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Wwdgrstf::from_bits(val as u8)
    }
    #[doc = "Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn set_wwdgrstf(&mut self, val: super::vals::Wwdgrstf) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Low-power reset flag Set by hardware when a reset occurs due to illegal Stop, Standby or Shutdown mode entry. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub const fn lpwrrstf(&self) -> super::vals::Lpwrrstf {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Lpwrrstf::from_bits(val as u8)
    }
    #[doc = "Low-power reset flag Set by hardware when a reset occurs due to illegal Stop, Standby or Shutdown mode entry. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn set_lpwrrstf(&mut self, val: super::vals::Lpwrrstf) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for RccCsr {
    #[inline(always)]
    fn default() -> RccCsr {
        RccCsr(0)
    }
}
#[doc = "Internal clock sources calibration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccIcscr(pub u32);
impl RccIcscr {
    #[doc = "HSI16 clock calibration These bits are initialized at startup with the factory-programmed HSI16 calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value."]
    #[inline(always)]
    pub const fn hsical(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "HSI16 clock calibration These bits are initialized at startup with the factory-programmed HSI16 calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value."]
    #[inline(always)]
    pub fn set_hsical(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "HSI16 clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\\[7:0\\] bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI16. The default value is 16, which, when added to the HSICAL value, should trim the HSI16 to 16 MHz 1 %."]
    #[inline(always)]
    pub const fn hsitrim(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "HSI16 clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\\[7:0\\] bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI16. The default value is 16, which, when added to the HSICAL value, should trim the HSI16 to 16 MHz 1 %."]
    #[inline(always)]
    pub fn set_hsitrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
}
impl Default for RccIcscr {
    #[inline(always)]
    fn default() -> RccIcscr {
        RccIcscr(0)
    }
}
#[doc = "PLL configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccPllcfgr(pub u32);
impl RccPllcfgr {
    #[doc = "Main PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC should be 00."]
    #[inline(always)]
    pub const fn pllsrc(&self) -> super::vals::Pllsrc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Pllsrc::from_bits(val as u8)
    }
    #[doc = "Main PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC should be 00."]
    #[inline(always)]
    pub fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Division factor for the main PLL input clock Set and cleared by software to divide the PLL input clock before the VCO. These bits can be written only when all PLLs are disabled. VCO input frequency = PLL input clock frequency / PLLM with 1 <= PLLM <= 16 ... Note: The software has to set these bits correctly to ensure that the VCO input frequency is within the range defined in the device datasheet."]
    #[inline(always)]
    pub const fn pllm(&self) -> super::vals::Pllm {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Pllm::from_bits(val as u8)
    }
    #[doc = "Division factor for the main PLL input clock Set and cleared by software to divide the PLL input clock before the VCO. These bits can be written only when all PLLs are disabled. VCO input frequency = PLL input clock frequency / PLLM with 1 <= PLLM <= 16 ... Note: The software has to set these bits correctly to ensure that the VCO input frequency is within the range defined in the device datasheet."]
    #[inline(always)]
    pub fn set_pllm(&mut self, val: super::vals::Pllm) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Main PLL multiplication factor for VCO Set and cleared by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled. VCO output frequency = VCO input frequency x PLLN with 8 =< PLLN =< 127 ... ... Note: The software has to set correctly these bits to assure that the VCO output frequency is within the range defined in the device datasheet."]
    #[inline(always)]
    pub const fn plln(&self) -> super::vals::Plln {
        let val = (self.0 >> 8usize) & 0x7f;
        super::vals::Plln::from_bits(val as u8)
    }
    #[doc = "Main PLL multiplication factor for VCO Set and cleared by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled. VCO output frequency = VCO input frequency x PLLN with 8 =< PLLN =< 127 ... ... Note: The software has to set correctly these bits to assure that the VCO output frequency is within the range defined in the device datasheet."]
    #[inline(always)]
    pub fn set_plln(&mut self, val: super::vals::Plln) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
    }
    #[doc = "Main PLL PLL P clock output enable Set and reset by software to enable the PLL P clock output of the PLL. In order to save power, when the PLL P clock output of the PLL is not used, the value of PLLPEN should be 0."]
    #[inline(always)]
    pub const fn pllpen(&self) -> super::vals::Pllpen {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pllpen::from_bits(val as u8)
    }
    #[doc = "Main PLL PLL P clock output enable Set and reset by software to enable the PLL P clock output of the PLL. In order to save power, when the PLL P clock output of the PLL is not used, the value of PLLPEN should be 0."]
    #[inline(always)]
    pub fn set_pllpen(&mut self, val: super::vals::Pllpen) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Main PLL division factor for PLL P clock. Set and cleared by software to control the frequency of the main PLL output clock PLL P clock. These bits can be written only if PLL is disabled. When the PLLPDIV\\[4:0\\] is set to 00000PLL P output clock frequency = VCO frequency / PLLP with PLLP =7, or 17 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
    #[inline(always)]
    pub const fn pllp(&self) -> super::vals::Pllp {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pllp::from_bits(val as u8)
    }
    #[doc = "Main PLL division factor for PLL P clock. Set and cleared by software to control the frequency of the main PLL output clock PLL P clock. These bits can be written only if PLL is disabled. When the PLLPDIV\\[4:0\\] is set to 00000PLL P output clock frequency = VCO frequency / PLLP with PLLP =7, or 17 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
    #[inline(always)]
    pub fn set_pllp(&mut self, val: super::vals::Pllp) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Main PLL Q clock output enable Set and reset by software to enable the PLL Q clock output of the PLL. In order to save power, when the PLL Q clock output of the PLL is not used, the value of PLLQEN should be 0."]
    #[inline(always)]
    pub const fn pllqen(&self) -> super::vals::Pllqen {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pllqen::from_bits(val as u8)
    }
    #[doc = "Main PLL Q clock output enable Set and reset by software to enable the PLL Q clock output of the PLL. In order to save power, when the PLL Q clock output of the PLL is not used, the value of PLLQEN should be 0."]
    #[inline(always)]
    pub fn set_pllqen(&mut self, val: super::vals::Pllqen) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Main PLL division factor for PLL Q clock. Set and cleared by software to control the frequency of the main PLL output clock PLL Q clock. This output can be selected for USB, RNG, SAI (48 MHz clock). These bits can be written only if PLL is disabled. PLL Q output clock frequency = VCO frequency / PLLQ with PLLQ = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
    #[inline(always)]
    pub const fn pllq(&self) -> super::vals::Pllq {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Pllq::from_bits(val as u8)
    }
    #[doc = "Main PLL division factor for PLL Q clock. Set and cleared by software to control the frequency of the main PLL output clock PLL Q clock. This output can be selected for USB, RNG, SAI (48 MHz clock). These bits can be written only if PLL is disabled. PLL Q output clock frequency = VCO frequency / PLLQ with PLLQ = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
    #[inline(always)]
    pub fn set_pllq(&mut self, val: super::vals::Pllq) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "PLL R clock output enable Set and reset by software to enable the PLL R clock output of the PLL (used as system clock). This bit cannot be written when PLL R clock output of the PLL is used as System Clock. In order to save power, when the PLL R clock output of the PLL is not used, the value of PLLREN should be 0."]
    #[inline(always)]
    pub const fn pllren(&self) -> super::vals::Pllren {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pllren::from_bits(val as u8)
    }
    #[doc = "PLL R clock output enable Set and reset by software to enable the PLL R clock output of the PLL (used as system clock). This bit cannot be written when PLL R clock output of the PLL is used as System Clock. In order to save power, when the PLL R clock output of the PLL is not used, the value of PLLREN should be 0."]
    #[inline(always)]
    pub fn set_pllren(&mut self, val: super::vals::Pllren) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Main PLL division factor for PLL R clock (system clock) Set and cleared by software to control the frequency of the main PLL output clock PLLCLK. This output can be selected as system clock. These bits can be written only if PLL is disabled. PLL R output clock frequency = VCO frequency / PLLR with PLLR = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
    #[inline(always)]
    pub const fn pllr(&self) -> super::vals::Pllr {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::Pllr::from_bits(val as u8)
    }
    #[doc = "Main PLL division factor for PLL R clock (system clock) Set and cleared by software to control the frequency of the main PLL output clock PLLCLK. This output can be selected as system clock. These bits can be written only if PLL is disabled. PLL R output clock frequency = VCO frequency / PLLR with PLLR = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain."]
    #[inline(always)]
    pub fn set_pllr(&mut self, val: super::vals::Pllr) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
    #[doc = "Main PLLP division factor Set and cleared by software to control the PLL P frequency. PLL P output clock frequency = VCO frequency / PLLPDIV. ...."]
    #[inline(always)]
    pub const fn pllpdiv(&self) -> super::vals::Pllpdiv {
        let val = (self.0 >> 27usize) & 0x1f;
        super::vals::Pllpdiv::from_bits(val as u8)
    }
    #[doc = "Main PLLP division factor Set and cleared by software to control the PLL P frequency. PLL P output clock frequency = VCO frequency / PLLPDIV. ...."]
    #[inline(always)]
    pub fn set_pllpdiv(&mut self, val: super::vals::Pllpdiv) {
        self.0 = (self.0 & !(0x1f << 27usize)) | (((val.to_bits() as u32) & 0x1f) << 27usize);
    }
}
impl Default for RccPllcfgr {
    #[inline(always)]
    fn default() -> RccPllcfgr {
        RccPllcfgr(0)
    }
}

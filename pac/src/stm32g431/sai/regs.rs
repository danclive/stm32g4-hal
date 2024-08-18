#[doc = "AClear flag register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aclrfr(pub u32);
impl Aclrfr {
    #[doc = "Clear overrun / underrun"]
    #[inline(always)]
    pub const fn ovrudr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear overrun / underrun"]
    #[inline(always)]
    pub fn set_ovrudr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mute detection flag"]
    #[inline(always)]
    pub const fn mutedet(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mute detection flag"]
    #[inline(always)]
    pub fn set_mutedet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Clear wrong clock configuration flag"]
    #[inline(always)]
    pub const fn wckcfg(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Clear wrong clock configuration flag"]
    #[inline(always)]
    pub fn set_wckcfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear codec not ready flag"]
    #[inline(always)]
    pub const fn cnrdy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Clear codec not ready flag"]
    #[inline(always)]
    pub fn set_cnrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Clear anticipated frame synchronization detection flag"]
    #[inline(always)]
    pub const fn cafsdet(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Clear anticipated frame synchronization detection flag"]
    #[inline(always)]
    pub fn set_cafsdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Clear late frame synchronization detection flag"]
    #[inline(always)]
    pub const fn lfsdet(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Clear late frame synchronization detection flag"]
    #[inline(always)]
    pub fn set_lfsdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Aclrfr {
    #[inline(always)]
    fn default() -> Aclrfr {
        Aclrfr(0)
    }
}
#[doc = "AConfiguration register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acr1(pub u32);
impl Acr1 {
    #[doc = "Audio block mode"]
    #[inline(always)]
    pub const fn mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Audio block mode"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Protocol configuration"]
    #[inline(always)]
    pub const fn prtcfg(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Protocol configuration"]
    #[inline(always)]
    pub fn set_prtcfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Data size"]
    #[inline(always)]
    pub const fn ds(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Data size"]
    #[inline(always)]
    pub fn set_ds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Least significant bit first"]
    #[inline(always)]
    pub const fn lsbfirst(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Least significant bit first"]
    #[inline(always)]
    pub fn set_lsbfirst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Clock strobing edge"]
    #[inline(always)]
    pub const fn ckstr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Clock strobing edge"]
    #[inline(always)]
    pub fn set_ckstr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Synchronization enable"]
    #[inline(always)]
    pub const fn syncen(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Synchronization enable"]
    #[inline(always)]
    pub fn set_syncen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Mono mode"]
    #[inline(always)]
    pub const fn mono(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Mono mode"]
    #[inline(always)]
    pub fn set_mono(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Output drive"]
    #[inline(always)]
    pub const fn out_dri(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Output drive"]
    #[inline(always)]
    pub fn set_out_dri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Audio block A enable"]
    #[inline(always)]
    pub const fn saiaen(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Audio block A enable"]
    #[inline(always)]
    pub fn set_saiaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA enable"]
    #[inline(always)]
    pub const fn dmaen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA enable"]
    #[inline(always)]
    pub fn set_dmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "No divider"]
    #[inline(always)]
    pub const fn nodiv(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "No divider"]
    #[inline(always)]
    pub fn set_nodiv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Master clock divider"]
    #[inline(always)]
    pub const fn mcjdiv(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x3f;
        val as u8
    }
    #[doc = "Master clock divider"]
    #[inline(always)]
    pub fn set_mcjdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 20usize)) | (((val as u32) & 0x3f) << 20usize);
    }
    #[doc = "OSR"]
    #[inline(always)]
    pub const fn osr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "OSR"]
    #[inline(always)]
    pub fn set_osr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "MCKEN"]
    #[inline(always)]
    pub const fn mcken(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "MCKEN"]
    #[inline(always)]
    pub fn set_mcken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Acr1 {
    #[inline(always)]
    fn default() -> Acr1 {
        Acr1(0)
    }
}
#[doc = "AConfiguration register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acr2(pub u32);
impl Acr2 {
    #[doc = "FIFO threshold"]
    #[inline(always)]
    pub const fn fth(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "FIFO threshold"]
    #[inline(always)]
    pub fn set_fth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "FIFO flush"]
    #[inline(always)]
    pub const fn fflus(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO flush"]
    #[inline(always)]
    pub fn set_fflus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Tristate management on data line"]
    #[inline(always)]
    pub const fn tris(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Tristate management on data line"]
    #[inline(always)]
    pub fn set_tris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Mute"]
    #[inline(always)]
    pub const fn mute(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Mute"]
    #[inline(always)]
    pub fn set_mute(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Mute value"]
    #[inline(always)]
    pub const fn muteval(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Mute value"]
    #[inline(always)]
    pub fn set_muteval(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Mute counter"]
    #[inline(always)]
    pub const fn mutecn(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x3f;
        val as u8
    }
    #[doc = "Mute counter"]
    #[inline(always)]
    pub fn set_mutecn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 7usize)) | (((val as u32) & 0x3f) << 7usize);
    }
    #[doc = "Complement bit"]
    #[inline(always)]
    pub const fn cpl(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Complement bit"]
    #[inline(always)]
    pub fn set_cpl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Companding mode"]
    #[inline(always)]
    pub const fn comp(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Companding mode"]
    #[inline(always)]
    pub fn set_comp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
}
impl Default for Acr2 {
    #[inline(always)]
    fn default() -> Acr2 {
        Acr2(0)
    }
}
#[doc = "AData register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adr(pub u32);
impl Adr {
    #[doc = "Data"]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Adr {
    #[inline(always)]
    fn default() -> Adr {
        Adr(0)
    }
}
#[doc = "AFRCR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Afrcr(pub u32);
impl Afrcr {
    #[doc = "Frame length"]
    #[inline(always)]
    pub const fn frl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Frame length"]
    #[inline(always)]
    pub fn set_frl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Frame synchronization active level length"]
    #[inline(always)]
    pub const fn fsall(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Frame synchronization active level length"]
    #[inline(always)]
    pub fn set_fsall(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Frame synchronization definition"]
    #[inline(always)]
    pub const fn fsdef(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Frame synchronization definition"]
    #[inline(always)]
    pub fn set_fsdef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Frame synchronization polarity"]
    #[inline(always)]
    pub const fn fspol(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Frame synchronization polarity"]
    #[inline(always)]
    pub fn set_fspol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Frame synchronization offset"]
    #[inline(always)]
    pub const fn fsoff(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Frame synchronization offset"]
    #[inline(always)]
    pub fn set_fsoff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Afrcr {
    #[inline(always)]
    fn default() -> Afrcr {
        Afrcr(0)
    }
}
#[doc = "AInterrupt mask register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aim(pub u32);
impl Aim {
    #[doc = "Overrun/underrun interrupt enable"]
    #[inline(always)]
    pub const fn ovrudrie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun/underrun interrupt enable"]
    #[inline(always)]
    pub fn set_ovrudrie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mute detection interrupt enable"]
    #[inline(always)]
    pub const fn mutedet(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mute detection interrupt enable"]
    #[inline(always)]
    pub fn set_mutedet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Wrong clock configuration interrupt enable"]
    #[inline(always)]
    pub const fn wckcfg(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wrong clock configuration interrupt enable"]
    #[inline(always)]
    pub fn set_wckcfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FIFO request interrupt enable"]
    #[inline(always)]
    pub const fn freqie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO request interrupt enable"]
    #[inline(always)]
    pub fn set_freqie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Codec not ready interrupt enable"]
    #[inline(always)]
    pub const fn cnrdyie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Codec not ready interrupt enable"]
    #[inline(always)]
    pub fn set_cnrdyie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Anticipated frame synchronization detection interrupt enable"]
    #[inline(always)]
    pub const fn afsdetie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Anticipated frame synchronization detection interrupt enable"]
    #[inline(always)]
    pub fn set_afsdetie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Late frame synchronization detection interrupt enable"]
    #[inline(always)]
    pub const fn lfsdet(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Late frame synchronization detection interrupt enable"]
    #[inline(always)]
    pub fn set_lfsdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Aim {
    #[inline(always)]
    fn default() -> Aim {
        Aim(0)
    }
}
#[doc = "ASlot register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aslotr(pub u32);
impl Aslotr {
    #[doc = "First bit offset"]
    #[inline(always)]
    pub const fn fboff(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "First bit offset"]
    #[inline(always)]
    pub fn set_fboff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Slot size"]
    #[inline(always)]
    pub const fn slotsz(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Slot size"]
    #[inline(always)]
    pub fn set_slotsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Number of slots in an audio frame"]
    #[inline(always)]
    pub const fn nbslot(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of slots in an audio frame"]
    #[inline(always)]
    pub fn set_nbslot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Slot enable"]
    #[inline(always)]
    pub const fn sloten(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Slot enable"]
    #[inline(always)]
    pub fn set_sloten(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Aslotr {
    #[inline(always)]
    fn default() -> Aslotr {
        Aslotr(0)
    }
}
#[doc = "AStatus register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asr(pub u32);
impl Asr {
    #[doc = "Overrun / underrun"]
    #[inline(always)]
    pub const fn ovrudr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun / underrun"]
    #[inline(always)]
    pub fn set_ovrudr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mute detection"]
    #[inline(always)]
    pub const fn mutedet(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mute detection"]
    #[inline(always)]
    pub fn set_mutedet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Wrong clock configuration flag. This bit is read only"]
    #[inline(always)]
    pub const fn wckcfg(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wrong clock configuration flag. This bit is read only"]
    #[inline(always)]
    pub fn set_wckcfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FIFO request"]
    #[inline(always)]
    pub const fn freq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO request"]
    #[inline(always)]
    pub fn set_freq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Codec not ready"]
    #[inline(always)]
    pub const fn cnrdy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Codec not ready"]
    #[inline(always)]
    pub fn set_cnrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Anticipated frame synchronization detection"]
    #[inline(always)]
    pub const fn afsdet(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Anticipated frame synchronization detection"]
    #[inline(always)]
    pub fn set_afsdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Late frame synchronization detection"]
    #[inline(always)]
    pub const fn lfsdet(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Late frame synchronization detection"]
    #[inline(always)]
    pub fn set_lfsdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FIFO level threshold"]
    #[inline(always)]
    pub const fn flvl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "FIFO level threshold"]
    #[inline(always)]
    pub fn set_flvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for Asr {
    #[inline(always)]
    fn default() -> Asr {
        Asr(0)
    }
}
#[doc = "BClear flag register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bclrfr(pub u32);
impl Bclrfr {
    #[doc = "Clear overrun / underrun"]
    #[inline(always)]
    pub const fn ovrudr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear overrun / underrun"]
    #[inline(always)]
    pub fn set_ovrudr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mute detection flag"]
    #[inline(always)]
    pub const fn mutedet(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mute detection flag"]
    #[inline(always)]
    pub fn set_mutedet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Clear wrong clock configuration flag"]
    #[inline(always)]
    pub const fn wckcfg(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Clear wrong clock configuration flag"]
    #[inline(always)]
    pub fn set_wckcfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear codec not ready flag"]
    #[inline(always)]
    pub const fn cnrdy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Clear codec not ready flag"]
    #[inline(always)]
    pub fn set_cnrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Clear anticipated frame synchronization detection flag"]
    #[inline(always)]
    pub const fn cafsdet(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Clear anticipated frame synchronization detection flag"]
    #[inline(always)]
    pub fn set_cafsdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Clear late frame synchronization detection flag"]
    #[inline(always)]
    pub const fn lfsdet(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Clear late frame synchronization detection flag"]
    #[inline(always)]
    pub fn set_lfsdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Bclrfr {
    #[inline(always)]
    fn default() -> Bclrfr {
        Bclrfr(0)
    }
}
#[doc = "BConfiguration register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcr1(pub u32);
impl Bcr1 {
    #[doc = "Audio block mode"]
    #[inline(always)]
    pub const fn mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Audio block mode"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Protocol configuration"]
    #[inline(always)]
    pub const fn prtcfg(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Protocol configuration"]
    #[inline(always)]
    pub fn set_prtcfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Data size"]
    #[inline(always)]
    pub const fn ds(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Data size"]
    #[inline(always)]
    pub fn set_ds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Least significant bit first"]
    #[inline(always)]
    pub const fn lsbfirst(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Least significant bit first"]
    #[inline(always)]
    pub fn set_lsbfirst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Clock strobing edge"]
    #[inline(always)]
    pub const fn ckstr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Clock strobing edge"]
    #[inline(always)]
    pub fn set_ckstr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Synchronization enable"]
    #[inline(always)]
    pub const fn syncen(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Synchronization enable"]
    #[inline(always)]
    pub fn set_syncen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Mono mode"]
    #[inline(always)]
    pub const fn mono(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Mono mode"]
    #[inline(always)]
    pub fn set_mono(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Output drive"]
    #[inline(always)]
    pub const fn out_dri(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Output drive"]
    #[inline(always)]
    pub fn set_out_dri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Audio block B enable"]
    #[inline(always)]
    pub const fn saiben(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Audio block B enable"]
    #[inline(always)]
    pub fn set_saiben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA enable"]
    #[inline(always)]
    pub const fn dmaen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA enable"]
    #[inline(always)]
    pub fn set_dmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "No divider"]
    #[inline(always)]
    pub const fn nodiv(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "No divider"]
    #[inline(always)]
    pub fn set_nodiv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Master clock divider"]
    #[inline(always)]
    pub const fn mcjdiv(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x3f;
        val as u8
    }
    #[doc = "Master clock divider"]
    #[inline(always)]
    pub fn set_mcjdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 20usize)) | (((val as u32) & 0x3f) << 20usize);
    }
    #[doc = "OSR"]
    #[inline(always)]
    pub const fn osr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "OSR"]
    #[inline(always)]
    pub fn set_osr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "MCKEN"]
    #[inline(always)]
    pub const fn mcken(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "MCKEN"]
    #[inline(always)]
    pub fn set_mcken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Bcr1 {
    #[inline(always)]
    fn default() -> Bcr1 {
        Bcr1(0)
    }
}
#[doc = "BConfiguration register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcr2(pub u32);
impl Bcr2 {
    #[doc = "FIFO threshold"]
    #[inline(always)]
    pub const fn fth(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "FIFO threshold"]
    #[inline(always)]
    pub fn set_fth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "FIFO flush"]
    #[inline(always)]
    pub const fn fflus(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO flush"]
    #[inline(always)]
    pub fn set_fflus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Tristate management on data line"]
    #[inline(always)]
    pub const fn tris(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Tristate management on data line"]
    #[inline(always)]
    pub fn set_tris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Mute"]
    #[inline(always)]
    pub const fn mute(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Mute"]
    #[inline(always)]
    pub fn set_mute(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Mute value"]
    #[inline(always)]
    pub const fn muteval(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Mute value"]
    #[inline(always)]
    pub fn set_muteval(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Mute counter"]
    #[inline(always)]
    pub const fn mutecn(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x3f;
        val as u8
    }
    #[doc = "Mute counter"]
    #[inline(always)]
    pub fn set_mutecn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 7usize)) | (((val as u32) & 0x3f) << 7usize);
    }
    #[doc = "Complement bit"]
    #[inline(always)]
    pub const fn cpl(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Complement bit"]
    #[inline(always)]
    pub fn set_cpl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Companding mode"]
    #[inline(always)]
    pub const fn comp(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Companding mode"]
    #[inline(always)]
    pub fn set_comp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
}
impl Default for Bcr2 {
    #[inline(always)]
    fn default() -> Bcr2 {
        Bcr2(0)
    }
}
#[doc = "BData register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bdr(pub u32);
impl Bdr {
    #[doc = "Data"]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bdr {
    #[inline(always)]
    fn default() -> Bdr {
        Bdr(0)
    }
}
#[doc = "BFRCR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfrcr(pub u32);
impl Bfrcr {
    #[doc = "Frame length"]
    #[inline(always)]
    pub const fn frl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Frame length"]
    #[inline(always)]
    pub fn set_frl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Frame synchronization active level length"]
    #[inline(always)]
    pub const fn fsall(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Frame synchronization active level length"]
    #[inline(always)]
    pub fn set_fsall(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Frame synchronization definition"]
    #[inline(always)]
    pub const fn fsdef(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Frame synchronization definition"]
    #[inline(always)]
    pub fn set_fsdef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Frame synchronization polarity"]
    #[inline(always)]
    pub const fn fspol(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Frame synchronization polarity"]
    #[inline(always)]
    pub fn set_fspol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Frame synchronization offset"]
    #[inline(always)]
    pub const fn fsoff(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Frame synchronization offset"]
    #[inline(always)]
    pub fn set_fsoff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Bfrcr {
    #[inline(always)]
    fn default() -> Bfrcr {
        Bfrcr(0)
    }
}
#[doc = "BInterrupt mask register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bim(pub u32);
impl Bim {
    #[doc = "Overrun/underrun interrupt enable"]
    #[inline(always)]
    pub const fn ovrudrie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun/underrun interrupt enable"]
    #[inline(always)]
    pub fn set_ovrudrie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mute detection interrupt enable"]
    #[inline(always)]
    pub const fn mutedet(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mute detection interrupt enable"]
    #[inline(always)]
    pub fn set_mutedet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Wrong clock configuration interrupt enable"]
    #[inline(always)]
    pub const fn wckcfg(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wrong clock configuration interrupt enable"]
    #[inline(always)]
    pub fn set_wckcfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FIFO request interrupt enable"]
    #[inline(always)]
    pub const fn freqie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO request interrupt enable"]
    #[inline(always)]
    pub fn set_freqie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Codec not ready interrupt enable"]
    #[inline(always)]
    pub const fn cnrdyie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Codec not ready interrupt enable"]
    #[inline(always)]
    pub fn set_cnrdyie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Anticipated frame synchronization detection interrupt enable"]
    #[inline(always)]
    pub const fn afsdetie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Anticipated frame synchronization detection interrupt enable"]
    #[inline(always)]
    pub fn set_afsdetie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Late frame synchronization detection interrupt enable"]
    #[inline(always)]
    pub const fn lfsdetie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Late frame synchronization detection interrupt enable"]
    #[inline(always)]
    pub fn set_lfsdetie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Bim {
    #[inline(always)]
    fn default() -> Bim {
        Bim(0)
    }
}
#[doc = "BSlot register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bslotr(pub u32);
impl Bslotr {
    #[doc = "First bit offset"]
    #[inline(always)]
    pub const fn fboff(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "First bit offset"]
    #[inline(always)]
    pub fn set_fboff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Slot size"]
    #[inline(always)]
    pub const fn slotsz(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Slot size"]
    #[inline(always)]
    pub fn set_slotsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Number of slots in an audio frame"]
    #[inline(always)]
    pub const fn nbslot(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of slots in an audio frame"]
    #[inline(always)]
    pub fn set_nbslot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Slot enable"]
    #[inline(always)]
    pub const fn sloten(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Slot enable"]
    #[inline(always)]
    pub fn set_sloten(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Bslotr {
    #[inline(always)]
    fn default() -> Bslotr {
        Bslotr(0)
    }
}
#[doc = "BStatus register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bsr(pub u32);
impl Bsr {
    #[doc = "Overrun / underrun"]
    #[inline(always)]
    pub const fn ovrudr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun / underrun"]
    #[inline(always)]
    pub fn set_ovrudr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mute detection"]
    #[inline(always)]
    pub const fn mutedet(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mute detection"]
    #[inline(always)]
    pub fn set_mutedet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Wrong clock configuration flag"]
    #[inline(always)]
    pub const fn wckcfg(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wrong clock configuration flag"]
    #[inline(always)]
    pub fn set_wckcfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FIFO request"]
    #[inline(always)]
    pub const fn freq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO request"]
    #[inline(always)]
    pub fn set_freq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Codec not ready"]
    #[inline(always)]
    pub const fn cnrdy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Codec not ready"]
    #[inline(always)]
    pub fn set_cnrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Anticipated frame synchronization detection"]
    #[inline(always)]
    pub const fn afsdet(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Anticipated frame synchronization detection"]
    #[inline(always)]
    pub fn set_afsdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Late frame synchronization detection"]
    #[inline(always)]
    pub const fn lfsdet(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Late frame synchronization detection"]
    #[inline(always)]
    pub fn set_lfsdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FIFO level threshold"]
    #[inline(always)]
    pub const fn flvl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "FIFO level threshold"]
    #[inline(always)]
    pub fn set_flvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for Bsr {
    #[inline(always)]
    fn default() -> Bsr {
        Bsr(0)
    }
}
#[doc = "PDM control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdmcr(pub u32);
impl Pdmcr {
    #[doc = "PDMEN"]
    #[inline(always)]
    pub const fn pdmen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PDMEN"]
    #[inline(always)]
    pub fn set_pdmen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "MICNBR"]
    #[inline(always)]
    pub const fn micnbr(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "MICNBR"]
    #[inline(always)]
    pub fn set_micnbr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "CKEN1"]
    #[inline(always)]
    pub const fn cken1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "CKEN1"]
    #[inline(always)]
    pub fn set_cken1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CKEN2"]
    #[inline(always)]
    pub const fn cken2(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CKEN2"]
    #[inline(always)]
    pub fn set_cken2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CKEN3"]
    #[inline(always)]
    pub const fn cken3(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CKEN3"]
    #[inline(always)]
    pub fn set_cken3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "CKEN4"]
    #[inline(always)]
    pub const fn cken4(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CKEN4"]
    #[inline(always)]
    pub fn set_cken4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Pdmcr {
    #[inline(always)]
    fn default() -> Pdmcr {
        Pdmcr(0)
    }
}
#[doc = "PDM delay register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdmdly(pub u32);
impl Pdmdly {
    #[doc = "DLYM1L"]
    #[inline(always)]
    pub const fn dlym1l(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "DLYM1L"]
    #[inline(always)]
    pub fn set_dlym1l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "DLYM1R"]
    #[inline(always)]
    pub const fn dlym1r(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "DLYM1R"]
    #[inline(always)]
    pub fn set_dlym1r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "DLYM2L"]
    #[inline(always)]
    pub const fn dlym2l(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "DLYM2L"]
    #[inline(always)]
    pub fn set_dlym2l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "DLYM2R"]
    #[inline(always)]
    pub const fn dlym2r(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "DLYM2R"]
    #[inline(always)]
    pub fn set_dlym2r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "DLYM3L"]
    #[inline(always)]
    pub const fn dlym3l(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "DLYM3L"]
    #[inline(always)]
    pub fn set_dlym3l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "DLYM3R"]
    #[inline(always)]
    pub const fn dlym3r(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "DLYM3R"]
    #[inline(always)]
    pub fn set_dlym3r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "DLYM4L"]
    #[inline(always)]
    pub const fn dlym4l(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "DLYM4L"]
    #[inline(always)]
    pub fn set_dlym4l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "DLYM4R"]
    #[inline(always)]
    pub const fn dlym4r(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "DLYM4R"]
    #[inline(always)]
    pub fn set_dlym4r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for Pdmdly {
    #[inline(always)]
    fn default() -> Pdmdly {
        Pdmdly(0)
    }
}

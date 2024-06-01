#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    crel: Crel,
    endn: Endn,
    _reserved2: [u8; 0x04],
    dbtp: Dbtp,
    test: Test,
    rwd: Rwd,
    cccr: Cccr,
    nbtp: Nbtp,
    tscc: Tscc,
    tscv: Tscv,
    tocc: Tocc,
    tocv: Tocv,
    _reserved11: [u8; 0x10],
    ecr: Ecr,
    psr: Psr,
    tdcr: Tdcr,
    _reserved14: [u8; 0x04],
    ir: Ir,
    ie: Ie,
    ils: Ils,
    ile: Ile,
    _reserved18: [u8; 0x20],
    rxgfc: Rxgfc,
    xidam: Xidam,
    hpms: Hpms,
    _reserved21: [u8; 0x04],
    rxf0s: Rxf0s,
    rxf0a: Rxf0a,
    rxf1s: Rxf1s,
    rxf1a: Rxf1a,
    _reserved25: [u8; 0x20],
    txbc: Txbc,
    txfqs: Txfqs,
    txbrp: Txbrp,
    txbar: Txbar,
    txbcr: Txbcr,
    txbto: Txbto,
    txbcf: Txbcf,
    txbtie: Txbtie,
    txbcie: Txbcie,
    txefs: Txefs,
    txefa: Txefa,
    _reserved36: [u8; 0x14],
    ckdiv: Ckdiv,
}
impl RegisterBlock {
    #[doc = "0x00 - FDCAN Core Release Register"]
    #[inline(always)]
    pub const fn crel(&self) -> &Crel {
        &self.crel
    }
    #[doc = "0x04 - FDCAN Core Release Register"]
    #[inline(always)]
    pub const fn endn(&self) -> &Endn {
        &self.endn
    }
    #[doc = "0x0c - This register is only writable if bits CCCR.CCE and CCCR.INIT are set. The CAN bit time may be programed in the range of 4 to 25 time quanta. The CAN time quantum may be programmed in the range of 1 to 1024 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock period. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (programmed values) \\[DTSEG1 + DTSEG2 + 3\\]
tq or (functional values) \\[Sync_Seg + Prop_Seg + Phase_Seg1 + Phase_Seg2\\]
tq. The Information Processing Time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point."]
    #[inline(always)]
    pub const fn dbtp(&self) -> &Dbtp {
        &self.dbtp
    }
    #[doc = "0x10 - Write access to the Test Register has to be enabled by setting bit CCCR\\[TEST\\]
to 1 . All Test Register functions are set to their reset values when bit CCCR\\[TEST\\]
is reset. Loop Back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus."]
    #[inline(always)]
    pub const fn test(&self) -> &Test {
        &self.test
    }
    #[doc = "0x14 - The RAM Watchdog monitors the READY output of the Message RAM. A Message RAM access starts the Message RAM Watchdog Counter with the value configured by the RWD\\[WDC\\]
bits. The counter is reloaded with RWD\\[WDC\\]
bits when the Message RAM signals successful completion by activating its READY output. In case there is no response from the Message RAM until the counter has counted down to 0, the counter stops and interrupt flag IR\\[WDI\\]
bit is set. The RAM Watchdog Counter is clocked by the fdcan_pclk clock."]
    #[inline(always)]
    pub const fn rwd(&self) -> &Rwd {
        &self.rwd
    }
    #[doc = "0x18 - For details about setting and resetting of single bits see Software initialization."]
    #[inline(always)]
    pub const fn cccr(&self) -> &Cccr {
        &self.cccr
    }
    #[doc = "0x1c - FDCAN_NBTP"]
    #[inline(always)]
    pub const fn nbtp(&self) -> &Nbtp {
        &self.nbtp
    }
    #[doc = "0x20 - FDCAN Timestamp Counter Configuration Register"]
    #[inline(always)]
    pub const fn tscc(&self) -> &Tscc {
        &self.tscc
    }
    #[doc = "0x24 - FDCAN Timestamp Counter Value Register"]
    #[inline(always)]
    pub const fn tscv(&self) -> &Tscv {
        &self.tscv
    }
    #[doc = "0x28 - FDCAN Timeout Counter Configuration Register"]
    #[inline(always)]
    pub const fn tocc(&self) -> &Tocc {
        &self.tocc
    }
    #[doc = "0x2c - FDCAN Timeout Counter Value Register"]
    #[inline(always)]
    pub const fn tocv(&self) -> &Tocv {
        &self.tocv
    }
    #[doc = "0x40 - FDCAN Error Counter Register"]
    #[inline(always)]
    pub const fn ecr(&self) -> &Ecr {
        &self.ecr
    }
    #[doc = "0x44 - FDCAN Protocol Status Register"]
    #[inline(always)]
    pub const fn psr(&self) -> &Psr {
        &self.psr
    }
    #[doc = "0x48 - FDCAN Transmitter Delay Compensation Register"]
    #[inline(always)]
    pub const fn tdcr(&self) -> &Tdcr {
        &self.tdcr
    }
    #[doc = "0x50 - The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled."]
    #[inline(always)]
    pub const fn ir(&self) -> &Ir {
        &self.ir
    }
    #[doc = "0x54 - The settings in the Interrupt Enable register determine which status changes in the Interrupt Register will be signaled on an interrupt line."]
    #[inline(always)]
    pub const fn ie(&self) -> &Ie {
        &self.ie
    }
    #[doc = "0x58 - The Interrupt Line Select register assigns an interrupt generated by a specific interrupt flag from the Interrupt Register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via ILE\\[EINT0\\]
and ILE\\[EINT1\\]."]
    #[inline(always)]
    pub const fn ils(&self) -> &Ils {
        &self.ils
    }
    #[doc = "0x5c - Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1."]
    #[inline(always)]
    pub const fn ile(&self) -> &Ile {
        &self.ile
    }
    #[doc = "0x80 - Global settings for Message ID filtering. The Global Filter Configuration controls the filter path for standard and extended messages as described in Figure706: Standard Message ID filter path and Figure707: Extended Message ID filter path."]
    #[inline(always)]
    pub const fn rxgfc(&self) -> &Rxgfc {
        &self.rxgfc
    }
    #[doc = "0x84 - FDCAN Extended ID and Mask Register"]
    #[inline(always)]
    pub const fn xidam(&self) -> &Xidam {
        &self.xidam
    }
    #[doc = "0x88 - This register is updated every time a Message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages."]
    #[inline(always)]
    pub const fn hpms(&self) -> &Hpms {
        &self.hpms
    }
    #[doc = "0x90 - FDCAN Rx FIFO 0 Status Register"]
    #[inline(always)]
    pub const fn rxf0s(&self) -> &Rxf0s {
        &self.rxf0s
    }
    #[doc = "0x94 - CAN Rx FIFO 0 Acknowledge Register"]
    #[inline(always)]
    pub const fn rxf0a(&self) -> &Rxf0a {
        &self.rxf0a
    }
    #[doc = "0x98 - FDCAN Rx FIFO 1 Status Register"]
    #[inline(always)]
    pub const fn rxf1s(&self) -> &Rxf1s {
        &self.rxf1s
    }
    #[doc = "0x9c - FDCAN Rx FIFO 1 Acknowledge Register"]
    #[inline(always)]
    pub const fn rxf1a(&self) -> &Rxf1a {
        &self.rxf1a
    }
    #[doc = "0xc0 - FDCAN Tx Buffer Configuration Register"]
    #[inline(always)]
    pub const fn txbc(&self) -> &Txbc {
        &self.txbc
    }
    #[doc = "0xc4 - The Tx FIFO/Queue status is related to the pending Tx requests listed in register TXBRP. Therefore the effect of Add/Cancellation requests may be delayed due to a running Tx scan (TXBRP not yet updated)."]
    #[inline(always)]
    pub const fn txfqs(&self) -> &Txfqs {
        &self.txfqs
    }
    #[doc = "0xc8 - FDCAN Tx Buffer Request Pending Register"]
    #[inline(always)]
    pub const fn txbrp(&self) -> &Txbrp {
        &self.txbrp
    }
    #[doc = "0xcc - FDCAN Tx Buffer Add Request Register"]
    #[inline(always)]
    pub const fn txbar(&self) -> &Txbar {
        &self.txbar
    }
    #[doc = "0xd0 - FDCAN Tx Buffer Cancellation Request Register"]
    #[inline(always)]
    pub const fn txbcr(&self) -> &Txbcr {
        &self.txbcr
    }
    #[doc = "0xd4 - FDCAN Tx Buffer Transmission Occurred Register"]
    #[inline(always)]
    pub const fn txbto(&self) -> &Txbto {
        &self.txbto
    }
    #[doc = "0xd8 - FDCAN Tx Buffer Cancellation Finished Register"]
    #[inline(always)]
    pub const fn txbcf(&self) -> &Txbcf {
        &self.txbcf
    }
    #[doc = "0xdc - FDCAN Tx Buffer Transmission Interrupt Enable Register"]
    #[inline(always)]
    pub const fn txbtie(&self) -> &Txbtie {
        &self.txbtie
    }
    #[doc = "0xe0 - FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
    #[inline(always)]
    pub const fn txbcie(&self) -> &Txbcie {
        &self.txbcie
    }
    #[doc = "0xe4 - FDCAN Tx Event FIFO Status Register"]
    #[inline(always)]
    pub const fn txefs(&self) -> &Txefs {
        &self.txefs
    }
    #[doc = "0xe8 - FDCAN Tx Event FIFO Acknowledge Register"]
    #[inline(always)]
    pub const fn txefa(&self) -> &Txefa {
        &self.txefa
    }
    #[doc = "0x100 - FDCAN CFG clock divider register"]
    #[inline(always)]
    pub const fn ckdiv(&self) -> &Ckdiv {
        &self.ckdiv
    }
}
#[doc = "CREL (r) register accessor: FDCAN Core Release Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crel::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crel`]
module"]
#[doc(alias = "CREL")]
pub type Crel = crate::Reg<crel::CrelSpec>;
#[doc = "FDCAN Core Release Register"]
pub mod crel;
#[doc = "ENDN (r) register accessor: FDCAN Core Release Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endn`]
module"]
#[doc(alias = "ENDN")]
pub type Endn = crate::Reg<endn::EndnSpec>;
#[doc = "FDCAN Core Release Register"]
pub mod endn;
#[doc = "DBTP (rw) register accessor: This register is only writable if bits CCCR.CCE and CCCR.INIT are set. The CAN bit time may be programed in the range of 4 to 25 time quanta. The CAN time quantum may be programmed in the range of 1 to 1024 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock period. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (programmed values) \\[DTSEG1 + DTSEG2 + 3\\]
tq or (functional values) \\[Sync_Seg + Prop_Seg + Phase_Seg1 + Phase_Seg2\\]
tq. The Information Processing Time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbtp`]
module"]
#[doc(alias = "DBTP")]
pub type Dbtp = crate::Reg<dbtp::DbtpSpec>;
#[doc = "This register is only writable if bits CCCR.CCE and CCCR.INIT are set. The CAN bit time may be programed in the range of 4 to 25 time quanta. The CAN time quantum may be programmed in the range of 1 to 1024 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock period. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (programmed values) \\[DTSEG1 + DTSEG2 + 3\\]
tq or (functional values) \\[Sync_Seg + Prop_Seg + Phase_Seg1 + Phase_Seg2\\]
tq. The Information Processing Time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point."]
pub mod dbtp;
#[doc = "TEST (rw) register accessor: Write access to the Test Register has to be enabled by setting bit CCCR\\[TEST\\]
to 1 . All Test Register functions are set to their reset values when bit CCCR\\[TEST\\]
is reset. Loop Back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`]
module"]
#[doc(alias = "TEST")]
pub type Test = crate::Reg<test::TestSpec>;
#[doc = "Write access to the Test Register has to be enabled by setting bit CCCR\\[TEST\\]
to 1 . All Test Register functions are set to their reset values when bit CCCR\\[TEST\\]
is reset. Loop Back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus."]
pub mod test;
#[doc = "RWD (rw) register accessor: The RAM Watchdog monitors the READY output of the Message RAM. A Message RAM access starts the Message RAM Watchdog Counter with the value configured by the RWD\\[WDC\\]
bits. The counter is reloaded with RWD\\[WDC\\]
bits when the Message RAM signals successful completion by activating its READY output. In case there is no response from the Message RAM until the counter has counted down to 0, the counter stops and interrupt flag IR\\[WDI\\]
bit is set. The RAM Watchdog Counter is clocked by the fdcan_pclk clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwd`]
module"]
#[doc(alias = "RWD")]
pub type Rwd = crate::Reg<rwd::RwdSpec>;
#[doc = "The RAM Watchdog monitors the READY output of the Message RAM. A Message RAM access starts the Message RAM Watchdog Counter with the value configured by the RWD\\[WDC\\]
bits. The counter is reloaded with RWD\\[WDC\\]
bits when the Message RAM signals successful completion by activating its READY output. In case there is no response from the Message RAM until the counter has counted down to 0, the counter stops and interrupt flag IR\\[WDI\\]
bit is set. The RAM Watchdog Counter is clocked by the fdcan_pclk clock."]
pub mod rwd;
#[doc = "CCCR (rw) register accessor: For details about setting and resetting of single bits see Software initialization.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccr`]
module"]
#[doc(alias = "CCCR")]
pub type Cccr = crate::Reg<cccr::CccrSpec>;
#[doc = "For details about setting and resetting of single bits see Software initialization."]
pub mod cccr;
#[doc = "NBTP (rw) register accessor: FDCAN_NBTP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nbtp`]
module"]
#[doc(alias = "NBTP")]
pub type Nbtp = crate::Reg<nbtp::NbtpSpec>;
#[doc = "FDCAN_NBTP"]
pub mod nbtp;
#[doc = "TSCC (rw) register accessor: FDCAN Timestamp Counter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscc`]
module"]
#[doc(alias = "TSCC")]
pub type Tscc = crate::Reg<tscc::TsccSpec>;
#[doc = "FDCAN Timestamp Counter Configuration Register"]
pub mod tscc;
#[doc = "TSCV (r) register accessor: FDCAN Timestamp Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscv::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscv`]
module"]
#[doc(alias = "TSCV")]
pub type Tscv = crate::Reg<tscv::TscvSpec>;
#[doc = "FDCAN Timestamp Counter Value Register"]
pub mod tscv;
#[doc = "TOCC (rw) register accessor: FDCAN Timeout Counter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocc`]
module"]
#[doc(alias = "TOCC")]
pub type Tocc = crate::Reg<tocc::ToccSpec>;
#[doc = "FDCAN Timeout Counter Configuration Register"]
pub mod tocc;
#[doc = "TOCV (r) register accessor: FDCAN Timeout Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocv::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocv`]
module"]
#[doc(alias = "TOCV")]
pub type Tocv = crate::Reg<tocv::TocvSpec>;
#[doc = "FDCAN Timeout Counter Value Register"]
pub mod tocv;
#[doc = "ECR (r) register accessor: FDCAN Error Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`]
module"]
#[doc(alias = "ECR")]
pub type Ecr = crate::Reg<ecr::EcrSpec>;
#[doc = "FDCAN Error Counter Register"]
pub mod ecr;
#[doc = "PSR (rw) register accessor: FDCAN Protocol Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`]
module"]
#[doc(alias = "PSR")]
pub type Psr = crate::Reg<psr::PsrSpec>;
#[doc = "FDCAN Protocol Status Register"]
pub mod psr;
#[doc = "TDCR (rw) register accessor: FDCAN Transmitter Delay Compensation Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdcr`]
module"]
#[doc(alias = "TDCR")]
pub type Tdcr = crate::Reg<tdcr::TdcrSpec>;
#[doc = "FDCAN Transmitter Delay Compensation Register"]
pub mod tdcr;
#[doc = "IR (rw) register accessor: The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`]
module"]
#[doc(alias = "IR")]
pub type Ir = crate::Reg<ir::IrSpec>;
#[doc = "The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled."]
pub mod ir;
#[doc = "IE (rw) register accessor: The settings in the Interrupt Enable register determine which status changes in the Interrupt Register will be signaled on an interrupt line.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
#[doc(alias = "IE")]
pub type Ie = crate::Reg<ie::IeSpec>;
#[doc = "The settings in the Interrupt Enable register determine which status changes in the Interrupt Register will be signaled on an interrupt line."]
pub mod ie;
#[doc = "ILS (rw) register accessor: The Interrupt Line Select register assigns an interrupt generated by a specific interrupt flag from the Interrupt Register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via ILE\\[EINT0\\]
and ILE\\[EINT1\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ils::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ils::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ils`]
module"]
#[doc(alias = "ILS")]
pub type Ils = crate::Reg<ils::IlsSpec>;
#[doc = "The Interrupt Line Select register assigns an interrupt generated by a specific interrupt flag from the Interrupt Register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via ILE\\[EINT0\\]
and ILE\\[EINT1\\]."]
pub mod ils;
#[doc = "ILE (rw) register accessor: Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ile::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ile::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ile`]
module"]
#[doc(alias = "ILE")]
pub type Ile = crate::Reg<ile::IleSpec>;
#[doc = "Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1."]
pub mod ile;
#[doc = "RXGFC (rw) register accessor: Global settings for Message ID filtering. The Global Filter Configuration controls the filter path for standard and extended messages as described in Figure706: Standard Message ID filter path and Figure707: Extended Message ID filter path.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxgfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxgfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxgfc`]
module"]
#[doc(alias = "RXGFC")]
pub type Rxgfc = crate::Reg<rxgfc::RxgfcSpec>;
#[doc = "Global settings for Message ID filtering. The Global Filter Configuration controls the filter path for standard and extended messages as described in Figure706: Standard Message ID filter path and Figure707: Extended Message ID filter path."]
pub mod rxgfc;
#[doc = "XIDAM (rw) register accessor: FDCAN Extended ID and Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xidam::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xidam::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xidam`]
module"]
#[doc(alias = "XIDAM")]
pub type Xidam = crate::Reg<xidam::XidamSpec>;
#[doc = "FDCAN Extended ID and Mask Register"]
pub mod xidam;
#[doc = "HPMS (r) register accessor: This register is updated every time a Message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpms`]
module"]
#[doc(alias = "HPMS")]
pub type Hpms = crate::Reg<hpms::HpmsSpec>;
#[doc = "This register is updated every time a Message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages."]
pub mod hpms;
#[doc = "RXF0S (r) register accessor: FDCAN Rx FIFO 0 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0s::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0s`]
module"]
#[doc(alias = "RXF0S")]
pub type Rxf0s = crate::Reg<rxf0s::Rxf0sSpec>;
#[doc = "FDCAN Rx FIFO 0 Status Register"]
pub mod rxf0s;
#[doc = "RXF0A (rw) register accessor: CAN Rx FIFO 0 Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf0a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0a`]
module"]
#[doc(alias = "RXF0A")]
pub type Rxf0a = crate::Reg<rxf0a::Rxf0aSpec>;
#[doc = "CAN Rx FIFO 0 Acknowledge Register"]
pub mod rxf0a;
#[doc = "RXF1S (r) register accessor: FDCAN Rx FIFO 1 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1s::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1s`]
module"]
#[doc(alias = "RXF1S")]
pub type Rxf1s = crate::Reg<rxf1s::Rxf1sSpec>;
#[doc = "FDCAN Rx FIFO 1 Status Register"]
pub mod rxf1s;
#[doc = "RXF1A (rw) register accessor: FDCAN Rx FIFO 1 Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1a`]
module"]
#[doc(alias = "RXF1A")]
pub type Rxf1a = crate::Reg<rxf1a::Rxf1aSpec>;
#[doc = "FDCAN Rx FIFO 1 Acknowledge Register"]
pub mod rxf1a;
#[doc = "TXBC (rw) register accessor: FDCAN Tx Buffer Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbc`]
module"]
#[doc(alias = "TXBC")]
pub type Txbc = crate::Reg<txbc::TxbcSpec>;
#[doc = "FDCAN Tx Buffer Configuration Register"]
pub mod txbc;
#[doc = "TXFQS (r) register accessor: The Tx FIFO/Queue status is related to the pending Tx requests listed in register TXBRP. Therefore the effect of Add/Cancellation requests may be delayed due to a running Tx scan (TXBRP not yet updated).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfqs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfqs`]
module"]
#[doc(alias = "TXFQS")]
pub type Txfqs = crate::Reg<txfqs::TxfqsSpec>;
#[doc = "The Tx FIFO/Queue status is related to the pending Tx requests listed in register TXBRP. Therefore the effect of Add/Cancellation requests may be delayed due to a running Tx scan (TXBRP not yet updated)."]
pub mod txfqs;
#[doc = "TXBRP (r) register accessor: FDCAN Tx Buffer Request Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbrp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbrp`]
module"]
#[doc(alias = "TXBRP")]
pub type Txbrp = crate::Reg<txbrp::TxbrpSpec>;
#[doc = "FDCAN Tx Buffer Request Pending Register"]
pub mod txbrp;
#[doc = "TXBAR (rw) register accessor: FDCAN Tx Buffer Add Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbar`]
module"]
#[doc(alias = "TXBAR")]
pub type Txbar = crate::Reg<txbar::TxbarSpec>;
#[doc = "FDCAN Tx Buffer Add Request Register"]
pub mod txbar;
#[doc = "TXBCR (rw) register accessor: FDCAN Tx Buffer Cancellation Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcr`]
module"]
#[doc(alias = "TXBCR")]
pub type Txbcr = crate::Reg<txbcr::TxbcrSpec>;
#[doc = "FDCAN Tx Buffer Cancellation Request Register"]
pub mod txbcr;
#[doc = "TXBTO (r) register accessor: FDCAN Tx Buffer Transmission Occurred Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbto::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbto`]
module"]
#[doc(alias = "TXBTO")]
pub type Txbto = crate::Reg<txbto::TxbtoSpec>;
#[doc = "FDCAN Tx Buffer Transmission Occurred Register"]
pub mod txbto;
#[doc = "TXBCF (r) register accessor: FDCAN Tx Buffer Cancellation Finished Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcf`]
module"]
#[doc(alias = "TXBCF")]
pub type Txbcf = crate::Reg<txbcf::TxbcfSpec>;
#[doc = "FDCAN Tx Buffer Cancellation Finished Register"]
pub mod txbcf;
#[doc = "TXBTIE (rw) register accessor: FDCAN Tx Buffer Transmission Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbtie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbtie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbtie`]
module"]
#[doc(alias = "TXBTIE")]
pub type Txbtie = crate::Reg<txbtie::TxbtieSpec>;
#[doc = "FDCAN Tx Buffer Transmission Interrupt Enable Register"]
pub mod txbtie;
#[doc = "TXBCIE (rw) register accessor: FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbcie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcie`]
module"]
#[doc(alias = "TXBCIE")]
pub type Txbcie = crate::Reg<txbcie::TxbcieSpec>;
#[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
pub mod txbcie;
#[doc = "TXEFS (r) register accessor: FDCAN Tx Event FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefs`]
module"]
#[doc(alias = "TXEFS")]
pub type Txefs = crate::Reg<txefs::TxefsSpec>;
#[doc = "FDCAN Tx Event FIFO Status Register"]
pub mod txefs;
#[doc = "TXEFA (rw) register accessor: FDCAN Tx Event FIFO Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txefa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefa`]
module"]
#[doc(alias = "TXEFA")]
pub type Txefa = crate::Reg<txefa::TxefaSpec>;
#[doc = "FDCAN Tx Event FIFO Acknowledge Register"]
pub mod txefa;
#[doc = "CKDIV (rw) register accessor: FDCAN CFG clock divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckdiv`]
module"]
#[doc(alias = "CKDIV")]
pub type Ckdiv = crate::Reg<ckdiv::CkdivSpec>;
#[doc = "FDCAN CFG clock divider register"]
pub mod ckdiv;

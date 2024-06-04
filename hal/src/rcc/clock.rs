use fugit::{HertzU32 as Hertz, RateExtU32};

use crate::pac;
use crate::pwr::{self, PowerConfiguration};

/// Constrained RCC peripheral
pub struct Rcc {
    cfg: Config,
    pub(crate) rcc: pac::Rcc,
}

impl Rcc {
    pub(super) fn new(rcc: pac::Rcc) -> Self {
        Rcc {
            cfg: Config::default(),
            rcc,
        }
    }

    pub fn pll(self) -> Self {
        self.clock_src(SysClockSrc::PLL)
    }

    pub fn hsi(self) -> Self {
        self.clock_src(SysClockSrc::HSI)
    }

    pub fn clock_src(mut self, mux: SysClockSrc) -> Self {
        self.cfg.sys_mux = mux;
        self
    }

    pub fn pll_cfg(mut self, cfg: PllConfig) -> Self {
        self.cfg.pll_cfg = cfg;
        self
    }

    pub fn ahb_psc(mut self, psc: Prescaler) -> Self {
        self.cfg.ahb_psc = psc;
        self
    }

    pub fn apb1_psc(mut self, psc: Prescaler) -> Self {
        self.cfg.apb1_psc = psc;
        self
    }

    pub fn apb2_psc(mut self, psc: Prescaler) -> Self {
        self.cfg.apb2_psc = psc;
        self
    }

    pub fn boost(mut self, enable_boost: bool) -> Self {
        self.cfg.enable_boost = enable_boost;
        self
    }

    pub fn pwr_cfg(mut self, pwr_cfg: PowerConfiguration) -> Self {
        self.cfg.pwr_cfg = pwr_cfg;
        self
    }

    /// Uses HSE (external oscillator) instead of HSI (internal RC oscillator) as the clock source.
    /// Will result in a hang if an external oscillator is not connected or it fails to start.
    // pub fn use_hse(mut self, freq: Hertz) -> Self {
    //     self.hse = Some(freq.raw());
    //     self
    // }

    /// Bypasses the high-speed external oscillator and uses an external clock input on the OSC_IN
    /// pin.
    ///
    /// For this configuration, the OSC_IN pin should be connected to a clock source with a
    /// frequency specified in the call to use_hse(), and the OSC_OUT pin should not be connected.
    ///
    /// This function has no effect unless use_hse() is also called.
    // pub fn bypass_hse_oscillator(self) -> Self {
    //     Self {
    //         hse_bypass: true,
    //         ..self
    //     }
    // }
    /// Initialises the hardware according to CFGR state returning a Clocks instance.
    /// Panics if overclocking is attempted.
    pub fn freeze(mut self) -> (Self, Clocks) {
        let pll_clk = self.pll_setup(self.cfg.pll_cfg);

        let (sys_clk, sw_bits) = match self.cfg.sys_mux {
            SysClockSrc::HSI => {
                self.enable_hsi();
                (HSI_FREQ.Hz(), 0b01)
            }
            SysClockSrc::HSE(freq) => {
                self.enable_hse(false);
                (freq, 0b10)
            }
            SysClockSrc::PLL => {
                // If PLL is selected as sysclock then the r output should have been configured
                if pll_clk.r.is_none() {
                    panic!("PLL output selected as sysclock but PLL output R is not configured")
                }
                (pll_clk.r.unwrap(), 0b11)
            }
        };

        let sys_freq = sys_clk.raw();
        let (ahb_freq, ahb_psc_bits) = match self.cfg.ahb_psc {
            Prescaler::Div2 => (sys_freq / 2, 0b1000),
            Prescaler::Div4 => (sys_freq / 4, 0b1001),
            Prescaler::Div8 => (sys_freq / 8, 0b1010),
            Prescaler::Div16 => (sys_freq / 16, 0b1011),
            Prescaler::Div64 => (sys_freq / 64, 0b1100),
            Prescaler::Div128 => (sys_freq / 128, 0b1101),
            Prescaler::Div256 => (sys_freq / 256, 0b1110),
            Prescaler::Div512 => (sys_freq / 512, 0b1111),
            _ => (sys_freq, 0b0000),
        };
        let (apb1_freq, apb1_psc_bits) = match self.cfg.apb1_psc {
            Prescaler::Div2 => (sys_freq / 2, 0b100),
            Prescaler::Div4 => (sys_freq / 4, 0b101),
            Prescaler::Div8 => (sys_freq / 8, 0b110),
            Prescaler::Div16 => (sys_freq / 16, 0b111),
            _ => (sys_freq, 0b000),
        };
        let (apb2_freq, apb2_psc_bits) = match self.cfg.apb2_psc {
            Prescaler::Div2 => (sys_freq / 2, 0b100),
            Prescaler::Div4 => (sys_freq / 4, 0b101),
            Prescaler::Div8 => (sys_freq / 8, 0b110),
            Prescaler::Div16 => (sys_freq / 16, 0b111),
            _ => (sys_freq, 0b000),
        };

        let present_vos_mode = pwr::current_vos();
        let target_vos_mode = self.cfg.pwr_cfg.vos();

        match (present_vos_mode, target_vos_mode) {
            // From VoltageScale::Range1 boost
            (
                pwr::VoltageScale::Range1 { enable_boost: true },
                pwr::VoltageScale::Range1 { enable_boost: true },
            ) => (), // No change
            (
                pwr::VoltageScale::Range1 { enable_boost: true },
                pwr::VoltageScale::Range1 {
                    enable_boost: false,
                },
            ) => todo!(),
            (pwr::VoltageScale::Range1 { enable_boost: true }, pwr::VoltageScale::Range2) => {
                todo!()
            }

            // From VoltageScale::Range1 normal
            (
                pwr::VoltageScale::Range1 {
                    enable_boost: false,
                },
                pwr::VoltageScale::Range1 { enable_boost: true },
            ) => {
                self.range1_normal_to_boost(
                    sys_freq,
                    apb1_psc_bits,
                    apb2_psc_bits,
                    sw_bits,
                    ahb_psc_bits,
                );
            }
            (
                pwr::VoltageScale::Range1 {
                    enable_boost: false,
                },
                pwr::VoltageScale::Range1 {
                    enable_boost: false,
                },
            ) => (), // No change
            (
                pwr::VoltageScale::Range1 {
                    enable_boost: false,
                },
                pwr::VoltageScale::Range2,
            ) => todo!(),

            // From VoltageScale::Range2
            (pwr::VoltageScale::Range2, pwr::VoltageScale::Range1 { enable_boost: true }) => {
                todo!()
            }
            (
                pwr::VoltageScale::Range2,
                pwr::VoltageScale::Range1 {
                    enable_boost: false,
                },
            ) => todo!(),
            (pwr::VoltageScale::Range2, pwr::VoltageScale::Range2) => (), // No change
        }

        self.configure_wait_states(sys_freq);

        self.rcc.rcc_cfgr().modify(|_, w| unsafe {
            w.hpre()
                .bits(ahb_psc_bits)
                .ppre1()
                .bits(apb1_psc_bits)
                .ppre2()
                .bits(apb2_psc_bits)
                .sw()
                .bits(sw_bits)
        });

        while self.rcc.rcc_cfgr().read().sws().bits() != sw_bits {}

        // From RM:
        // The timer clock frequencies are automatically defined by hardware. There are two cases:
        // 1. If the APB prescaler equals 1, the timer clock frequencies are set to the same
        // frequency as that of the APB domain.
        // 2. Otherwise, they are set to twice (×2) the frequency of the APB domain.
        let apb1_tim_clk = match self.cfg.apb1_psc {
            Prescaler::NotDivided => apb1_freq,
            _ => apb1_freq * 2,
        };

        let apb2_tim_clk = match self.cfg.apb2_psc {
            Prescaler::NotDivided => apb2_freq,
            _ => apb2_freq * 2,
        };

        (
            self,
            Clocks {
                pll_clk,
                sys_clk,
                core_clk: ahb_freq.Hz(),
                ahb_clk: ahb_freq.Hz(),
                apb1_clk: apb1_freq.Hz(),
                apb1_tim_clk: apb1_tim_clk.Hz(),
                apb2_clk: apb2_freq.Hz(),
                apb2_tim_clk: apb2_tim_clk.Hz(),
            },
        )
    }

    pub fn unlock_rtc(&mut self) {
        self.rcc.rcc_apb1enr1().modify(|_, w| w.pwren().set_bit());
        let pwr = unsafe { &(*pac::Pwr::ptr()) };
        pwr.pwr_cr1().modify(|_, w| w.dbp().set_bit());
    }

    fn pll_setup(&self, pll_cfg: PllConfig) -> PLLClocks {
        // Disable PLL
        self.rcc.rcc_cr().modify(|_, w| w.pllon().clear_bit());
        while self.rcc.rcc_cr().read().pllrdy().bit_is_set() {}

        // Enable the input clock feeding the PLL
        let (pll_input_freq, pll_src_bits) = match pll_cfg.mux {
            PLLSrc::HSI => {
                self.enable_hsi();
                (HSI_FREQ, 0b10)
            }
            PLLSrc::HSE(freq) => {
                self.enable_hse(false);
                (freq.raw(), 0b11)
            }
            PLLSrc::HSE_BYPASS(freq) => {
                self.enable_hse(true);
                (freq.raw(), 0b11)
            }
        };

        // Calculate the frequency of the internal PLL VCO.
        let pll_freq = pll_input_freq / pll_cfg.m.divisor() * pll_cfg.n.multiplier();

        // Calculate the output frequencies for the P, Q, and R outputs
        let p = pll_cfg
            .p
            .map(|p| ((pll_freq / p.divisor()).Hz(), p.register_setting()));

        let q = pll_cfg
            .q
            .map(|q| ((pll_freq / q.divisor()).Hz(), q.register_setting()));

        let r = pll_cfg
            .r
            .map(|r| ((pll_freq / r.divisor()).Hz(), r.register_setting()));

        // Set the M input divider, the N multiplier for the PLL, and the PLL source.
        self.rcc.rcc_pllcfgr().modify(|_, w| unsafe {
            // Set N, M, and source
            let w = w
                .plln()
                .bits(pll_cfg.n.register_setting())
                .pllm()
                .bits(pll_cfg.m.register_setting())
                .pllsrc()
                .bits(pll_src_bits);

            // Set and enable P if requested
            let w = match p {
                Some((_, register_setting)) => {
                    w.pllpdiv().bits(register_setting).pllpen().set_bit()
                }
                None => w,
            };

            // Set and enable Q if requested
            let w = match q {
                Some((_, register_setting)) => w.pllq().bits(register_setting).pllqen().set_bit(),
                None => w,
            };

            // Set and enable R if requested
            let w = match r {
                Some((_, register_setting)) => w.pllr().bits(register_setting).pllren().set_bit(),
                None => w,
            };

            w
        });

        // Enable PLL
        self.rcc.rcc_cr().modify(|_, w| w.pllon().set_bit());
        while self.rcc.rcc_cr().read().pllrdy().bit_is_clear() {}

        PLLClocks {
            r: r.map(|r| r.0),
            q: q.map(|q| q.0),
            p: p.map(|p| p.0),
        }
    }

    pub(crate) fn enable_hsi(&self) {
        self.rcc.rcc_cr().modify(|_, w| w.hsion().set_bit());
        while self.rcc.rcc_cr().read().hsirdy().bit_is_clear() {}
    }

    pub(crate) fn enable_hse(&self, bypass: bool) {
        self.rcc
            .rcc_cr()
            .modify(|_, w| w.hseon().set_bit().hsebyp().bit(bypass));
        while self.rcc.rcc_cr().read().hserdy().bit_is_clear() {}
    }

    pub(crate) fn enable_lse(&self, bypass: bool) {
        self.rcc
            .rcc_bdcr()
            .modify(|_, w| w.lseon().set_bit().lsebyp().bit(bypass));
        while self.rcc.rcc_bdcr().read().lserdy().bit_is_clear() {}
    }

    pub(crate) fn enable_lsi(&self) {
        self.rcc.rcc_csr().modify(|_, w| w.lsion().set_bit());
        while self.rcc.rcc_csr().read().lsirdy().bit_is_clear() {}
    }

    fn range1_normal_to_boost(
        &mut self,
        sys_freq: u32,
        apb1_psc_bits: u8,
        apb2_psc_bits: u8,
        sw_bits: u8,
        ahb_psc_bits: u8,
    ) {
        // (From RM0440 chapter "Power control (PWR)")
        // The sequence to switch from Range11 normal mode to Range1 boost mode is:
        // 1. The system clock must be divided by 2 using the AHB prescaler before switching to a
        // higher system frequency.
        let half_apb = (self.rcc.rcc_cfgr().read().hpre().bits() + 1).clamp(0b1000, 0b1111);
        self.rcc
            .rcc_cfgr()
            .modify(|_r, w| unsafe { w.hpre().bits(half_apb) });
        while self.rcc.rcc_cfgr().read().hpre().bits() != half_apb {}

        // 2. Clear the R1MODE bit is in the PWR_CR5 register.
        unsafe { pwr::set_boost(true) };

        // 3. Adjust the number of wait states according to the new frequency target in range1 boost mode
        self.configure_wait_states(sys_freq);

        // 4. Configure and switch to new system frequency.
        self.rcc.rcc_cfgr().modify(|_, w| unsafe {
            w.ppre1()
                .bits(apb1_psc_bits)
                .ppre2()
                .bits(apb2_psc_bits)
                .sw()
                .bits(sw_bits)
        });

        while self.rcc.rcc_cfgr().read().sws().bits() != sw_bits {}

        // 5. Wait for at least 1us and then reconfigure the AHB prescaler to get the needed HCLK
        // clock frequency.
        let us_per_s = 1_000_000;
        // Number of cycles @ sys_freq for 1us, rounded up, this will
        // likely end up being 2us since the AHB prescaler is changed
        let delay_cycles = (sys_freq + us_per_s - 1) / us_per_s;
        cortex_m::asm::delay(delay_cycles);

        self.rcc
            .rcc_cfgr()
            .modify(|_, w| unsafe { w.hpre().bits(ahb_psc_bits) });
    }

    fn configure_wait_states(&mut self, sys_freq: u32) {
        // Calculate wait states depending on voltage scale and sys_freq
        //
        // See 'Number of wait states according to CPU clock (HCLK) frequency' in RM0440
        let latency = match self.cfg.pwr_cfg.vos() {
            pwr::VoltageScale::Range1 { enable_boost: true } => match sys_freq {
                0..=34_000_000 => 0b0000,
                34_000_001..=68_000_000 => 0b0001,
                68_000_001..=102_000_000 => 0b0010,
                102_000_001..=136_000_000 => 0b0011,
                136_000_001..=170_000_000 => 0b0100,
                170_000_001.. => panic!(
                    "Too high f_sys: {}, max with voltage scale in 'range1 boost mode' is: 170MHz",
                    sys_freq
                ),
            },
            pwr::VoltageScale::Range1 {
                enable_boost: false,
            } => match sys_freq {
                0..=30_000_000 => 0b0000,
                30_000_001..=60_000_000 => 0b0001,
                60_000_001..=90_000_000 => 0b0010,
                90_000_001..=120_000_000 => 0b0011,
                120_000_001..=150_000_000 => 0b0100,
                150_000_001.. => panic!(
                    "Too high f_sys: {}, max with voltage scale in 'range1 normal mode' is: 150MHz",
                    sys_freq
                ),
            },
            pwr::VoltageScale::Range2 => match sys_freq {
                0..=12_000_000 => 0b0000,
                12_000_001..=24_000_000 => 0b0001,
                24_000_001..=26_000_000 => 0b0010,
                26_000_001.. => panic!(
                    "Too high f_sys: {}, max with voltage scale in 'range2' is: 26MHz",
                    sys_freq
                ),
            },
        };

        unsafe {
            // Adjust flash wait states
            let flash = &(*pac::Flash::ptr());
            flash.acr().modify(|_, w| w.latency().bits(latency))
        }
    }

    pub fn get_reset_reason(&self) -> ResetReason {
        let csr = self.rcc.rcc_csr().read();

        ResetReason {
            low_power: csr.lpwrrstf().bit(),
            window_watchdog: csr.wwdgrstf().bit(),
            independent_watchdog: csr.iwdgrstf().bit(),
            software: csr.sftrstf().bit(),
            brown_out: csr.borrstf().bit(),
            reset_pin: csr.pinrstf().bit(),
            option_byte: csr.oblrstf().bit(),
        }
    }

    pub fn clear_reset_reason(&mut self) {
        self.rcc.rcc_csr().modify(|_, w| w.rmvf().set_bit());
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ResetReason {
    /// Low-power reset flag
    ///
    /// Set by hardware when a reset occurs to illegal Stop, Standby or Shutdown mode entry.
    pub low_power: bool,

    /// Window watchdog reset flag
    ///
    /// Set by hardware when a window watchdog reset occurs.
    pub window_watchdog: bool,

    /// Independent window watchdog reset flag
    ///
    /// Set by hardware when an independent watchdog reset occurs.
    pub independent_watchdog: bool,

    /// Software reset flag
    ///
    /// Set by hardware when a software reset occurs.
    pub software: bool,

    /// Brown out reset flag
    ///
    /// Set by hardware when a brown out reset occurs.
    pub brown_out: bool,

    /// Pin reset flag
    ///
    /// Set by hardware when a reset from the NRST pin occurs.
    pub reset_pin: bool,

    /// Option byte loader reset flag
    ///
    /// Set by hardware when a reset from the Option Byte loading occurs.
    pub option_byte: bool,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Config {
    sys_mux: SysClockSrc,
    pll_cfg: PllConfig,
    ahb_psc: Prescaler,
    apb1_psc: Prescaler,
    apb2_psc: Prescaler,
    /// Required for f_sys > 150MHz
    enable_boost: bool,
    // Power Configuration
    pwr_cfg: PowerConfiguration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            sys_mux: SysClockSrc::HSI,
            pll_cfg: PllConfig::default(),
            ahb_psc: Prescaler::NotDivided,
            apb1_psc: Prescaler::NotDivided,
            apb2_psc: Prescaler::NotDivided,
            enable_boost: false,
            pwr_cfg: PowerConfiguration::default(),
        }
    }
}

/// Built-in high speed clock frequency
pub const HSI_FREQ: u32 = 16_000_000; // Hz

/// Clock frequencies
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Clocks {
    /// System frequency
    pub sys_clk: Hertz,
    /// Core frequency
    pub core_clk: Hertz,
    /// AHB frequency
    pub ahb_clk: Hertz,
    /// APB 1 frequency
    pub apb1_clk: Hertz,
    /// APB 1 timers frequency (Timers 2-7)
    pub apb1_tim_clk: Hertz,
    /// APB 2 frequency
    pub apb2_clk: Hertz,
    /// APB 2 timers frequency (Timers 1, 8, 20, 15, 16, 17 and HRTIM1)
    pub apb2_tim_clk: Hertz,
    /// PLL frequency
    pub pll_clk: PLLClocks,
}

/// PLL Clock frequencies
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct PLLClocks {
    /// R frequency
    pub r: Option<Hertz>,
    /// Q frequency
    pub q: Option<Hertz>,
    /// P frequency
    pub p: Option<Hertz>,
}

impl Default for Clocks {
    fn default() -> Clocks {
        let freq = HSI_FREQ.Hz();
        Clocks {
            sys_clk: freq,
            ahb_clk: freq,
            core_clk: freq,
            apb1_clk: freq,
            apb1_tim_clk: freq,
            apb2_clk: freq,
            apb2_tim_clk: freq,
            pll_clk: PLLClocks {
                r: None,
                q: None,
                p: None,
            },
        }
    }
}

/// System clock mux source
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SysClockSrc {
    PLL,
    HSI,
    HSE(Hertz),
}

/// PLL config
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct PllConfig {
    pub mux: PLLSrc,
    pub m: PllMDiv,
    pub n: PllNMul,
    pub r: Option<PllRDiv>,
    pub q: Option<PllQDiv>,
    pub p: Option<PllPDiv>,
}

impl Default for PllConfig {
    fn default() -> PllConfig {
        PllConfig {
            mux: PLLSrc::HSI,
            m: PllMDiv::DIV_2,
            n: PllNMul::MUL_8,
            r: Some(PllRDiv::DIV_2),
            q: None,
            p: None,
        }
    }
}

/// PLL clock input source
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PLLSrc {
    HSI,
    HSE(Hertz),
    HSE_BYPASS(Hertz),
}

/// Divider for the PLL clock input (M)
/// This must be set based on the input clock to keep the PLL input frequency within the limits
/// specified in the datasheet.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PllMDiv {
    DIV_1 = 0,
    DIV_2,
    DIV_3,
    DIV_4,
    DIV_5,
    DIV_6,
    DIV_7,
    DIV_8,
    DIV_9,
    DIV_10,
    DIV_11,
    DIV_12,
    DIV_13,
    DIV_14,
    DIV_15,
    DIV_16,
}

impl PllMDiv {
    pub fn divisor(&self) -> u32 {
        (*self as u32) + 1
    }

    pub fn register_setting(&self) -> u8 {
        *self as u8
    }
}

/// Divider for the PLL Q Output
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PllQDiv {
    DIV_2 = 0,
    DIV_4,
    DIV_6,
    DIV_8,
}

impl PllQDiv {
    pub fn divisor(&self) -> u32 {
        ((*self as u32) + 1) * 2
    }

    pub fn register_setting(&self) -> u8 {
        *self as u8
    }
}

/// Divider for the PLL R Output
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PllRDiv {
    DIV_2 = 0,
    DIV_4,
    DIV_6,
    DIV_8,
}

impl PllRDiv {
    pub fn divisor(&self) -> u32 {
        ((*self as u32) + 1) * 2
    }

    pub fn register_setting(&self) -> u8 {
        *self as u8
    }
}

/// Divider for the PLL P Output
///
/// Note: The P divider has a PLLP register that can be used to set the divider to either 7 or 17.
/// It is a complete mystery why anyone would want to do that instead of using the PLLPDIV register
/// so it's not supported.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PllPDiv {
    DIV_2 = 2,
    DIV_3,
    DIV_4,
    DIV_5,
    DIV_6,
    DIV_7,
    DIV_8,
    DIV_9,
    DIV_10,
    DIV_11,
    DIV_12,
    DIV_13,
    DIV_14,
    DIV_15,
    DIV_16,
    DIV_17,
    DIV_18,
    DIV_19,
    DIV_20,
    DIV_21,
    DIV_22,
    DIV_23,
    DIV_24,
    DIV_25,
    DIV_26,
    DIV_27,
    DIV_28,
    DIV_29,
    DIV_30,
    DIV_31,
}

impl PllPDiv {
    pub fn divisor(&self) -> u32 {
        *self as u32
    }

    pub fn register_setting(&self) -> u8 {
        *self as u8
    }
}

/// Main PLL multiplication factor for VCO
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PllNMul {
    MUL_8 = 8,
    MUL_9,
    MUL_10,
    MUL_11,
    MUL_12,
    MUL_13,
    MUL_14,
    MUL_15,
    MUL_16,
    MUL_17,
    MUL_18,
    MUL_19,
    MUL_20,
    MUL_21,
    MUL_22,
    MUL_23,
    MUL_24,
    MUL_25,
    MUL_26,
    MUL_27,
    MUL_28,
    MUL_29,
    MUL_30,
    MUL_31,
    MUL_32,
    MUL_33,
    MUL_34,
    MUL_35,
    MUL_36,
    MUL_37,
    MUL_38,
    MUL_39,
    MUL_40,
    MUL_41,
    MUL_42,
    MUL_43,
    MUL_44,
    MUL_45,
    MUL_46,
    MUL_47,
    MUL_48,
    MUL_49,
    MUL_50,
    MUL_51,
    MUL_52,
    MUL_53,
    MUL_54,
    MUL_55,
    MUL_56,
    MUL_57,
    MUL_58,
    MUL_59,
    MUL_60,
    MUL_61,
    MUL_62,
    MUL_63,
    MUL_64,
    MUL_65,
    MUL_66,
    MUL_67,
    MUL_68,
    MUL_69,
    MUL_70,
    MUL_71,
    MUL_72,
    MUL_73,
    MUL_74,
    MUL_75,
    MUL_76,
    MUL_77,
    MUL_78,
    MUL_79,
    MUL_80,
    MUL_81,
    MUL_82,
    MUL_83,
    MUL_84,
    MUL_85,
    MUL_86,
    MUL_87,
    MUL_88,
    MUL_89,
    MUL_90,
    MUL_91,
    MUL_92,
    MUL_93,
    MUL_94,
    MUL_95,
    MUL_96,
    MUL_97,
    MUL_98,
    MUL_99,
    MUL_100,
    MUL_101,
    MUL_102,
    MUL_103,
    MUL_104,
    MUL_105,
    MUL_106,
    MUL_107,
    MUL_108,
    MUL_109,
    MUL_110,
    MUL_111,
    MUL_112,
    MUL_113,
    MUL_114,
    MUL_115,
    MUL_116,
    MUL_117,
    MUL_118,
    MUL_119,
    MUL_120,
    MUL_121,
    MUL_122,
    MUL_123,
    MUL_124,
    MUL_125,
    MUL_126,
    MUL_127,
}

impl PllNMul {
    pub fn multiplier(&self) -> u32 {
        *self as u32
    }

    pub fn register_setting(&self) -> u8 {
        *self as u8
    }
}

/// Prescaler
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Prescaler {
    NotDivided,
    Div2,
    Div4,
    Div8,
    Div16,
    Div32,
    Div64,
    Div128,
    Div256,
    Div512,
}

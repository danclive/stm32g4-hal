//! Clock configuration.
use fugit::{HertzU32 as Hertz, RateExtU32};

use crate::pac::{
    self,
    rcc::{self, RegisterBlock as RccRB},
};

use crate::pwr::{self, PowerConfiguration};

pub use config::*;

mod config;
mod enable;
pub mod lsco;
pub mod mco;
pub mod rtc;

/// Extension trait that constrains the `RCC` peripheral
pub trait RccExt {
    /// Constrains the `RCC` peripheral so it plays nicely with the other abstractions
    fn constrain(self) -> Rcc;
}

impl RccExt for pac::Rcc {
    fn constrain(self) -> Rcc {
        Rcc {
            config: Config::default(),
            rb: self,
            clocks: Clocks::default(),
        }
    }
}

/// Built-in high speed clock frequency
pub const HSI: u32 = 16_000_000; // Hz
pub const HSI48: u32 = 48_000_000; // Hz
pub const LSI: u32 = 32_000; // Hz

/// Constrained RCC peripheral
pub struct Rcc {
    config: Config,
    pub(crate) rb: pac::Rcc,
    pub(crate) clocks: Clocks,
}

impl Rcc {
    pub fn hse(mut self, freq: Hertz, bypass: bool) -> Self {
        self.config.hse = Some((freq, bypass));
        self
    }

    pub fn clock_src(mut self, mux: SysClockSrc) -> Self {
        self.config.sys_mux = mux;
        self
    }

    pub fn pll_cfg(mut self, cfg: PllConfig) -> Self {
        self.config.pll_cfg = cfg;
        self
    }

    pub fn ahb_psc(mut self, psc: Prescaler) -> Self {
        self.config.ahb_psc = psc;
        self
    }

    pub fn apb1_psc(mut self, psc: Prescaler) -> Self {
        self.config.apb1_psc = psc;
        self
    }

    pub fn apb2_psc(mut self, psc: Prescaler) -> Self {
        self.config.apb2_psc = psc;
        self
    }

    pub fn boost(mut self, enable_boost: bool) -> Self {
        self.config.enable_boost = enable_boost;
        self
    }

    pub fn pwr_cfg(mut self, pwr_cfg: PowerConfiguration) -> Self {
        self.config.pwr_cfg = pwr_cfg;
        self
    }

    /// Initialises the hardware according to CFGR state returning a Clocks instance.
    /// Panics if overclocking is attempted.
    pub fn freeze(mut self) -> Self {
        self.enable_hsi();
        self.enable_hsi48();
        self.enable_lsi();

        let hse_clk = match self.config.hse {
            Some((freq, bypass)) => {
                self.enable_hse(bypass);
                Some(freq)
            }
            None => {
                self.enable_hsi();
                None
            }
        };

        let lse_clk = match self.config.lse {
            Some((freq, bypass)) => {
                self.enable_lse(bypass);
                Some(freq)
            }
            None => None,
        };

        let pll_clk = self.pll_setup();

        let (sys_clk, sw_bits) = match self.config.sys_mux {
            SysClockSrc::HSI => {
                self.enable_hsi();
                (HSI.Hz(), 0b01)
            }
            SysClockSrc::HSE => {
                if let Some((freq, _)) = self.config.hse {
                    (freq, 0b10)
                } else {
                    panic!("HSE selected as sysclock but HSE is not configured")
                }
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
        let (ahb_freq, ahb_psc_bits) = match self.config.ahb_psc {
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
        let (apb1_freq, apb1_psc_bits) = match self.config.apb1_psc {
            Prescaler::Div2 => (sys_freq / 2, 0b100),
            Prescaler::Div4 => (sys_freq / 4, 0b101),
            Prescaler::Div8 => (sys_freq / 8, 0b110),
            Prescaler::Div16 => (sys_freq / 16, 0b111),
            _ => (sys_freq, 0b000),
        };
        let (apb2_freq, apb2_psc_bits) = match self.config.apb2_psc {
            Prescaler::Div2 => (sys_freq / 2, 0b100),
            Prescaler::Div4 => (sys_freq / 4, 0b101),
            Prescaler::Div8 => (sys_freq / 8, 0b110),
            Prescaler::Div16 => (sys_freq / 16, 0b111),
            _ => (sys_freq, 0b000),
        };

        let present_vos_mode = pwr::current_vos();
        let target_vos_mode = self.config.pwr_cfg.vos();

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

        self.rb.rcc_cfgr().modify(|_, w| unsafe {
            w.hpre()
                .bits(ahb_psc_bits)
                .ppre1()
                .bits(apb1_psc_bits)
                .ppre2()
                .bits(apb2_psc_bits)
                .sw()
                .bits(sw_bits)
        });

        while self.rb.rcc_cfgr().read().sws().bits() != sw_bits {}

        // From RM:
        // The timer clock frequencies are automatically defined by hardware. There are two cases:
        // 1. If the APB prescaler equals 1, the timer clock frequencies are set to the same
        // frequency as that of the APB domain.
        // 2. Otherwise, they are set to twice (×2) the frequency of the APB domain.
        let apb1_tim_clk = match self.config.apb1_psc {
            Prescaler::NotDivided => apb1_freq,
            _ => apb1_freq * 2,
        };

        let apb2_tim_clk = match self.config.apb2_psc {
            Prescaler::NotDivided => apb2_freq,
            _ => apb2_freq * 2,
        };

        self.mco_setup();
        self.lsco_setup();

        self.clocks = Clocks {
            pll_clk,
            sys_clk,
            ahb_clk: ahb_freq.Hz(),
            apb1_clk: apb1_freq.Hz(),
            apb1_tim_clk: apb1_tim_clk.Hz(),
            apb2_clk: apb2_freq.Hz(),
            apb2_tim_clk: apb2_tim_clk.Hz(),
            hse_clk,
            lse_clk,
            hsi_clk: Some(HSI.Hz()),
            hsi48_clk: Some(HSI48.Hz()),
            lsi_clk: Some(LSI.Hz()),
        };

        self
    }

    pub fn clocks(&self) -> &Clocks {
        &self.clocks
    }

    pub fn unlock_rtc(&mut self) {
        self.rb.rcc_apb1enr1().modify(|_, w| w.pwren().set_bit());
        let pwr = unsafe { &(*pac::Pwr::PTR) };
        pwr.pwr_cr1().modify(|_, w| w.dbp().set_bit());
    }

    fn pll_setup(&self) -> PLLClocks {
        // Disable PLL
        self.rb.rcc_cr().modify(|_, w| w.pllon().clear_bit());
        while self.rb.rcc_cr().read().pllrdy().bit_is_set() {}

        let (pll_input_freq, pll_src_bits) = match self.config.pll_cfg.mux {
            PLLSrc::HSE => {
                if let Some((freq, _)) = self.config.hse {
                    (freq.raw(), 0b11)
                } else {
                    panic!("HSE selected as PLL source but HSE is not configured")
                }
            }
            PLLSrc::HSI => (HSI, 0b10),
        };

        // Calculate the frequency of the internal PLL VCO.
        let pll_freq =
            pll_input_freq / self.config.pll_cfg.m.divisor() * self.config.pll_cfg.n.multiplier();

        // Calculate the output frequencies for the P, Q, and R outputs
        let p = self
            .config
            .pll_cfg
            .p
            .map(|p| ((pll_freq / p.divisor()).Hz(), p.register_setting()));

        let q = self
            .config
            .pll_cfg
            .q
            .map(|q| ((pll_freq / q.divisor()).Hz(), q.register_setting()));

        let r = self
            .config
            .pll_cfg
            .r
            .map(|r| ((pll_freq / r.divisor()).Hz(), r.register_setting()));

        // Set the M input divider, the N multiplier for the PLL, and the PLL source.
        self.rb.rcc_pllcfgr().modify(|_, w| unsafe {
            // Set N, M, and source
            let w = w
                .plln()
                .bits(self.config.pll_cfg.n.register_setting())
                .pllm()
                .bits(self.config.pll_cfg.m.register_setting())
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
        self.rb.rcc_cr().modify(|_, w| w.pllon().set_bit());
        while self.rb.rcc_cr().read().pllrdy().bit_is_clear() {}

        PLLClocks {
            r: r.map(|r| r.0),
            q: q.map(|q| q.0),
            p: p.map(|p| p.0),
        }
    }

    pub(crate) fn enable_hsi(&self) {
        self.rb.rcc_cr().modify(|_, w| w.hsion().set_bit());
        while self.rb.rcc_cr().read().hsirdy().bit_is_clear() {}
    }

    pub(crate) fn enable_hse(&self, bypass: bool) {
        self.rb.rcc_cr().modify(|_, w| w.hsebyp().bit(bypass));
        self.rb.rcc_cr().modify(|_, w| w.hseon().set_bit());
        while self.rb.rcc_cr().read().hserdy().bit_is_clear() {}
    }

    pub(crate) fn enable_hsi48(&self) {
        self.rb.rcc_crrcr().modify(|_, w| w.hsi48on().set_bit());
        while self.rb.rcc_crrcr().read().hsi48rdy().bit_is_clear() {}
    }

    pub(crate) fn enable_lse(&self, bypass: bool) {
        self.rb.rcc_bdcr().modify(|_, w| w.lsebyp().bit(bypass));
        self.rb.rcc_bdcr().modify(|_, w| w.lseon().set_bit());
        while self.rb.rcc_bdcr().read().lserdy().bit_is_clear() {}
    }

    pub(crate) fn enable_lsi(&self) {
        self.rb.rcc_csr().modify(|_, w| w.lsion().set_bit());
        while self.rb.rcc_csr().read().lsirdy().bit_is_clear() {}
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
        let half_apb = (self.rb.rcc_cfgr().read().hpre().bits() + 1).clamp(0b1000, 0b1111);
        self.rb
            .rcc_cfgr()
            .modify(|_r, w| unsafe { w.hpre().bits(half_apb) });
        while self.rb.rcc_cfgr().read().hpre().bits() != half_apb {}

        // 2. Clear the R1MODE bit is in the PWR_CR5 register.
        unsafe { pwr::set_boost(true) };

        // 3. Adjust the number of wait states according to the new frequency target in range1 boost mode
        self.configure_wait_states(sys_freq);

        // 4. Configure and switch to new system frequency.
        self.rb.rcc_cfgr().modify(|_, w| unsafe {
            w.ppre1()
                .bits(apb1_psc_bits)
                .ppre2()
                .bits(apb2_psc_bits)
                .sw()
                .bits(sw_bits)
        });

        while self.rb.rcc_cfgr().read().sws().bits() != sw_bits {}

        // 5. Wait for at least 1us and then reconfigure the AHB prescaler to get the needed HCLK
        // clock frequency.
        let us_per_s = 1_000_000;
        // Number of cycles @ sys_freq for 1us, rounded up, this will
        // likely end up being 2us since the AHB prescaler is changed
        let delay_cycles = sys_freq.div_ceil(us_per_s);
        cortex_m::asm::delay(delay_cycles);

        self.rb
            .rcc_cfgr()
            .modify(|_, w| unsafe { w.hpre().bits(ahb_psc_bits) });
    }

    fn configure_wait_states(&mut self, sys_freq: u32) {
        // Calculate wait states depending on voltage scale and sys_freq
        //
        // See 'Number of wait states according to CPU clock (HCLK) frequency' in RM0440
        let latency = match self.config.pwr_cfg.vos() {
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
            let flash = &(*pac::Flash::PTR);
            flash.acr().modify(|_, w| w.latency().bits(latency));
        }
    }

    pub fn get_reset_reason(&self) -> ResetReason {
        let csr = self.rb.rcc_csr().read();

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
        self.rb.rcc_csr().modify(|_, w| w.rmvf().set_bit());
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

/// Clock frequencies
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Clocks {
    /// System frequency
    sys_clk: Hertz,
    /// AHB frequency
    ahb_clk: Hertz,
    /// APB 1 frequency
    apb1_clk: Hertz,
    /// APB 1 timers frequency (Timers 2-7)
    apb1_tim_clk: Hertz,
    /// APB 2 frequency
    apb2_clk: Hertz,
    /// APB 2 timers frequency (Timers 1, 8, 20, 15, 16, 17 and HRTIM1)
    apb2_tim_clk: Hertz,
    /// PLL frequency
    pll_clk: PLLClocks,

    hse_clk: Option<Hertz>,
    lse_clk: Option<Hertz>,
    hsi_clk: Option<Hertz>,
    hsi48_clk: Option<Hertz>,
    lsi_clk: Option<Hertz>,
}

impl Clocks {
    /// Returns the system (core) frequency
    pub fn sys_clk(&self) -> Hertz {
        self.sys_clk
    }

    /// Returns the frequency of the AHB
    pub fn ahb_clk(&self) -> Hertz {
        self.ahb_clk
    }

    /// Returns the frequency of the APB1
    pub fn apb1_clk(&self) -> Hertz {
        self.apb1_clk
    }

    pub fn pclk1(&self) -> Hertz {
        self.apb1_clk()
    }

    /// Returns the frequency of the APB1 TIM
    pub fn apb1_tim_clk(&self) -> Hertz {
        self.apb1_tim_clk
    }

    /// Returns the frequency of the APB2
    pub fn apb2_clk(&self) -> Hertz {
        self.apb2_clk
    }

    pub fn pclk2(&self) -> Hertz {
        self.apb2_clk()
    }

    /// Returns the frequency of the APB2 TIM
    pub fn apb2_tim_clk(&self) -> Hertz {
        self.apb2_tim_clk
    }

    /// Returns the frequency of the PLL
    pub fn pll_clk(&self) -> PLLClocks {
        self.pll_clk
    }

    /// Returns the frequency of the HSE
    pub fn hse_clk(&self) -> Option<Hertz> {
        self.hse_clk
    }

    /// Returns the frequency of the LSE
    pub fn lse_clk(&self) -> Option<Hertz> {
        self.lse_clk
    }

    /// Returns the frequency of the HSI
    pub fn hsi_clk(&self) -> Option<Hertz> {
        self.hsi_clk
    }

    /// Returns the frequency of the HSI48
    pub fn hsi48_clk(&self) -> Option<Hertz> {
        self.hsi48_clk
    }

    /// Returns the frequency of the LSI
    pub fn lsi_clk(&self) -> Option<Hertz> {
        self.lsi_clk
    }
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
        let freq = HSI.Hz();
        Clocks {
            sys_clk: freq,
            ahb_clk: freq,
            apb1_clk: freq,
            apb1_tim_clk: freq,
            apb2_clk: freq,
            apb2_tim_clk: freq,
            pll_clk: PLLClocks {
                r: None,
                q: None,
                p: None,
            },
            hse_clk: None,
            lse_clk: None,
            hsi_clk: None,
            hsi48_clk: None,
            lsi_clk: None,
        }
    }
}

/// Bus associated to peripheral
pub trait RccBus: crate::Sealed {
    /// Bus type;
    type Bus;
}

/// Enable/disable peripheral
#[allow(clippy::missing_safety_doc)]
pub trait Enable: RccBus {
    /// Enables peripheral
    fn enable(rcc: &RccRB);

    /// Disables peripheral
    fn disable(rcc: &RccRB);

    /// Check if peripheral enabled
    fn is_enabled() -> bool;

    fn enable_for_sleep_stop(rcc: &RccRB);

    /// Check if peripheral disabled
    #[inline]
    fn is_disabled() -> bool {
        !Self::is_enabled()
    }

    /// # Safety
    ///
    /// Enables peripheral. Takes access to RCC internally
    unsafe fn enable_unchecked() {
        let rcc = &*pac::Rcc::PTR;
        Self::enable(rcc);
    }

    /// # Safety
    ///
    /// Disables peripheral. Takes access to RCC internally
    unsafe fn disable_unchecked() {
        let rcc = pac::Rcc::PTR;
        Self::disable(&*rcc);
    }
}

/// Reset peripheral
#[allow(clippy::missing_safety_doc)]
pub trait Reset: RccBus {
    /// Resets peripheral
    fn reset(rcc: &RccRB);

    /// # Safety
    ///
    /// Resets peripheral. Takes access to RCC internally
    unsafe fn reset_unchecked() {
        let rcc = pac::Rcc::PTR;
        Self::reset(&*rcc);
    }
}

macro_rules! bus_struct {
    ($( $(#[$attr:meta])* $busX:ident => ($EN:ident, $en:ident, $RST:ident, $rst:ident, $SMEN:ident, $smen:ident, $doc:literal),)+) => {
        $(
            $(#[$attr])*
            #[doc = $doc]
            pub struct $busX {
                _0: (),
            }

            $(#[$attr])*
            impl $busX {
                #[allow(unused)]
                #[inline(always)]
                pub(crate) fn enr(rcc: &RccRB) -> &rcc::$EN {
                    &rcc.$en()
                }

                #[allow(unused)]
                #[inline(always)]
                pub(crate) fn rstr(rcc: &RccRB) -> &rcc::$RST {
                    &rcc.$rst()
                }

                #[allow(unused)]
                #[inline(always)]
                pub(crate) fn smenr(rcc: &RccRB) -> &rcc::$SMEN {
                    &rcc.$smen()
                }
            }
        )+
    };
}

bus_struct! {
    AHB1 => (RccAhb1enr, rcc_ahb1enr, RccAhb1rstr, rcc_ahb1rstr, RccAhb1smenr, rcc_ahb1smenr, "Advanced High-performance Bus 1 (AHB1) registers"),
    AHB2 => (RccAhb2enr, rcc_ahb2enr, RccAhb2rstr, rcc_ahb2rstr, RccAhb2smenr, rcc_ahb2smenr, "Advanced High-performance Bus 2 (AHB2) registers"),
    AHB3 => (RccAhb3enr, rcc_ahb3enr, RccAhb3rstr, rcc_ahb3rstr, RccAhb3smenr, rcc_ahb3smenr, "Advanced High-performance Bus 3 (AHB3) registers"),
    APB1_1 => (RccApb1enr1, rcc_apb1enr1, RccApb1rstr1, rcc_apb1rstr1, RccApb1smenr1, rcc_apb1smenr1, "Advanced Peripheral Bus 1_1 (APB1_1) registers"),
    APB1_2 => (RccApb1enr2, rcc_apb1enr2, RccApb1rstr2, rcc_apb1rstr2, RccApb1smenr2, rcc_apb1smenr2, "Advanced Peripheral Bus 1_2 (APB1_2) registers"),
    APB2 => (RccApb2enr, rcc_apb2enr, RccApb2rstr, rcc_apb2rstr, RccApb2smenr, rcc_apb2smenr, "Advanced Peripheral Bus 2 (APB2) registers"),
}

/// Frequency on bus that peripheral is connected in
pub trait BusClock {
    /// Calculates frequency depending on `Clock` state
    fn clock(clocks: &Clocks) -> Hertz;
}

impl<T> BusClock for T
where
    T: RccBus,
    T::Bus: BusClock,
{
    fn clock(clocks: &Clocks) -> Hertz {
        T::Bus::clock(clocks)
    }
}

impl BusClock for AHB1 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.ahb_clk()
    }
}

impl BusClock for AHB2 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.ahb_clk()
    }
}

impl BusClock for AHB3 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.ahb_clk()
    }
}

impl BusClock for APB1_1 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.apb1_clk()
    }
}

impl BusClock for APB1_2 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.apb1_clk()
    }
}

impl BusClock for APB2 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.apb2_clk()
    }
}

/// Frequency on bus that timer is connected in
pub trait BusTimerClock {
    /// Calculates base frequency of timer depending on `Clock` state
    fn timer_clock(clocks: &Clocks) -> Hertz;
}

impl<T> BusTimerClock for T
where
    T: RccBus,
    T::Bus: BusTimerClock,
{
    fn timer_clock(clocks: &Clocks) -> Hertz {
        T::Bus::timer_clock(clocks)
    }
}

impl BusTimerClock for APB1_1 {
    fn timer_clock(clocks: &Clocks) -> Hertz {
        clocks.apb1_tim_clk()
    }
}

impl BusTimerClock for APB1_2 {
    fn timer_clock(clocks: &Clocks) -> Hertz {
        clocks.apb1_tim_clk()
    }
}

impl BusTimerClock for APB2 {
    fn timer_clock(clocks: &Clocks) -> Hertz {
        clocks.apb2_tim_clk()
    }
}

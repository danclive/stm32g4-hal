use super::*;

/// ADC Clock Source selection
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ClockSource {
    /// Use the System Clock as Clock Source
    SystemClock,
    /// use the Internal PLL as Clock Source
    PLL_P,
}

impl From<ClockSource> for u8 {
    fn from(c: ClockSource) -> u8 {
        match c {
            ClockSource::PLL_P => 0b01,
            ClockSource::SystemClock => 0b10,
        }
    }
}

/// ClockMode config for the ADC
/// Check the datasheet for the maximum speed the ADC supports
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ClockMode {
    /// (Asynchronous clock mode), adc_ker_ck. generated at product level (refer to Section 6: Reset and clock control (RCC)
    Asynchronous,
    /// (Synchronous clock mode). adc_hclk/1 This configuration must be enabled only if the AHB clock prescaler is set to 1 and if the system clock has a 50% duty cycle.
    Synchronous_Div_1,
    /// Synchronous clock mode. adc_hclk/2
    Synchronous_Div_2,
    /// Synchronous clock mode. adc_hclk/4
    Synchronous_Div_4,
}

impl From<ClockMode> for u8 {
    fn from(c: ClockMode) -> u8 {
        match c {
            ClockMode::Asynchronous => 0b00,
            ClockMode::Synchronous_Div_1 => 0b001,
            ClockMode::Synchronous_Div_2 => 0b010,
            ClockMode::Synchronous_Div_4 => 0b011,
        }
    }
}

impl From<u8> for ClockMode {
    fn from(b: u8) -> ClockMode {
        match b {
            0b000 => ClockMode::Asynchronous,
            0b001 => ClockMode::Synchronous_Div_1,
            0b010 => ClockMode::Synchronous_Div_2,
            0b011 => ClockMode::Synchronous_Div_4,
            _ => unimplemented!(),
        }
    }
}

/// Clock config for the ADC
/// Check the datasheet for the maximum speed the ADC supports
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Clock {
    /// Clock not divided
    DIV_1,
    /// Clock divided by 2
    DIV_2,
    /// Clock divided by 4
    DIV_4,
    /// Clock divided by 6
    DIV_6,
    /// Clock divided by 8
    DIV_8,
    /// Clock divided by 10
    DIV_10,
    /// Clock divided by 12
    DIV_12,
    /// Clock divided by 16
    DIV_16,
    /// Clock divided by 32
    DIV_32,
    /// Clock divided by 64
    DIV_64,
    /// Clock divided by 128
    DIV_128,
    /// Clock divided by 256
    DIV_256,
}

impl From<Clock> for u8 {
    fn from(c: Clock) -> u8 {
        match c {
            Clock::DIV_1 => 0b000,
            Clock::DIV_2 => 0b001,
            Clock::DIV_4 => 0b010,
            Clock::DIV_6 => 0b011,
            Clock::DIV_8 => 0b100,
            Clock::DIV_10 => 0b101,
            Clock::DIV_12 => 0b110,
            Clock::DIV_16 => 0b111,
            Clock::DIV_32 => 0b1000,
            Clock::DIV_64 => 0b1001,
            Clock::DIV_128 => 0b1010,
            Clock::DIV_256 => 0b1011,
        }
    }
}

impl From<u8> for Clock {
    fn from(b: u8) -> Clock {
        match b {
            0b000 => Clock::DIV_1,
            0b001 => Clock::DIV_2,
            0b010 => Clock::DIV_4,
            0b011 => Clock::DIV_6,
            0b100 => Clock::DIV_8,
            0b101 => Clock::DIV_10,
            0b110 => Clock::DIV_12,
            0b111 => Clock::DIV_16,
            0b1000 => Clock::DIV_32,
            0b1001 => Clock::DIV_64,
            0b1010 => Clock::DIV_128,
            0b1011 => Clock::DIV_256,
            _ => unimplemented!(),
        }
    }
}

/// The number of cycles to sample a given channel for
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SampleTime {
    /// 2.5 cycles
    Cycles_2_5,
    /// 6.5 cycles
    Cycles_6_5,
    /// 12_5 cycles
    Cycles_12_5,
    /// 24.5 cycles
    Cycles_24_5,
    /// 47.5 cycles
    Cycles_47_5,
    /// 92.5 cycles
    Cycles_92_5,
    /// 247.5 cycles
    Cycles_247_5,
    /// 640.5 cycles
    Cycles_640_5,
}

impl From<u8> for SampleTime {
    fn from(f: u8) -> SampleTime {
        match f {
            0 => SampleTime::Cycles_2_5,
            1 => SampleTime::Cycles_6_5,
            2 => SampleTime::Cycles_12_5,
            3 => SampleTime::Cycles_24_5,
            4 => SampleTime::Cycles_47_5,
            5 => SampleTime::Cycles_92_5,
            6 => SampleTime::Cycles_247_5,
            7 => SampleTime::Cycles_640_5,
            _ => unimplemented!(),
        }
    }
}

impl From<SampleTime> for u8 {
    fn from(l: SampleTime) -> u8 {
        match l {
            SampleTime::Cycles_2_5 => 0,
            SampleTime::Cycles_6_5 => 1,
            SampleTime::Cycles_12_5 => 2,
            SampleTime::Cycles_24_5 => 3,
            SampleTime::Cycles_47_5 => 4,
            SampleTime::Cycles_92_5 => 5,
            SampleTime::Cycles_247_5 => 6,
            SampleTime::Cycles_640_5 => 7,
        }
    }
}

impl Default for SampleTime {
    fn default() -> SampleTime {
        SampleTime::Cycles_2_5
    }
}

/// Possible oversampling shift
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OverSamplingShift {
    /// No right shift
    NoShift,
    /// Shift of 1 toward the right
    Shift_1,
    /// Shift of 2 toward the right
    Shift_2,
    /// Shift of 3 toward the right
    Shift_3,
    /// Shift of 4 toward the right
    Shift_4,
    /// Shift of 5 toward the right
    Shift_5,
    /// Shift of 6 toward the right
    Shift_6,
    /// Shift of 7 toward the right
    Shift_7,
    /// Shift of 8 toward the right
    Shift_8,
}
impl From<OverSamplingShift> for u8 {
    fn from(oss: OverSamplingShift) -> u8 {
        match oss {
            OverSamplingShift::NoShift => 0,
            OverSamplingShift::Shift_1 => 1,
            OverSamplingShift::Shift_2 => 2,
            OverSamplingShift::Shift_3 => 3,
            OverSamplingShift::Shift_4 => 4,
            OverSamplingShift::Shift_5 => 5,
            OverSamplingShift::Shift_6 => 6,
            OverSamplingShift::Shift_7 => 7,
            OverSamplingShift::Shift_8 => 8,
        }
    }
}

/// Possible oversampling modes
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OverSampling {
    /// Oversampling 2x
    Ratio_2,
    /// Oversampling 4x
    Ratio_4,
    /// Oversampling 8x
    Ratio_8,
    /// Oversampling 16x
    Ratio_16,
    /// Oversampling 32x
    Ratio_32,
    /// Oversampling 64x
    Ratio_64,
    /// Oversampling 128x
    Ratio_128,
    /// Oversampling 256x
    Ratio_256,
}
impl From<OverSampling> for u8 {
    fn from(os: OverSampling) -> u8 {
        match os {
            OverSampling::Ratio_2 => 0,
            OverSampling::Ratio_4 => 1,
            OverSampling::Ratio_8 => 2,
            OverSampling::Ratio_16 => 3,
            OverSampling::Ratio_32 => 4,
            OverSampling::Ratio_64 => 5,
            OverSampling::Ratio_128 => 6,
            OverSampling::Ratio_256 => 7,
        }
    }
}

/// The place in the sequence a given channel should be captured
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Sequence {
    /// 1
    SEQ_1,
    /// 2
    SEQ_2,
    /// 3
    SEQ_3,
    /// 4
    SEQ_4,
    /// 5
    SEQ_5,
    /// 6
    SEQ_6,
    /// 7
    SEQ_7,
    /// 8
    SEQ_8,
    /// 9
    SEQ_9,
    /// 10
    SEQ_10,
    /// 11
    SEQ_11,
    /// 12
    SEQ_12,
    /// 13
    SEQ_13,
    /// 14
    SEQ_14,
    /// 15
    SEQ_15,
    /// 16
    SEQ_16,
}

impl From<Sequence> for u8 {
    fn from(s: Sequence) -> u8 {
        match s {
            Sequence::SEQ_1 => 0,
            Sequence::SEQ_2 => 1,
            Sequence::SEQ_3 => 2,
            Sequence::SEQ_4 => 3,
            Sequence::SEQ_5 => 4,
            Sequence::SEQ_6 => 5,
            Sequence::SEQ_7 => 6,
            Sequence::SEQ_8 => 7,
            Sequence::SEQ_9 => 8,
            Sequence::SEQ_10 => 9,
            Sequence::SEQ_11 => 10,
            Sequence::SEQ_12 => 11,
            Sequence::SEQ_13 => 12,
            Sequence::SEQ_14 => 13,
            Sequence::SEQ_15 => 14,
            Sequence::SEQ_16 => 15,
        }
    }
}

impl From<u8> for Sequence {
    fn from(bits: u8) -> Self {
        match bits {
            0 => Sequence::SEQ_1,
            1 => Sequence::SEQ_2,
            2 => Sequence::SEQ_3,
            3 => Sequence::SEQ_4,
            4 => Sequence::SEQ_5,
            5 => Sequence::SEQ_6,
            6 => Sequence::SEQ_7,
            7 => Sequence::SEQ_8,
            8 => Sequence::SEQ_9,
            9 => Sequence::SEQ_10,
            10 => Sequence::SEQ_11,
            11 => Sequence::SEQ_12,
            12 => Sequence::SEQ_13,
            13 => Sequence::SEQ_14,
            14 => Sequence::SEQ_15,
            15 => Sequence::SEQ_16,
            _ => unimplemented!(),
        }
    }
}

/// Resolution to sample at
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Resolution {
    /// 12-bit
    Twelve,
    /// 10-bit
    Ten,
    /// 8-bit
    Eight,
    /// 6-bit
    Six,
}
impl Resolution {
    /// Return the maximum value of a sample with the given Resolution
    pub fn to_max_sample(self) -> u32 {
        match self {
            Resolution::Twelve => (1 << 12) - 1,
            Resolution::Ten => (1 << 10) - 1,
            Resolution::Eight => (1 << 8) - 1,
            Resolution::Six => (1 << 6) - 1,
        }
    }
}
impl From<Resolution> for u8 {
    fn from(r: Resolution) -> u8 {
        match r {
            Resolution::Twelve => 0b00,
            Resolution::Ten => 0b01,
            Resolution::Eight => 0b10,
            Resolution::Six => 0b11,
        }
    }
}
impl From<u8> for Resolution {
    fn from(r: u8) -> Resolution {
        match r {
            0b00 => Resolution::Twelve,
            0b01 => Resolution::Ten,
            0b10 => Resolution::Eight,
            0b11 => Resolution::Six,
            _ => unimplemented!(),
        }
    }
}

impl Default for Resolution {
    fn default() -> Resolution {
        Resolution::Twelve
    }
}

/// Data register alignment
#[derive(Debug, Clone, Copy)]
pub enum Align {
    /// Right align output data
    Right,
    /// Left align output data
    Left,
}
impl From<Align> for bool {
    fn from(a: Align) -> bool {
        match a {
            Align::Right => false,
            Align::Left => true,
        }
    }
}

/// Continuous mode enable/disable
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Continuous {
    /// Single mode, continuous disabled
    Single,
    /// Continuous mode enabled
    Continuous,

    /// Discontinuous mode enabled
    ///
    /// Will perform `subgroup_len` number samples per trigger
    Discontinuous,
}

/// Number of channels to sample per trigger in discontinuous mode
///
/// NOTE: This only applies to discontinuous
#[derive(Debug, Clone, Copy)]
pub enum SubGroupLength {
    /// One single sample per trigger
    One = 0b000,

    /// Two samples per trigger
    Two = 0b001,

    /// Three samples per trigger
    Three = 0b010,

    /// Four samples per trigger
    Four = 0b011,

    /// Five samples per trigger
    Five = 0b100,

    /// Six samples per trigger
    Six = 0b101,

    /// Seven samples per trigger
    Seven = 0b110,

    /// Eight samples per trigger
    Eight = 0b111,
}

/// DMA mode
#[derive(Debug, Clone, Copy)]
pub enum Dma {
    /// No DMA, disabled
    Disabled,
    /// Single DMA, DMA will be disabled after each conversion sequence
    Single,
    /// Continuous DMA, DMA will remain enabled after conversion
    Continuous,
}

/// End-of-conversion interrupt enabled/disabled
#[derive(Debug, Clone, Copy)]
pub enum Eoc {
    /// End-of-conversion interrupt disabled
    Disabled,
    /// End-of-conversion interrupt enabled per conversion
    Conversion,
    /// End-of-conversion interrupt enabled per sequence
    Sequence,
}

/// Input Type Selection
#[derive(Debug, Clone, Copy)]
pub enum InputType {
    /// Single-Ended Input Channels
    SingleEnded,
    /// Differential Input Channels
    Differential,
}
impl From<InputType> for bool {
    fn from(it: InputType) -> bool {
        match it {
            InputType::SingleEnded => false,
            InputType::Differential => true,
        }
    }
}

/// Sets the input type per channel
#[derive(Debug, Clone, Copy, Default)]
pub struct DifferentialSelection(pub(crate) u32);

impl DifferentialSelection {
    /// Set pin to Single-Ended or Differential
    pub fn set<PIN, ADC>(&mut self, it: InputType)
    where
        PIN: Channel<ADC, ID = u8>,
    {
        match it {
            InputType::SingleEnded => {
                self.singleended_by_id(PIN::channel());
            }
            InputType::Differential => {
                self.differential_by_id(PIN::channel());
            }
        }
    }

    /// Set to single ended by id
    fn singleended_by_id(&mut self, id: u8) {
        self.0 &= !(1 << id);
    }

    /// Set to differential by id
    fn differential_by_id(&mut self, id: u8) {
        self.0 |= 1 << id;
    }

    /// get the differential setting of channel
    pub fn get_channel(&self, channel: u8) -> InputType {
        if self.0 & (1 << channel) > 0 {
            InputType::Differential
        } else {
            InputType::SingleEnded
        }
    }

    /// Sets all channels to SingleEnded
    pub fn clear_all(&mut self) {
        self.0 = 0;
    }
}

/// Possible trigger modes
#[derive(Debug, Clone, Copy)]
pub enum TriggerMode {
    /// Don't listen to external trigger
    Disabled,
    /// Listen for rising edges of external trigger
    RisingEdge,
    /// Listen for falling edges of external trigger
    FallingEdge,
    /// Listen for both rising and falling edges of external trigger
    BothEdges,
}
impl From<TriggerMode> for u8 {
    fn from(tm: TriggerMode) -> u8 {
        match tm {
            TriggerMode::Disabled => 0,
            TriggerMode::RisingEdge => 1,
            TriggerMode::FallingEdge => 2,
            TriggerMode::BothEdges => 3,
        }
    }
}

/// Possible external triggers the ADC can listen to
///
/// This applies to ADC1, ADC2
#[derive(Debug, Clone, Copy, Default)]
pub enum ExternalTrigger {
    /// TIM1 compare channel 1
    #[default]
    Tim_1_cc_1,
    /// TIM1 compare channel 2
    Tim_1_cc_2,
    /// TIM1 compare channel 3
    Tim_1_cc_3,
    /// TIM2 compare channel 2
    Tim_2_cc_2,
    /// TIM3 trigger out
    Tim_3_trgo,
    /// TIM4 compare channel 4
    Tim_4_cc_4,
    /// External interupt line 11
    Exti_11,
    /// TIM8 trigger out
    Tim_8_trgo,
    /// TIM8 trigger out 2
    Tim_8_trgo_2,
    /// TIM1 trigger out
    Tim_1_trgo,
    /// TIM1 trigger out 2
    Tim_1_trgo_2,
    /// TIM2 trigger out
    Tim_2_trgo,
    /// TIM4 trigger out
    Tim_4_trgo,
    /// TIM6 trigger out
    Tim_6_trgo,
    /// TIM15 trigger out
    Tim_15_trgo,
    /// TIM3 compare channel 4
    Tim_3_cc_4,
    /// TIM20 trigger out
    Tim_20_trgo,
    /// TIM20 trigger out 2
    Tim_20_trgo_2,
    /// TIM20 compare channel 1
    Tim_20_cc_1,
    /// TIM20 compare channel 2
    Tim_20_cc_2,
    /// TIM20 compare channel 3
    Tim_20_cc_3,
    /// hrtim_adc_trg1
    Hrtim_adc_trg_1,
    /// hrtim_adc_trg3
    Hrtim_adc_trg_3,
    /// hrtim_adc_trg5
    Hrtim_adc_trg_5,
    /// hrtim_adc_trg6
    Hrtim_adc_trg_6,
    /// hrtim_adc_trg7
    Hrtim_adc_trg_7,
    /// hrtim_adc_trg8
    Hrtim_adc_trg_8,
    /// hrtim_adc_trg9
    Hrtim_adc_trg_9,
    /// hrtim_adc_trg10
    Hrtim_adc_trg_10,
    /// LP_timeout
    Lp_timeout,
    /// TIM7 trigger out
    Tim_7_trgo,
}

impl From<ExternalTrigger> for u8 {
    fn from(et: ExternalTrigger) -> u8 {
        match et {
            ExternalTrigger::Tim_1_cc_1 => 0b00000,
            ExternalTrigger::Tim_1_cc_2 => 0b00001,
            ExternalTrigger::Tim_1_cc_3 => 0b00010,
            ExternalTrigger::Tim_2_cc_2 => 0b00011,
            ExternalTrigger::Tim_3_trgo => 0b00100,
            ExternalTrigger::Tim_4_cc_4 => 0b00101,
            ExternalTrigger::Exti_11 => 0b00110,
            ExternalTrigger::Tim_8_trgo => 0b00111,
            ExternalTrigger::Tim_8_trgo_2 => 0b01000,
            ExternalTrigger::Tim_1_trgo => 0b01001,
            ExternalTrigger::Tim_1_trgo_2 => 0b01010,
            ExternalTrigger::Tim_2_trgo => 0b01011,
            ExternalTrigger::Tim_4_trgo => 0b01100,
            ExternalTrigger::Tim_6_trgo => 0b01101,
            ExternalTrigger::Tim_15_trgo => 0b01110,
            ExternalTrigger::Tim_3_cc_4 => 0b01111,
            ExternalTrigger::Tim_20_trgo => 0b10000,
            ExternalTrigger::Tim_20_trgo_2 => 0b10001,
            ExternalTrigger::Tim_20_cc_1 => 0b10010,
            ExternalTrigger::Tim_20_cc_2 => 0b10011,
            ExternalTrigger::Tim_20_cc_3 => 0b10100,
            ExternalTrigger::Hrtim_adc_trg_1 => 0b10101,
            ExternalTrigger::Hrtim_adc_trg_3 => 0b10110,
            ExternalTrigger::Hrtim_adc_trg_5 => 0b10111,
            ExternalTrigger::Hrtim_adc_trg_6 => 0b11000,
            ExternalTrigger::Hrtim_adc_trg_7 => 0b11001,
            ExternalTrigger::Hrtim_adc_trg_8 => 0b11010,
            ExternalTrigger::Hrtim_adc_trg_9 => 0b11011,
            ExternalTrigger::Hrtim_adc_trg_10 => 0b11100,
            ExternalTrigger::Lp_timeout => 0b11101,
            ExternalTrigger::Tim_7_trgo => 0b11110,
            // Reserved => 0b11111
        }
    }
}

/// Configuration for the adc.
/// There are some additional parameters on the adc peripheral that can be
/// added here when needed but this covers several basic usecases.
#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub clock_mode: ClockMode,
    pub resolution: Resolution,
    pub align: Align,
    pub external_trigger: (TriggerMode, ExternalTrigger),
    pub continuous: Continuous,
    pub subgroup_len: SubGroupLength,
    pub end_of_conversion_interrupt: Eoc,
    pub overrun_interrupt: bool,
    pub sample_time: SampleTime,
    pub vdda: Option<u32>,
    pub auto_delay: bool,

    /// Sets the differential input type of the Adc
    pub difsel: DifferentialSelection,
    pub dma: Dma,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            clock_mode: ClockMode::Synchronous_Div_1,
            resolution: Resolution::Twelve,
            align: Align::Right,
            external_trigger: (TriggerMode::Disabled, ExternalTrigger::default()),
            continuous: Continuous::Single,
            subgroup_len: SubGroupLength::One,
            end_of_conversion_interrupt: Eoc::Disabled,
            overrun_interrupt: false,
            sample_time: SampleTime::Cycles_640_5,
            vdda: None,
            auto_delay: false,
            difsel: DifferentialSelection::default(),
            dma: Dma::Disabled,
        }
    }
}

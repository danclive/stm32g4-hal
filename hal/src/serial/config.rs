use fugit::HertzU32 as Hertz;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum WordLength {
    DataBits7,
    DataBits8,
    DataBits9,
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum Parity {
    ParityNone,
    ParityEven,
    ParityOdd,
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum StopBits {
    #[doc = "1 stop bit"]
    STOP1 = 0b00,
    #[doc = "0.5 stop bits"]
    STOP0P5 = 0b01,
    #[doc = "2 stop bits"]
    STOP2 = 0b10,
    #[doc = "1.5 stop bits"]
    STOP1P5 = 0b11,
}

impl StopBits {
    pub fn bits(self) -> u8 {
        self as u8
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum FifoThreshold {
    #[doc = "1/8 of its depth"]
    FIFO_1_BYTE = 0b000,
    #[doc = "1/4 of its depth"]
    FIFO_2_BYTES = 0b001,
    #[doc = "1/2 of its depth"]
    FIFO_4_BYTES = 0b010,
    #[doc = "3/4 of its depth"]
    FIFO_6_BYTES = 0b011,
    #[doc = "7/8 of its depth"]
    FIFO_7_BYTES = 0b100,
    #[doc = "fifo empty/full"]
    FIFO_8_BYTES = 0b101,
}

impl FifoThreshold {
    pub fn bits(self) -> u8 {
        self as u8
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct LPConfig {
    pub baudrate: Hertz,
    pub wordlength: WordLength,
    pub parity: Parity,
    pub stopbits: StopBits,
    pub swaptxrx: bool,
    pub invertrx: bool,
    pub inverttx: bool,
    pub fifo_enable: bool,
    pub tx_fifo_threshold: FifoThreshold,
    pub rx_fifo_threshold: FifoThreshold,
    pub tx_fifo_interrupt: bool,
    pub rx_fifo_interrupt: bool,
}

impl Default for LPConfig {
    fn default() -> LPConfig {
        let baudrate = Hertz::from_raw(19_200);
        LPConfig {
            baudrate,
            wordlength: WordLength::DataBits8,
            parity: Parity::ParityNone,
            stopbits: StopBits::STOP1,
            swaptxrx: false,
            invertrx: false,
            inverttx: false,
            fifo_enable: false,
            tx_fifo_threshold: FifoThreshold::FIFO_8_BYTES,
            rx_fifo_threshold: FifoThreshold::FIFO_8_BYTES,
            tx_fifo_interrupt: false,
            rx_fifo_interrupt: false,
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Config {
    pub baudrate: Hertz,
    pub wordlength: WordLength,
    pub parity: Parity,
    pub stopbits: StopBits,
    pub swaptxrx: bool,
    pub invertrx: bool,
    pub inverttx: bool,
    pub fifo_enable: bool,
    pub tx_fifo_threshold: FifoThreshold,
    pub rx_fifo_threshold: FifoThreshold,
    pub tx_fifo_interrupt: bool,
    pub rx_fifo_interrupt: bool,
    #[doc = "Number of bits no activity on rx line"]
    pub receiver_timeout: Option<u32>,
}

impl Default for Config {
    fn default() -> Config {
        let baudrate = Hertz::from_raw(115_200);
        Config {
            baudrate,
            wordlength: WordLength::DataBits8,
            parity: Parity::ParityNone,
            stopbits: StopBits::STOP1,
            swaptxrx: false,
            invertrx: false,
            inverttx: false,
            fifo_enable: false,
            tx_fifo_threshold: FifoThreshold::FIFO_8_BYTES,
            rx_fifo_threshold: FifoThreshold::FIFO_8_BYTES,
            tx_fifo_interrupt: false,
            rx_fifo_interrupt: false,
            receiver_timeout: None,
        }
    }
}

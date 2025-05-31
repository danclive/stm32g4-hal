use crate::bb;

use super::*;

macro_rules! bus_enable {
    ($PER:ident => $bit:literal) => {
        impl Enable for crate::pac::$PER {
            #[inline(always)]
            fn enable(rcc: &RccRB) {
                unsafe {
                    bb::set(Self::Bus::enr(rcc), $bit);
                }
                // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                // cortex_m::asm::dsb();
            }

            #[inline(always)]
            fn disable(rcc: &RccRB) {
                unsafe {
                    bb::clear(Self::Bus::enr(rcc), $bit);
                }
            }

            #[inline(always)]
            fn is_enabled() -> bool {
                let rcc = pac::Rcc::PTR;
                (Self::Bus::enr(unsafe { &*rcc }).read().bits() >> $bit) & 0x1 != 0
            }

            #[inline(always)]
            fn enable_for_sleep_stop(rcc: &RccRB) {
                unsafe {
                    bb::set(Self::Bus::smenr(rcc), $bit);
                }
            }
        }
    };
}

macro_rules! bus_reset {
    ($PER:ident => $bit:literal) => {
        impl Reset for crate::pac::$PER {
            #[inline(always)]
            fn reset(rcc: &RccRB) {
                unsafe {
                    bb::set(Self::Bus::rstr(rcc), $bit);
                    bb::clear(Self::Bus::rstr(rcc), $bit);
                }
            }
        }
    };
}

macro_rules! bus {
    ($busX:ty: { $($PER:ident: $bit:literal),+ $(,)*}) => {
        $(
            impl crate::Sealed for crate::pac::$PER {}
            impl RccBus for crate::pac::$PER {
                type Bus = $busX;
            }
            bus_enable!($PER => $bit);
            bus_reset!($PER => $bit);
        )+
    };
}

bus! {
    APB2: { Syscfg: 0 }
}

bus! {
    AHB1: {
        Dma1: 0,
        Dma2: 1,
        Dmamux: 2,
        Cordic: 3,
        Fmac: 4,
        Flash: 5,
        Crc: 12,
    }
}

bus! {
    AHB2: {
        Gpioa: 0,
        Gpiob: 1,
        Gpioc: 2,
        Gpiod: 3,
        Gpioe: 4,
        Gpiof: 5,
        Gpiog: 6,
        Adc1: 13,
        Adc2: 13,
        Dac1: 16,
        Dac2: 17,
        Dac3: 18,
        Dac4: 19,
        Rng: 26,
    }
}

#[cfg(any(
    feature = "stm32g471",
    feature = "stm32g473",
    feature = "stm32g474",
    feature = "stm32g483",
    feature = "stm32g484"
))]
bus! {
    AHB2: {
        Adc3: 14,
    }
}

#[cfg(any(
    feature = "stm32g473",
    feature = "stm32g474",
    feature = "stm32g483",
    feature = "stm32g484"
))]
bus! {
    AHB2: {
        Adc4: 14,
        Adc5: 14,
    }
}

#[cfg(any(feature = "stm32g431", feature = "stm32g441", feature = "stm32g484",))]
bus! {
    AHB2: {
        Aes: 24,
    }
}

#[cfg(any(
    feature = "stm32g473",
    feature = "stm32g474",
    feature = "stm32g483",
    feature = "stm32g484"
))]
bus! {
    AHB3: {
        Fmc: 0,
        Quadspi: 8,
    }
}

bus! {
    APB1_1: {
        Tim2: 0,
        Tim3: 1,
        Tim4: 2,
        Tim6: 4,
        Tim7: 5,
        Crs: 8,
        Spi2: 14,
        Spi3: 15,
        Usart2: 17,
        Usart3: 18,
        Uart4: 19,
        I2c1: 21,
        I2c2: 22,
        UsbFsDevice: 23,
        Fdcan1: 25,
        Pwr: 28,
        I2c3: 30,
        Lptimer1: 31,
    }
}

bus! {
    APB1_2: {
        Lpuart1: 0,
        Ucpd1: 8,
    }
}

#[cfg(any(
    feature = "stm32g471",
    feature = "stm32g473",
    feature = "stm32g474",
    feature = "stm32g483",
    feature = "stm32g484",
    feature = "stm32g491",
))]
bus! {
    APB1_1: {
        Fdcan2: 25,
    }
}

#[cfg(any(
    feature = "stm32g471",
    feature = "stm32g473",
    feature = "stm32g474",
    feature = "stm32g483",
    feature = "stm32g484"
))]
bus! {
    APB1_1: {
        Tim5: 3,
        Uart5: 20,
    }
}

#[cfg(any(
    feature = "stm32g471",
    feature = "stm32g473",
    feature = "stm32g474",
    feature = "stm32g483",
    feature = "stm32g484"
))]
bus! {
    APB1_2: {
        I2c4: 1,
    }
}

bus! {
    APB2: {
        Tim1: 11,
        Spi1: 12,
        Tim8: 13,
        Usart1: 14,
        Tim15: 16,
        Tim16: 17,
        Tim17: 18,
        Sai: 21,
    }
}

#[cfg(any(
    feature = "stm32g471",
    feature = "stm32g473",
    feature = "stm32g474",
    feature = "stm32g483",
    feature = "stm32g484"
))]
bus! {
    APB2: {
        Spi4: 15,
    }
}

#[cfg(any(
    feature = "stm32g473",
    feature = "stm32g474",
    feature = "stm32g483",
    feature = "stm32g484"
))]
bus! {
    APB1_1: {
        Fdcan3: 25,
    }
}

#[cfg(any(
    feature = "stm32g473",
    feature = "stm32g474",
    feature = "stm32g483",
    feature = "stm32g484"
))]
bus! {
    APB2: {
        Tim20: 20,
    }
}

#[cfg(any(feature = "stm32g474", feature = "stm32g484"))]
bus! {
    APB2: {
        HrtimTima: 26,
        HrtimTimb: 26,
        HrtimTimc: 26,
        HrtimTimd: 26,
        HrtimTime: 26,
        HrtimTimf: 26,
    }
}

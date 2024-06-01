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
                cortex_m::asm::dsb();
            }

            #[inline(always)]
            fn disable(rcc: &RccRB) {
                unsafe {
                    bb::clear(Self::Bus::enr(rcc), $bit);
                }
            }

            #[inline(always)]
            fn is_enabled() -> bool {
                let rcc = pac::Rcc::ptr();
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
    ($($PER:ident => ($busX:ty, $bit:literal),)+) => {
        $(
            impl crate::Sealed for crate::pac::$PER {}
            impl RccBus for crate::pac::$PER {
                type Bus = $busX;
            }
            bus_enable!($PER => $bit);
            bus_reset!($PER => $bit);
        )+
    }
}

bus! {
    Syscfg => (APB2, 0),
}

bus! {
    Dma1 => (AHB1, 0),
    Dma2 => (AHB1, 1),
    Dmamux => (AHB1, 2),
    Cordic => (AHB1, 3),
    Fmac => (AHB1, 4),
    Flash => (AHB1, 5),
    Crc => (AHB1, 12),
}

bus! {
    Gpioa => (AHB2, 0),
    Gpiob => (AHB2, 1),
    Gpioc => (AHB2, 2),
    Gpiod => (AHB2, 3),
    Gpioe => (AHB2, 4),
    Gpiof => (AHB2, 5),
    Gpiog => (AHB2, 6),
    Adc1 => (AHB2, 13),
    Adc2 => (AHB2, 13),
    Dac1 => (AHB2, 16),
    Dac2 => (AHB2, 17),
    Dac3 => (AHB2, 18),
    Dac4 => (AHB2, 19),
    Rng => (AHB2, 26),
}

#[cfg(any(
    feature = "stm32g471",
    feature = "stm32g473",
    feature = "stm32g474",
    feature = "stm32g483",
    feature = "stm32g484"
))]
bus! {
    Adc3 => (AHB2, 14),
}

#[cfg(any(
    feature = "stm32g473",
    feature = "stm32g474",
    feature = "stm32g483",
    feature = "stm32g484"
))]
bus! {
    Adc4 => (AHB2, 14),
    Adc5 => (AHB2, 14),
}

#[cfg(any(feature = "stm32g431", feature = "stm32g441", feature = "stm32g484",))]
bus! {
    Aes => (AHB2, 24),
}

#[cfg(any(
    feature = "stm32g473",
    feature = "stm32g474",
    feature = "stm32g483",
    feature = "stm32g484"
))]
bus! {
    Fmc => (AHB3, 0),
    Quadspi => (AHB3, 8),
}

bus! {
    Tim2 => (APB1_1, 0),
    Tim3 => (APB1_1, 1),
    Tim4 => (APB1_1, 2),
    Tim6 => (APB1_1, 4),
    Tim7 => (APB1_1, 5),
    Crs => (APB1_1, 8),
    Spi2 => (APB1_1, 14),
    Spi3 => (APB1_1, 15),
    Usart2 => (APB1_1, 17),
    Usart3 => (APB1_1, 18),
    Uart4 => (APB1_1, 19),
    I2c1 => (APB1_1, 21),
    I2c2 => (APB1_1, 22),
    UsbFsDevice => (APB1_1, 23),
    Fdcan1 => (APB1_1, 25),
    Pwr => (APB1_1, 28),
    I2c3 => (APB1_1, 30),
    Lptimer1 => (APB1_1, 31),
    Lpuart1 => (APB1_2, 0),
    Ucpd1 => (APB1_2, 8),
}

#[cfg(any(
    feature = "stm32g471",
    feature = "stm32g473",
    feature = "stm32g474",
    feature = "stm32g483",
    feature = "stm32g484",
    feature = "stm32g491",
    feature = "stm32g4A1"
))]
bus! {
    Fdcan2 => (APB1_1, 25),
}

#[cfg(any(
    feature = "stm32g471",
    feature = "stm32g473",
    feature = "stm32g474",
    feature = "stm32g483",
    feature = "stm32g484"
))]
bus! {
    Tim5 => (APB1_1, 3),
    Uart5 => (APB1_1, 20),
    I2c4 => (APB1_2, 1),
}

bus! {
    Tim1 => (APB2, 11),
    Spi1 => (APB2, 12),
    Tim8 => (APB2, 13),
    Usart1 => (APB2, 14),
    Tim15 => (APB2, 16),
    Tim16 => (APB2, 17),
    Tim17 => (APB2, 18),
    Sai => (APB2, 21),
}

#[cfg(any(
    feature = "stm32g471",
    feature = "stm32g473",
    feature = "stm32g474",
    feature = "stm32g483",
    feature = "stm32g484"
))]
bus! {
    Spi4 => (APB2, 15),
}

#[cfg(any(
    feature = "stm32g473",
    feature = "stm32g474",
    feature = "stm32g483",
    feature = "stm32g484"
))]
bus! {
    Fdcan3 => (APB1_1, 25),
    Tim20 => (APB2, 20),
}

#[cfg(any(feature = "stm32g474", feature = "stm32g484"))]
bus! {
    HrtimTima => (APB2, 26),
    HrtimTimb => (APB2, 26),
    HrtimTimc => (APB2, 26),
    HrtimTimd => (APB2, 26),
    HrtimTime => (APB2, 26),
    HrtimTimf => (APB2, 26),
}

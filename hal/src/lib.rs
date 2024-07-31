#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![no_std]

pub use crate::pac::interrupt;

pub mod adc;
pub mod bb;
pub mod dac;
pub mod delay;
pub mod dma;
pub mod gpio;
pub mod i2c;
pub mod opamp;
pub mod pwm;
pub mod pwm_input;
pub mod pwr;
pub mod rcc;
pub mod signature;
pub mod syscfg;
pub mod timer;

#[cfg(feature = "rtic")]
pub mod rtic;

pub mod prelude;

pub use fugit;

mod sealed {
    pub trait Sealed {}
}
pub(crate) use sealed::Sealed;

fn stripped_type_name<T>() -> &'static str {
    let s = core::any::type_name::<T>();
    let p = s.split("::");
    p.last().unwrap()
}

pub use stm32g4_pac;

#[cfg(feature = "stm32g431")]
pub use stm32g4_pac::stm32g431 as pac;
#[cfg(feature = "stm32g441")]
pub use stm32g4_pac::stm32g441 as pac;
#[cfg(feature = "stm32g471")]
pub use stm32g4_pac::stm32g471 as pac;
#[cfg(feature = "stm32g473")]
pub use stm32g4_pac::stm32g473 as pac;
#[cfg(feature = "stm32g474")]
pub use stm32g4_pac::stm32g474 as pac;
#[cfg(feature = "stm32g483")]
pub use stm32g4_pac::stm32g483 as pac;
#[cfg(feature = "stm32g484")]
pub use stm32g4_pac::stm32g484 as pac;
#[cfg(feature = "stm32g491")]
pub use stm32g4_pac::stm32g491 as pac;

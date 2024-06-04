#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

#[cfg(feature = "rt")]
pub use crate::pac::interrupt;

pub mod bb;
pub mod gpio;
pub mod pwr;
pub mod rcc;
pub mod syscfg;

mod sealed {
    pub trait Sealed {}
}
pub(crate) use sealed::Sealed;

fn stripped_type_name<T>() -> &'static str {
    let s = core::any::type_name::<T>();
    let p = s.split("::");
    p.last().unwrap()
}

#[cfg(feature = "stm32g431")]
pub use stm32g4_pac::{generic, stm32g431 as pac};
#[cfg(feature = "stm32g441")]
pub use stm32g4_pac::{generic, stm32g441 as pac};
#[cfg(feature = "stm32g471")]
pub use stm32g4_pac::{generic, stm32g471 as pac};
#[cfg(feature = "stm32g473")]
pub use stm32g4_pac::{generic, stm32g473 as pac};
#[cfg(feature = "stm32g474")]
pub use stm32g4_pac::{generic, stm32g474 as pac};
#[cfg(feature = "stm32g483")]
pub use stm32g4_pac::{generic, stm32g483 as pac};
#[cfg(feature = "stm32g484")]
pub use stm32g4_pac::{generic, stm32g484 as pac};
#[cfg(feature = "stm32g491")]
pub use stm32g4_pac::{generic, stm32g491 as pac};

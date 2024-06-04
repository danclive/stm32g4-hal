#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

pub mod generic;
use generic::*;

#[cfg(feature = "stm32g431")]
pub mod stm32g431;
#[cfg(feature = "stm32g441")]
pub mod stm32g441;
#[cfg(feature = "stm32g471")]
pub mod stm32g471;
#[cfg(feature = "stm32g473")]
pub mod stm32g473;
#[cfg(feature = "stm32g474")]
pub mod stm32g474;
#[cfg(feature = "stm32g483")]
pub mod stm32g483;
#[cfg(feature = "stm32g484")]
pub mod stm32g484;
#[cfg(feature = "stm32g491")]
pub mod stm32g491;

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

cfg_if::cfg_if! {
    if #[cfg(feature = "stm32g431")] {
        #[path = "stm32g431/generic.rs"]
        pub mod generic;

        pub mod stm32g431;
    }
}

use generic::*;

// "stm32g441",
// "stm32g471",
// "stm32g473",
// "stm32g474",
// "stm32g483",
// "stm32g484",
// "stm32g491"

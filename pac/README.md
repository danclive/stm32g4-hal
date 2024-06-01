# PACs for STM32G4 microcontrollers

This repository contains Peripheral Access Crates (PACs) for STM32G4 series Cortex-M microcontrollers.

All these crates are automatically generated using [svd2rust].

For a more user-friendly interface to the peripherals, the [`stm32g4-hal`] crates might be more appropriate.

Please refer to the [changelog] to see what changed in the last releases.

[changelog]: ./CHANGELOG.md
[`stm32g4-hal`]: https://github.com/danclive/stm32g4-hal
[svd2rust]: https://github.com/rust-embedded/svd2rust

## Supported Devices

* stm32g431
* stm32g441
* stm32g471
* stm32g473
* stm32g474
* stm32g483
* stm32g484
* stm32g491

## Usage

To use, add line to your Cargo.toml:

```toml
stm32g4-pac = { version = "0.0.1", features = ["stm32g431"] }
```

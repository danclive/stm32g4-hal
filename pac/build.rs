use std::path::PathBuf;
use std::{env, fs};

fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());

        let (device_file, svd_file) = if env::var_os("CARGO_FEATURE_STM32G431").is_some() {
            ("src/stm32g431/device.x", "svds/STM32G431.svd")
        } else if env::var_os("CARGO_FEATURE_STM32G441").is_some() {
            ("src/stm32g441/device.x", "svds/STM32G441.svd")
        } else if env::var_os("CARGO_FEATURE_STM32G471").is_some() {
            ("src/stm32g471/device.x", "svds/STM32G471.svd")
        } else if env::var_os("CARGO_FEATURE_STM32G473").is_some() {
            ("src/stm32g473/device.x", "svds/STM32G473.svd")
        } else if env::var_os("CARGO_FEATURE_STM32G474").is_some() {
            ("src/stm32g474/device.x", "svds/STM32G474.svd")
        } else if env::var_os("CARGO_FEATURE_STM32G483").is_some() {
            ("src/stm32g483/device.x", "svds/STM32G483.svd")
        } else if env::var_os("CARGO_FEATURE_STM32G484").is_some() {
            ("src/stm32g484/device.x", "svds/STM32G484.svd")
        } else if env::var_os("CARGO_FEATURE_STM32G491").is_some() {
            ("src/stm32g491/device.x", "svds/STM32G491.svd")
        } else {
            panic!("No device features selected");
        };

        fs::copy(device_file, out.join("device.x")).unwrap();
        fs::copy(svd_file, out.join("peripheral.svd")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }

    println!("cargo:rerun-if-changed=build.rs");
}

use std::path::PathBuf;
use std::{env, fs};

fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());

        let (device_file, svd_file) = if env::var_os("CARGO_FEATURE_STM32G431").is_some() {
            ("src/stm32g431/device.x", "svds/STM32G431.svd")
        } else {
            panic!("No device features selected");
        };

        fs::copy(device_file, out.join("device.x")).unwrap();
        fs::copy(svd_file, out.join("peripheral.svd")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }

    println!("cargo:rerun-if-changed=build.rs");
}

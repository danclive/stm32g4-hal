use std::fs;
use xshell::{cmd, Shell};

pub static CHIPS: &[(&str, &str)] = &[
    ("stm32g431", "STM32G431"),
    ("stm32g441", "STM32G441"),
    ("stm32g471", "STM32G471"),
    ("stm32g473", "STM32G473"),
    ("stm32g474", "STM32G474"),
    ("stm32g483", "STM32G483"),
    ("stm32g484", "STM32G484"),
    ("stm32g491", "STM32G491"),
];

pub fn generate() {
    let sh = Shell::new().unwrap();

    for (chip, svd) in CHIPS {
        let svd_path = format!("svds/{}.svd", svd);
        let crate_dir = format!("src/{}", chip);

        let _ = fs::remove_dir_all(&crate_dir);
        fs::create_dir_all(&crate_dir).unwrap();

        cmd!(
            sh,
            "svd2rust -m -g -s -i {svd_path} -o {crate_dir} --reexport-core-peripherals --reexport-interrupt --atomics --atomics_feature atomics --impl-debug --impl-defmt defmt"
        )
        .run()
        .unwrap();

        cmd!(sh, "form -i {crate_dir}/mod.rs -o {crate_dir}")
            .run()
            .unwrap();

        cmd!(sh, "rm {crate_dir}/mod.rs").run().unwrap();

        cmd!(sh, "mv {crate_dir}/lib.rs {crate_dir}/mod.rs")
            .run()
            .unwrap();
    }

    cmd!(sh, "cargo fmt").run().unwrap();
}

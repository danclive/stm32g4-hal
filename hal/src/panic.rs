use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[cfg(feature = "defmt")]
pub fn panic(info: &PanicInfo) -> ! {
    use defmt::error as println;

    if let Some(location) = info.location() {
        let (file, line, column) = (location.file(), location.line(), location.column());
        println!(
            "!! PANIC !!\n!! A panic occured in '{}', at line {}, column {}:\n!! {:#?}\n",
            file,
            line,
            column,
            defmt::Display2Format(&info.message())
        );
    } else {
        println!(
            "!! PANIC !!\n!! {:#?}\n",
            defmt::Display2Format(&info.message())
        );
    }

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

#[cfg(not(feature = "defmt"))]
pub fn panic(info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

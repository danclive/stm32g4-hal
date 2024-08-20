use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[cfg(feature = "defmt")]
pub fn panic(info: &PanicInfo) -> ! {
    use defmt::error as println;

    println!("!! PANIC !!");

    if let Some(location) = info.location() {
        let (file, line, column) = (location.file(), location.line(), location.column());
        println!(
            "!! A panic occured in '{}', at line {}, column {}:",
            file, line, column
        );
    } else {
        println!("!! A panic occured at an unknown location:");
    }

    println!("{:#?}", defmt::Display2Format(info));

    // if let Some(message) = info.message() {
    //     println!("{}", defmt::Display2Format(message));
    // }

    if let Some(s) = info.payload().downcast_ref::<&str>() {
        println!("panic occurred: {}", s);
    } else {
        println!("panic occurred");
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

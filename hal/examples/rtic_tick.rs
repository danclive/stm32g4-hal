//! Blinks an LED

// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

use stm32g4_hal as hal;

use crate::hal::gpio::{gpioc, Output, Pin};
use crate::hal::prelude::*;
use crate::hal::{pac, pwr, rcc, tim2_monotonic};

use rtic::app;

tim2_monotonic!(Mono, 1_000_000);

pub use defmt::{debug, error, info, trace, warn};
use defmt_rtt as _;

#[app(device = pac, peripherals = true, dispatchers = [SPI1])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: Pin<'C', 4, Output>,
    }

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local) {
        let p = ctx.device;

        let pwr = p
            .pwr
            .constrain()
            .vos(pwr::VoltageScale::Range1 { enable_boost: true })
            .freeze();
        let rcc = p.rcc.constrain();

        let (_rcc, clocks) = rcc
            .clock_src(rcc::SysClockSrc::PLL)
            .pll_cfg(rcc::PllConfig {
                mux: rcc::PLLSrc::HSE(25.MHz()),
                m: rcc::PllMDiv::DIV_5,
                n: rcc::PllNMul::MUL_68,
                r: Some(rcc::PllRDiv::DIV_2),
                q: Some(rcc::PllQDiv::DIV_2),
                p: Some(rcc::PllPDiv::DIV_2),
            })
            .pwr_cfg(pwr)
            .freeze();
        info!("clock: {:?}", clocks);

        info!("start");

        // Mono::start(&mut ctx.core.NVIC, p.tim2, &clocks);
        p.tim2.monotonic(&mut ctx.core.NVIC, &clocks);

        info!("Init Led");
        let gpioc = gpioc::Pins::new(p.gpioc);
        let led = gpioc.pc4.into_push_pull_output();

        // Spawn heartbeat task
        heartbeat::spawn().ok();

        (Shared {}, Local { led })
    }

    #[idle]
    fn idle(_ctx: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(local = [led], priority = 1)]
    async fn heartbeat(ctx: heartbeat::Context) {
        // Loop forever.
        loop {
            // Turn On LED
            info!("Set Led High");
            ctx.local.led.set_high();

            // Delay for 1 second
            // Mono::delay_ms(&mut Mono, 1000).await;
            Mono::delay(1000.millis().into()).await;

            // Turn off LED
            info!("Set Led low");
            ctx.local.led.set_low();

            // Delay for 1 second
            // Mono::delay_ms(&mut Mono, 1000).await;
            Mono::delay(1000.millis().into()).await;

            // Some backends provide a manual way of pending an
            // interrupt.
            rtic::pend(pac::Interrupt::USART1);
        }
    }

    #[task(binds = USART1, local = [times: u32 = 0])]
    fn uart1(cx: uart1::Context) {
        // Safe access to local `static mut` variable
        *cx.local.times += 1;

        info!(
            "UART1 called {} time{}",
            *cx.local.times,
            if *cx.local.times > 1 { "s" } else { "" }
        );
    }
}

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use defmt::error as println;

    println!("");
    println!("");

    if let Some(location) = info.location() {
        let (file, line, column) = (location.file(), location.line(), location.column());
        println!(
            "!! A panic occured in '{}', at line {}, column {}:",
            file, line, column
        );
    } else {
        println!("!! A panic occured at an unknown location:");
    }

    #[cfg(not(nightly))]
    {
        #[cfg(not(feature = "defmt"))]
        println!("{:#?}", info);

        #[cfg(feature = "defmt")]
        println!("{:#?}", defmt::Display2Format(info));
    }

    #[cfg(nightly)]
    {
        if let Some(message) = info.message() {
            #[cfg(not(feature = "defmt"))]
            println!("{}", message);

            #[cfg(feature = "defmt")]
            println!("{}", defmt::Display2Format(message));
        }
    }

    if let Some(s) = info.payload().downcast_ref::<&str>() {
        println!("panic occurred: {}", s);
    } else {
        println!("panic occurred");
    }

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

//! Blinks an LED

// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

use stm32g4_hal as hal;

use crate::hal::gpio::{Output, Pin};
use crate::hal::prelude::*;
use crate::hal::{pac, pwr, rcc::clock};

use crate::hal::{rtic::Monotonic, systick_monotonic};

use rtic::app;

systick_monotonic!(Mono, 1_000);

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
    fn init(ctx: init::Context) -> (Shared, Local) {
        let p = ctx.device;

        let pwr = p
            .pwr
            .constrain()
            .vos(pwr::VoltageScale::Range1 { enable_boost: true })
            .freeze();
        let rcc = p.rcc.constrain();

        let (_rcc, clocks) = rcc
            .clock_src(clock::SysClockSrc::PLL)
            .pll_cfg(clock::PllConfig {
                mux: clock::PLLSrc::HSE(25.MHz()),
                m: clock::PllMDiv::DIV_5,
                n: clock::PllNMul::MUL_68,
                r: Some(clock::PllRDiv::DIV_2),
                q: Some(clock::PllQDiv::DIV_2),
                p: Some(clock::PllPDiv::DIV_2),
            })
            .pwr_cfg(pwr)
            .freeze();
        info!("clock: {:?}", clocks);

        info!("start");
        // Initialize the systick interrupt & obtain the token to prove that we did
        Mono::start(ctx.core.SYST, clocks.sys_clk().to_Hz());

        info!("Init Led");
        let gpioc = p.gpioc.split();
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
            Mono::delay(1000.millis()).await;

            // Turn off LED
            info!("Set Led low");
            ctx.local.led.set_low();

            // Delay for 1 second
            // Mono::delay_ms(&mut Mono, 1000).await;
            Mono::delay(1000.millis()).await;

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
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

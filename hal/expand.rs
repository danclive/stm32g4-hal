#![feature(prelude_import)]
//! Blinks an LED
#![no_main]
#![no_std]
#[prelude_import]
use core::prelude::rust_2018::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use stm32g4_hal as hal;
use crate::hal::gpio::{Output, Pin, PushPull};
use crate::hal::pac;
use crate::hal::prelude::*;
use crate::hal::{rtic::systick, systick_monotonic};
use rtic::app;
/// A `Monotonic` based on SysTick.
pub struct Mono;
impl Mono {
    /// Starts the `Monotonic`.
    ///
    /// The `sysclk` parameter is the speed at which SysTick runs at. This value should come from
    /// the clock generation function of the used HAL.
    ///
    /// Panics if it is impossible to achieve the desired monotonic tick rate based
    /// on the given `sysclk` parameter. If that happens, adjust the desired monotonic tick rate.
    ///
    /// This method must be called only once.
    pub fn start(systick: ::stm32g4_hal::rtic::systick::SYST, sysclk: u32) {
        #[no_mangle]
        #[allow(non_snake_case)]
        unsafe extern "C" fn SysTick() {
            use ::stm32g4_hal::rtic::TimerQueueBackend;
            ::stm32g4_hal::rtic::systick::SystickBackend::timer_queue()
                .on_monotonic_interrupt();
        }
        ::stm32g4_hal::rtic::systick::SystickBackend::_start(systick, sysclk, 1_000);
    }
}
impl ::stm32g4_hal::rtic::TimerQueueBasedMonotonic for Mono {
    type Backend = ::stm32g4_hal::rtic::systick::SystickBackend;
    type Instant = ::stm32g4_hal::fugit::Instant<
        <Self::Backend as ::stm32g4_hal::rtic::TimerQueueBackend>::Ticks,
        1,
        { 1_000 },
    >;
    type Duration = ::stm32g4_hal::fugit::Duration<
        <Self::Backend as ::stm32g4_hal::rtic::TimerQueueBackend>::Ticks,
        1,
        { 1_000 },
    >;
}
impl ::rtic_time::embedded_hal::delay::DelayNs for Mono {
    fn delay_ns(&mut self, ns: u32) {
        let now = <Self as ::rtic_time::Monotonic>::now();
        let mut done = now
            + <Self as ::rtic_time::Monotonic>::Duration::nanos_at_least(ns.into());
        if now != done {
            done += <Self as ::rtic_time::Monotonic>::Duration::from_ticks(1);
        }
        while <Self as ::rtic_time::Monotonic>::now() < done {}
    }
    fn delay_us(&mut self, us: u32) {
        let now = <Self as ::rtic_time::Monotonic>::now();
        let mut done = now
            + <Self as ::rtic_time::Monotonic>::Duration::micros_at_least(us.into());
        if now != done {
            done += <Self as ::rtic_time::Monotonic>::Duration::from_ticks(1);
        }
        while <Self as ::rtic_time::Monotonic>::now() < done {}
    }
    fn delay_ms(&mut self, ms: u32) {
        let now = <Self as ::rtic_time::Monotonic>::now();
        let mut done = now
            + <Self as ::rtic_time::Monotonic>::Duration::millis_at_least(ms.into());
        if now != done {
            done += <Self as ::rtic_time::Monotonic>::Duration::from_ticks(1);
        }
        while <Self as ::rtic_time::Monotonic>::now() < done {}
    }
}
impl ::rtic_time::embedded_hal_async::delay::DelayNs for Mono {
    #[inline]
    async fn delay_ns(&mut self, ns: u32) {
        <Self as ::rtic_time::Monotonic>::delay(
                <Self as ::rtic_time::Monotonic>::Duration::nanos_at_least(ns.into()),
            )
            .await;
    }
    #[inline]
    async fn delay_us(&mut self, us: u32) {
        <Self as ::rtic_time::Monotonic>::delay(
                <Self as ::rtic_time::Monotonic>::Duration::micros_at_least(us.into()),
            )
            .await;
    }
    #[inline]
    async fn delay_ms(&mut self, ms: u32) {
        <Self as ::rtic_time::Monotonic>::delay(
                <Self as ::rtic_time::Monotonic>::Duration::millis_at_least(ms.into()),
            )
            .await;
    }
}
pub use defmt::{debug, error, info, trace, warn};
use defmt_rtt as _;
/// The RTIC application module
pub mod app {
    /// Always include the device crate which contains the vector table
    use pac as you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml;
    /// Holds the maximum priority level for use by async HAL drivers.
    #[no_mangle]
    static RTIC_ASYNC_MAX_LOGICAL_PRIO: u8 = 1 << pac::NVIC_PRIO_BITS;
    use super::*;
    /// User code end
    ///Shared resources
    struct Shared {}
    ///Local resources
    struct Local {
        led: Pin<'C', 4, Output>,
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_init_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
        /// Core peripherals
        pub core: rtic::export::Peripherals,
        /// The space used to allocate async executors in bytes.
        pub executors_size: usize,
        /// Device peripherals (PAC)
        pub device: pac::Peripherals,
        /// Critical section token for init
        pub cs: rtic::export::CriticalSection<'a>,
    }
    impl<'a> __rtic_internal_init_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new(
            core: rtic::export::Peripherals,
            executors_size: usize,
        ) -> Self {
            __rtic_internal_init_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
                device: pac::Peripherals::steal(),
                cs: rtic::export::CriticalSection::new(),
                core,
                executors_size,
            }
        }
    }
    #[allow(non_snake_case)]
    ///Initialization function
    pub mod init {
        #[doc(inline)]
        pub use super::__rtic_internal_init_Context as Context;
    }
    #[inline(always)]
    #[allow(non_snake_case)]
    fn init(ctx: init::Context) -> (Shared, Local) {
        match () {
            () => {
                if {
                    const CHECK: bool = {
                        const fn check() -> bool {
                            let module_path = "rtic_systick::app".as_bytes();
                            if if 12usize > module_path.len() {
                                false
                            } else {
                                module_path[0usize] == 114u8 && module_path[1usize] == 116u8
                                    && module_path[2usize] == 105u8
                                    && module_path[3usize] == 99u8
                                    && module_path[4usize] == 95u8
                                    && module_path[5usize] == 115u8
                                    && module_path[6usize] == 121u8
                                    && module_path[7usize] == 115u8
                                    && module_path[8usize] == 116u8
                                    && module_path[9usize] == 105u8
                                    && module_path[10usize] == 99u8
                                    && module_path[11usize] == 107u8
                                    && if 12usize == module_path.len() {
                                        true
                                    } else {
                                        module_path[12usize] == b':'
                                    }
                            } {
                                return true;
                            }
                            false
                        }
                        check()
                    };
                    CHECK
                } {
                    unsafe { defmt::export::acquire() };
                    defmt::export::header(
                        &{
                            defmt::export::make_istr({
                                #[link_section = ".defmt.{\"package\":\"stm32g4-hal\",\"tag\":\"defmt_info\",\"data\":\"start\",\"disambiguator\":\"1309137235051004289\",\"crate_name\":\"rtic_systick\"}"]
                                #[export_name = "{\"package\":\"stm32g4-hal\",\"tag\":\"defmt_info\",\"data\":\"start\",\"disambiguator\":\"1309137235051004289\",\"crate_name\":\"rtic_systick\"}"]
                                static DEFMT_LOG_STATEMENT: u8 = 0;
                                &DEFMT_LOG_STATEMENT as *const u8 as u16
                            })
                        },
                    );
                    unsafe { defmt::export::release() }
                }
            }
        };
        Mono::start(ctx.core.SYST, 16_000_000);
        let p = ctx.device;
        match () {
            () => {
                if {
                    const CHECK: bool = {
                        const fn check() -> bool {
                            let module_path = "rtic_systick::app".as_bytes();
                            if if 12usize > module_path.len() {
                                false
                            } else {
                                module_path[0usize] == 114u8 && module_path[1usize] == 116u8
                                    && module_path[2usize] == 105u8
                                    && module_path[3usize] == 99u8
                                    && module_path[4usize] == 95u8
                                    && module_path[5usize] == 115u8
                                    && module_path[6usize] == 121u8
                                    && module_path[7usize] == 115u8
                                    && module_path[8usize] == 116u8
                                    && module_path[9usize] == 105u8
                                    && module_path[10usize] == 99u8
                                    && module_path[11usize] == 107u8
                                    && if 12usize == module_path.len() {
                                        true
                                    } else {
                                        module_path[12usize] == b':'
                                    }
                            } {
                                return true;
                            }
                            false
                        }
                        check()
                    };
                    CHECK
                } {
                    unsafe { defmt::export::acquire() };
                    defmt::export::header(
                        &{
                            defmt::export::make_istr({
                                #[link_section = ".defmt.{\"package\":\"stm32g4-hal\",\"tag\":\"defmt_info\",\"data\":\"Init Led\",\"disambiguator\":\"16168843029675312228\",\"crate_name\":\"rtic_systick\"}"]
                                #[export_name = "{\"package\":\"stm32g4-hal\",\"tag\":\"defmt_info\",\"data\":\"Init Led\",\"disambiguator\":\"16168843029675312228\",\"crate_name\":\"rtic_systick\"}"]
                                static DEFMT_LOG_STATEMENT: u8 = 0;
                                &DEFMT_LOG_STATEMENT as *const u8 as u16
                            })
                        },
                    );
                    unsafe { defmt::export::release() }
                }
            }
        };
        let gpioc = p.gpioc.split();
        let led = gpioc.pc4.into_push_pull_output();
        (Shared {}, Local { led })
    }
    impl<'a> __rtic_internal_idleLocalResources<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new() -> Self {
            __rtic_internal_idleLocalResources {
                led: &mut *(&mut *__rtic_internal_local_resource_led.get_mut())
                    .as_mut_ptr(),
                __rtic_internal_marker: ::core::marker::PhantomData,
            }
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Local resources `idle` has access to
    pub struct __rtic_internal_idleLocalResources<'a> {
        #[allow(missing_docs)]
        pub led: &'static mut Pin<'C', 4, Output>,
        #[doc(hidden)]
        pub __rtic_internal_marker: ::core::marker::PhantomData<&'a ()>,
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_idle_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
        /// Local Resources this task has access to
        pub local: idle::LocalResources<'a>,
    }
    impl<'a> __rtic_internal_idle_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new() -> Self {
            __rtic_internal_idle_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
                local: idle::LocalResources::new(),
            }
        }
    }
    #[allow(non_snake_case)]
    ///Idle loop
    pub mod idle {
        #[doc(inline)]
        pub use super::__rtic_internal_idleLocalResources as LocalResources;
        #[doc(inline)]
        pub use super::__rtic_internal_idle_Context as Context;
    }
    #[allow(non_snake_case)]
    fn idle(mut ctx: idle::Context) -> ! {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        let led = ctx.local.led;
        loop {
            led.set_high();
            led.set_low();
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic0"]
    static __rtic_internal_local_resource_led: rtic::RacyCell<
        core::mem::MaybeUninit<Pin<'C', 4, Output>>,
    > = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
    #[doc(hidden)]
    #[no_mangle]
    unsafe extern "C" fn main() -> ! {
        rtic::export::interrupt::disable();
        let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal()
            .into();
        let _ = you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::SPI1;
        #[inline(never)]
        fn __rtic_init_resources<F>(f: F)
        where
            F: FnOnce(),
        {
            f();
        }
        let mut executors_size = 0;
        extern "C" {
            pub static _stack_start: u32;
            pub static __ebss: u32;
        }
        let stack_start = &_stack_start as *const _ as u32;
        let ebss = &__ebss as *const _ as u32;
        if stack_start > ebss {
            if rtic::export::msp::read() <= ebss {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Stack overflow after allocating executors"),
                    );
                };
            }
        }
        __rtic_init_resources(|| {
            let (shared_resources, local_resources) = init(
                init::Context::new(core.into(), executors_size),
            );
            __rtic_internal_local_resource_led
                .get_mut()
                .write(core::mem::MaybeUninit::new(local_resources.led));
            rtic::export::interrupt::enable();
        });
        idle(idle::Context::new())
    }
}

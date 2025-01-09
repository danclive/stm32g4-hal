//! External interrupt controller

use crate::pac::Exti;

/// EXTI trigger event
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy)]
pub enum Event {
    GPIO0 = 0,
    GPIO1 = 1,
    GPIO2 = 2,
    GPIO3 = 3,
    GPIO4 = 4,
    GPIO5 = 5,
    GPIO6 = 6,
    GPIO7 = 7,
    GPIO8 = 8,
    GPIO9 = 9,
    GPIO10 = 10,
    GPIO11 = 11,
    GPIO12 = 12,
    GPIO13 = 13,
    GPIO14 = 14,
    GPIO15 = 15,
    PVD = 16,
    RTC_ALARM = 17,
    USB = 18,
    LSE_CSS = 19,
    RTC_WAKEUP = 20,
    COMP1 = 21,
    COMP2 = 22,
    I2C1 = 23,
    I2C2 = 24,
    USART1 = 25,
    USART2 = 26,
    I2C3 = 27,
    USART3 = 28,
    COMP3 = 29,
    COMP4 = 30,
    COMP5 = 31,
    COMP6 = 32,
    COMP7 = 33,
    UART4 = 34,
    UART5 = 35,
    LPUART1 = 36,
    LPTIM1 = 37,
    PVM1 = 40,
    PVM2 = 41,
    I2C4 = 42,
    UCPD1 = 43,
}

pub trait ExtiExt {
    fn listen(&self, ev: Event);
    fn unlisten(&self, ev: Event);
    fn is_pending(&self, ev: Event) -> bool;
    fn unpend(&self, ev: Event);
}

impl ExtiExt for Exti {
    /// CPU Interrupt Enable
    fn listen(&self, ev: Event) {
        let line = ev as u8;

        unsafe {
            match line {
                0..=31 => {
                    self.imr1().modify(|r, w| w.bits(r.bits() | (1 << line)));
                }
                32..=37 | 40..=43 => {
                    self.imr2()
                        .modify(|r, w| w.bits(r.bits() | (1 << (line - 32))));
                }
                _ => {}
            }
        }
    }

    /// CPU Interrupt Disable
    fn unlisten(&self, ev: Event) {
        let line = ev as u8;

        unsafe {
            match line {
                0..=31 => {
                    self.imr1().modify(|r, w| w.bits(r.bits() & !(1 << line)));
                    let _ = self.imr1().read();
                    let _ = self.imr1().read(); // Delay 2 peripheral clocks
                }
                32..=37 | 40..=43 => {
                    self.imr2()
                        .modify(|r, w| w.bits(r.bits() & !(1 << (line - 32))));
                    let _ = self.imr2().read();
                    let _ = self.imr2().read(); // Delay 2 peripheral clocks
                }
                _ => {}
            }
        }
    }

    /// Indicate if the interrupt is currently pending
    ///
    /// Configurable events only
    fn is_pending(&self, ev: Event) -> bool {
        let line = ev as u8;

        match line {
            0..=17 | 19..=22 | 29..=31 => self.pr1().read().bits() & (1 << line) != 0,
            32 | 33 | 40 | 41 | 42 | 43 => self.pr2().read().bits() & (1 << (line - 32)) != 0,
            _ => false,
        }
    }

    /// Clear interrupt and pending flag
    ///
    /// Configurable events only
    fn unpend(&self, ev: Event) {
        let line = ev as u8;

        unsafe {
            match line {
                0..=17 | 19..=22 | 29..=31 => {
                    self.pr1().write(|w| w.bits(1 << line));
                }
                32 | 33 | 40 | 41 | 42 | 43 => {
                    self.pr2().write(|w| w.bits(1 << (line - 32)));
                }
                _ => {}
            }
        }
    }
}

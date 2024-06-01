use crate::gpio::*;

const fn gpiox<const P: char>() -> *const crate::pac::gpioa::RegisterBlock {
    match P {
        'A' => crate::pac::Gpioa::ptr(),
        'B' => crate::pac::Gpiob::ptr() as _,
        'C' => crate::pac::Gpioc::ptr() as _,
        #[cfg(feature = "gpiod")]
        'D' => crate::pac::Gpiod::ptr() as _,
        #[cfg(feature = "gpioe")]
        'E' => crate::pac::Gpioe::ptr() as _,
        #[cfg(feature = "gpiof")]
        'F' => crate::pac::Gpiof::ptr() as _,
        #[cfg(feature = "gpiog")]
        'G' => crate::pac::Gpiog::ptr() as _,
        #[cfg(feature = "gpioh")]
        'H' => crate::pac::Gpioh::ptr() as _,
        #[cfg(feature = "gpioi")]
        'I' => crate::pac::Gpioi::ptr() as _,
        #[cfg(feature = "gpioj")]
        'J' => crate::pac::Gpioj::ptr() as _,
        #[cfg(feature = "gpiok")]
        'K' => crate::pac::Gpiok::ptr() as _,
        _ => panic!("Unknown GPIO port"),
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE> {
    /// Set the output of the pin regardless of its mode.
    /// Primarily used to set the output value of the pin
    /// before changing its mode to an output to avoid
    /// a short spike of an incorrect value
    #[inline(always)]
    pub fn _set_state(&mut self, state: PinState) {
        match state {
            PinState::High => self._set_high(),
            PinState::Low => self._set_low(),
        }
    }
    #[inline(always)]
    pub fn _set_high(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { (*gpiox::<P>()).bsrr().write(|w| w.bits(1 << N)) }
    }
    #[inline(always)]
    pub fn _set_low(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { (*gpiox::<P>()).bsrr().write(|w| w.bits(1 << (16 + N))) }
    }
    #[inline(always)]
    pub fn _is_set_low(&self) -> bool {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*gpiox::<P>()).odr().read().bits() & (1 << N) == 0 }
    }
    #[inline(always)]
    pub fn _is_low(&self) -> bool {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*gpiox::<P>()).idr().read().bits() & (1 << N) == 0 }
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE>
where
    MODE: marker::OutputSpeed,
{
    /// Set pin speed
    pub fn set_speed(&mut self, speed: Speed) {
        let offset = 2 * { N };

        unsafe {
            (*gpiox::<P>())
                .ospeedr()
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | ((speed as u32) << offset)));
        }
    }

    /// Set pin speed
    pub fn speed(mut self, speed: Speed) -> Self {
        self.set_speed(speed);
        self
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE>
where
    MODE: marker::Active,
{
    /// Set the internal pull-up and pull-down resistor
    pub fn set_internal_resistor(&mut self, resistor: Pull) {
        let offset = 2 * { N };
        let value = resistor as u32;
        unsafe {
            (*gpiox::<P>())
                .pupdr()
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (value << offset)));
        }
    }

    /// Set the internal pull-up and pull-down resistor
    pub fn internal_resistor(mut self, resistor: Pull) -> Self {
        self.set_internal_resistor(resistor);
        self
    }

    /// Enables / disables the internal pull up
    pub fn internal_pull_up(self, on: bool) -> Self {
        if on {
            self.internal_resistor(Pull::Up)
        } else {
            self.internal_resistor(Pull::None)
        }
    }

    /// Enables / disables the internal pull down
    pub fn internal_pull_down(self, on: bool) -> Self {
        if on {
            self.internal_resistor(Pull::Down)
        } else {
            self.internal_resistor(Pull::None)
        }
    }
}

//! Random Number Generator

use core::{cmp, mem};

use crate::pac;
use crate::rcc::{Enable, Reset};

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ErrorKind {
    ClockError = 0,
    SeedError = 1,
}

pub struct Rng {
    rb: pac::Rng,
}

pub trait RngExt {
    fn rng(self) -> Rng;
}

impl RngExt for pac::Rng {
    fn rng(self) -> Rng {
        rng(self)
    }
}

pub fn rng(rb: pac::Rng) -> Rng {
    // Enable and reset Rng
    unsafe {
        let rcc = &(*pac::Rcc::PTR);
        <pac::Rng>::enable(rcc);
        <pac::Rng>::reset(rcc);

        if rcc.rcc_crrcr().read().hsi48on().bit_is_clear() {
            rcc.rcc_crrcr().modify(|_, w| w.hsi48on().set_bit());

            while rcc.rcc_crrcr().read().hsi48rdy().bit_is_clear() {}
        }
    }

    rb.cr().modify(|_, w| w.ced().set_bit().rngen().set_bit());

    Rng { rb }
}

impl Rng {
    /// Returns 32 bits of randomness, or error
    pub fn value(&mut self) -> Result<u32, ErrorKind> {
        loop {
            let status = self.rb.sr().read();

            if status.cecs().bit() {
                return Err(ErrorKind::ClockError);
            }
            if status.secs().bit() {
                return Err(ErrorKind::SeedError);
            }
            if status.drdy().bit() {
                return Ok(self.rb.dr().read().rndata().bits());
            }
        }
    }

    pub fn release(self) -> pac::Rng {
        self.rb
    }

    /// Returns a reference to the inner peripheral
    pub fn inner(&self) -> &pac::Rng {
        &self.rb
    }

    /// Returns a mutable reference to the inner peripheral
    pub fn inner_mut(&mut self) -> &mut pac::Rng {
        &mut self.rb
    }
}

impl core::iter::Iterator for Rng {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.value().ok()
    }
}

pub trait RngCore<W> {
    fn random(&mut self) -> Result<W, ErrorKind>;
    fn fill(&mut self, dest: &mut [W]) -> Result<(), ErrorKind>;
}

macro_rules! rng_core {
    ($($type:ty),+) => {
        $(
            impl RngCore<$type> for Rng {
                /// Returns a single element with random value, or error
                fn random(&mut self) -> Result<$type, ErrorKind> {
                    let val = self.value()?;
                    Ok(val as $type)
                }

                /// Fills buffer with random values, or return error
                fn fill(&mut self, buffer: &mut [$type]) -> Result<(), ErrorKind> {
                    const BATCH_SIZE: usize = 4 / mem::size_of::<$type>();
                    let mut i = 0_usize;
                    while i < buffer.len() {
                        let random_word = self.value()?;

                        // using to_ne_bytes does not work for u8 and would make the macro
                        // implementation more complicated
                        #[allow(clippy::transmute_num_to_bytes)]
                        let bytes: [$type; BATCH_SIZE] = unsafe { mem::transmute(random_word) };
                        let n = cmp::min(BATCH_SIZE, buffer.len() - i);
                        buffer[i..i + n].copy_from_slice(&bytes[..n]);
                        i += n;
                    }
                    Ok(())
                }
            }
        )+
    };
}

// Only for types larger than 32 bits
macro_rules! rng_core_large {
    ($($type:ty),+) => {
        $(
            impl RngCore<$type> for Rng {
                fn random(&mut self) -> Result<$type, ErrorKind> {
                    const WORDS: usize = mem::size_of::<$type>() / mem::size_of::<u32>();
                    let mut res: $type = 0;

                    for i in 0..WORDS {
                        res |= (self.value()? as $type) << (i * (mem::size_of::<u32>() * 8))
                    }

                    Ok(res)
                }

                fn fill(&mut self, dest: &mut [$type]) -> Result<(), ErrorKind> {
                    let len = dest.len() * (mem::size_of::<$type>() / mem::size_of::<u32>());
                    let ptr = dest.as_mut_ptr() as *mut u32;
                    let slice_u32 = unsafe { core::slice::from_raw_parts_mut(ptr, len) };
                    self.fill(slice_u32)
                }
            }
        )+
    };
}

macro_rules! rng_core_transmute {
    ($($type:ty = $from:ty),+) => {
        $(
            impl RngCore<$type> for Rng {
                fn random(&mut self) -> Result<$type, ErrorKind> {
                    let num = <Self as RngCore<$from>>::random(self)?;
                    Ok(unsafe { mem::transmute::<$from, $type>(num) })
                }

                fn fill(&mut self, dest: &mut [$type]) -> Result<(), ErrorKind> {
                    let unsigned_slice = unsafe { mem::transmute::<&mut [$type], &mut [$from]>(dest) };
                    <Self as RngCore<$from>>::fill(self, unsigned_slice)
                }
            }
        )+
    };
}

rng_core!(u8, u16, u32);

// Alignment of these types must be a multiple of mem::align_of::<32>()
rng_core_large!(u64, u128);

// A and B must have the same alignment
// rng_core_transmute!(A = B)
// assert!(mem::align_of::<A>() == mem::align_of::<B>())
rng_core_transmute!(
    i8 = u8,
    i16 = u16,
    i32 = u32,
    i64 = u64,
    i128 = u128,
    isize = usize
);

// If usize is 32 bits, use the rng_core! impl
#[cfg(target_pointer_width = "32")]
rng_core!(usize);

// If usize is 64 bits, use the rng_core_large! impl
#[cfg(target_pointer_width = "64")]
rng_core_large!(usize);

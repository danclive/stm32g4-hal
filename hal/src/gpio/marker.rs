//! Marker traits

use super::*;

/// Marker trait that show if `ExtiPin` can be implemented
pub trait Interruptible {}
/// Marker trait for readable pin modes
pub trait Readable {}
/// Marker trait for slew rate configurable pin modes
pub trait OutputSpeed {}
/// Marker trait for active pin modes
pub trait Active {}
/// Marker trait for all pin modes except alternate
pub trait NotAlt {}
/// Marker trait for pins with alternate function `A` mapping
pub trait IntoAf<const A: u8> {}

impl<MODE> Interruptible for Output<MODE> {}
impl Interruptible for Input {}
impl<const A: u8, OutType> Interruptible for Alt<A, OutType> {}

impl Readable for Input {}
impl Readable for Output<OpenDrain> {}
impl<const A: u8, OutType> Readable for Alt<A, OutType> {}

impl Active for Input {}
impl<OutType> Active for Output<OutType> {}
impl<const A: u8, OutType> Active for Alt<A, OutType> {}

impl<OutType> OutputSpeed for Output<OutType> {}
impl<const A: u8, OutType> OutputSpeed for Alt<A, OutType> {}

impl NotAlt for Input {}
impl<OutType> NotAlt for Output<OutType> {}
impl NotAlt for Analog {}

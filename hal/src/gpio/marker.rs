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
impl Readable for Input {}
impl Readable for Output<OpenDrain> {}
impl<const A: u8, Otype> Interruptible for Alternate<A, Otype> {}
impl<const A: u8, Otype> Readable for Alternate<A, Otype> {}
impl Active for Input {}
impl<Otype> OutputSpeed for Output<Otype> {}
impl<const A: u8, Otype> OutputSpeed for Alternate<A, Otype> {}
impl<Otype> Active for Output<Otype> {}
impl<const A: u8, Otype> Active for Alternate<A, Otype> {}
impl NotAlt for Input {}
impl<Otype> NotAlt for Output<Otype> {}
impl NotAlt for Analog {}

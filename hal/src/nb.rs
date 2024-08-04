//! Non-blocking

use core::fmt;

/// A non-blocking result
pub type Result<T, E> = ::core::result::Result<T, Error<E>>;

/// A non-blocking error
///
/// The main use of this enum is to add a `WouldBlock` variant to an existing
/// error enum.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error<E> {
    /// A different kind of error
    Other(E),
    /// This operation requires blocking behavior to complete
    WouldBlock,
}

impl<E> defmt::Format for Error<E>
where
    E: defmt::Format,
{
    fn format(&self, f: defmt::Formatter) {
        match *self {
            Error::Other(ref e) => defmt::Format::format(e, f),
            Error::WouldBlock => defmt::write!(f, "WouldBlock",),
        }
    }
}

impl<E> fmt::Debug for Error<E>
where
    E: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Other(ref e) => fmt::Debug::fmt(e, f),
            Error::WouldBlock => f.write_str("WouldBlock"),
        }
    }
}

impl<E> Error<E> {
    /// Maps an `Error<E>` to `Error<T>` by applying a function to a contained
    /// `Error::Other` value, leaving an `Error::WouldBlock` value untouched.
    pub fn map<T, F>(self, op: F) -> Error<T>
    where
        F: FnOnce(E) -> T,
    {
        match self {
            Error::Other(e) => Error::Other(op(e)),
            Error::WouldBlock => Error::WouldBlock,
        }
    }
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Error<E> {
        Error::Other(error)
    }
}

/// Turns the non-blocking expression `$e` into a blocking operation.
///
/// This is accomplished by continuously calling the expression `$e` until it no
/// longer returns `Error::WouldBlock`
///
/// # Input
///
/// An expression `$e` that evaluates to `nb::Result<T, E>`
///
/// # Output
///
/// - `Ok(t)` if `$e` evaluates to `Ok(t)`
/// - `Err(e)` if `$e` evaluates to `Err(nb::Error::Other(e))`
#[macro_export]
macro_rules! block {
    ($e:expr) => {
        loop {
            #[allow(unreachable_patterns)]
            match $e {
                Err($crate::nb::Error::Other(e)) =>
                {
                    #[allow(unreachable_code)]
                    break Err(e)
                }
                Err($crate::nb::Error::WouldBlock) => {}
                Ok(x) => break Ok(x),
            }
        }
    };
}

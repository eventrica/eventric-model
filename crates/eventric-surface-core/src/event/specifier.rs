use eventric_stream::{
    error::Error,
    event::Specifier,
};

use crate::event::identifier::Identified;

// =================================================================================================
// Specifier
// =================================================================================================

pub trait Specified {
    fn specifier() -> Result<Specifier, Error>;
}

impl<T> Specified for T
where
    T: Identified,
{
    fn specifier() -> Result<Specifier, Error> {
        T::identifier().cloned().map(Specifier::new)
    }
}

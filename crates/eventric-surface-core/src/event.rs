pub(crate) mod codec;
pub(crate) mod identifier;
pub(crate) mod macros;
pub(crate) mod tag;

use eventric_stream::{
    error::Error,
    event::{
        Identifier,
        Specifier,
        Tag,
    },
};
use serde::{
    Serialize,
    de::DeserializeOwned,
};

// =================================================================================================
// Event
// =================================================================================================

// Event

pub trait Event: DeserializeOwned + Identified + Tagged + Serialize {}

// Identified

pub trait Identified {
    fn identifier() -> Result<&'static Identifier, Error>;
}

// Specified

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

// Tagged

pub trait Tagged {
    fn tags(&self) -> Result<Vec<Tag>, Error>;
}

// -------------------------------------------------------------------------------------------------

// Re-Exports

pub use self::codec::{
    Codec,
    JsonCodec,
};

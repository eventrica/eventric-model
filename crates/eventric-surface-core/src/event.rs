pub(crate) mod codec;
pub(crate) mod identifier;
pub(crate) mod macros;
pub(crate) mod tag;

use eventric_stream::{
    error::Error,
    event::{
        Identifier,
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

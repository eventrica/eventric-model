#![allow(clippy::multiple_crate_versions)]

// =================================================================================================
// Eventric Surface
// =================================================================================================

pub mod event {
    pub use eventric_surface_core::event::{
        Codec,
        Event,
        Identified,
        Specified,
        Tagged,
    };
    pub use eventric_surface_macros::{
        Event,
        Identified,
        Tagged,
    };

    pub mod json {
        pub use eventric_surface_core::event::JsonCodec as Codec;
    }
}

pub mod projection {
    pub use eventric_surface_core::projection::{
        Dispatch,
        Projection,
        Queried,
        Recognize,
        Update,
    };
    pub use eventric_surface_macros::{
        Projection,
        QuerySource,
    };
}

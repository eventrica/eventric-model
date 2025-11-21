#![allow(clippy::multiple_crate_versions)]

// =================================================================================================
// Eventric Surface
// =================================================================================================

pub mod decision {
    pub use eventric_surface_core::decision::{
        Decision,
        Projections,
        Query,
        Update,
    };
    pub use eventric_surface_macros::{
        Decision,
        Projections,
        Update,
    };
}

pub mod event {
    pub use eventric_surface_core::event::{
        Codec,
        Event,
        Identifier,
        Specifier,
        Tags,
    };
    pub use eventric_surface_macros::{
        Event,
        Identifier,
        Tags,
    };

    pub mod json {
        pub use eventric_surface_core::event::JsonCodec as Codec;
    }
}

pub mod projection {
    pub use eventric_surface_core::projection::{
        Dispatch,
        DispatchEvent,
        Projection,
        Query,
        Recognize,
        Update,
        UpdateEvent,
    };
    pub use eventric_surface_macros::{
        Dispatch,
        Projection,
        Query,
        Recognize,
    };
}

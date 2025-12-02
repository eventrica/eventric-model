use std::ops::{
    Deref,
    DerefMut,
};

use crate::decision::execute::Events;

// =================================================================================================
// Update
// =================================================================================================

pub trait Context
where
    Self::Context: Deref<Target = Events> + DerefMut + Into<Events>,
{
    type Context;

    fn context(&self) -> Self::Context;
}

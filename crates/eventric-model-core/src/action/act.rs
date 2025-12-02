use eventric_stream::error::Error;

use crate::action::Context;

// =================================================================================================
// Act
// =================================================================================================

pub trait Act: Context
where
    Self::Err: From<Error>,
{
    type Err;
    type Ok = ();

    fn action(&mut self, context: &mut Self::Context) -> Result<Self::Ok, Self::Err>;
}

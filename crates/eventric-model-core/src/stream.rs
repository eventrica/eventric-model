use eventric_stream::stream::{
    Stream,
    append::AppendSelect as _,
    iterate::IterateSelect as _,
};

use crate::action::Action;

// =================================================================================================
// Stream
// =================================================================================================

// Enactor

pub trait Enactor {
    fn enact<A>(&mut self, action: A) -> Result<A::Ok, A::Err>
    where
        A: Action;
}

// Stream

impl Enactor for Stream {
    fn enact<A>(&mut self, mut action: A) -> Result<A::Ok, A::Err>
    where
        A: Action,
    {
        let mut after = None;
        let mut context = action.context();

        let selections = action.select(&context)?;

        let (events, select) = self.iter_select(selections, None);

        for event in events {
            let event = event?;
            let position = *event.event.position();

            after = Some(position);

            action.update(&mut context, &event)?;
        }

        let ok = action.action(&mut context)?;
        let events = context.into().take();

        if !events.is_empty() {
            self.append_select(events, select, after)?;
        }

        Ok(ok)
    }
}

use eventric_stream::stream::{
    append::AppendSelect,
    select::Select,
};

use crate::action::Action;

// =================================================================================================
// Core
// =================================================================================================

// Enactor

pub trait Enactor {
    fn enact<A>(&mut self, action: A) -> Result<A::Ok, A::Err>
    where
        A: Action;
}

impl<T> Enactor for T
where
    T: AppendSelect + Select,
{
    fn enact<A>(&mut self, mut action: A) -> Result<A::Ok, A::Err>
    where
        A: Action,
    {
        let mut after = None;
        let mut context = action.context();

        let selections = action.select(&context)?;

        let (events, select) = self.select_multiple(selections, None);

        for event in events {
            let event_and_mask = event?;
            let position = *event_and_mask.event.position();

            after = Some(position);

            action.update(&mut context, &event_and_mask)?;
        }

        let ok = action.action(&mut context)?;
        let events = context.into().take();

        if !events.is_empty() {
            self.append_select_multiple(events, select, after)?;
        }

        Ok(ok)
    }
}

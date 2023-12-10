use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct SelectRandomIdentifierEvent;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectRandomIdentifierEvent>();
    }
}

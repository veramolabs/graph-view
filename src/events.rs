use bevy::prelude::*;
use forceatlas2::Settings;
use graph::page_rank::PageRankConfig;

#[derive(Event, Debug)]
pub struct SelectRandomIdentifierEvent;

#[derive(Event, Debug)]
pub struct SelectIdentifierEvent(pub Entity);

#[derive(Event, Debug)]
pub struct DeselectIdentifierEvent;

#[derive(Event, Debug)]
pub struct SelectRandomConnectedIdentifierEvent;

#[derive(Event, Debug)]
pub struct MoveIdentifiersRndEvent;

#[derive(Event, Debug)]
pub struct AddIdentifiersEvent {
    pub count: u32,
}

#[derive(Event, Debug)]
pub struct AddConnectionsEvent {
    pub count: u32,
}

#[derive(Event)]
pub struct Forceatlas2Event {
    pub settings: Settings<f32>,
    pub iterations: u32,
}
#[derive(Event)]
pub struct PageRankEvent {
    pub config: PageRankConfig,
}

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectRandomIdentifierEvent>()
            .add_event::<SelectRandomConnectedIdentifierEvent>()
            .add_event::<SelectIdentifierEvent>()
            .add_event::<DeselectIdentifierEvent>()
            .add_event::<AddIdentifiersEvent>()
            .add_event::<MoveIdentifiersRndEvent>()
            .add_event::<Forceatlas2Event>()
            .add_event::<PageRankEvent>()
            .add_event::<AddConnectionsEvent>();
    }
}

use bevy::prelude::*;

#[derive(Component)]
pub struct Identifier;

#[derive(Component)]
pub struct Connection {
    pub from: Entity,
    pub to: Entity,
}

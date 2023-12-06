use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod simulation;
use simulation::SimulationPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(SimulationPlugin)
        .run();
}


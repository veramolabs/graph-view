use bevy::prelude::*;
use bevy_egui::EguiContext;
use bevy_inspector_egui::egui;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_window::PrimaryWindow;

use crate::{
    events::{
        DeselectIdentifierEvent, MoveIdentifiersRndEvent, SelectRandomConnectedIdentifierEvent,
        SelectRandomIdentifierEvent,
    },
    resources::Configuration,
    simulation::{force_atlas_ui, simulation_ui},
    util::{calculate_from_translation_and_focus, random_point_in_sphere},
};

#[derive(Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct UiState {
    pub show_config: bool,
    pub show_actions: bool,
    pub show_forceatlas: bool,
    pub show_simulation: bool,
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ui)
            .init_resource::<UiState>()
            .register_type::<UiState>()
            .add_systems(
                Update,
                (
                    configuration_ui.run_if(|state: Res<UiState>| state.show_config),
                    actions_ui.run_if(|state: Res<UiState>| state.show_actions),
                    force_atlas_ui.run_if(|state: Res<UiState>| state.show_forceatlas),
                    simulation_ui.run_if(|state: Res<UiState>| state.show_simulation),
                ),
            );
    }
}

fn ui(query: Query<&mut EguiContext, With<PrimaryWindow>>, mut state: ResMut<UiState>) {
    let mut egui_context = query.single().clone();
    egui::TopBottomPanel::top("Top").show(egui_context.get_mut(), |ui| {
        ui.horizontal(|ui| {
            if ui.button("Config").clicked() {
                state.show_config = !state.show_config;
            };
            if ui.button("Actions").clicked() {
                state.show_actions = !state.show_actions;
            };
            if ui.button("Atlas").clicked() {
                state.show_forceatlas = !state.show_forceatlas;
            };
            if ui.button("Simulate").clicked() {
                state.show_simulation = !state.show_simulation;
            };
        });
    });

    //   egui::SidePanel::left("Top").show(egui_context.get_mut(), |ui| {
    //     ScrollArea::vertical()
    //         .auto_shrink([false; 2])
    //         .show(ui, |ui| {
    //             ui.heading("Note that while a slider is being dragged, the panel is being resized, or the scrollbar is being moved, items in the 3d scene cannot be picked even if the mouse is over them.");
    //         })
    // });
}

pub fn configuration_ui(
    mut configuration: ResMut<Configuration>,
    query: Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    let mut egui_context = query.single().clone();

    egui::Window::new("Configuration")
        .vscroll(false)
        .hscroll(false)
        .default_width(250.0)
        .resizable(false)
        .show(egui_context.get_mut(), |ui| {
            ui.add(egui::Slider::new(&mut configuration.container_size, 0.0..=100.0).text("Space"));
            ui.add(
                egui::Slider::new(&mut configuration.animation_duration, 1..=10)
                    .text("Duration (sec)"),
            );
        });
}

pub fn actions_ui(
    configuration: ResMut<Configuration>,
    query: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut camera_q: Query<&mut PanOrbitCamera, With<PanOrbitCamera>>,
    mut ev_rnd_id: EventWriter<SelectRandomIdentifierEvent>,
    mut ev_rnd_c_id: EventWriter<SelectRandomConnectedIdentifierEvent>,
    mut ev_move: EventWriter<MoveIdentifiersRndEvent>,
    mut ev_deselect: EventWriter<DeselectIdentifierEvent>,
) {
    let mut egui_context = query.single().clone();

    egui::Window::new("Actions")
        .vscroll(false)
        .hscroll(false)
        .default_width(250.0)
        .resizable(false)
        .show(egui_context.get_mut(), |ui| {
            if ui.button("Select random identifier").clicked() {
                ev_rnd_id.send(SelectRandomIdentifierEvent);
            }
            if ui.button("Select random connected identifier").clicked() {
                ev_rnd_c_id.send(SelectRandomConnectedIdentifierEvent);
            }
            if ui.button("Deselect identifier").clicked() {
                ev_deselect.send(DeselectIdentifierEvent);
            }
            if ui.button("Move identifiers randomly").clicked() {
                ev_move.send(MoveIdentifiersRndEvent);
            }

            ui.separator();
            if ui.button("Move camera randomly").clicked() {
                if let Ok(mut camera) = camera_q.get_single_mut() {
                    let (x, y, z) = random_point_in_sphere(configuration.container_size);
                    let (alpha, beta, radius) =
                        calculate_from_translation_and_focus(Vec3::new(x, y, z), Vec3::ZERO);
                    camera.target_alpha = alpha;
                    camera.target_beta = beta;
                    camera.target_radius = radius;
                    camera.target_focus = Vec3::ZERO;
                };
            }

            if ui.button("Zoom out").clicked() {
                if let Ok(mut camera) = camera_q.get_single_mut() {
                    let new_position = Vec3::ZERO + configuration.container_size * 1.5;
                    let (alpha, beta, radius) =
                        calculate_from_translation_and_focus(new_position, Vec3::ZERO);
                    camera.target_alpha = alpha;
                    camera.target_beta = beta;
                    camera.target_radius = radius;
                    camera.target_focus = Vec3::ZERO;
                };
            }
        });
}

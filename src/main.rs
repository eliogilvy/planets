use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{PrimaryWindow, WindowMode},
};

mod camera;
use camera::SpaceCameraPlugin;

mod diagnostics;
use diagnostics::SpaceDiagnosticsPlugin;

mod planets;
use planets::PlanetsPlugin;

// A program to simulate F = G (m1m2/r**2)

// Mouse sensitivity
const MOUSE_SENSITIVITY: f32 = 1.5;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin::default(),
            SpaceCameraPlugin,
            SpaceDiagnosticsPlugin,
            PlanetsPlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup_window)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

// Window setup
fn setup_window(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_query.single_mut();
    window.mode = WindowMode::Fullscreen;
}

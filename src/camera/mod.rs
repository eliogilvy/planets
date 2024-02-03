use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
    render::camera,
    window::PrimaryWindow,
};

use crate::MOUSE_SENSITIVITY;

pub struct SpaceCameraPlugin;

#[derive(Component)]
struct MainCamera;

impl Plugin for SpaceCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, (handle_camera_pan, zoom_control));
    }
}

// Spawn relevant cameras
fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    // Initial scale
    camera.projection.scale = 2.;
    commands.spawn((camera, MainCamera));
}

// Handles dragging the cursor while clicking
fn handle_camera_pan(
    mut mouse_delta: EventReader<MouseMotion>,
    input: Res<Input<MouseButton>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    let mut camera_transform = camera_query.single_mut();

    for md in mouse_delta.read() {
        // Pan camera while right clicking
        if input.pressed(MouseButton::Right) && !input.just_pressed(MouseButton::Right) {
            camera_transform.translation.x -= md.delta.x * MOUSE_SENSITIVITY;
            camera_transform.translation.y += md.delta.y * MOUSE_SENSITIVITY;
        }
    }
}

// Handles clicking on a planet
fn handle_clicking_planet(
    window: Query<&Window, With<PrimaryWindow>>,
    input: Res<Input<MouseButton>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    let mut camera_transform = camera_query.single_mut();

    if let Some(position) = window.single().cursor_position() {
        if input.just_pressed(MouseButton::Left) {
            camera_transform.translation.x = position.x;
            camera_transform.translation.y = position.y;
        }
    }
}

// To zoom in and out
fn zoom_control(
    mut scroll: EventReader<MouseWheel>,
    mut camera_query: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    let mut projection = camera_query.single_mut();

    // 1 for zoom in, -1 for zoom out
    for ev in scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                if ev.y == -1. {
                    projection.scale *= MOUSE_SENSITIVITY;
                } else if ev.y == 1. {
                    projection.scale /= MOUSE_SENSITIVITY;
                }
            }
            MouseScrollUnit::Pixel => {
                if ev.y == -1. {
                    projection.scale *= 1.25;
                } else if ev.y == 1. {
                    projection.scale /= 1.25;
                }
            }
        }
    }
}

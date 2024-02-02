use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*};

// Diagnostics

/// Fps marker
#[derive(Component)]
struct FpsRoot;

/// Fps text marker
#[derive(Component)]
struct FpsText;

pub struct SpaceDiagnosticsPlugin;

impl Plugin for SpaceDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_fps)
            .add_systems(Update, update_fps);
    }
}

// Setup Fps counter
fn setup_fps(mut commands: Commands) {
    let fps_bundle = (
        FpsRoot,
        NodeBundle {
            background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
            z_index: ZIndex::Global(i32::MAX),
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(1.),
                top: Val::Percent(1.),
                bottom: Val::Auto,
                right: Val::Auto,
                padding: UiRect::all(Val::Px(4.)),
                ..Default::default()
            },
            ..Default::default()
        },
    );
    let text_fps_bundle = (
        FpsText,
        TextBundle {
            // use two sections, so it is easy to update just the number
            text: Text::from_sections([
                TextSection {
                    value: "FPS: ".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        // if you want to use your game's font asset,
                        // uncomment this and provide the handle:
                        // font: my_font_handle
                        ..default()
                    },
                },
                TextSection {
                    value: " N/A".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        // if you want to use your game's font asset,
                        // uncomment this and provide the handle:
                        // font: my_font_handle
                        ..default()
                    },
                },
            ]),
            ..Default::default()
        },
    );

    let root = commands.spawn(fps_bundle).id();
    let text_fps = commands.spawn(text_fps_bundle).id();

    commands.entity(root).push_children(&[text_fps]);
}

// Update the fps
fn update_fps(diagnostics: Res<DiagnosticsStore>, mut fps_query: Query<&mut Text, With<FpsText>>) {
    for mut fps in &mut fps_query {
        if let Some(value) = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            fps.sections[1].value = format!("{value:>4.0}");

            fps.sections[1].style.color = if value >= 60. {
                Color::GREEN
            } else {
                Color::RED
            }
        } else {
            fps.sections[1].value = "N/A".into();
            fps.sections[1].style.color = Color::WHITE;
        }
    }
}

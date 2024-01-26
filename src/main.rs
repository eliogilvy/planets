use bevy::{prelude::*, sprite::MaterialMesh2dBundle, ecs::world};

// A program to simulate F = G (m1m2/r**2)

// Planetery sizes
const SUN_SIZE: Vec3 = Vec3::new(75., 75., 0.);
const PLANET_SIZE: Vec3 = Vec3::new(30., 30., 0.);

// Atomic units in meters
const AU: f32 = 149.6e6 * 1000.;
// Gravity (G)
const GRAVITY: f32 = 6.67428e-11;
// For scaling
const SCALE: f32 = 250. / AU;
// To represent duration of orbit
const TIMESTEP: f32 = 3600. * 24.;

// Planetary masses
const MASS_OF_SUN: f32 = 1.98892 * 10e30;
const MASS_OF_EARTH: f32 = 5.9742 * 10e24;

#[derive(Component)]
struct Planet;

#[derive(Component)]
struct Star;

#[derive(Component)]
struct Mass(f32);

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Bundle)]
struct SpaceObjectBundle {
    mass: Mass,
    velocity: Velocity,
    material2d: MaterialMesh2dBundle<ColorMaterial>,
}

// An space object, either sun or planet, should have a mass and velocity
impl SpaceObjectBundle {
    fn new(
        mass: Mass,
        velocity: Velocity,
        material2d: MaterialMesh2dBundle<ColorMaterial>,
    ) -> Self {
        SpaceObjectBundle {
            mass: mass,
            velocity: velocity,
            material2d: material2d,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_planets))
        .add_systems(FixedUpdate, (apply_gravity, update_planets).chain())
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn Sun
    commands.spawn((
        Star,
        SpaceObjectBundle::new(
            Mass(MASS_OF_SUN),
            Velocity { x: 0., y: 0. },
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::default().into()).into(),
                material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)).with_scale(SUN_SIZE),
                ..default()
            },
        ),
    ));

    // Spawn earth
    commands.spawn((
        Planet,
        SpaceObjectBundle::new(
            Mass(MASS_OF_EARTH),
            Velocity { x: 0., y: 0. },
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::default().into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: Transform::from_translation(Vec3::new(-1. * AU * SCALE, 0., 0.))
                    .with_scale(PLANET_SIZE),
                ..default()
            },
        ),
    ));
}
//todo
fn apply_gravity(
    mut planet_query: Query<(&mut Transform, &mut Velocity, &Mass), (With<Planet>, Without<Star>)>,
    sun_query: Query<&Transform, (With<Star>, Without<Planet>)>,
) {
    let sun_transform = sun_query.single();
    for (mut planet_transform, mut planet_velocity, planet_mass) in planet_query.iter_mut() {
        let distance_x = sun_transform.translation.x - planet_transform.translation.x;
        let distance_y = sun_transform.translation.y - planet_transform.translation.y;
        let distance = (distance_x.powi(2) + distance_y.powi(2)).sqrt();

        // F = G (m1m2/r**2)
        let force = GRAVITY * (MASS_OF_SUN * planet_mass.0) / distance.powi(2);

        let theta = distance_y.atan2(distance_x);
        let force_x = theta.cos() * force;
        let force_y = theta.sin() * force;
        let total_force = force_x + force_y;

        planet_velocity.x += force_x / planet_mass.0 * TIMESTEP;
        planet_velocity.y += force_y / planet_mass.0 * TIMESTEP;
    }
}

fn update_planets(
    mut planet_query: Query<(&mut Transform, &Velocity), (With<Planet>, Without<Star>)>,
) {
    for (mut planet_transform, planet_velocity) in planet_query.iter_mut() {
        planet_transform.translation.x = planet_transform.translation.x * SCALE + 800. / 2.;
        // planet_transform.translation.y = planet_transform.translation.y + planet_velocity.y * SCALE + 800. / 2.;
    }
}

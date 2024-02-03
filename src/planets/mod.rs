use std::{default, f32::consts::PI};

use bevy::{
    gizmos,
    prelude::*,
    render::render_resource::PrimitiveTopology,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

/// Everything to do with updating planet positions
pub struct PlanetsPlugin;

impl Plugin for PlanetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_planets)
            .add_systems(Update, (apply_gravity, update_planets, draw_planet_trails));
    }
}

// Atomic units in meters
const AU: f64 = 149.6e6 * 1000.;
// Gravity (G)
const GRAVITY: f64 = 6.67428e-11;
// For scaling
const SCALE: f64 = 250. / AU;
// To represent duration of orbit
const TIMESTEP: f64 = 3600. * 24.;

const SUN_DIAMETER: f32 = 75.;

const SUN_RADIUS: f32 = 69634.;
const MERCURY_RADIUS: f32 = 2440. / SUN_RADIUS;
const VENUS_RADIUS: f32 = 6052. / SUN_RADIUS;
const EARTH_RADIUS: f32 = 6371. / SUN_RADIUS;
const MARS_RADIUS: f32 = 3390. / SUN_RADIUS;
const JUPITER_RADIUS: f32 = 69911. / SUN_RADIUS;
const SATURN_RADIUS: f32 = 58232. / SUN_RADIUS;
const URANUS_RADIUS: f32 = 25362. / SUN_RADIUS;
const NEPTUNE_RADIUS: f32 = 24622. / SUN_RADIUS;

// Planetery sizes
const SUN_SIZE: Vec3 = Vec3::new(SUN_DIAMETER, SUN_DIAMETER, 0.);
const MERCURY_SIZE: Vec3 = Vec3::new(
    MERCURY_RADIUS * SUN_DIAMETER,
    MERCURY_RADIUS * SUN_DIAMETER,
    0.,
);
const VENUS_SIZE: Vec3 = Vec3::new(VENUS_RADIUS * SUN_DIAMETER, VENUS_RADIUS * SUN_DIAMETER, 0.);
const EARTH_SIZE: Vec3 = Vec3::new(EARTH_RADIUS * SUN_DIAMETER, EARTH_RADIUS * SUN_DIAMETER, 0.);
const MARS_SIZE: Vec3 = Vec3::new(MARS_RADIUS * SUN_DIAMETER, MARS_RADIUS * SUN_DIAMETER, 0.);
const JUPITER_SIZE: Vec3 = Vec3::new(
    JUPITER_RADIUS * SUN_DIAMETER,
    JUPITER_RADIUS * SUN_DIAMETER,
    0.,
);
const SATURN_SIZE: Vec3 = Vec3::new(
    SATURN_RADIUS * SUN_DIAMETER,
    SATURN_RADIUS * SUN_DIAMETER,
    0.,
);
const URANUS_SIZE: Vec3 = Vec3::new(
    URANUS_RADIUS * SUN_DIAMETER,
    URANUS_RADIUS * SUN_DIAMETER,
    0.,
);
const NEPTUNE_SIZE: Vec3 = Vec3::new(
    NEPTUNE_RADIUS * SUN_DIAMETER,
    NEPTUNE_RADIUS * SUN_DIAMETER,
    0.,
);

// Plantary colors
const SUN_COLOR: Color = Color::YELLOW;
const MERCURY_COLOR: Color = Color::RED;
const VENUS_COLOR: Color = Color::BEIGE;
const EARTH_COLOR: Color = Color::BLUE;
const MARS_COLOR: Color = Color::ORANGE_RED;
const JUPITER_COLOR: Color = Color::GREEN;
const SATURN_COLOR: Color = Color::BEIGE;
const URANUS_COLOR: Color = Color::rgb(0., 255., 255.);
const NEPTUNE_COLOR: Color = Color::WHITE;

// Relative positions
const SUN_POSITION: Position = Position { x: 0., y: 0. };
const MERCURY_POSITION: Position = Position {
    x: 0.387 * AU,
    y: 0.,
};
const VENUS_POSITION: Position = Position {
    x: 0.72 * AU,
    y: 0.,
};
const EARTH_POSITION: Position = Position { x: -1. * AU, y: 0. };
const MARS_POSITION: Position = Position {
    x: -1.524 * AU,
    y: 0.,
};
const JUPITER_POSITION: Position = Position { x: 5.2 * AU, y: 0. };
const SATURN_POSITION: Position = Position {
    x: 9.54 * AU,
    y: 0.,
};
const URANUS_POSITION: Position = Position {
    x: 19.2 * AU,
    y: 0.,
};
const NEPTUNE_POSITION: Position = Position {
    x: 30.06 * AU,
    y: 0.,
};

// Planetary masses
const MASS_OF_SUN: f64 = 1.98892e30;
const MASS_OF_VENUS: f64 = 4.87e24;
const MASS_OF_MERCURY: f64 = 3.3e23;
const MASS_OF_EARTH: f64 = 5.9742e24;
const MASS_OF_MARS: f64 = 6.39e23;
const MASS_OF_JUPITER: f64 = 1898e24;
const MASS_OF_SATURN: f64 = 568e24;
const MASS_OF_URANUS: f64 = 86.8e24;
const MASS_OF_NEPTUNE: f64 = 102e24;

#[derive(Component)]
struct Planet;

#[derive(Component, Clone, Copy)]
struct Position {
    x: f64,
    y: f64,
}

#[derive(Component)]
struct Mass(f64);

#[derive(Component)]
struct Velocity {
    x: f64,
    y: f64,
}

#[derive(Resource)]
struct PlanetTimer(Timer);

const MAX_TRAIL_SIZE: usize = 1000;

#[derive(Component)]
struct TrailMaterial;

#[derive(Component)]
struct PlanetTrail {
    positions: Vec<Vec2>,
}

impl PlanetTrail {
    fn default() -> Self {
        PlanetTrail {
            positions: Vec::new(),
        }
    }
    // Manages adding a position and maintining certain size
    fn add_position(&mut self, position: &Position) {
        println!("{}", self.positions.len());
        if self.positions.len() > MAX_TRAIL_SIZE {
            self.positions.remove(0);
        }
        let x = position.x * SCALE;
        let y = position.y * SCALE;
        self.positions.push(Vec2::new(x as f32, y as f32));
    }
}

#[derive(Bundle)]
struct SpaceObjectBundle {
    planet: Planet,
    mass: Mass,
    velocity: Velocity,
    position: Position,
    material2d: MaterialMesh2dBundle<ColorMaterial>,
    trail: PlanetTrail,
}

// An space object, either sun or planet, should have a mass and velocity
impl SpaceObjectBundle {
    fn new(
        mass: Mass,
        velocity: Velocity,
        position: Position,
        material2d: MaterialMesh2dBundle<ColorMaterial>,
    ) -> Self {
        let mut positions: Vec<Position> = Vec::new();
        positions.push(position);

        SpaceObjectBundle {
            planet: Planet,
            mass: mass,
            velocity: velocity,
            material2d: material2d,
            position: position,
            trail: PlanetTrail::default(),
        }
    }
}

// Show the planets
fn spawn_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // List of planets with their mass, velocity, position, and size
    let mut planet_list: Vec<(f64, Velocity, Position, Vec3, Color)> = Vec::new();

    // Sun
    planet_list.push((
        MASS_OF_SUN,
        Velocity { x: 0., y: 0. },
        SUN_POSITION,
        SUN_SIZE,
        SUN_COLOR,
    ));

    // Mercury
    planet_list.push((
        MASS_OF_MERCURY,
        Velocity {
            x: 0.,
            y: 47.4 * 1000.,
        },
        MERCURY_POSITION,
        MERCURY_SIZE,
        MERCURY_COLOR,
    ));

    // Venus
    planet_list.push((
        MASS_OF_VENUS,
        Velocity {
            x: 0.,
            y: 35. * 1000.,
        },
        VENUS_POSITION,
        VENUS_SIZE,
        VENUS_COLOR,
    ));

    // Earth
    planet_list.push((
        MASS_OF_EARTH,
        Velocity {
            x: 0.,
            y: 29.783 * 1000.,
        },
        EARTH_POSITION,
        EARTH_SIZE,
        EARTH_COLOR,
    ));

    // Mars
    planet_list.push((
        MASS_OF_MARS,
        Velocity {
            x: 0.,
            y: 24.077 * 1000.,
        },
        MARS_POSITION,
        MARS_SIZE,
        MARS_COLOR,
    ));

    // Jupiter
    planet_list.push((
        MASS_OF_JUPITER,
        Velocity {
            x: 0.,
            y: 13.1 * 1000.,
        },
        JUPITER_POSITION,
        JUPITER_SIZE,
        JUPITER_COLOR,
    ));

    // Saturn
    planet_list.push((
        MASS_OF_SATURN,
        Velocity {
            x: 0.,
            y: 9.7 * 1000.,
        },
        SATURN_POSITION,
        SATURN_SIZE,
        SATURN_COLOR,
    ));

    // Uranus
    planet_list.push((
        MASS_OF_URANUS,
        Velocity {
            x: 0.,
            y: 6.8 * 1000.,
        },
        URANUS_POSITION,
        URANUS_SIZE,
        URANUS_COLOR,
    ));

    // Neptune
    planet_list.push((
        MASS_OF_NEPTUNE,
        Velocity {
            x: 0.,
            y: 4.7 * 1000.,
        },
        NEPTUNE_POSITION,
        NEPTUNE_SIZE,
        NEPTUNE_COLOR,
    ));

    for (mass, velocity, position, size, color) in planet_list.iter() {
        commands.spawn(SpaceObjectBundle::new(
            Mass(*mass),
            Velocity {
                x: velocity.x,
                y: velocity.y,
            },
            *position,
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::default().into()).into(),
                material: materials.add(ColorMaterial::from(*color)),
                transform: Transform::from_translation(Vec3::new(
                    position.x as f32,
                    position.y as f32,
                    0.,
                ))
                .with_scale(*size),
                ..default()
            },
        ));
    }
}

// Runs f = G * M1 * M2 / d * d
fn apply_gravity(
    mut planet_query: Query<(Entity, &mut Position, &mut Velocity, &Mass), With<Planet>>,
) {
    let mut velocity_store: Vec<(f64, f64)> = Vec::new();

    for (entity, position, velocity, mass) in planet_query.iter() {
        let mut total_fx = 0.;
        let mut total_fy = 0.;
        for (other_entity, other_position, _other_veloctiy, other_mass) in planet_query.iter() {
            if entity != other_entity {
                let total_force = calculate_force(position, mass, other_position, other_mass);
                total_fx += total_force.0;
                total_fy += total_force.1;
            }
        }
        let mut new_x = velocity.x;
        let mut new_y = velocity.y;
        new_x += total_fx / mass.0 * TIMESTEP;
        new_y += total_fy / mass.0 * TIMESTEP;
        velocity_store.push((new_x, new_y));
    }

    let mut i = 0;
    for (_entity, mut position, mut velocity, _mass) in planet_query.iter_mut() {
        if let Some(result) = velocity_store.get(i) {
            let (x, y) = result;
            velocity.x = *x;
            velocity.y = *y;
            position.x += velocity.x * TIMESTEP;
            position.y += velocity.y * TIMESTEP;
            i += 1;
        }
    }
}

fn calculate_force(
    position: &Position,
    mass: &Mass,
    other_position: &Position,
    other_mass: &Mass,
) -> (f64, f64) {
    let distance_x = other_position.x - position.x;
    let distance_y = other_position.y - position.y;

    let total_distance = (distance_x.powi(2) + distance_y.powi(2)).sqrt();

    let force =
        GRAVITY as f64 * mass.0 as f64 * other_mass.0 as f64 / total_distance.powi(2) as f64;
    let theta = distance_y.atan2(distance_x) as f64;

    let force_x = theta.cos() * force;
    let force_y = theta.sin() * force;

    (force_x, force_y)
}

// Update planet positions after force has been applied
fn update_planets(
    mut planet_query: Query<(&mut Transform, &Position, &mut PlanetTrail), With<Planet>>,
) {
    for (mut transform, position, mut trail) in planet_query.iter_mut() {
        let x = position.x * SCALE;
        let y = position.y * SCALE;
        transform.translation.x = x as f32;
        transform.translation.y = y as f32;
        trail.add_position(position);
    }
}

fn draw_planet_trails(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    planet_trails: Query<&PlanetTrail, With<Planet>>,
    mut gizmos: Gizmos,
) {
    for trail in planet_trails.iter() {
        gizmos.linestrip_2d(trail.positions.clone(), Color::GREEN);
    }
}

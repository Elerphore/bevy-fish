use bevy::{
    app::{App, FixedUpdate, Startup},
    asset::Assets,
    color::Color,
    math::{Vec2, Vec3, Vec3Swizzles},
    prelude::{Camera2dBundle, Circle, Commands, Component, IntoSystemConfigs, Query, ResMut},
    render::mesh::Mesh,
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
    transform::components::Transform,
    window::Window,
    DefaultPlugins,
};
use rand::{rngs::ThreadRng, thread_rng, Rng};

const MAX_SPEED: f32 = 5.0;
const MIN_SPEED: f32 = 2.0;
const AVOID_FACTOR: f32 = 0.1;
const MATCHING_FACTOR: f32 = 0.05;
const CENTERING_FACTOR: f32 = 0.01;
const NEIGHBOR_RADIUS: f32 = 50.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                avoid_factor,
                aligment_factor_avg_vector,
                aligment_factor,
                cohesion_factor,
                screen_bounce,
                speed_limiter,
                move_fish,
            )
                .chain(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rnd: ThreadRng = thread_rng();
    let circle = Circle { radius: 8.0 };
    let color = Color::hsl(360., 0.95, 0.7);

    for _ in 0..299 {
        let transform = Transform::from_xyz(
            rnd.gen_range(-100.0..100.0),
            rnd.gen_range(-100.0..100.0),
            0.0,
        );
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(circle)),
                material: materials.add(color),
                transform,
                ..Default::default()
            },
            Fish {
                vec: Vec2::new(rnd.gen_range(-1.0..1.0), rnd.gen_range(-1.0..1.0)).normalize()
                    * MIN_SPEED,
            },
            MetaData {
                x_vel_avg: Vec2::ZERO,
                x_pos_avg: Vec2::ZERO,
            },
        ));
    }

    commands.spawn((Camera2dBundle::default(), CameraMarker));
}

fn move_fish(mut query: Query<(&mut Transform, &mut Fish)>) {
    for (mut transform, fish) in query.iter_mut() {
        transform.translation.x += fish.vec.x;
        transform.translation.y += fish.vec.y;
    }
}

fn aligment_factor(mut query: Query<(&mut Fish, &MetaData)>) {
    for (mut fish, metadata) in query.iter_mut() {
        let vec = (metadata.x_vel_avg - fish.vec.extend(0.0).xy()) * MATCHING_FACTOR;
        fish.vec += vec;
    }
}

fn aligment_factor_avg_vector(mut query: Query<(&mut Transform, &mut Fish, &mut MetaData)>) {
    let mut total_vel = Vec2::ZERO;
    let mut total_pos = Vec2::ZERO;

    let count = query.iter().count() as f32;

    for (transform, fish, _) in query.iter() {
        total_vel += fish.vec;
        total_pos += transform.translation.xy();
    }

    for (_, _, mut metadata) in query.iter_mut() {
        metadata.x_vel_avg = total_vel / count;
        metadata.x_pos_avg = total_pos / count;
    }
}

fn cohesion_factor(mut query: Query<(&mut Transform, &mut MetaData, &mut Fish)>) {
    for (transform, metadata, mut fish) in query.iter_mut() {
        let vec = (metadata.x_pos_avg - transform.translation.xy()) * CENTERING_FACTOR;
        fish.vec += vec;
    }
}

fn speed_limiter(mut query: Query<(&mut Transform, &mut Fish)>) {
    for (_transform, mut fish) in query.iter_mut() {
        let speed = fish.vec.length();

        if speed > MAX_SPEED {
            fish.vec = fish.vec.normalize() * MAX_SPEED;
        }

        if speed < MIN_SPEED {
            fish.vec = fish.vec.normalize() * MIN_SPEED;
        }
    }
}

fn avoid_factor(mut query: Query<&mut Fish>) {
    let mut iter = query.iter_combinations_mut();

    while let Some([mut fish_one, fish_two]) = iter.fetch_next() {
        let distance = (fish_one.vec - fish_two.vec).length();

        if distance < NEIGHBOR_RADIUS {
            let avoidance_vector = (fish_one.vec - fish_two.vec).normalize() * AVOID_FACTOR;
            fish_one.vec += avoidance_vector;
        }
    }
}

fn screen_bounce(windows: Query<&Window>, mut query: Query<(&mut Fish, &Transform)>) {
    let window = windows.single();
    let height = window.height();
    let width = window.width();

    let left = -width / 2.0;
    let right = width / 2.0;
    let bottom = -height / 2.0;
    let top = height / 2.0;

    const MARGIN: f32 = 100.0;

    for (mut fish, transform) in query.iter_mut() {
        if transform.translation.x < left + MARGIN {
            fish.vec.x = fish.vec.x.abs();
        }

        if transform.translation.x > right - MARGIN {
            fish.vec.x = -fish.vec.x.abs();
        }

        if transform.translation.y > top - MARGIN {
            fish.vec.y = -fish.vec.y.abs();
        }

        if transform.translation.y < bottom + MARGIN {
            fish.vec.y = fish.vec.y.abs();
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
struct Fish {
    vec: Vec2,
}

#[derive(Component, Debug)]
struct MetaData {
    x_vel_avg: Vec2,
    x_pos_avg: Vec2,
}

#[derive(Component)]
struct CameraMarker;

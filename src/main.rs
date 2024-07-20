use bevy::{
    app::{App, FixedUpdate, Startup},
    asset::Assets,
    color::Color,
    math::{Vec3, Vec3Swizzles},
    prelude::{Camera2dBundle, Circle, Commands, Component, IntoSystemConfigs, Query, ResMut},
    render::mesh::Mesh,
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
    transform::components::Transform,
    DefaultPlugins,
};
use rand::{rngs::ThreadRng, thread_rng, Rng};

const _X_EXTENT: f32 = 900.;
const MAX_SPEED: f32 = 6.0;
const MIN_SPEED: f32 = 3.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (avoid_factor, speed_limiter, move_fish).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rnd: ThreadRng = thread_rng();
    let circle = Circle { radius: 32.0 };
    let color = Color::hsl(360., 0.95, 0.7);

    for _ in 0..10 {
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
                vec: transform.translation.xyz(),
            },
        ));
    }

    commands.spawn((Camera2dBundle::default(), CameraMarker));
}

fn move_fish(mut query: Query<(&mut Transform, &mut Fish)>) {
    query.iter_mut().for_each(|(mut transform, mut fish)| {
        transform.translation.x += fish.vec.x;
        transform.translation.y += fish.vec.y;
    })
}

fn speed_limiter(mut query: Query<(&mut Transform, &mut Fish)>) {
    query.iter_mut().for_each(|(mut transform, mut fish)| {
        let speed = (fish.vec.x * fish.vec.x + fish.vec.y * fish.vec.y).sqrt();

        if speed > MAX_SPEED {
            fish.vec.x = (fish.vec.x / speed) * MAX_SPEED;
            fish.vec.y = (fish.vec.y / speed) * MIN_SPEED;
        }

        if speed < MIN_SPEED {
            fish.vec.x = (fish.vec.x / speed) * MIN_SPEED;
            fish.vec.y = (fish.vec.y / speed) * MIN_SPEED;
        }
    })
}

fn avoid_factor(mut query: Query<&mut Fish>) {
    let mut iter = query.iter_combinations_mut();
    let avoidfactor = 0.05;

    while let Some([mut fish_one, fish_two]) = iter.fetch_next() {
        let close_dx = fish_one.vec.x - fish_two.vec.x;
        let close_dy = fish_one.vec.y - fish_two.vec.y;

        fish_one.vec.x += close_dx * avoidfactor;
        fish_one.vec.y += close_dy * avoidfactor;
    }
}

#[derive(Component, Debug, Clone, Copy)]
struct Fish {
    vec: Vec3,
}

#[derive(Component)]
struct CameraMarker;

use bevy::{
    app::{App, Startup}, asset::Assets, color::Color, math::{vec2, Vec2}, prelude::{Camera2dBundle, Circle, Commands, Component, ResMut}, render::mesh::Mesh, sprite::{ColorMaterial, Mesh2dHandle}, DefaultPlugins
};

const X_EXTENT: f32 = 900.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let circle = Circle { radius: 32.0 };
    let mash_handle = Mesh2dHandle(meshes.add(circle));

    let color = Color::hsl(360., 0.95, 0.7);

    commands.spawn(Camera2dBundle::default());

    commands.spawn(
        (Fish {
            vec: vec2(0.0, 0.0),
        }),
    );
}

#[derive(Component)]
struct Fish {
    vec: Vec2,
}

#[derive(Component)]
struct CameraMarker;

use bevy::prelude::*;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Player;

#[derive(Component, Default, Clone)]
struct PlayerView;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_player)
        .add_systems(Startup, arena.spawn())
        .run();
}

fn add_player(mut commands: Commands) {
    commands.spawn((Player, Name("Sheitan".to_string())));
}

/// set up a simple 3D scene
fn arena() -> impl SceneList {
    bsn_list! {
        #CircularBase
        Mesh3d(asset_value(Circle::new(4.0)))
        MeshMaterial3d::<StandardMaterial>(asset_value(Color::WHITE))
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
        --
        #Sphere
        Mesh3d(asset_value(Sphere::new(1.0)))
        MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb_u8(124, 144, 255)))
        Transform::from_xyz(0.0, 0.5, 0.0)
        --
        #Cube
        Mesh3d(asset_value(Cuboid::new(1.0, 1.0, 1.0)))
        MeshMaterial3d<StandardMaterial>(asset_value(Color::srgb_u8(94, 47, 170)))
        Transform::from_xyz(-1.0, 1.0, -5.0)
        --
        PointLight {
            shadow_maps_enabled: true,
        }
        Transform::from_xyz(4.0, 8.0, 4.0)
        --
        Camera3d
        Transform::from_xyz(-0.1, 3.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y)
    }
}

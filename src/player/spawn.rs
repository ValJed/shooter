use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{Gun, Jumps, Player, PlayerCamera, PlayerName};

pub fn add_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player,
        RigidBody::Dynamic,
        Collider::capsule_y(0.5, 0.4),
        LockedAxes::ROTATION_LOCKED,
        Friction::coefficient(0.0),
        PlayerName("Sheitan".to_string()),
        Mesh3d(meshes.add(Capsule3d::new(0.5, 0.5))), // radius 0.5, length 0.5 (= 2 * 0.25)
        MeshMaterial3d(materials.add(Color::srgb(0.9, 0.7, 0.2))),
        Transform::from_xyz(0.0, 2.0, 3.0),
        Jumps::new(),
        Velocity::default(),
        children![(
            Camera3d::default(),
            PlayerCamera,
            Transform::from_xyz(0.0, 0.6, -0.2),
            children![render_gun(&asset_server)]
        )],
    ));
}

pub fn render_gun(asset_server: &AssetServer) -> impl Bundle {
    (
        Name("Gun".into()),
        Gun,
        WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/gun/scene.gltf")),
        ),
        Transform::from_xyz(0.25, -0.2, -0.5)
            .with_rotation(Quat::from_rotation_y(PI))
            .with_scale(Vec3::splat(0.5)),
    )
}

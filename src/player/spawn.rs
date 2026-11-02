use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{Jumps, Player, PlayerCamera, PlayerName};

pub fn add_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
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
        )],
    ));
}

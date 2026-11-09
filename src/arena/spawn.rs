use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// set up a simple 3D scene
pub fn render_arena(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_floor(&mut commands, &mut meshes, &mut materials);
    spawn_sphere(&mut commands, &mut meshes, &mut materials);
    spawn_cubes(&mut commands, &mut meshes, &mut materials);
    spawn_light(&mut commands);
}

fn spawn_floor(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Floor"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        RigidBody::Fixed,
        Collider::halfspace(Vec3::Y).unwrap(),
    ));
}

fn spawn_sphere(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Sphere"),
        Mesh3d(meshes.add(Sphere::new(1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 1.0, 0.0),
        RigidBody::Fixed,
        Collider::ball(1.0),
    ));
}

fn spawn_cubes(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let material = materials.add(Color::srgb_u8(94, 47, 170));

    for x in -3..=3 {
        for z in -3..=3 {
            commands.spawn((
                Name::new(format!("Cube {x},{z}")),
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(x as f32 * 3.0, 0.5, z as f32 * 3.0 - 10.0),
                RigidBody::Fixed,
                Collider::cuboid(0.5, 0.5, 0.5),
            ));
        }
    }
}

fn spawn_light(commands: &mut Commands) {
    commands.spawn((
        Name::new("Light"),
        PointLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

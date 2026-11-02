use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// set up a simple 3D scene
pub fn render_arena() -> impl SceneList {
    bsn_list! {
        #Floor
        Mesh3d(asset_value(Plane3d::default().mesh().size(20.0, 20.0)))
        MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb(0.3, 0.5, 0.3)))
        Collider::halfspace(Vec3::Y).unwrap()
        RigidBody::Fixed

        --
        #Sphere
        Mesh3d(asset_value(Sphere::new(1.0)))
        MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb_u8(124, 144, 255)))
        Transform::from_xyz(0.0, 0.5, 0.0)
        Collider::ball(1.0)
        RigidBody::Fixed

        --
        #Cube
        Mesh3d(asset_value(Cuboid::new(1.0, 1.0, 1.0)))
        MeshMaterial3d<StandardMaterial>(asset_value(Color::srgb_u8(94, 47, 170)))
        Transform::from_xyz(-1.0, 1.0, -5.0)
        Collider::cuboid(0.5, 0.5, 0.5)
        RigidBody::Fixed

        --
        PointLight {
            shadow_maps_enabled: true,
        }
        Transform::from_xyz(4.0, 8.0, 4.0)
    }
}

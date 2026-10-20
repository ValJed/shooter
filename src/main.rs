use bevy::prelude::*;

#[derive(Component, Default)]
struct Velocity(Vec3);

#[derive(Component)]
struct Player;

#[derive(Component, Default, Clone)]
struct PlayerCamera;

#[derive(Component)]
struct Jump {
    remaining: u8,
    max: u8,
}

impl Jump {
    fn jump(
        mut commands: Commands,
        time: Res<Time>,
        mut query: Query<&mut Transform, With<Player>>,
    ) {
    }
    fn is_jumping(mut query: Query<&mut Transform, With<Player>>) {}
    fn can_jump() {}
}

#[derive(Component)]
struct Name(String);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Action {
    Forward,
    Back,
    Left,
    Right,
    Jump,
}

impl Action {
    const fn keys(self) -> &'static [KeyCode] {
        match self {
            Action::Forward => &[KeyCode::KeyW, KeyCode::ArrowUp],
            Action::Back => &[KeyCode::KeyS, KeyCode::ArrowDown],
            Action::Left => &[KeyCode::KeyA, KeyCode::ArrowLeft],
            Action::Right => &[KeyCode::KeyD, KeyCode::ArrowRight],
            Action::Jump => &[KeyCode::Space],
        }
    }

    fn pressed(self, keys: &ButtonInput<KeyCode>) -> bool {
        keys.any_pressed(self.keys().iter().copied())
    }

    // fn just_pressed(self, keys: &ButtonInput<KeyCode>) -> bool {
    //     keys.any_just_pressed(self.keys().iter().copied())
    // }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_player)
        .add_systems(Startup, arena.spawn())
        .add_systems(Update, move_player)
        .run();
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut dir = Vec3::ZERO;

    if Action::Forward.pressed(&keys) {
        dir.z -= 1.0
    }

    if Action::Back.pressed(&keys) {
        dir.z += 1.0
    }

    if Action::Right.pressed(&keys) {
        dir.x += 1.0
    }

    if Action::Left.pressed(&keys) {
        dir.x -= 1.0
    }
    if Action::Jump.pressed(&keys) {
        info!("jump")
    }

    for mut transform in &mut query {
        transform.translation += dir.normalize_or_zero() * 3.0 * time.delta_secs();
    }
}

fn add_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player,
        Name("Sheitan".to_string()),
        Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.9, 0.7, 0.2))),
        Transform::from_xyz(0.0, 0.9, 3.0),
        children![(
            Camera3d::default(),
            // Inside the player's view
            Transform::from_xyz(0.0, 0.6, -0.2),
        )],
    ));
}

fn player_jump(mut commands: Commands) {}

/// set up a simple 3D scene
fn arena() -> impl SceneList {
    bsn_list! {
        #Floor
        Mesh3d(asset_value(Plane3d::default().mesh().size(20.0, 20.0)))
        MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb(0.3, 0.5, 0.3)))

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
    }
}

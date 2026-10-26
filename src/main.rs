use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use bevy_rapier3d::prelude::*;

const MAX_JUMP: u8 = 2;

#[derive(Component, Default)]
struct Velocity(Vec3);

#[derive(Component)]
struct Player;

#[derive(Component, Default, Clone)]
struct PlayerCamera;

#[derive(Component)]
struct Jumps {
    is_jumping: bool,
    remaining: u8,
}

impl Jumps {
    fn new() -> Self {
        Self {
            is_jumping: false,
            remaining: MAX_JUMP,
        }
    }
    fn start_jump(
        mut commands: Commands,
        time: Res<Time>,
        mut query: Query<(&mut Transform, &mut Jumps, &mut Velocity), With<Player>>,
    ) {
        let Ok((mut transform, mut jumps, mut velocity)) = query.single_mut() else {
            return;
        };

        if jumps.remaining == 0 {
            return;
        }

        jumps.is_jumping = true;
        jumps.remaining -= 1;
        // velocity.from_xyz = 1.0;
    }

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
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, add_player)
        .add_systems(Startup, arena.spawn())
        .add_systems(Startup, set_cursor)
        .add_systems(Update, move_player)
        .add_systems(Update, look)
        .add_systems(Update, toggle_cursor)
        .run();
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut PlayerCamera), With<Player>>,
) {
    // if let Ok(mut camera) = query.single_mut() {
    //     tf.rotate_y(-delta.x * sensitivity);
    // }
    println!("keys: {:?}", keys);
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
        RigidBody::Dynamic,
        Collider::capsule_y(0.5, 0.4),
        LockedAxes::ROTATION_LOCKED,
        Friction::coefficient(0.0),
        Name("Sheitan".to_string()),
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

fn set_cursor(mut cursor: Query<&mut CursorOptions>) {
    let Ok(mut cursor) = cursor.single_mut() else {
        return;
    };
    cursor.grab_mode = CursorGrabMode::Locked;
    cursor.visible = false;
}

fn toggle_cursor(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut cursor: Query<&mut CursorOptions>,
) {
    let Ok(mut cursor) = cursor.single_mut() else {
        return;
    };

    // Escape releases the cursor
    if keys.just_pressed(KeyCode::Escape) {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
    }
    // Click to grab it again
    if mouse.just_pressed(MouseButton::Left) {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    }
}

fn look(
    mut motion: MessageReader<MouseMotion>,
    mut player: Query<&mut Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
) {
    let delta: Vec2 = motion.read().map(|m| m.delta).sum();
    if delta == Vec2::ZERO {
        return;
    }
    let sensitivity = 0.002;

    // Yaw: rotate the whole player around Y
    if let Ok(mut tf) = player.single_mut() {
        tf.rotate_y(-delta.x * sensitivity);
    }

    // Pitch: rotate only the camera, clamped so you can't flip over
    if let Ok(mut tf) = camera.single_mut() {
        let (pitch, _, _) = tf.rotation.to_euler(EulerRot::XYZ);
        let pitch = (pitch - delta.y * sensitivity).clamp(-1.5, 1.5);
        tf.rotation = Quat::from_rotation_x(pitch);
    }
}

fn player_jump(mut commands: Commands) {}

/// set up a simple 3D scene
fn arena() -> impl SceneList {
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

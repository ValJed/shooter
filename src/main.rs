use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};
use bevy_rapier3d::prelude::*;

const SPEED: f32 = 5.0;
const MAX_JUMP: u8 = 2;

#[derive(Component)]
struct Player;

#[derive(Component, Default, Clone)]
struct PlayerCamera;

#[derive(Component, Debug)]
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

    fn start_jump(&mut self) -> bool {
        let can_jump = self.can_jump();
        println!("can_jump: {:?}", can_jump);
        if !self.can_jump() {
            return false;
        }

        self.is_jumping = true;
        self.remaining -= 1;

        true
    }

    fn can_jump(&self) -> bool {
        self.remaining > 0
    }

    fn land(&mut self) {
        self.is_jumping = false;
        self.remaining = MAX_JUMP;
    }
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

    fn just_pressed(self, keys: &ButtonInput<KeyCode>) -> bool {
        keys.any_just_pressed(self.keys().iter().copied())
    }
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
        .add_systems(Update, jump_system)
        .run();
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Transform, &mut Velocity, &mut Jumps), With<Player>>,
) {
    let Ok((tf, mut vel, mut jumps)) = query.single_mut() else {
        return;
    };

    let mut input = Vec2::ZERO;
    if Action::Forward.pressed(&keys) {
        input.y += 1.0
    }
    if Action::Back.pressed(&keys) {
        input.y -= 1.0
    }
    if Action::Right.pressed(&keys) {
        input.x += 1.0
    }
    if Action::Left.pressed(&keys) {
        input.x -= 1.0
    }
    if Action::Jump.just_pressed(&keys) {
        if jumps.start_jump() {
            vel.linear.y = 5.0;
        }
    }

    let local = Vec3::new(input.x, 0.0, -input.y).normalize_or_zero() * SPEED;
    let world = tf.rotation * local;

    vel.linear.x = world.x;
    vel.linear.z = world.z;
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

fn set_cursor(mut query: Query<&mut CursorOptions>) {
    let Ok(mut cursor) = query.single_mut() else {
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

fn jump_system(
    mut commands: Commands,
    mut query: Query<(&mut Velocity, &mut Jumps), With<Player>>,
) {
    let Ok((mut vel, mut jumps)) = query.single_mut() else {
        return;
    };
    if !jumps.is_jumping {
        return;
    }

    println!("vel in jump system: {:?}", vel);

    if vel.linear.y == 0.0 {
        println!("LANDING");
        jumps.land()
    }
}

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

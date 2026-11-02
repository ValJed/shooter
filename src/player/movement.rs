use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};
use bevy_rapier3d::prelude::*;

use super::{Jumps, Player, PlayerCamera};

const SPEED: f32 = 5.0;
const MAX_JUMP: u8 = 2;

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

impl Jumps {
    pub fn new() -> Self {
        Self {
            is_jumping: false,
            remaining: MAX_JUMP,
        }
    }

    fn start_jump(&mut self) -> bool {
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

pub fn move_player(
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

pub fn jump_system(mut query: Query<(&Velocity, &mut Jumps), With<Player>>) {
    let Ok((vel, mut jumps)) = query.single_mut() else {
        return;
    };
    if !jumps.is_jumping {
        return;
    }

    if vel.linear.y == 0.0 {
        jumps.land()
    }
}

pub fn look(
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

pub fn toggle_cursor(
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

pub fn set_cursor(mut query: Query<&mut CursorOptions>) {
    let Ok(mut cursor) = query.single_mut() else {
        return;
    };
    cursor.grab_mode = CursorGrabMode::Locked;
    cursor.visible = false;
}

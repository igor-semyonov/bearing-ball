use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Player(pub usize);

#[derive(Component)]
pub struct Collider;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct Gravity(pub f32);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Bounds {
    pub right: f32,
    pub left: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            right: -160.0 / 2.0 - 20.0 / 2.0,
            left: -750.0 + 160.0 / 2.0 + 10.0 / 2.0,
            top: 480.0 - 160.0 / 2.0 - 10.0 / 2.0,
            bottom: -480.0 + 160.0 / 2.0 + 10.0 / 2.0,
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<
        (
            &Player,
            &Transform,
            &mut Velocity,
            &Bounds,
        ),
        With<Player>,
    >,
) {
    const PLAYER_MOVE_LEFT: [KeyCode; 2] = [KeyCode::KeyA, KeyCode::KeyJ];
    const PLAYER_MOVE_RIGHT: [KeyCode; 2] = [KeyCode::KeyD, KeyCode::KeyL];
    const PLAYER_JUMP: [KeyCode; 2] = [KeyCode::KeyW, KeyCode::KeyI];
    const PLAYER_SPEED: f32 = 900.0;
    const PLAYER_JUMP_SPEED: f32 = 1200.0;
    for (&player_id, transform, mut velocity, bound) in query {
        let mut direction = 0.0;
        if keyboard_input.pressed(PLAYER_MOVE_LEFT[*player_id]) {
            direction -= PLAYER_SPEED;
        }
        if keyboard_input.pressed(PLAYER_MOVE_RIGHT[*player_id]) {
            direction += PLAYER_SPEED;
        }
        velocity.x = direction;
        if keyboard_input.pressed(PLAYER_JUMP[*player_id]) {
            if transform.translation.y <= bound.bottom {
                velocity.y = PLAYER_JUMP_SPEED;
            }
        }
    }
}

pub fn apply_player_bounds(
    query: Query<
        (
            &mut Transform,
            &mut Velocity,
            &Bounds,
        ),
        With<Player>,
    >,
) {
    for (mut transform, mut velocity, bound) in query {
        if transform.translation.x > bound.right {
            transform.translation.x = bound.right;
            velocity.x = 0.0;
        } else if transform.translation.x < bound.left {
            transform.translation.x = bound.left;
            velocity.x = 0.0;
        }
        if transform.translation.y > bound.top {
            transform.translation.y = bound.top;
            velocity.y = 0.0;
        } else if transform.translation.y < bound.bottom {
            transform.translation.y = bound.bottom;
            velocity.y = 0.0;
        }
    }
} 
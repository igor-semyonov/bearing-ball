use bevy::prelude::*;
use crate::config::{Config, string_to_keycode};

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
    config: Res<Config>,
) {
    let player_speed = config.player.speed;
    let player_jump_speed = config.player.jump_speed;
    
    for (&player_id, transform, mut velocity, bound) in query {
        let mut direction = 0.0;
        
        // Get the appropriate controls for this player
        let (move_left, move_right, jump) = if *player_id == 0 {
            (
                string_to_keycode(&config.controls.player1_move_left),
                string_to_keycode(&config.controls.player1_move_right),
                string_to_keycode(&config.controls.player1_jump),
            )
        } else {
            (
                string_to_keycode(&config.controls.player2_move_left),
                string_to_keycode(&config.controls.player2_move_right),
                string_to_keycode(&config.controls.player2_jump),
            )
        };
        
        if keyboard_input.pressed(move_left) {
            direction -= player_speed;
        }
        if keyboard_input.pressed(move_right) {
            direction += player_speed;
        }
        velocity.x = direction;
        if keyboard_input.pressed(jump) {
            if transform.translation.y <= bound.bottom {
                velocity.y = player_jump_speed;
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
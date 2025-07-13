use crate::prelude::*;
use crate::config::{Config, string_to_keycode};
use bevy::math::bounding::{
    Aabb2d, BoundingCircle, IntersectsVolume,
};

pub fn apply_velocity(
    mut query: Query<(
        &mut Transform,
        &Velocity,
    )>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in &mut query {
        transform
            .translation
            .x += velocity.x * time.delta_secs();
        transform
            .translation
            .y += velocity.y * time.delta_secs();
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
    let player_speed = config
        .player
        .speed;
    let player_jump_speed = config
        .player
        .jump_speed;

    for (&player_id, transform, mut velocity, bound) in
        query
    {
        let mut direction = 0.0;

        // Get the appropriate controls for this player
        let (move_left, move_right, jump) =
            if *player_id == 0 {
                (
                    string_to_keycode(
                        &config
                            .controls
                            .player1_move_left,
                    ),
                    string_to_keycode(
                        &config
                            .controls
                            .player1_move_right,
                    ),
                    string_to_keycode(
                        &config
                            .controls
                            .player1_jump,
                    ),
                )
            } else {
                (
                    string_to_keycode(
                        &config
                            .controls
                            .player2_move_left,
                    ),
                    string_to_keycode(
                        &config
                            .controls
                            .player2_move_right,
                    ),
                    string_to_keycode(
                        &config
                            .controls
                            .player2_jump,
                    ),
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
            if transform
                .translation
                .y
                <= bound.bottom
            {
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
        if transform
            .translation
            .x
            > bound.right
        {
            transform
                .translation
                .x = bound.right;
            velocity.x = 0.0;
        } else if transform
            .translation
            .x
            < bound.left
        {
            transform
                .translation
                .x = bound.left;
            velocity.x = 0.0;
        }
        if transform
            .translation
            .y
            > bound.top
        {
            transform
                .translation
                .y = bound.top;
            velocity.y = 0.0;
        } else if transform
            .translation
            .y
            < bound.bottom
        {
            transform
                .translation
                .y = bound.bottom;
            velocity.y = 0.0;
        }
    }
}

pub fn apply_gravity(
    mut query: Query<
        (
            &mut Velocity,
            &Gravity,
        ),
        Or<(
            With<Ball>,
            With<Player>,
        )>,
    >,
    time: Res<Time>,
) {
    for (mut velocity, gravity) in &mut query {
        velocity.y -= gravity.0 * time.delta_secs();
    }
}

pub fn check_for_ball_collisions(
    // mut commands: Commands,
    mut ball_query: Query<
        (
            &mut Transform,
            &mut Velocity,
        ),
        With<Ball>,
    >,
    player_query: Query<
        (
            &Transform,
            &Velocity,
        ),
        (
            With<Player>,
            Without<Ball>,
        ),
    >,
    net_query: Query<
        (&Transform,),
        (
            With<Net>,
            Without<Ball>,
        ),
    >,
    mut collision_event: EventWriter<
        CollisionEvent,
    >,
) {
    let Ok((mut ball_transform, mut ball_velocity)) =
        ball_query.single_mut()
    else {
        return;
    };

    for (player_transform, player_velocity) in &player_query
    {
        let ball_bounding_circle = BoundingCircle::new(
            ball_transform
                .translation
                .truncate(),
            70.0 / 2.0,
        );
        let player_bounding_circle = BoundingCircle::new(
            player_transform
                .translation
                .truncate(),
            160.0 / 2.0,
        );
        if !ball_bounding_circle
            .intersects(&player_bounding_circle)
        {
            continue;
        }
        let collision_normal =
            (player_bounding_circle.closest_point(
                ball_transform
                    .translation
                    .truncate(),
            ) - player_transform
                .translation
                .truncate())
            .normalize();
        ball_transform.translation = player_transform
            .translation
            + (collision_normal
                * ((160.0 + 70.0) / 2.0 + 0.01))
                .extend(5.0);
        if ball_velocity.x == 0.0
            && ball_transform
                .translation
                .x
                == player_transform
                    .translation
                    .x
        {
            ball_velocity.y *= -1.0;
        } else {
            **ball_velocity = **ball_velocity
                - 2.0
                    * ball_velocity.dot(collision_normal)
                    * collision_normal;
        }
        let player_velocity_projected_onto_collision_normal =
            player_velocity
                .xy()
                .dot(collision_normal)
                * collision_normal;
        **ball_velocity +=
            player_velocity_projected_onto_collision_normal;
        collision_event.write_default();
    }

    let Ok((net_transform,)) = net_query.single() else {
        return;
    };

    let ball_bounding_circle = BoundingCircle::new(
        ball_transform
            .translation
            .truncate(),
        70.0 / 2.0,
    );
    let net_bounding_box = Aabb2d::new(
        net_transform
            .translation
            .truncate(),
        net_transform
            .scale
            .truncate()
            / 2.0,
    );
    if ball_bounding_circle.intersects(&net_bounding_box) {
        let collision_normal =
            (net_bounding_box.closest_point(
                ball_transform
                    .translation
                    .truncate(),
            ) - ball_transform
                .translation
                .truncate())
            .normalize();
        let theta =
            ball_velocity.angle_to(collision_normal);
        let new_ball_velocity = -ball_velocity
            .rotate(Vec2::from_angle(2.0 * theta));
        ball_velocity.x = new_ball_velocity.x;
        ball_velocity.y = new_ball_velocity.y;
        collision_event.write_default();
    }
}

pub fn apply_ball_bounds(
    mut query: Query<
        (
            &mut Transform,
            &mut Velocity,
            &Bounds,
        ),
        With<Ball>,
    >,
    mut score: ResMut<Score>,
) {
    let Ok((mut transform, mut velocity, bound)) =
        query.single_mut()
    else {
        return;
    };

    if transform
        .translation
        .x
        > bound.right
    {
        transform
            .translation
            .x = bound.right;
        velocity.x *= -1.0;
    } else if transform
        .translation
        .x
        < bound.left
    {
        transform
            .translation
            .x = bound.left;
        velocity.x *= -1.0;
    }
    if transform
        .translation
        .y
        > bound.top
    {
        transform
            .translation
            .y = bound.top;
        velocity.y *= -0.8; // CEILING_DAMPING_FACTOR
    } else if transform
        .translation
        .y
        < bound.bottom
    {
        transform
            .translation
            .y = bound.bottom;
        velocity.y *= -1.0;
        if transform
            .translation
            .x
            > 0.0
        {
            score[0] += 1;
        } else {
            score[1] += 1;
        }
    }
}

#[allow(dead_code)]
pub fn process_just_scored(_query: Query<&mut Velocity>) {
    // if *game_state == GameModeState::JustScored {
    //     for mut velocity in query {
    //         velocity.x = 0.0;
    //         velocity.y = 0.0;
    //     }
    // }
}

pub fn play_collision_sound(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    sound: Res<CollisionSound>,
    config: Res<Config>,
) {
    use bevy::audio::Volume;
    if !collision_events.is_empty() {
        collision_events.clear();
        commands.spawn((
            AudioPlayer(sound.clone()),
            PlaybackSettings {
                volume: Volume::Linear(
                    config
                        .audio
                        .volume,
                ),
                ..PlaybackSettings::DESPAWN
            },
        ));
    }
}

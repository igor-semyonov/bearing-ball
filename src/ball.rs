use bevy::prelude::*;
use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use crate::player::{Velocity, Gravity, Bounds, Player};
use crate::score::Score;

#[derive(Component)]
pub struct Ball;

pub fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

pub fn apply_gravity(
    mut query: Query<
        (
            &mut Velocity,
            &Gravity
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
    mut ball_query: Query<(
        &mut Transform,
        &mut Velocity,
    ), With<Ball>>,
    player_query: Query<(
        &Transform,
        &Velocity,
    ), (With<Player>, Without<Ball>)>,
    net_query: Query<(&Transform,), (With<crate::net::Net>, Without<Ball>)>,
    mut collision_event: EventWriter<crate::sound::CollisionEvent>,
) {
    let Ok((mut ball_transform, mut ball_velocity)) = ball_query.single_mut() else {
        return;
    };
    
    for (player_transform, player_velocity) in &player_query {
        let ball_bounding_circle = BoundingCircle::new(
            ball_transform.translation.truncate(),
            70.0 / 2.0,
        );
        let player_bounding_circle = BoundingCircle::new(
            player_transform.translation.truncate(),
            160.0 / 2.0,
        );
        if !ball_bounding_circle.intersects(&player_bounding_circle) {
            continue;
        }
        let collision_normal =
            (player_bounding_circle.closest_point(ball_transform.translation.truncate())
                - player_transform.translation.truncate())
                .normalize();
        ball_transform.translation = player_transform.translation
            + (collision_normal * ((160.0 + 70.0) / 2.0 + 0.01)).extend(5.0);
        if ball_velocity.x == 0.0 && ball_transform.translation.x == player_transform.translation.x {
            ball_velocity.y *= -1.0;
        } else {
            **ball_velocity = **ball_velocity - 2.0 * ball_velocity.dot(collision_normal) * collision_normal;
        }
        let player_velocity_projected_onto_collision_normal =
            player_velocity.xy().dot(collision_normal) * collision_normal;
        **ball_velocity += player_velocity_projected_onto_collision_normal;
        collision_event.write_default();
    }
    
    let Ok((net_transform,)) = net_query.single() else {
        return;
    };
    
    let ball_bounding_circle = BoundingCircle::new(
        ball_transform.translation.truncate(),
        70.0 / 2.0,
    );
    let net_bounding_box = Aabb2d::new(
        net_transform.translation.truncate(),
        net_transform.scale.truncate() / 2.0,
    );
    if ball_bounding_circle.intersects(&net_bounding_box) {
        let collision_normal =
            (net_bounding_box.closest_point(ball_transform.translation.truncate())
                - ball_transform.translation.truncate())
                .normalize();
        let theta = ball_velocity.angle_to(collision_normal);
        let new_ball_velocity = -ball_velocity.rotate(Vec2::from_angle(2.0 * theta));
        ball_velocity.x = new_ball_velocity.x;
        ball_velocity.y = new_ball_velocity.y;
        collision_event.write_default();
    }
}

pub fn apply_ball_bounds(
    mut query: Query<(
        &mut Transform,
        &mut Velocity,
        &Bounds,
    ), With<Ball>>,
    mut score: ResMut<crate::score::Score>,
) {
    let Ok((mut transform, mut velocity, bound)) = query.single_mut() else {
        return;
    };
    
    if transform.translation.x > bound.right {
        transform.translation.x = bound.right;
        velocity.x *= -1.0;
    } else if transform.translation.x < bound.left {
        transform.translation.x = bound.left;
        velocity.x *= -1.0;
    }
    if transform.translation.y > bound.top {
        transform.translation.y = bound.top;
        velocity.y *= -0.8; // CEILING_DAMPING_FACTOR
    } else if transform.translation.y < bound.bottom {
        transform.translation.y = bound.bottom;
        velocity.y *= -1.0;
        if transform.translation.x > 0.0 {
            score[0] += 1;
        } else {
            score[1] += 1;
        }
    }
}

pub fn process_just_scored(
    _query: Query<&mut Velocity>,
) {
    // if *game_state == GameModeState::JustScored {
    //     for mut velocity in query {
    //         velocity.x = 0.0;
    //         velocity.y = 0.0;
    //     }
    // }
} 
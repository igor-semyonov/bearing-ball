pub mod fixed_update;
pub mod update;

use crate::prelude::*;
use crate::config::Config;
use bevy::text::{TextColor, TextFont};
use bevy::ui::{Node, PositionType, Val};
use iyes_perf_ui::entries::PerfUiDefaultEntries;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
) {
    // spawn a camera to be able to see anything
    commands.spawn((
        Camera2d, MainCamera,
    ));

    // create a simple Perf UI with default settings
    // and all entries provided by the crate:
    commands.spawn((PerfUiDefaultEntries::default(),));

    // Calculate arena boundaries
    let arena_half_width = config
        .arena
        .width
        / 2.0;
    let arena_half_height = config
        .arena
        .height
        / 2.0;
    let wall_thickness = config
        .arena
        .wall_thickness;
    let player_size = config
        .player
        .size;
    let player_gravity = config
        .gravity
        .player;
    let player_spacing = config
        .arena
        .player_spacing;

    // Player 1 (left side)
    let player1_x = -player_spacing;
    let player1_y = -arena_half_height
        + player_size / 2.0
        + wall_thickness;

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(
            materials.add(
                Color::srgb(
                    1.0, 0.5, 0.0,
                ),
            ),
        ),
        Transform::from_translation(
            Vec3::new(
                player1_x, player1_y, 0.0,
            ),
        )
        .with_scale(Vec2::splat(player_size).extend(1.)),
        Player(0),
        Collider,
        Velocity(Vec2::splat(0.0)),
        Gravity(player_gravity),
        Bounds {
            right: -player_size / 2.0 - wall_thickness,
            left: -arena_half_width
                + player_size / 2.0
                + wall_thickness,
            top: arena_half_height
                - player_size / 2.0
                - wall_thickness,
            bottom: -arena_half_height
                + player_size / 2.0
                + wall_thickness,
        },
    ));

    // Player 2 (right side)
    let player2_x = player_spacing;
    let player2_y = -arena_half_height
        + player_size / 2.0
        + wall_thickness;

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(
            materials.add(
                Color::srgb(
                    0.0, 0.0, 1.0,
                ),
            ),
        ),
        Transform::from_translation(
            Vec3::new(
                player2_x, player2_y, 0.0,
            ),
        )
        .with_scale(Vec2::splat(player_size).extend(1.)),
        Player(1),
        Collider,
        Velocity(Vec2::splat(0.0)),
        Gravity(player_gravity),
        Bounds {
            right: arena_half_width
                - player_size / 2.0
                - wall_thickness,
            left: player_size / 2.0 + wall_thickness,
            top: arena_half_height
                - player_size / 2.0
                - wall_thickness,
            bottom: -arena_half_height
                + player_size / 2.0
                + wall_thickness,
        },
    ));

    // Ball
    let ball_size = config
        .ball
        .size;
    let ball_gravity = config
        .gravity
        .ball;
    let ball_start_x = config
        .arena
        .ball_start_x;
    let ball_start_y = config
        .arena
        .ball_start_y;

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(
            materials.add(
                Color::srgb(
                    0.5, 1.0, 0.5,
                ),
            ),
        ),
        Transform::from_translation(
            Vec3::new(
                ball_start_x,
                ball_start_y,
                5.0,
            ),
        )
        .with_scale(Vec2::splat(ball_size).extend(1.)),
        Ball,
        Velocity(
            Vec2::new(
                0.0, 1.0,
            ) * 0.0,
        ),
        Gravity(ball_gravity),
        Bounds {
            right: arena_half_width
                - ball_size / 2.0
                - wall_thickness,
            left: -arena_half_width
                + ball_size / 2.0
                + wall_thickness,
            top: arena_half_height
                - ball_size / 2.0
                - wall_thickness,
            bottom: -arena_half_height
                + ball_size / 2.0
                + wall_thickness,
        },
    ));

    // Walls
    commands.spawn(
        Wall::new(
            WallLocation::Left,
            &*config,
        ),
    );
    commands.spawn(
        Wall::new(
            WallLocation::Right,
            &*config,
        ),
    );
    commands.spawn(
        Wall::new(
            WallLocation::Bottom,
            &*config,
        ),
    );
    commands.spawn(
        Wall::new(
            WallLocation::Top,
            &*config,
        ),
    );

    // Net
    let net_width = config
        .net
        .width;
    let net_height = config
        .net
        .height;

    commands.spawn((
        Net,
        Sprite::from_color(
            Color::srgb(
                1.0, 1.0, 1.0,
            ),
            Vec2::ONE,
        ),
        Transform {
            translation: Vec3::new(
                0.0,
                -arena_half_height + net_height / 2.0,
                2.0,
            ),
            scale: Vec3::new(
                net_width, net_height, 1.0,
            ),
            ..default()
        },
    ));

    // Sound
    let ball_collision_sound =
        asset_server.load("sounds/collision.ogg");
    commands.insert_resource(
        CollisionSound(ball_collision_sound),
    );

    // Scoreboard
    let font_size = config
        .ui
        .font_size;
    let margin = config
        .ui
        .scoreboard_margin;

    commands.spawn((
        Text::new(""),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(
            Color::srgb(
                0.5, 0.5, 1.0,
            ),
        ),
        ScoreboardUi,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(margin),
            left: Val::Px(margin),
            ..default()
        },
        children![
            (
                TextSpan::default(),
                TextFont {
                    font_size,
                    ..default()
                },
                TextColor(
                    Color::srgb(
                        1.0, 0.5, 0.5
                    )
                ),
            ),
            (
                TextSpan::default(),
                TextFont {
                    font_size,
                    ..default()
                },
                TextColor(
                    Color::srgb(
                        1.0, 0.5, 0.5
                    )
                ),
            )
        ],
    ));
}

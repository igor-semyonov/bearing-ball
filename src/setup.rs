use bevy::prelude::*;
use iyes_perf_ui::entries::PerfUiDefaultEntries;
use crate::player::{Player, Collider, Velocity, Gravity, Bounds, MainCamera};
use crate::ball::Ball;
use crate::wall::{Wall, WallLocation};
use crate::net::Net;
use crate::sound::CollisionSound;
use crate::score::ScoreboardUi;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // spawn a camera to be able to see anything
    commands.spawn((
        Camera2d, MainCamera,
    ));

    // create a simple Perf UI with default settings
    // and all entries provided by the crate:
    commands.spawn((PerfUiDefaultEntries::default(),));

    // players
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.0))),
        Transform::from_translation(
            Vec3::new(
                -800.0,
                -480.0 + 160.0 / 2.0 + 10.0 / 2.0,
                0.0,
            ),
        )
        .with_scale(
            Vec2::splat(160.0).extend(1.),
        ),
        Player(0),
        Collider,
        Velocity(Vec2::splat(0.0)),
        Gravity(3500.0),
        Bounds {
            right: -160.0 / 2.0 - 20.0 / 2.0,
            left: -750.0 + 160.0 / 2.0 + 10.0 / 2.0,
            top: 480.0 - 160.0 / 2.0 - 10.0 / 2.0,
            bottom: -480.0 + 160.0 / 2.0 + 10.0 / 2.0,
        },
    ));
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.0, 1.0))),
        Transform::from_translation(
            Vec3::new(
                800.0,
                -480.0 + 160.0 / 2.0 + 10.0 / 2.0,
                0.0,
            ),
        )
        .with_scale(
            Vec2::splat(160.0).extend(1.),
        ),
        Player(1),
        Collider,
        Velocity(Vec2::splat(0.0)),
        Gravity(3500.0),
        Bounds {
            right: 750.0 - 160.0 / 2.0 - 10.0 / 2.0,
            left: 160.0 / 2.0 + 20.0 / 2.0,
            top: 480.0 - 160.0 / 2.0 - 10.0 / 2.0,
            bottom: -480.0 + 160.0 / 2.0 + 10.0 / 2.0,
        },
    ));

    // Ball
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::srgb(0.5, 1.0, 0.5))),
        Transform::from_translation(Vec3::new(-800.0, 300.0, 5.0))
            .with_scale(
                Vec2::splat(70.0).extend(1.),
            ),
        Ball,
        Velocity(Vec2::new(0.0, 1.0) * 0.0),
        Gravity(1500.0),
        Bounds {
            right: 750.0 - 70.0 / 2.0 - 10.0 / 2.0,
            left: -750.0 + 70.0 / 2.0 + 10.0 / 2.0,
            top: 480.0 - 70.0 / 2.0 - 10.0 / 2.0,
            bottom: -480.0 + 70.0 / 2.0 + 10.0 / 2.0,
        },
    ));

    // Walls
    commands.spawn(Wall::new(WallLocation::Left));
    commands.spawn(Wall::new(WallLocation::Right));
    commands.spawn(Wall::new(WallLocation::Bottom));
    commands.spawn(Wall::new(WallLocation::Top));

    // Net
    commands.spawn((
        Net,
        Sprite::from_color(
            Color::srgb(1.0, 1.0, 1.0),
            Vec2::ONE,
        ),
        Transform {
            translation: Vec3::new(
                0.0,
                -480.0 + 200.0 / 2.0,
                2.0,
            ),
            scale: Vec3::new(
                20.0,
                200.0,
                1.0,
            ),
            ..default()
        },
    ));

    // Sound
    let ball_collision_sound = asset_server.load("sounds/collision.ogg");
    commands.insert_resource(CollisionSound(ball_collision_sound));

    // Scoreboard
    commands.spawn((
        Text::new(""),
        bevy::text::TextFont {
            font_size: 33.0,
            ..default()
        },
        bevy::text::TextColor(Color::srgb(0.5, 0.5, 1.0)),
        ScoreboardUi,
        bevy::ui::Node {
            position_type: bevy::ui::PositionType::Absolute,
            top: bevy::ui::Val::Px(5.0),
            left: bevy::ui::Val::Px(5.0),
            ..default()
        },
        children![
            (
                bevy::text::TextSpan::default(),
                bevy::text::TextFont {
                    font_size: 33.0,
                    ..default()
                },
                bevy::text::TextColor(Color::srgb(1.0, 0.5, 0.5)),
            ),
            (
                bevy::text::TextSpan::default(),
                bevy::text::TextFont {
                    font_size: 33.0,
                    ..default()
                },
                bevy::text::TextColor(Color::srgb(1.0, 0.5, 0.5)),
            )
        ],
    ));
} 

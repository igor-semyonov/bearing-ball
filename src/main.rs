#![allow(unused_imports)]
use bevy::{
    math::bounding::{
        Aabb2d, BoundingCircle, BoundingVolume,
        IntersectsVolume,
    },
    prelude::*,
};
use iyes_perf_ui::prelude::*;
// use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
// use bevy::ecs::prelude::Event;
// use bevy::input::mouse::MouseMotion;
// use bevy::input::mouse::MouseWheel;
// use bevy::render::{
//     render_asset::RenderAssetUsages,
//     render_resource::{
//         Extent3d, TextureDimension, TextureFormat,
//     },
// };
// use bevy::window::PrimaryWindow;
// use iyes_perf_ui::entry::PerfUiEntry;

const ACCELERATION_DUE_TO_GRAVITY: f32 = 500.0;

const BALL_INITIAL_POSITION: Vec3 = Vec3::new(
    0.0, 50.0, 1.0,
);
const PLAYER1_INITIAL_POSITION_X: f32 = 0.0;

const BALL_COLOR: Color = Color::srgb(
    0.5, 1.0, 0.5,
);
const PLAYER1_COLOR: Color = Color::srgb(
    0.8, 0.2, 0.8,
);
const BALL_DIAMETER: f32 = 30.;
const PLAYER_DIAMETER: f32 = 90.;

const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(
    0.0, 1.0,
);
const BALL_SPEED: f32 = 0.0;

const FLOOR_HEIGHT: f32 = -500.0;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Ball;

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Component)]
#[require(
    Transform, Collider
)]
struct Net;

#[derive(Component)]
struct Player;

#[derive(Component, Default)]
struct Collider;

#[derive(Component)]
struct MainCamera;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(
                    Window {
                        title: "Double Pendulum".into(),
                        name: Some("double_pendulum".into()),
                        mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                        // present_mode: bevy::window::PresentMode::AutoNoVsync,
                        ..Default::default()
                    },
                ),
                ..Default::default()
            },
        ),
    );
    app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());
    app.add_plugins(
        bevy::diagnostic::EntityCountDiagnosticsPlugin,
    );
    app.add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin);
    app.add_plugins(
        bevy::render::diagnostic::RenderDiagnosticsPlugin,
    );
    app.add_plugins(PerfUiPlugin);

    app.insert_resource(
        ClearColor(
            Color::srgb(
                0.0, 0.0, 0.0,
            ),
        ),
    );
    app.add_event::<CollisionEvent>();
    app.add_systems(
        Startup,
        (setup,),
    );
    app.add_systems(
        FixedUpdate,
        (
            apply_velocity,
            apply_gravity,
            check_for_collisions,
        ),
    );
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // spawn a camera to be able to see anything
    commands.spawn((
        Camera2d, MainCamera,
    ));

    // create a simple Perf UI with default settings
    // and all entries provided by the crate:
    commands.spawn((PerfUiDefaultEntries::default(),));
    // commands.spawn(PerfUiAllEntries::default());

    // players
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(PLAYER1_COLOR)),
        Transform::from_translation(
            Vec3::new(
                PLAYER1_INITIAL_POSITION_X,
                FLOOR_HEIGHT,
                0.0,
            ),
        )
        .with_scale(
            Vec2::splat(PLAYER_DIAMETER).extend(1.),
        ),
        Player,
        Collider,
        Velocity(Vec2::splat(0.0)),
    ));

    // Ball
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(BALL_COLOR)),
        Transform::from_translation(BALL_INITIAL_POSITION)
            .with_scale(
                Vec2::splat(BALL_DIAMETER).extend(1.),
            ),
        Ball,
        Velocity(
            BALL_INITIAL_DIRECTION.normalize() * BALL_SPEED,
        ),
    ));
}

fn apply_velocity(
    mut query: Query<(
        &mut Transform,
        &mut Velocity,
    )>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity) in &mut query {
        transform
            .translation
            .x += velocity.x * time.delta_secs();
        if transform
            .translation
            .y
            <= FLOOR_HEIGHT
        {
            transform
                .translation
                .y = FLOOR_HEIGHT;
            velocity.y = 0.0;
        }
        transform
            .translation
            .y += velocity.y * time.delta_secs();
    }
}

fn apply_gravity(
    mut query: Query<(&mut Velocity,)>,
    time: Res<Time>,
) {
    for mut velocity in &mut query {
        velocity
            .0
            .y -=
            ACCELERATION_DUE_TO_GRAVITY * time.delta_secs();
    }
}

fn check_for_collisions(
    mut commands: Commands,
    ball_query: Single<
        (
            &mut Velocity,
            &Transform,
        ),
        With<Ball>,
    >,
    collider_query: Query<
        (
            Entity,
            &Transform,
        ),
        With<Collider>,
    >,
    mut collider_event: EventWriter<CollisionEvent>,
) {
    let (mut ball_velocity, ball_transform) =
        ball_query.into_inner();

    for (collider_entity, collider_transform, ) in &collider_query{
    }
}

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

const ACCELERATION_DUE_TO_GRAVITY: f32 = 4500.0;
const PLAYER_SPEED: f32 = 900.0;
const PLAYER_JUMP_SPEED: f32 = 1800.0;
const CEILING_DAMPING_FACTOR: f32 = 0.9;

const PLAYER_INITIAL_X: f32 = 800.0;
const PLAYER0_INITIAL_POSITION_X: f32 = -PLAYER_INITIAL_X;
const PLAYER1_INITIAL_POSITION_X: f32 = PLAYER_INITIAL_X;
const BALL_Z: f32 = 5.0;
const BALL_INITIAL_POSITION: Vec3 = Vec3::new(
    PLAYER0_INITIAL_POSITION_X,
    300.0,
    BALL_Z,
);

const WALL_COLOR: Color = Color::srgb(
    0.8, 0.8, 0.8,
);
const NET_COLOR: Color = Color::srgb(
    1.0, 1.0, 1.0,
);
const BALL_COLOR: Color = Color::srgb(
    0.5, 1.0, 0.5,
);
const PLAYER0_COLOR: Color = Color::srgb(
    1.0, 0.5, 0.0,
);
const PLAYER1_COLOR: Color = Color::srgb(
    0.0, 0.0, 1.0,
);
const TEXT_COLOR: Color = Color::srgb(
    0.5, 0.5, 1.0,
);
const SCORE_COLOR: Color = Color::srgb(
    1.0, 0.5, 0.5,
);

const BALL_DIAMETER: f32 = 90.;
const PLAYER_DIAMETER: f32 = 270.;

const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(
    0.0, 1.0,
);
const BALL_INITIAL_SPEED: f32 = 0.0;

const BOUNDARY_BOTTOM: f32 = -500.0;
const BOUNDARY_TOP: f32 = 500.0;
const BOUNDARY_RIGHT: f32 = 1200.0;
const BOUNDARY_LEFT: f32 = -1200.0;
const WALL_THICKNESS: f32 = 10.0;

const NET_THICKNESS: f32 = 20.0;
const NET_HEIGHT: f32 = 200.0;

// key bindings
const PLAYER_MOVE_LEFT: [KeyCode; 2] =
    [KeyCode::KeyA, KeyCode::KeyJ];
const PLAYER_MOVE_RIGHT: [KeyCode; 2] =
    [KeyCode::KeyD, KeyCode::KeyL];
const PLAYER_JUMP: [KeyCode; 2] =
    [KeyCode::KeyW, KeyCode::KeyI];

const SCOREBOARD_FONT_SIZE: f32 = 33.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(
    States, Default, Debug, Clone, Eq, PartialEq, Hash,
)]
enum GameModeState {
    Paused,
    #[default]
    Running,
}

#[derive(Resource, Deref, DerefMut)]
struct Score([u32; 2]);

#[derive(Component)]
struct ScoreboardUi;

#[derive(Component)]
struct Ball;

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Resource, Deref)]
struct CollisionSound(Handle<AudioSource>);

#[derive(Component)]
#[require(
    Transform, Collider
)]
struct Net;

#[derive(Component, Deref, DerefMut, Clone, Copy)]
struct Player(usize);

#[derive(Component)]
struct Bounds {
    right: f32,
    left: f32,
    top: f32,
    bottom: f32,
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            right: -PLAYER_DIAMETER / 2.0
                - NET_THICKNESS / 2.0,
            left: BOUNDARY_LEFT
                + PLAYER_DIAMETER / 2.0
                + WALL_THICKNESS / 2.0,
            top: BOUNDARY_TOP
                - PLAYER_DIAMETER / 2.0
                - WALL_THICKNESS / 2.0,
            bottom: BOUNDARY_BOTTOM
                + PLAYER_DIAMETER / 2.0
                + WALL_THICKNESS / 2.0,
        }
    }
}

#[derive(Component, Default)]
struct Collider;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
#[require(
    Sprite, Transform, Collider
)]
struct Wall;

/// Which side of the arena is this wall located on?
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    /// Location of the *center* of the wall, used in
    /// `transform.translation()`
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(
                BOUNDARY_LEFT,
                0.,
            ),
            WallLocation::Right => Vec2::new(
                BOUNDARY_RIGHT,
                0.,
            ),
            WallLocation::Bottom => Vec2::new(
                0.,
                BOUNDARY_BOTTOM,
            ),
            WallLocation::Top => Vec2::new(
                0.,
                BOUNDARY_TOP,
            ),
        }
    }

    /// (x, y) dimensions of the wall, used in
    /// `transform.scale()`
    fn size(&self) -> Vec2 {
        let arena_height = BOUNDARY_TOP - BOUNDARY_BOTTOM;
        let arena_width = BOUNDARY_RIGHT - BOUNDARY_LEFT;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(
                    WALL_THICKNESS,
                    arena_height + WALL_THICKNESS,
                )
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(
                    arena_width + WALL_THICKNESS,
                    WALL_THICKNESS,
                )
            }
        }
    }
}

impl Wall {
    // This "builder method" allows us to reuse logic across
    // our wall entities, making our code easier to read
    // and less prone to bugs when we change the logic
    // Notice the use of Sprite and Transform alongside
    // Wall, overwriting the default values defined for the
    // required components
    fn new(
        location: WallLocation,
    ) -> (
        Wall,
        Sprite,
        Transform,
    ) {
        (
            Wall,
            Sprite::from_color(
                WALL_COLOR,
                Vec2::ONE,
            ),
            Transform {
                // We need to convert our Vec2 into a Vec3,
                // by giving it a z-coordinate
                // This is used to determine the order of
                // our sprites
                translation: location
                    .position()
                    .extend(0.0),
                // The z-scale of 2D objects must always be
                // 1.0, or their ordering
                // will be affected in surprising ways. See https://github.com/bevyengine/bevy/issues/4149
                scale: location
                    .size()
                    .extend(1.0),
                ..default()
            },
        )
    }
}

#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
struct GameplaySet;

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
    app.insert_resource(Time::<Fixed>::from_hz(256.0))
        .init_state::<GameModeState>()
        .insert_resource(Score([0; 2]));
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

    // systems
    app.add_systems(
        Startup,
        (setup,),
    );

    app.add_systems(
        FixedUpdate,
        (
            apply_velocity,
            apply_gravity,
            apply_ball_bounds,
            // process_just_scored,
            check_for_ball_collisions,
            // play_collision_sound,
            player_movement,
            apply_player_bounds,
        )
            .chain()
            .in_set(GameplaySet),
    );
    app.configure_sets(
        FixedUpdate,
        GameplaySet
            .run_if(in_state(GameModeState::Running)),
    );
    app.add_systems(
        Update,
        update_scoreboard
            .run_if(in_state(GameModeState::Running)),
    );
    app.run();
}

fn setup(
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
    // commands.spawn(PerfUiAllEntries::default());

    // players
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(PLAYER0_COLOR)),
        Transform::from_translation(
            Vec3::new(
                PLAYER0_INITIAL_POSITION_X,
                BOUNDARY_BOTTOM
                    + PLAYER_DIAMETER / 2.0
                    + WALL_THICKNESS / 2.0,
                0.0,
            ),
        )
        .with_scale(
            Vec2::splat(PLAYER_DIAMETER).extend(1.),
        ),
        Player(0),
        Collider,
        Velocity(Vec2::splat(0.0)),
        Bounds {
            right: -PLAYER_DIAMETER / 2.0
                - NET_THICKNESS / 2.0,
            left: BOUNDARY_LEFT
                + PLAYER_DIAMETER / 2.0
                + WALL_THICKNESS / 2.0,
            top: BOUNDARY_TOP
                - PLAYER_DIAMETER / 2.0
                - WALL_THICKNESS / 2.0,
            bottom: BOUNDARY_BOTTOM
                + PLAYER_DIAMETER / 2.0
                + WALL_THICKNESS / 2.0,
        },
    ));
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(PLAYER1_COLOR)),
        Transform::from_translation(
            Vec3::new(
                PLAYER1_INITIAL_POSITION_X,
                BOUNDARY_BOTTOM
                    + PLAYER_DIAMETER / 2.0
                    + WALL_THICKNESS / 2.0,
                0.0,
            ),
        )
        .with_scale(
            Vec2::splat(PLAYER_DIAMETER).extend(1.),
        ),
        Player(1),
        Collider,
        Velocity(Vec2::splat(0.0)),
        Bounds {
            right: BOUNDARY_RIGHT
                - PLAYER_DIAMETER / 2.0
                - WALL_THICKNESS / 2.0,
            left: PLAYER_DIAMETER / 2.0
                + NET_THICKNESS / 2.0,
            top: BOUNDARY_TOP
                - PLAYER_DIAMETER / 2.0
                - WALL_THICKNESS / 2.0,
            bottom: BOUNDARY_BOTTOM
                + PLAYER_DIAMETER / 2.0
                + WALL_THICKNESS / 2.0,
        },
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
            BALL_INITIAL_DIRECTION.normalize()
                * BALL_INITIAL_SPEED,
        ),
        Bounds {
            right: BOUNDARY_RIGHT
                - BALL_DIAMETER / 2.0
                - WALL_THICKNESS / 2.0,
            left: BOUNDARY_LEFT
                + BALL_DIAMETER / 2.0
                + WALL_THICKNESS / 2.0,
            top: BOUNDARY_TOP
                - BALL_DIAMETER / 2.0
                - WALL_THICKNESS / 2.0,
            bottom: BOUNDARY_BOTTOM
                + BALL_DIAMETER / 2.0
                + WALL_THICKNESS / 2.0,
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
            NET_COLOR,
            Vec2::ONE,
        ),
        Transform {
            translation: Vec3::new(
                0.0,
                BOUNDARY_BOTTOM + NET_HEIGHT / 2.0,
                2.0,
            ),
            scale: Vec3::new(
                NET_THICKNESS,
                NET_HEIGHT,
                1.0,
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
    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: SCOREBOARD_FONT_SIZE,
            ..default()
        },
        TextColor(TEXT_COLOR),
        ScoreboardUi,
        Node {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        },
        children![
            (
                TextSpan::default(),
                TextFont {
                    font_size: SCOREBOARD_FONT_SIZE,
                    ..default()
                },
                TextColor(SCORE_COLOR),
            ),
            (
                TextSpan::default(),
                TextFont {
                    font_size: SCOREBOARD_FONT_SIZE,
                    ..default()
                },
                TextColor(SCORE_COLOR),
            )
        ],
    ));
}

fn apply_velocity(
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

fn apply_gravity(
    mut query: Query<
        &mut Velocity,
        Or<(
            With<Ball>,
            With<Player>,
        )>,
    >,
    time: Res<Time>,
) {
    for mut velocity in &mut query {
        velocity
            .0
            .y -=
            ACCELERATION_DUE_TO_GRAVITY * time.delta_secs();
    }
}

fn check_for_ball_collisions(
    // mut commands: Commands,
    ball_query: Single<
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
    net_query: Single<
        (&Transform,),
        (
            With<Net>,
            Without<Ball>,
        ),
    >,
    mut collision_event: EventWriter<CollisionEvent>,
) {
    let (mut ball_transform, mut ball_velocity) =
        ball_query.into_inner();

    for (player_transform, player_velocity) in &player_query
    {
        let ball_bounding_circle = BoundingCircle::new(
            ball_transform
                .translation
                .truncate(),
            BALL_DIAMETER / 2.0,
        );
        let player_bounding_circle = BoundingCircle::new(
            player_transform
                .translation
                .truncate(),
            PLAYER_DIAMETER / 2.0,
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
                * ((PLAYER_DIAMETER + BALL_DIAMETER)
                    / 2.0
                    + 0.01))
                .extend(BALL_Z);
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
            let theta =
                ball_velocity.angle_to(collision_normal);
            let new_ball_velocity = -ball_velocity
                .rotate(Vec2::from_angle(2.0 * theta));
            ball_velocity.x = new_ball_velocity.x;
            ball_velocity.y = new_ball_velocity.y;
        }
        let player_velocity_projected_onto_collision_normal =
            player_velocity
                .xy()
                .dot(collision_normal)
                * collision_normal;
        ball_velocity.x +=
            player_velocity_projected_onto_collision_normal
                .x;
        ball_velocity.y +=
            player_velocity_projected_onto_collision_normal
                .y;
        collision_event.write_default();
    }
    let (net_transform,) = &net_query.into_inner();
    let ball_bounding_circle = BoundingCircle::new(
        ball_transform
            .translation
            .truncate(),
        BALL_DIAMETER / 2.0,
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

fn play_collision_sound(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    sound: Res<CollisionSound>,
) {
    if !collision_events.is_empty() {
        collision_events.clear();
        commands.spawn((
            AudioPlayer(sound.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }
}

fn apply_ball_bounds(
    query: Single<
        (
            &mut Transform,
            &mut Velocity,
            &Bounds,
        ),
        With<Ball>,
    >,
    mut score: ResMut<Score>,
) {
    let (mut transform, mut velocity, bound) =
        query.into_inner();
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
        velocity.y *= -CEILING_DAMPING_FACTOR;
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

fn process_just_scored(
    // game_state: Res<GameModeState>,
    query: Query<&mut Velocity>,
) {
    // if *game_state == GameModeState::JustScored {
    //     for mut velocity in query {
    //         velocity.x = 0.0;
    //         velocity.y = 0.0;
    //     }
    // }
}

fn apply_player_bounds(
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

fn player_movement(
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
    for (&player_id, transform, mut velocity, bound) in
        query
    {
        let mut direction = 0.0;
        if keyboard_input
            .pressed(PLAYER_MOVE_LEFT[*player_id])
        {
            direction -= PLAYER_SPEED;
        }
        if keyboard_input
            .pressed(PLAYER_MOVE_RIGHT[*player_id])
        {
            direction += PLAYER_SPEED;
        }
        velocity.x = direction;

        if keyboard_input.pressed(PLAYER_JUMP[*player_id]) {
            if transform
                .translation
                .y
                <= bound.bottom
            {
                velocity.y = PLAYER_JUMP_SPEED;
            }
        }
    }
}

fn update_scoreboard(
    score: Res<Score>,
    score_root: Single<
        Entity,
        (
            With<ScoreboardUi>,
            With<Text>,
        ),
    >,
    mut writer: TextUiWriter,
) {
    *writer.text(
        *score_root,
        1,
    ) = score[0].to_string();
    *writer.text(
        *score_root,
        2,
    ) = format!(
        " - {}",
        score[1].to_string()
    );
}

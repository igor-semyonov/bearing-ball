#![allow(unused_imports)]
use bevy::prelude::*;
use iyes_perf_ui::prelude::*;
mod player;
mod ball;
mod wall;
mod net;
mod score;
mod game_state;
mod sound;
mod setup;

use player::*;
use ball::*;
use wall::*;
use net::*;
use score::*;
use game_state::*;
use sound::*;
use setup::setup;

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
                        ..Default::default()
                    },
                ),
                ..Default::default()
            },
        ),
    );
    app.insert_resource(Time::<Fixed>::from_hz(256.0))
        .insert_resource(Score([0; 2]))
        .init_state::<GameModeState>();
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
        (setup, sound::change_global_volume),
    );

    app.add_systems(
        FixedUpdate,
        (
            ball::apply_velocity,
            ball::apply_gravity,
            ball::apply_ball_bounds,
            // ball::process_just_scored,
            ball::check_for_ball_collisions,
            sound::play_collision_sound,
            player::player_movement,
            player::apply_player_bounds,
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
        score::update_scoreboard
            .run_if(in_state(GameModeState::Running)),
    );
    app.run();
}

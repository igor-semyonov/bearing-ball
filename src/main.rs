#![allow(unused_imports)]
use bevy::prelude::*;
use bevy::window::{
    Window, WindowPlugin, WindowResolution,
};
use iyes_perf_ui::prelude::*;
mod ball;
mod config;
mod game_state;
mod net;
mod player;
mod score;
mod setup;
mod sound;
mod wall;

use ball::*;
use config::{Config, get_window_mode, load_config};
use game_state::*;
use net::*;
use player::*;
use score::*;
use setup::setup;
use sound::*;
use wall::*;

fn main() {
    let config = load_config();
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(
                    Window {
                        title: config
                            .window
                            .title
                            .clone()
                            .into(),
                        name: Some("bearing_ball".into()),
                        mode: get_window_mode(
                            &config
                                .window
                                .mode,
                        ),
                        resolution: WindowResolution::new(
                            config
                                .window
                                .width
                                as f32,
                            config
                                .window
                                .height
                                as f32,
                        ),
                        ..Default::default()
                    },
                ),
                ..Default::default()
            },
        ),
    );
    app.insert_resource(Time::<Fixed>::from_hz(256.0))
        .insert_resource(Score([0; 2]))
        .insert_resource(config.clone())
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
        Startup, setup,
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

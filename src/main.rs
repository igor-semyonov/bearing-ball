use bevy::window::{
    Window, WindowPlugin, WindowResolution,
};
use iyes_perf_ui::prelude::*;

mod components;
mod config;
mod events;
mod game_state;
mod prelude;
mod resources;
mod systems;

use prelude::*;

use config::load_config;
use game_state::*;

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
                        mode: config.get_window_mode(),
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
    app.add_event::<events::CollisionEvent>();

    // systems
    app.add_systems(
        Startup,
        systems::setup,
    );

    app.add_systems(
        FixedUpdate,
        (
            systems::fixed_update::apply_velocity,
            systems::fixed_update::apply_gravity,
            systems::fixed_update::apply_ball_bounds,
            // systems::fixed_update::process_just_scored,
            systems::fixed_update::check_for_ball_collisions,
            systems::fixed_update::play_collision_sound,
            systems::fixed_update::player_movement,
            systems::fixed_update::apply_player_bounds,
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
        systems::update::update_scoreboard
            .run_if(in_state(GameModeState::Running)),
    );
    app.run();
}

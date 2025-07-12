use bevy::prelude::*;
use bevy::window::{
    MonitorSelection, VideoModeSelection, WindowMode,
};
use sensible::Default;
use serde::Deserialize;
use std::fs;

#[derive(Default, Debug, Deserialize, Clone, Resource)]
pub struct Config {
    pub player: PlayerConfig,
    pub arena: ArenaConfig,
    pub gravity: GravityConfig,
    pub ball: BallConfig,
    pub net: NetConfig,
    pub controls: ControlsConfig,
    pub window: WindowConfig,
    pub ui: UiConfig,
    pub audio: AudioConfig,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct PlayerConfig {
    #[default(900.0)]
    pub speed: f32,
    #[default(1200.0)]
    pub jump_speed: f32,
    #[default(160.0)]
    pub size: f32,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct ArenaConfig {
    #[default(1500.0)]
    pub width: f32,
    #[default(960.0)]
    pub height: f32,
    #[default(600.0)]
    pub player_spacing: f32,
    #[default(10.0)]
    pub wall_thickness: f32,
    #[default(-600.0)]
    pub ball_start_x: f32,
    #[default(300.0)]
    pub ball_start_y: f32,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct GravityConfig {
    #[default(3500.0)]
    pub player: f32,
    #[default(1500.0)]
    pub ball: f32,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct BallConfig {
    #[default(70.0)]
    pub size: f32,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct NetConfig {
    #[default(20.0)]
    pub width: f32,
    #[default(200.0)]
    pub height: f32,
}

/// Controls config struct. The values are passed in as
/// keycodes. The defaults are set to Colemak
#[derive(Default, Debug, Deserialize, Clone)]
pub struct ControlsConfig {
    #[default("KeyA".to_string())]
    pub player1_move_left: String,
    #[default("KeyS".to_string())]
    pub player1_move_right: String,
    #[default("KeyW".to_string())]
    pub player1_jump: String,
    #[default("KeyN".to_string())]
    pub player2_move_left: String,
    #[default("KeyI".to_string())]
    pub player2_move_right: String,
    #[default("KeyU".to_string())]
    pub player2_jump: String,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct WindowConfig {
    #[default("Bearing Ball".to_string())]
    pub title: String,
    #[default("BorderlessFullscreen".to_string())]
    pub mode: String,
    #[default(1920)]
    pub width: u32,
    #[default(1080)]
    pub height: u32,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct UiConfig {
    #[default(32.0)]
    pub font_size: f32,
    #[default(5.0)]
    pub scoreboard_margin: f32,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct AudioConfig {
    #[default(0.2)]
    pub volume: f32,
}

// impl Default for Config {
//     fn default() -> Self {
//         Self {
//             player: PlayerConfig {
//                 speed: 900.0,
//                 jump_speed: 1200.0,
//                 size: 160.0,
//             },
//             arena: ArenaConfig {
//                 width: 1500.0,
//                 height: 960.0,
//                 player_spacing: 800.0,
//                 wall_thickness: 10.0,
//                 ball_start_x: -800.0,
//                 ball_start_y: 300.0,
//             },
//             gravity: GravityConfig {
//                 player: 3500.0,
//                 ball: 1500.0,
//             },
//             ball: BallConfig {
//                 size: 70.0,
//             },
//             wall: WallConfig {
//                 width: 10.0,
//             },
//             net: NetConfig {
//                 width: 20.0,
//                 height: 200.0,
//             },
//             controls: ControlsConfig {
//                 player1_move_left: "KeyA".to_string(),
//                 player1_move_right: "KeyD".to_string(),
//                 player1_jump: "KeyW".to_string(),
//                 player2_move_left: "KeyJ".to_string(),
//                 player2_move_right: "KeyL".to_string(),
//                 player2_jump: "KeyI".to_string(),
//             },
//             window: WindowConfig {
//                 title: "Bearing Ball".to_string(),
//                 mode: "BorderlessFullscreen".to_string(),
//                 width: 1920,
//                 height: 1080,
//             },
//             ui: UiConfig {
//                 font_size: 33.0,
//                 scoreboard_margin: 5.0,
//             },
//             audio: AudioConfig {
//                 volume: 0.2,
//             },
//         }
//     }
// }

pub fn load_config() -> Config {
    match fs::read_to_string("balls.toml") {
        Ok(content) => toml::from_str(&content).unwrap_or_else(|e| {
            eprintln!("Failed to parse balls.toml: {e}, using defaults");
            Config::default()
        }),
        Err(_) => {
            eprintln!("balls.toml not found, using defaults");
            Config::default()
        }
    }
}

pub fn string_to_keycode(key_str: &str) -> KeyCode {
    match key_str {
        "KeyA" => KeyCode::KeyA,
        "KeyB" => KeyCode::KeyB,
        "KeyC" => KeyCode::KeyC,
        "KeyD" => KeyCode::KeyD,
        "KeyE" => KeyCode::KeyE,
        "KeyF" => KeyCode::KeyF,
        "KeyG" => KeyCode::KeyG,
        "KeyH" => KeyCode::KeyH,
        "KeyI" => KeyCode::KeyI,
        "KeyJ" => KeyCode::KeyJ,
        "KeyK" => KeyCode::KeyK,
        "KeyL" => KeyCode::KeyL,
        "KeyM" => KeyCode::KeyM,
        "KeyN" => KeyCode::KeyN,
        "KeyO" => KeyCode::KeyO,
        "KeyP" => KeyCode::KeyP,
        "KeyQ" => KeyCode::KeyQ,
        "KeyR" => KeyCode::KeyR,
        "KeyS" => KeyCode::KeyS,
        "KeyT" => KeyCode::KeyT,
        "KeyU" => KeyCode::KeyU,
        "KeyV" => KeyCode::KeyV,
        "KeyW" => KeyCode::KeyW,
        "KeyX" => KeyCode::KeyX,
        "KeyY" => KeyCode::KeyY,
        "KeyZ" => KeyCode::KeyZ,
        "Space" => KeyCode::Space,
        "ShiftLeft" => KeyCode::ShiftLeft,
        "ShiftRight" => KeyCode::ShiftRight,
        "ControlLeft" => KeyCode::ControlLeft,
        "ControlRight" => KeyCode::ControlRight,
        "AltLeft" => KeyCode::AltLeft,
        "AltRight" => KeyCode::AltRight,
        "ArrowLeft" => KeyCode::ArrowLeft,
        "ArrowRight" => KeyCode::ArrowRight,
        "ArrowUp" => KeyCode::ArrowUp,
        "ArrowDown" => KeyCode::ArrowDown,
        _ => {
            eprintln!(
                "Unknown key code: {}, defaulting to KeyA",
                key_str
            );
            KeyCode::KeyA
        }
    }
}

pub fn get_window_mode(mode_str: &str) -> WindowMode {
    match mode_str {
        "BorderlessFullscreen" => {
            WindowMode::BorderlessFullscreen(
                MonitorSelection::Current,
            )
        }
        "Fullscreen" => WindowMode::Fullscreen(
            MonitorSelection::Current,
            VideoModeSelection::Current,
        ),
        "Windowed" => WindowMode::Windowed,
        _ => {
            eprintln!(
                "Unknown window mode: {}, defaulting to BorderlessFullscreen",
                mode_str
            );
            WindowMode::BorderlessFullscreen(
                MonitorSelection::Current,
            )
        }
    }
}

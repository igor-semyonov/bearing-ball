use std::fs;
use serde::Deserialize;
use bevy::prelude::*;
use bevy::window::{WindowMode, MonitorSelection, VideoModeSelection};

#[derive(Debug, Deserialize, Clone, Resource)]
pub struct Config {
    pub player: PlayerConfig,
    pub arena: ArenaConfig,
    pub gravity: GravityConfig,
    pub ball: BallConfig,
    pub wall: WallConfig,
    pub net: NetConfig,
    pub controls: ControlsConfig,
    pub window: WindowConfig,
    pub ui: UiConfig,
    pub audio: AudioConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PlayerConfig {
    pub speed: f32,
    pub jump_speed: f32,
    pub size: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ArenaConfig {
    pub width: f32,
    pub height: f32,
    pub player_spacing: f32,
    pub wall_thickness: f32,
    pub ball_start_x: f32,
    pub ball_start_y: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GravityConfig {
    pub player: f32,
    pub ball: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BallConfig {
    pub size: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WallConfig {
    pub width: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NetConfig {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ControlsConfig {
    pub player1_move_left: String,
    pub player1_move_right: String,
    pub player1_jump: String,
    pub player2_move_left: String,
    pub player2_move_right: String,
    pub player2_jump: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub mode: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UiConfig {
    pub font_size: f32,
    pub scoreboard_margin: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AudioConfig {
    pub volume: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            player: PlayerConfig { speed: 900.0, jump_speed: 1200.0, size: 160.0 },
            arena: ArenaConfig { 
                width: 1500.0, 
                height: 960.0,
                player_spacing: 800.0,
                wall_thickness: 10.0,
                ball_start_x: -800.0,
                ball_start_y: 300.0,
            },
            gravity: GravityConfig { player: 3500.0, ball: 1500.0 },
            ball: BallConfig { size: 70.0 },
            wall: WallConfig { width: 10.0 },
            net: NetConfig { width: 20.0, height: 200.0 },
            controls: ControlsConfig {
                player1_move_left: "KeyA".to_string(),
                player1_move_right: "KeyD".to_string(),
                player1_jump: "KeyW".to_string(),
                player2_move_left: "KeyJ".to_string(),
                player2_move_right: "KeyL".to_string(),
                player2_jump: "KeyI".to_string(),
            },
            window: WindowConfig {
                title: "Bearing Ball".to_string(),
                mode: "BorderlessFullscreen".to_string(),
                width: 1920,
                height: 1080,
            },
            ui: UiConfig {
                font_size: 33.0,
                scoreboard_margin: 5.0,
            },
            audio: AudioConfig {
                volume: 0.2,
            },
        }
    }
}

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
            eprintln!("Unknown key code: {}, defaulting to KeyA", key_str);
            KeyCode::KeyA
        }
    }
}

pub fn get_window_mode(mode_str: &str) -> WindowMode {
    match mode_str {
        "BorderlessFullscreen" => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
        "Fullscreen" => WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current),
        "Windowed" => WindowMode::Windowed,
        _ => {
            eprintln!("Unknown window mode: {}, defaulting to BorderlessFullscreen", mode_str);
            WindowMode::BorderlessFullscreen(MonitorSelection::Current)
        }
    }
} 
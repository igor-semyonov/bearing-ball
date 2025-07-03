use bevy::prelude::*;
use crate::Config;
use bevy::prelude::Resource;

#[derive(Component)]
pub struct Wall;

pub enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    pub fn position(&self, config: &Config) -> Vec2 {
        let width = config.arena.width / 2.0;
        let height = config.arena.height / 2.0;
        match self {
            WallLocation::Left => Vec2::new(-width, 0.),
            WallLocation::Right => Vec2::new(width, 0.),
            WallLocation::Bottom => Vec2::new(0., -height),
            WallLocation::Top => Vec2::new(0., height),
        }
    }
    pub fn size(&self, config: &Config) -> Vec2 {
        let width = config.arena.width;
        let height = config.arena.height;
        let wall_width = config.wall.width;
        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(wall_width, height + wall_width)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(width + wall_width, wall_width)
            }
        }
    }
}

impl Wall {
    pub fn new(location: WallLocation, config: &Config) -> (Wall, Sprite, Transform) {
        (
            Wall,
            Sprite::from_color(Color::srgb(0.8, 0.8, 0.8), Vec2::ONE),
            Transform {
                translation: location.position(config).extend(0.0),
                scale: location.size(config).extend(1.0),
                ..default()
            },
        )
    }
} 
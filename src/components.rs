use crate::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Player(pub usize);

#[derive(Component)]
pub struct Collider;

#[derive(Component, Deref, DerefMut)]
pub struct Gravity(pub f32);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Bounds {
    pub right: f32,
    pub left: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(Component)]
pub struct Net;

#[derive(Component)]
pub struct ScoreboardUi;

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
        let width = config
            .arena
            .width
            / 2.0;
        let height = config
            .arena
            .height
            / 2.0;
        match self {
            WallLocation::Left => Vec2::new(
                -width, 0.,
            ),
            WallLocation::Right => Vec2::new(
                width, 0.,
            ),
            WallLocation::Bottom => Vec2::new(
                0., -height,
            ),
            WallLocation::Top => Vec2::new(
                0., height,
            ),
        }
    }
    pub fn size(&self, config: &Config) -> Vec2 {
        let arena_width = config
            .arena
            .width;
        let arena_height = config
            .arena
            .height;
        let wall_thickness = config
            .arena
            .wall_thickness;
        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(
                    wall_thickness,
                    arena_height + wall_thickness,
                )
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(
                    arena_width + wall_thickness,
                    wall_thickness,
                )
            }
        }
    }
}

impl Wall {
    pub fn new(
        location: WallLocation,
        config: &Config,
    ) -> (
        Wall,
        Sprite,
        Transform,
    ) {
        (
            Wall,
            Sprite::from_color(
                Color::srgb(
                    0.8, 0.8, 0.8,
                ),
                Vec2::ONE,
            ),
            Transform {
                translation: location
                    .position(config)
                    .extend(0.0),
                scale: location
                    .size(config)
                    .extend(1.0),
                ..default()
            },
        )
    }
}

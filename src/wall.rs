use bevy::prelude::*;

#[derive(Component)]
pub struct Wall;

pub enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    pub fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(-750.0, 0.),
            WallLocation::Right => Vec2::new(750.0, 0.),
            WallLocation::Bottom => Vec2::new(0., -480.0),
            WallLocation::Top => Vec2::new(0., 480.0),
        }
    }
    pub fn size(&self) -> Vec2 {
        let arena_height = 480.0 - (-480.0);
        let arena_width = 750.0 - (-750.0);
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);
        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(10.0, arena_height + 10.0)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + 10.0, 10.0)
            }
        }
    }
}

impl Wall {
    pub fn new(location: WallLocation) -> (Wall, Sprite, Transform) {
        (
            Wall,
            Sprite::from_color(Color::srgb(0.8, 0.8, 0.8), Vec2::ONE),
            Transform {
                translation: location.position().extend(0.0),
                scale: location.size().extend(1.0),
                ..default()
            },
        )
    }
} 
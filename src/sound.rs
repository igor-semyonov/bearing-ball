use crate::config::Config;
use bevy::audio::Volume;
use bevy::prelude::*;

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Resource, Deref)]
pub struct CollisionSound(pub Handle<AudioSource>);

pub fn play_collision_sound(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    sound: Res<CollisionSound>,
    config: Res<Config>,
) {
    if !collision_events.is_empty() {
        collision_events.clear();
        commands.spawn((
            AudioPlayer(sound.clone()),
            PlaybackSettings {
                volume: Volume::Linear(
                    config
                        .audio
                        .volume,
                ),
                ..PlaybackSettings::DESPAWN
            },
        ));
    }
}

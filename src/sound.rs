use bevy::prelude::*;
use bevy::audio::Volume;

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Resource, Deref)]
pub struct CollisionSound(pub Handle<AudioSource>);

pub fn change_global_volume(mut volume: ResMut<GlobalVolume>) {
    volume.volume = Volume::Linear(0.2); // Set global volume to 50%
}

pub fn play_collision_sound(
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

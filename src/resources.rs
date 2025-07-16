use bevy::prelude::*;

#[derive(Resource, Deref)]
pub struct CollisionSound(pub Handle<AudioSource>);

#[derive(Resource, Deref, DerefMut)]
pub struct Score(pub [u32; 2]);


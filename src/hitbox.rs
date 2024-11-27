use bevy::prelude::*;

pub struct HitboxPlugin;

impl Plugin for HitboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnHitboxEvent>();
    }
}

#[derive(Event)]
pub struct SpawnHitboxEvent(Hitbox);

pub struct Hitbox {}

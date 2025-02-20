use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct HitboxPlugin;

impl Plugin for HitboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnHitboxEvent>().add_systems(
            Update,
            (
                spawn_hitbox.run_if(on_event::<SpawnHitboxEvent>()),
                update_hitbox_timer,
                despawn_hitbox,
            ),
        );
    }
}

#[derive(Event)]
pub struct SpawnHitboxEvent(pub Hitbox);

#[derive(Clone, Copy)]
pub enum Target {
    Single(Entity),
    Enemies,
    All,
}

#[derive(Component, Clone)]
pub struct Hitbox {
    pub sender: Entity,
    pub collider: Collider,
    pub position: Vec3,
    pub target: Target,
    pub lifetime: Timer,
}

fn spawn_hitbox(mut commands: Commands, mut spawn_events: EventReader<SpawnHitboxEvent>) {
    for spawn_event in spawn_events.read() {
        commands
            .spawn(TransformBundle::from_transform(
                Transform::from_translation(spawn_event.0.position),
            ))
            .insert((spawn_event.0.collider.clone(), Sensor))
            .insert(spawn_event.0.clone());
    }
}

fn update_hitbox_timer(time: Res<Time>, mut hitboxes: Query<&mut Hitbox>) {
    let dt = time.delta();
    for mut hitbox in &mut hitboxes {
        hitbox.lifetime.tick(dt);
    }
}

fn despawn_hitbox(mut commands: Commands, hitboxes: Query<(Entity, &Hitbox)>) {
    for (entity, hitbox) in &hitboxes {
        if hitbox.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

use bevy::{color::palettes, math::VectorSpace, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::{
    build::BuildPlugin,
    hitbox::{Hitbox, SpawnHitboxEvent, Target},
    player_input::{InputMap, InputParam, PlayerAction},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BuildPlugin)
            .add_event::<InteractionEvent>()
            .insert_state(PlayerState::Normal)
            .init_resource::<InteractableEntities>()
            .init_resource::<InputMap>()
            .insert_resource(CameraZoom(0.0))
            .add_systems(Startup, setup)
            .add_systems(
                FixedUpdate,
                (
                    update_camera,
                    update_interactables.run_if(on_event::<CollisionEvent>()),
                    highlight_interactables,
                ),
            )
            .add_systems(
                Update,
                (
                    move_player.run_if(in_state(PlayerState::Normal)),
                    interact,
                    start_building.run_if(in_state(PlayerState::Normal)),
                    cancel_building_mode.run_if(in_state(PlayerState::BuildingMode)),
                    attack.run_if(in_state(PlayerState::Normal)),
                ),
            );
    }
}

#[derive(Event)]
pub struct InteractionEvent {
    pub entity_1: Entity,
    pub entity_2: Entity,
}

#[derive(States, PartialEq, Eq, Hash, Debug, Clone)]
pub enum PlayerState {
    Normal,
    BuildingMode,
}

#[derive(Resource, Default)]
struct InteractableEntities {
    interactables: Vec<Entity>,
    removed: Vec<Entity>,
}

#[derive(Resource)]
struct PlayerData {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

#[derive(Component)]
pub struct Interactable;

#[derive(Component)]
pub struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_data = PlayerData {
        mesh: meshes.add(Cuboid::from_size(Vec3 {
            x: 0.5,
            y: 1.0,
            z: 0.5,
        })),

        material: materials.add(StandardMaterial::from_color(palettes::basic::AQUA)),
    };

    commands
        .spawn(PbrBundle {
            mesh: player_data.mesh.clone(),
            material: player_data.material.clone(),
            transform: Transform::from_translation(Vec3::Y),
            ..Default::default()
        })
        .insert((
            RigidBody::Dynamic,
            Collider::capsule_y(0.5, 0.3),
            Velocity::default(),
            LockedAxes::ROTATION_LOCKED,
        ))
        .insert(Player)
        .with_children(|builder| {
            builder
                .spawn(Collider::ball(0.2))
                .insert(Sensor)
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(TransformBundle::from_transform(Transform::from_xyz(
                    2.0, 0.0, 0.0,
                )));
        });

    commands.insert_resource(player_data);
}

fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Velocity, With<Player>>,
    mut player_sensor: Query<&mut Transform, With<Parent>>,
) {
    let mut player_velocity = player.single_mut();
    let dt = time.delta_seconds();

    let mut velocity = Vec3::ZERO;
    if input.pressed(KeyCode::KeyW) {
        velocity -= Vec3::Z;
    }
    if input.pressed(KeyCode::KeyS) {
        velocity += Vec3::Z;
    }
    if input.pressed(KeyCode::KeyA) {
        velocity -= Vec3::X;
    }
    if input.pressed(KeyCode::KeyD) {
        velocity += Vec3::X;
    }

    if velocity != Vec3::ZERO {
        velocity = velocity.normalize();
        let mut player_sensor = player_sensor.single_mut();
        player_sensor.translation = velocity * 0.5;
    }

    let player_speed = 150.0;
    player_velocity.linvel = velocity * dt * player_speed;
}

#[derive(Resource)]
pub struct CameraZoom(pub f32);

fn update_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    camera_zoom: Res<CameraZoom>,
) {
    let player_transform = player.single();
    let mut camera_transform = camera.single_mut();

    let camera_offset = Vec3 {
        x: 0.0,
        y: 10.0,
        z: 2.0,
    };

    camera_transform.translation = player_transform.translation + camera_offset;
    let z = camera_transform.forward();
    camera_transform.translation += z * camera_zoom.0;
}

fn update_interactables(
    player: Query<&Children, With<Player>>,
    sensor: Query<Entity, With<Parent>>,
    mut interactables: Query<Entity, With<Interactable>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut interactable_entities: ResMut<InteractableEntities>,
) {
    let player_children = player.single();
    let sensor_entity = player_children[0];
    assert_eq!(sensor_entity, sensor.single());

    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_1, entity_2, _) => {
                if *entity_1 == sensor_entity || *entity_2 == sensor_entity {
                    for interactable_entity in &mut interactables {
                        if *entity_1 == interactable_entity || *entity_2 == interactable_entity {
                            interactable_entities.interactables.push(
                                if *entity_1 == interactable_entity {
                                    *entity_1
                                } else {
                                    *entity_2
                                },
                            );
                        }
                    }
                }
            }

            CollisionEvent::Stopped(entity_1, entity_2, _) => {
                if *entity_1 == sensor_entity || *entity_2 == sensor_entity {
                    if let Some(index) = interactable_entities
                        .interactables
                        .iter()
                        .position(|entity| entity == entity_1 || entity == entity_2)
                    {
                        let removed = interactable_entities.interactables.swap_remove(index);
                        interactable_entities.removed.push(removed);
                    }
                }
            }
        }
    }
}

fn highlight_interactables(
    mut interactable_entities: ResMut<InteractableEntities>,
    interactables: Query<&Handle<StandardMaterial>, With<Interactable>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for entity in &interactable_entities.interactables {
        if let Ok(handle) = interactables.get(*entity) {
            let material = materials.get_mut(handle).unwrap();
            material.emissive = LinearRgba::rgb(0.0, 0.3, 0.0);
        }
    }

    for removed_entity in &interactable_entities.removed {
        if let Ok(handle) = interactables.get(*removed_entity) {
            let material = materials.get_mut(handle).unwrap();
            material.emissive = LinearRgba::ZERO;
        }
    }

    interactable_entities.removed.clear();
}

fn interact(
    player: Query<Entity, With<Player>>,
    input: InputParam,
    interactable_entities: Res<InteractableEntities>,
    mut interaction_event: EventWriter<InteractionEvent>,
) {
    if !input.action_just_pressed(PlayerAction::Interact) {
        return;
    }

    let player_entity = player.single();

    if let Some(entity) = interactable_entities.interactables.first() {
        interaction_event.send(InteractionEvent {
            entity_1: player_entity,
            entity_2: *entity,
        });
    }
}

fn start_building(input: InputParam, mut next_player_state: ResMut<NextState<PlayerState>>) {
    if !input.action_just_pressed(PlayerAction::Build) {
        return;
    }

    next_player_state.set(PlayerState::BuildingMode);
}

fn cancel_building_mode(input: InputParam, mut next_player_state: ResMut<NextState<PlayerState>>) {
    if !input.action_just_pressed(PlayerAction::Cancel) {
        return;
    }

    next_player_state.set(PlayerState::Normal);
}

#[derive(Event)]
pub struct AttackEvent(pub Entity);

fn attack(
    input: InputParam,
    player: Query<Entity, With<Player>>,
    sensor: Query<&GlobalTransform, With<Parent>>,
    mut attack_event: EventWriter<SpawnHitboxEvent>,
) {
    if !input.action_just_pressed(PlayerAction::Attack) {
        return;
    }

    let player = player.single();
    let position = sensor.single().translation();

    attack_event.send(SpawnHitboxEvent(Hitbox {
        sender: player,
        collider: Collider::cuboid(0.2, 0.5, 0.2),
        position,
        target: Target::Enemies,
        lifetime: Timer::from_seconds(0.5, TimerMode::Once),
    }));
}

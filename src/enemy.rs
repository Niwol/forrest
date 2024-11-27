use core::f32;
use std::{f32::consts::PI, time::Duration};

use bevy::{color::palettes, prelude::*};
use bevy_rapier3d::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (update_move_timer, move_enemy));
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
struct EnemyAI {
    move_timer: Timer,
    target_position: Option<Vec3>,
}

impl EnemyAI {
    fn new() -> Self {
        Self {
            move_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
            target_position: None,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_start_enemies(&mut commands, &mut meshes, &mut materials);
}

fn spawn_start_enemies(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let center_offset = Vec3 {
        x: 10.0,
        y: 0.0,
        z: 0.0,
    };

    let num_enemies = 10;

    for i in 0..num_enemies {
        let step = (i as f32 / num_enemies as f32) * PI * 2.0;
        let dist = rand::random::<f32>() * 5.0 + 2.0;
        let x = f32::cos(step) * dist;
        let y = 0.2;
        let z = f32::sin(step) * dist;
        let location = Vec3 { x, y, z } + center_offset;

        spawn_enemy(commands, location, meshes, materials);
    }
}

fn spawn_enemy(
    commands: &mut Commands,
    location: Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::from_size(Vec3::ONE * 0.4)),
            material: materials.add(StandardMaterial::from_color(palettes::basic::RED)),
            transform: Transform::from_translation(location),
            ..Default::default()
        })
        .insert((
            RigidBody::Dynamic,
            Collider::ball(0.2),
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
        ))
        .insert((Enemy, EnemyAI::new()));
}

fn update_move_timer(time: Res<Time>, mut enemies: Query<(&Transform, &mut EnemyAI)>) {
    let dt = time.delta();
    for (enemy_transform, mut enemy_ai) in &mut enemies {
        enemy_ai.move_timer.tick(dt);

        if enemy_ai.move_timer.just_finished() {
            let x = (rand::random::<f32>() - 0.5) * 5.0;
            let y = 0.0;
            let z = (rand::random::<f32>() - 0.5) * 5.0;

            let mut position = enemy_transform.translation + Vec3 { x, y, z };
            position.y = 0.2;
            enemy_ai.target_position = Some(position);

            let new_update_time = rand::random::<f32>() + 5.0;
            enemy_ai
                .move_timer
                .set_duration(Duration::from_secs_f32(new_update_time));
        }
    }
}

fn move_enemy(time: Res<Time>, mut enemies: Query<(&mut Velocity, &mut EnemyAI, &Transform)>) {
    let dt = time.delta_seconds();
    for (mut enemy_velocity, mut enemy_ai, enemy_transform) in &mut enemies {
        let Some(target_position) = enemy_ai.target_position else {
            enemy_velocity.linvel = Vec3::ZERO;
            continue;
        };

        let to_target = target_position - enemy_transform.translation;
        if to_target.length() <= 0.1 {
            enemy_ai.target_position = None;
            enemy_velocity.linvel = Vec3::ZERO;
            continue;
        }

        let to_target = to_target.normalize();
        let enemy_speed = 50.0;

        enemy_velocity.linvel = to_target * enemy_speed * dt;
    }
}

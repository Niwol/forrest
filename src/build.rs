use core::f32;
use std::f32::consts::PI;

use bevy::{color::palettes, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::{
    inventory::Inventory,
    player::PlayerState,
    player_input::{InputMap, PlayerAction},
};

const CELL_SIZE: f32 = 1.0;

pub struct BuildPlugin;

impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayerState::BuildingMode), enter_build_mode)
            .add_systems(OnExit(PlayerState::BuildingMode), exit_building_mode)
            .add_systems(
                Update,
                (move_preview, build, draw_building_grid)
                    .run_if(in_state(PlayerState::BuildingMode)),
            );
    }
}

#[derive(Component)]
struct BuildingPreview;

#[derive(Component)]
struct Building;

fn enter_build_mode(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::from_size(Vec3 {
                x: 2.0,
                y: 1.5,
                z: 2.0,
            })),
            material: materials.add(StandardMaterial::from_color(
                palettes::basic::MAROON.with_alpha(0.3),
            )),
            transform: Transform::from_xyz(0.0, 0.75, 0.0),
            ..Default::default()
        })
        .insert(BuildingPreview);
}

fn exit_building_mode(
    mut commands: Commands,
    building_preview: Query<
        (Entity, &Handle<Mesh>, &Handle<StandardMaterial>),
        With<BuildingPreview>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mesh_handle, material_handle) in &building_preview {
        commands.entity(entity).despawn();
        meshes.remove(mesh_handle);
        materials.remove(material_handle);
    }
}

fn move_preview(
    input: Res<ButtonInput<KeyCode>>,
    mut preview: Query<&mut Transform, With<BuildingPreview>>,
) {
    let mut direction = Vec3::ZERO;

    if input.just_pressed(KeyCode::KeyW) {
        direction = -Vec3::Z;
    }
    if input.just_pressed(KeyCode::KeyS) {
        direction = Vec3::Z;
    }
    if input.just_pressed(KeyCode::KeyA) {
        direction = -Vec3::X;
    }
    if input.just_pressed(KeyCode::KeyD) {
        direction = Vec3::X;
    }

    let mut preview_transform = preview.single_mut();
    preview_transform.translation += direction * CELL_SIZE;
}

fn build(
    input: Res<ButtonInput<KeyCode>>,
    input_map: Res<InputMap>,
    mut inventory: ResMut<Inventory>,
    mut commands: Commands,
    preview: Query<&Transform, With<BuildingPreview>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Some(key_code) = input_map.map.get(&PlayerAction::Build) else {
        return;
    };
    if !input.just_pressed(*key_code) {
        return;
    }

    let cost = 30;
    if cost > inventory.wood {
        return;
    }

    inventory.wood -= cost;

    let preview_transform = preview.single();

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::from_size(Vec3 {
                x: 2.0,
                y: 1.5,
                z: 2.0,
            })),
            material: materials.add(StandardMaterial::from_color(palettes::basic::MAROON)),
            transform: *preview_transform,
            ..Default::default()
        })
        .insert((RigidBody::Fixed, Collider::cuboid(1.0, 0.75, 1.0)))
        .insert(Building);

    next_player_state.set(PlayerState::Normal);
}

fn draw_building_grid(mut gizmos: Gizmos) {
    gizmos
        .grid(
            -Vec3::Z,
            Quat::from_rotation_x(PI / 2.0),
            UVec2::ONE * 100,
            Vec2::ONE * CELL_SIZE,
            Color::srgba(0.5, 0.5, 0.5, 0.2),
        )
        .outer_edges();
}

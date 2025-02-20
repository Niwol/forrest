use std::f32::consts::PI;

use bevy::{color::palettes, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::{
    health::Health,
    inventory::Inventory,
    player::{Interactable, InteractionEvent},
};

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                handle_tree_interaction.run_if(on_event::<InteractionEvent>()),
                despawn_dead_trees,
            ),
        );
    }
}

#[derive(Component)]
pub struct Tree;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_start_trees(&mut commands, &mut meshes, &mut materials);
}

fn spawn_start_trees(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let nb_trees = 10;

    for i in 0..nb_trees {
        let step = (i as f32 / nb_trees as f32) * PI * 2.0;
        let dist = rand::random::<f32>() * 10.0 + 2.0;
        let x = f32::cos(step) * dist;
        let y = 1.5;
        let z = f32::sin(step) * dist;
        let location = Vec3 { x, y, z };

        spawn_tree(commands, location, meshes, materials);
    }
}

fn spawn_tree(
    commands: &mut Commands,
    loaction: Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::from_size(Vec3 {
                x: 0.3,
                y: 3.0,
                z: 0.3,
            })),
            material: materials.add(StandardMaterial::from_color(palettes::basic::GREEN)),
            transform: Transform::from_translation(loaction),
            ..Default::default()
        })
        .insert((RigidBody::Fixed, Collider::capsule_y(1.5, 0.15)))
        .insert(Health::new_full(10))
        .insert(Tree)
        .insert(Interactable);
}

fn handle_tree_interaction(
    mut trees: Query<&mut Health, With<Tree>>,
    mut interaction_events: EventReader<InteractionEvent>,
    mut inventory: ResMut<Inventory>,
) {
    for interacion_event in interaction_events.read() {
        // Need more robust entity check -> not always entity_2
        if let Ok(mut tree_health) = trees.get_mut(interacion_event.entity_2) {
            tree_health.change(-3);

            if tree_health.is_dead() {
                inventory.wood += 10;
            }
        }
    }
}

fn despawn_dead_trees(
    mut commands: Commands,
    trees: Query<
        (Entity, &Handle<Mesh>, &Handle<StandardMaterial>, &Health),
        (With<Tree>, Changed<Health>),
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (tree_entity, mesh_handle, material_handle, health) in &trees {
        if health.is_dead() {
            commands.entity(tree_entity).despawn_recursive();
            meshes.remove(mesh_handle);
            materials.remove(material_handle);
        }
    }
}

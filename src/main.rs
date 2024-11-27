use bevy::{input::mouse::MouseWheel, math::VectorSpace, prelude::*};
use bevy_rapier3d::prelude::*;
use forrest::{
    enemy::EnemyPlugin,
    inventory::InventoryPlugin,
    player::{CameraZoom, PlayerPlugin},
    tree::TreePlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default().disabled())
        .add_plugins((PlayerPlugin, TreePlugin, EnemyPlugin, InventoryPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (exit, toggle_debug_view, handle_zoom))
        .run();
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Ground
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::from_size(Vec3 {
            x: 100.0,
            y: 0.2,
            z: 100.0,
        })),
        ..Default::default()
    });

    rapier_config.gravity = Vec3::ZERO;
}

fn exit(input: Res<ButtonInput<KeyCode>>, mut exit_event: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit_event.send_default();
    }
}

fn toggle_debug_view(input: Res<ButtonInput<KeyCode>>, mut debug_view: ResMut<DebugRenderContext>) {
    if input.just_pressed(KeyCode::F1) {
        debug_view.enabled = !debug_view.enabled;
    }
}

fn handle_zoom(mut mouse_weel: EventReader<MouseWheel>, mut camera_zoom: ResMut<CameraZoom>) {
    for mouse_weel in mouse_weel.read() {
        camera_zoom.0 += mouse_weel.y;
    }
}

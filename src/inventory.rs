use bevy::{color::palettes, prelude::*};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>()
            .add_systems(Startup, setup_ui)
            .add_systems(Update, update_ui);
    }
}

#[derive(Resource)]
pub struct Inventory {
    pub wood: u32,
}

#[derive(Component)]
struct WoodText;

impl Default for Inventory {
    fn default() -> Self {
        Self { wood: 0 }
    }
}

fn setup_ui(mut commands: Commands) {
    commands
        .spawn(TextBundle::from_sections([
            TextSection {
                value: String::from("Wood: "),
                style: TextStyle {
                    font_size: 30.0,
                    color: palettes::basic::GREEN.into(),
                    ..Default::default()
                },
            },
            TextSection {
                value: String::from("0"),
                style: TextStyle {
                    font_size: 30.0,
                    color: palettes::basic::GREEN.into(),
                    ..Default::default()
                },
            },
        ]))
        .insert(WoodText);
}

fn update_ui(mut ui_text: Query<&mut Text, With<WoodText>>, inventory: Res<Inventory>) {
    if !inventory.is_changed() {
        return;
    }

    let mut wood_text = ui_text.single_mut();

    wood_text.sections[1].value = inventory.wood.to_string();
}

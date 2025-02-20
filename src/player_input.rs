use bevy::{ecs::system::SystemParam, prelude::*, utils::HashMap};

#[derive(Resource)]
pub struct InputMap {
    pub map: HashMap<PlayerAction, KeyCode>,
}

impl Default for InputMap {
    fn default() -> Self {
        Self {
            map: HashMap::from([
                (PlayerAction::Interact, KeyCode::KeyF),
                (PlayerAction::Attack, KeyCode::Space),
                (PlayerAction::Build, KeyCode::KeyB),
                (PlayerAction::Cancel, KeyCode::KeyC),
            ]),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum PlayerAction {
    Interact,
    Attack,
    Build,
    Cancel,
}

#[derive(SystemParam)]
pub struct InputParam<'w> {
    input_map: Res<'w, InputMap>,
    input: Res<'w, ButtonInput<KeyCode>>,
}
impl<'w> InputParam<'w> {
    pub fn action_just_pressed(&self, player_action: PlayerAction) -> bool {
        let Some(key_code) = self.input_map.map.get(&player_action) else {return false;};

        self.input.just_pressed(*key_code)
    }

    pub fn action_pressed(&self, player_action: PlayerAction) -> bool {
        let Some(key_code) = self.input_map.map.get(&player_action) else {return false;};

        self.input.pressed(*key_code)
    }

    pub fn action_just_released(&self, player_action: PlayerAction) -> bool {
        let Some(key_code) = self.input_map.map.get(&player_action) else {return false;};

        self.input.just_released(*key_code)
    }
}
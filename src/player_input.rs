use bevy::{prelude::*, utils::HashMap};

#[derive(Resource)]
pub struct InputMap {
    pub map: HashMap<PlayerAction, KeyCode>,
}

impl Default for InputMap {
    fn default() -> Self {
        Self {
            map: HashMap::from([
                (PlayerAction::Interact, KeyCode::KeyF),
                (PlayerAction::Build, KeyCode::KeyB),
                (PlayerAction::Cancel, KeyCode::KeyC),
            ]),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum PlayerAction {
    Interact,
    Build,
    Cancel,
}

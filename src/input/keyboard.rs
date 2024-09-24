use bevy::prelude::*;
use bevy_mod_openxr::resources::OxrViews;

use crate::movement::PlayerInputState;

#[derive(Resource)]
pub struct InputMap {
    pub key_forward: KeyCode,
    pub key_backward: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_jump: KeyCode,
}

impl Default for InputMap {
    fn default() -> Self {
        Self {
            key_forward: KeyCode::KeyW,
            key_backward: KeyCode::KeyS,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            key_jump: KeyCode::Space,
        }
    }
}

pub fn read_keyboard_input(
    input_map: Res<InputMap>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_state: Query<&mut PlayerInputState>,
    views: Res<OxrViews>,
) {
    if !views.is_empty() {
        return;
    }

    for mut input in player_state.iter_mut() {
        let forward = keys.pressed(input_map.key_forward);
        let backward = keys.pressed(input_map.key_backward);
        let left = keys.pressed(input_map.key_left);
        let right = keys.pressed(input_map.key_right);

        let forward = if forward { 1.0 } else { 0.0 };
        let backward = if backward { -1.0 } else { 0.0 };
        let left = if left { 1.0 } else { 0.0 };
        let right = if right { -1.0 } else { 0.0 };

        input.forward = forward + backward;
        input.left = left + right;

        input.jump = keys.pressed(input_map.key_jump);
    }
}

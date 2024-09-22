use bevy::prelude::*;

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
) {
    for mut state in player_state.iter_mut() {
        state.forward = keys.pressed(input_map.key_forward);
        state.backward = keys.pressed(input_map.key_backward);
        state.left = keys.pressed(input_map.key_left);
        state.right = keys.pressed(input_map.key_right);
        state.jump = keys.pressed(input_map.key_jump);
    }
}

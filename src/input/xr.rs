use bevy::prelude::*;
use bevy_mod_xr::actions::ActionType;
use bevy_xr_utils::xr_utils_actions::{
    ActiveSet, XRUtilsAction, XRUtilsActionSet, XRUtilsActionState, XRUtilsBinding,
};

use crate::movement::PlayerInputState;

pub fn setup_xr_actions(mut commands: Commands) {
    let set = commands
        .spawn((
            XRUtilsActionSet {
                name: "movement".into(),
                pretty_name: "Movement Set".into(),
                priority: u32::MIN,
            },
            ActiveSet,
        ))
        .id();

    let action = commands
        .spawn((
            XRUtilsAction {
                action_name: "move".into(),
                localized_name: "move_localized".into(),
                action_type: ActionType::Vector,
            },
            MoveAction,
        ))
        .id();

    let binding_left_index = commands
        .spawn(XRUtilsBinding {
            profile: "/interaction_profiles/valve/index_controller".into(),
            binding: "/user/hand/left/input/thumbstick".into(),
        })
        .id();

    let binding_left_touch = commands
        .spawn(XRUtilsBinding {
            profile: "/interaction_profiles/oculus/touch_controller".into(),
            binding: "/user/hand/left/input/thumbstick".into(),
        })
        .id();

    let binding_right_index = commands
        .spawn(XRUtilsBinding {
            profile: "/interaction_profiles/valve/index_controller".into(),
            binding: "/user/hand/right/input/thumbstick".into(),
        })
        .id();

    let binding_right_touch = commands
        .spawn(XRUtilsBinding {
            profile: "/interaction_profiles/oculus/touch_controller".into(),
            binding: "/user/hand/right/input/thumbstick".into(),
        })
        .id();

    commands.entity(action).add_child(binding_left_index);
    commands.entity(action).add_child(binding_left_touch);
    commands.entity(action).add_child(binding_right_index);
    commands.entity(action).add_child(binding_right_touch);
    commands.entity(set).add_child(action);
}

#[derive(Component)]
pub struct MoveAction;

pub fn read_xr_input(
    move_action: Query<&XRUtilsActionState, With<MoveAction>>,
    mut input_state: Query<&mut PlayerInputState>,
) {
    for action_state in move_action.iter() {
        for mut input in input_state.iter_mut() {
            if let XRUtilsActionState::Vector(value) = action_state {
                input.forward = value.current_state[1];
                input.left = -value.current_state[0];
            } else {
                panic!("Invalid action state variant");
            }
        }
    }
}

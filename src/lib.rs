use bevy::{asset::embedded_asset, prelude::*};
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;
use bevy_vrm::VrmPlugins;

pub mod animation;
mod eye_offset;
mod first_person;
mod head;
pub mod input;
mod look;
pub mod movement;
pub mod player;
mod velocity;

pub struct VrControllerPlugin;

impl Plugin for VrControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TnuaAvian3dPlugin::default(),
            TnuaControllerPlugin::default(),
            VrmPlugins,
        ))
        .init_resource::<input::InputMap>()
        .add_event::<look::CameraLookEvent>()
        .add_systems(
            Update,
            (
                animation::init_animations,
                animation::load::load_animation_nodes,
                animation::weights::play_avatar_animations,
                eye_offset::calc_eye_offset,
                first_person::setup_first_person,
                head::set_avatar_head,
                input::read_keyboard_input,
                look::grab_mouse,
                velocity::calc_average_velocity,
                (
                    (look::read_mouse_input, look::apply_camera_look).chain(),
                    (
                        (movement::void_teleport, movement::move_player).chain(),
                        head::rotate_avatar_head,
                    ),
                )
                    .chain(),
            ),
        );

        embedded_asset!(app, "animation/default-animations.glb");
    }
}

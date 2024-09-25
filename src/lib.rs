use bevy::{asset::embedded_asset, prelude::*};
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;
use bevy_vrm::VrmPlugins;
use bevy_xr_utils::xr_utils_actions::XRUtilsActionSystemSet;

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
        .init_resource::<input::keyboard::InputMap>()
        .add_event::<input::mouse::CameraLookEvent>()
        .add_systems(
            Startup,
            input::xr::setup_xr_actions.before(XRUtilsActionSystemSet::CreateEvents),
        )
        .add_systems(
            Update,
            (
                animation::init_animations,
                animation::load::load_animation_nodes,
                animation::weights::play_avatar_animations,
                eye_offset::calc_eye_offset,
                first_person::setup_first_person,
                head::set_avatar_head,
                look::grab_mouse,
                player::set_xr_render_layers,
                velocity::calc_average_velocity,
                (
                    input::mouse::read_mouse_input,
                    look::apply_camera_look,
                    (
                        head::rotate_avatar_head,
                        (
                            (
                                input::keyboard::read_keyboard_input,
                                input::xr::read_xr_input,
                            ),
                            movement::void_teleport,
                            movement::move_player,
                            movement::move_xr_root,
                        ),
                    )
                        .chain(),
                )
                    .chain(),
            ),
        );

        embedded_asset!(app, "animation/default-animations.glb");
    }
}

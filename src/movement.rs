use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_mod_openxr::{helper_traits::ToVec3, resources::OxrViews};
use bevy_mod_xr::session::XrTrackingRoot;
use bevy_tnua::prelude::*;

use crate::player::{
    PlayerBody, PlayerHeight, PlayerJumpHeight, PlayerSpawn, PlayerSpeed, VoidTeleport,
};

#[derive(Component, Default)]
pub struct PlayerInputState {
    pub forward: f32,
    pub left: f32,
    pub jump: bool,
}

pub fn move_player(
    mut last_time: Local<f32>,
    mut players: Query<
        (
            &Transform,
            &mut PlayerInputState,
            &PlayerHeight,
            &PlayerSpeed,
            &PlayerJumpHeight,
            &mut TnuaController,
        ),
        With<PlayerBody>,
    >,
    time: Res<Time>,
) {
    debug_assert!(*last_time >= 0.0);

    for (transform, mut input, height, speed, jump_height, mut controller) in players.iter_mut() {
        let dir_forward = transform.rotation.mul_vec3(Vec3::NEG_Z);
        let dir_left = transform.rotation.mul_vec3(Vec3::NEG_X);

        let mut move_direction = Vec3::ZERO;

        move_direction += dir_forward * input.forward;
        move_direction += dir_left * input.left;

        let desired_velocity = move_direction.normalize_or_zero() * speed.0;

        if input.jump {
            controller.action(TnuaBuiltinJump {
                height: jump_height.0,
                ..default()
            });
        }

        controller.basis(TnuaBuiltinWalk {
            coyote_time: 0.2,
            desired_velocity,
            float_height: (height.0 / 2.0) + 0.1,
            ..default()
        });

        *input = PlayerInputState::default();
    }

    *last_time = time.elapsed_seconds();
}

pub fn apply_xr_pose(
    mut players: Query<
        (&mut Transform, &PlayerHeight),
        (With<PlayerBody>, Without<XrTrackingRoot>),
    >,
    views: Res<OxrViews>,
) {
    let Some(view) = views.first() else {
        return;
    };

    for (mut player_tr, player_height) in players.iter_mut() {
        // player_tr.translation += view.pose.position.to_vec3();
        // player_tr.translation.y -= player_height.0 / 2.0;
    }
}

pub fn move_xr_root(
    players: Query<(&Transform, &PlayerHeight), (With<PlayerBody>, Without<XrTrackingRoot>)>,
    mut xr_root: Query<&mut Transform, (With<XrTrackingRoot>, Without<PlayerBody>)>,
    views: Res<OxrViews>,
) {
    let Some(view) = views.first() else {
        return;
    };

    let Ok(mut root_tr) = xr_root.get_single_mut() else {
        return;
    };

    for (player_tr, player_height) in players.iter() {
        root_tr.translation = player_tr.translation;
        root_tr.translation.y -= player_height.0 / 2.0;
        // root_tr.translation -= view.pose.position.to_vec3();
    }
}

pub fn void_teleport(
    mut players: Query<(
        &PlayerSpawn,
        &VoidTeleport,
        &mut Transform,
        &mut LinearVelocity,
        &mut AngularVelocity,
    )>,
) {
    for (spawn, void_level, mut transform, mut linvel, mut angvel) in players.iter_mut() {
        if transform.translation.y < void_level.0 {
            debug!("Player fell into void! Teleporting player to spawn...");
            transform.translation = spawn.0;
            angvel.x = 0.0;
            angvel.y = 0.0;
            angvel.z = 0.0;
            linvel.x = 0.0;
            linvel.y = 0.0;
            linvel.z = 0.0;

            // TODO: Reset camera rotation
        }
    }
}

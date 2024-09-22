use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::prelude::*;

use crate::player::{PlayerHeight, PlayerJumpHeight, PlayerSpawn, PlayerSpeed, VoidTeleport};

#[derive(Component, Default)]
pub struct PlayerInputState {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
}

pub fn move_player(
    mut last_time: Local<f32>,
    mut players: Query<(
        &Transform,
        &mut PlayerInputState,
        &PlayerHeight,
        &PlayerSpeed,
        &PlayerJumpHeight,
        &mut TnuaController,
    )>,
    time: Res<Time>,
) {
    debug_assert!(*last_time >= 0.0);

    for (transform, mut input, height, speed, jump_height, mut controller) in players.iter_mut() {
        let dir_forward = transform.rotation.mul_vec3(Vec3::NEG_Z);
        let dir_left = transform.rotation.mul_vec3(Vec3::NEG_X);

        let mut move_direction = Vec3::ZERO;

        if input.forward {
            move_direction += dir_forward;
        }
        if input.backward {
            move_direction -= dir_forward;
        }
        if input.left {
            move_direction += dir_left;
        }
        if input.right {
            move_direction -= dir_left;
        }

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

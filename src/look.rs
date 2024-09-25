use bevy::{prelude::*, window::CursorGrabMode, window::Window};
use bevy_mod_openxr::{helper_traits::ToQuat, resources::OxrViews};

use crate::{
    input::mouse::CameraLookEvent,
    player::{CameraFreeLook, PlayerBody, PlayerCamera},
};

const CAM_LERP_FACTOR: f32 = 30.0;

pub fn apply_camera_look(
    mut cameras: Query<
        (&mut Transform, &CameraFreeLook),
        (With<PlayerCamera>, Without<PlayerBody>),
    >,
    mut free_yaw: Local<Option<Quat>>,
    mut look_events: EventReader<CameraLookEvent>,
    mut players: Query<(&mut Transform, &Children), (With<PlayerBody>, Without<Camera>)>,
    mut target_pitch_roll: Local<Quat>,
    mut target_yaw: Local<Quat>,
    time: Res<Time>,
    views: Res<OxrViews>,
) {
    if let Some(view) = views.first() {
        let rotation = view.pose.orientation.to_quat();

        let mut yaw = rotation;
        let mut pitch_roll = rotation;

        yaw.x = 0.0;
        yaw.z = 0.0;
        *target_yaw = yaw.normalize();

        pitch_roll.y = 0.0;
        *target_pitch_roll = pitch_roll.normalize();
    } else {
        for look in look_events.read() {
            *target_yaw = Quat::from_rotation_y(look.x);
            *target_pitch_roll = Quat::from_rotation_x(look.y);
        }
    }

    let lerp_factor = time.delta_seconds() * CAM_LERP_FACTOR;

    for (mut player_tr, children) in players.iter_mut() {
        for child in children.iter() {
            if let Ok((mut camera_tr, free)) = cameras.get_mut(*child) {
                let target = if free.0 {
                    if let Some(free_yaw) = *free_yaw {
                        (*target_yaw * free_yaw.inverse()) * *target_pitch_roll
                    } else {
                        *free_yaw = Some(*target_yaw);
                        *target_pitch_roll
                    }
                } else {
                    player_tr.rotation = player_tr.rotation.lerp(*target_yaw, lerp_factor);

                    if free_yaw.is_some() {
                        *free_yaw = None;
                    }

                    *target_pitch_roll
                };

                camera_tr.rotation = camera_tr.rotation.lerp(target, lerp_factor);
            }
        }
    }
}

pub fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    for mut window in windows.iter_mut() {
        if mouse.just_pressed(MouseButton::Left) {
            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;
        }

        if key.just_pressed(KeyCode::Escape) {
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;
        }
    }
}

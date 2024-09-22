use std::f32::consts::FRAC_PI_2;

use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode, window::Window};

use crate::player::{CameraFreeLook, PlayerBody};

#[derive(Resource, Event, Debug, Default, Deref, DerefMut)]
pub struct CameraLookEvent(pub Vec2);

const PITCH_BOUND: f32 = FRAC_PI_2 - 1E-3;
const SENSITIVITY: f32 = 0.001;

pub fn read_mouse_input(
    #[cfg(target_family = "wasm")] mut is_firefox: Local<Option<bool>>,
    mut look_events: EventWriter<CameraLookEvent>,
    mut look_xy: Local<Vec2>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    windows: Query<&Window>,
) {
    if mouse_motion_events.is_empty() {
        return;
    }

    if !windows
        .iter()
        .any(|window| window.cursor.grab_mode == CursorGrabMode::Locked)
    {
        return;
    }

    let mut delta = Vec2::ZERO;

    for motion in mouse_motion_events.read() {
        delta -= motion.delta;
    }

    delta *= SENSITIVITY;

    #[cfg(target_family = "wasm")]
    {
        // Adjust the sensitivity when running in Firefox.
        // I think because of incorrect values within mouse move events.
        if let Some(is_firefox) = *is_firefox {
            if is_firefox {
                delta *= 10.0;
            }
        } else {
            let window = web_sys::window().unwrap();
            let navigator = window.navigator().user_agent().unwrap();
            *is_firefox = Some(navigator.to_lowercase().contains("firefox"));
        }
    }

    *look_xy += delta;
    look_xy.y = look_xy.y.clamp(-PITCH_BOUND, PITCH_BOUND);

    look_events.send(CameraLookEvent(*look_xy));
}

const CAM_LERP_FACTOR: f32 = 30.0;

pub fn apply_camera_look(
    mut cameras: Query<(&mut Transform, &CameraFreeLook), With<Camera>>,
    mut free_yaw: Local<Option<Quat>>,
    mut look_events: EventReader<CameraLookEvent>,
    mut players: Query<(&mut Transform, &Children), With<PlayerBody>>,
    mut target_pitch: Local<Quat>,
    mut target_yaw: Local<Quat>,
    time: Res<Time>,
) {
    for look in look_events.read() {
        *target_yaw = Quat::from_rotation_y(look.x);
        *target_pitch = Quat::from_rotation_x(look.y);
    }

    let lerp_factor = time.delta_seconds() * CAM_LERP_FACTOR;

    for (mut player_tr, children) in players.iter_mut() {
        for child in children.iter() {
            if let Ok((mut camera_tr, free)) = cameras.get_mut(*child) {
                let target = if free.0 {
                    if let Some(free_yaw) = *free_yaw {
                        (*target_yaw * free_yaw.inverse()) * *target_pitch
                    } else {
                        *free_yaw = Some(*target_yaw);
                        *target_pitch
                    }
                } else {
                    player_tr.rotation = player_tr.rotation.lerp(*target_yaw, lerp_factor);

                    if free_yaw.is_some() {
                        *free_yaw = None;
                    }

                    *target_pitch
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

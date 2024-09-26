use std::f32::consts::FRAC_PI_2;

use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};

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
    #[cfg(feature = "xr")]
    #[cfg(not(target_family = "wasm"))]
    views: Res<bevy_mod_openxr::resources::OxrViews>,
) {
    #[cfg(feature = "xr")]
    #[cfg(not(target_family = "wasm"))]
    if !views.is_empty() {
        return;
    }

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

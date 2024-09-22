use bevy::{prelude::*, utils::HashMap};

use crate::animation::AnimationName;

use super::load::{AvatarAnimation, AvatarAnimationClips};

const ASSET: &str = "embedded://bevy_vr_controller/animation/default-animations.glb";

pub fn default_character_animations(asset_server: &AssetServer) -> AvatarAnimationClips {
    let mut map = HashMap::default();

    let gltf = asset_server.load(ASSET);

    map.insert(
        AnimationName::Falling,
        AvatarAnimation {
            clip: asset_server.load(format!("{}#Animation0", ASSET)),
            gltf: gltf.clone(),
        },
    );
    map.insert(
        AnimationName::Idle,
        AvatarAnimation {
            clip: asset_server.load(format!("{}#Animation1", ASSET)),
            gltf: gltf.clone(),
        },
    );
    map.insert(
        AnimationName::WalkLeft,
        AvatarAnimation {
            clip: asset_server.load(format!("{}#Animation2", ASSET)),
            gltf: gltf.clone(),
        },
    );
    map.insert(
        AnimationName::WalkRight,
        AvatarAnimation {
            clip: asset_server.load(format!("{}#Animation3", ASSET)),
            gltf: gltf.clone(),
        },
    );
    map.insert(
        AnimationName::Walk,
        AvatarAnimation {
            clip: asset_server.load(format!("{}#Animation5", ASSET)),
            gltf,
        },
    );

    AvatarAnimationClips(map)
}

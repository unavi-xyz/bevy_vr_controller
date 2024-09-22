use avian3d::prelude::*;
use bevy::{prelude::*, render::view::RenderLayers};
use bevy_tnua::prelude::TnuaControllerBundle;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use bevy_vrm::{
    first_person::{FirstPersonFlag, RENDER_LAYERS},
    loader::Vrm,
    VrmBundle,
};

use crate::{first_person::FirstPerson, velocity::AverageVelocity};

pub struct PlayerSettings {
    pub height: f32,
    pub void_level: Option<f32>,
    pub vrm: Option<Handle<Vrm>>,
    pub width: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            height: 1.6,
            void_level: None,
            vrm: None,
            width: 0.4,
        }
    }
}

impl PlayerSettings {
    pub fn spawn(&self, commands: &mut Commands) -> SpawnedPlayer {
        let mut body = commands.spawn((
            Collider::capsule(self.width / 2.0, self.height - self.width),
            LockedAxes::ROTATION_LOCKED,
            PlayerBody,
            PlayerHeight(self.height),
            PlayerSpeed(1.0),
            PlayerJumpHeight(1.0),
            PlayerSpawn(Vec3::default()),
            RigidBody::Dynamic,
            SpatialBundle::default(),
            TnuaAvian3dSensorShape(Collider::cylinder((self.width / 2.0) * 0.95, 0.0)),
            TnuaControllerBundle::default(),
        ));

        if let Some(value) = self.void_level {
            body.insert(VoidTeleport(value));
        }

        let body = body.id();

        let avatar = commands
            .spawn((
                AverageVelocity {
                    target: Some(body),
                    ..default()
                },
                PlayerAvatar,
                VrmBundle {
                    scene_bundle: SceneBundle {
                        transform: Transform::from_xyz(0.0, -self.height / 2.0, 0.0),
                        ..default()
                    },
                    vrm: self.vrm.clone().unwrap_or_default(),
                    ..default()
                },
                FirstPerson,
            ))
            .id();

        let camera = commands
            .spawn((
                Camera3dBundle {
                    transform: Transform::from_xyz(0.0, -self.height / 2.0, 0.0),
                    ..default()
                },
                CameraFreeLook(false),
                PlayerCamera,
                PlayerHeight(self.height),
                RenderLayers::layer(0).union(&RENDER_LAYERS[&FirstPersonFlag::FirstPersonOnly]),
            ))
            .id();

        commands.entity(body).push_children(&[avatar, camera]);

        SpawnedPlayer {
            avatar,
            body,
            camera,
        }
    }
}

pub struct SpawnedPlayer {
    pub avatar: Entity,
    pub body: Entity,
    pub camera: Entity,
}

#[derive(Component)]
pub struct PlayerAvatar;

#[derive(Component)]
pub struct PlayerBody;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct PlayerHeight(pub(crate) f32);

#[derive(Component)]
pub struct PlayerSpeed(pub f32);

#[derive(Component)]
pub struct PlayerJumpHeight(pub f32);

#[derive(Component)]
pub struct PlayerSpawn(pub Vec3);

#[derive(Component)]
pub struct CameraFreeLook(pub bool);

/// Teleport the player to spawn if they fall below a certain Y level.
#[derive(Component)]
pub struct VoidTeleport(pub f32);

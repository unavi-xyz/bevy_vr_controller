use bevy::{prelude::*, transform::systems::propagate_transforms};

mod systems;

pub struct HumanoidIKPlugin;

impl Plugin for HumanoidIKPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RunHumanoidIk>().add_systems(
            Update,
            (
                systems::setup_ik_system,
                systems::modify_ik_state,
                (
                    propagate_transforms,
                    systems::reset_rotations,
                    propagate_transforms,
                    systems::humanoid_ik_system,
                )
                    .chain()
                    .after(crate::animation::weights::play_avatar_animations)
                    .run_if(resource_equals::<RunHumanoidIk>(RunHumanoidIk(true))),
            ),
        );
    }
}

#[derive(Resource, Default, PartialEq, Eq)]
pub struct RunHumanoidIk(pub bool);

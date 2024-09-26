use crate::animation;
use crate::animation::load::AvatarAnimationClips;
use bevy::prelude::*;
use bevy::transform::systems::propagate_transforms;
use bevy_mod_picking::PickableBundle;
use bevy_mod_xr::session::{XrState, XrTrackingRoot};
use bevy_transform_gizmo::GizmoTransformable;
use bevy_vrm::BoneName;
use bevy_xr_utils::tracking_utils::{XrTrackedLeftGrip, XrTrackedRightGrip};

#[derive(Component)]
struct HumanoidIK {
    left_target: Entity,
    right_target: Entity,
    pole_vector_left: Vec3,
    pole_vector_right: Vec3,
}

struct IKChain {
    joints: Vec<Entity>,
    lengths: Vec<f32>,
}

#[derive(Component)]
struct HumanoidIkChain {
    left_chain: IKChain,
    right_chain: IKChain,
}

#[derive(Component)]
pub struct RestPose {
    global: Quat,
    local: Quat,
}

fn setup_ik_system(
    mut commands: Commands,
    bones: Query<(Entity, &BoneName)>,
    transforms: Query<&Transform>,
    mut has_run: Local<bool>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    globals: Query<&GlobalTransform>,
    tracking_root: Query<Entity, With<XrTrackingRoot>>,
) {
    if *has_run {
        return;
    }

    let mut hip = None;

    let mut left_shoulder = None;
    let mut left_upper_arm = None;
    let mut left_lower_arm = None;
    let mut left_hand = None;

    for (entity, name) in bones.iter() {
        match name {
            BoneName::Hips => hip.replace(entity),
            BoneName::LeftShoulder => left_shoulder.replace(entity),
            BoneName::LeftUpperArm => left_upper_arm.replace(entity),
            BoneName::LeftLowerArm => left_lower_arm.replace(entity),
            BoneName::LeftHand => left_hand.replace(entity),
            _ => continue,
        };
    }

    let (Some(shoulder), Some(upper), Some(lower), Some(hand)) =
        (left_shoulder, left_upper_arm, left_lower_arm, left_hand)
    else {
        return;
    };

    let left_joints = vec![shoulder, upper, lower, hand];

    let mut right_shoulder = None;
    let mut right_upper_arm = None;
    let mut right_lower_arm = None;
    let mut right_hand = None;

    for (entity, name) in bones.iter() {
        match name {
            BoneName::RightShoulder => right_shoulder.replace(entity),
            BoneName::RightUpperArm => right_upper_arm.replace(entity),
            BoneName::RightLowerArm => right_lower_arm.replace(entity),
            BoneName::RightHand => right_hand.replace(entity),
            _ => continue,
        };
    }

    let (Some(shoulder), Some(upper), Some(lower), Some(hand)) =
        (right_shoulder, right_upper_arm, right_lower_arm, right_hand)
    else {
        return;
    };

    let right_joints = vec![shoulder, upper, lower, hand];

    let mut left_lengths: Vec<f32> = left_joints
        .iter()
        .map(|a| transforms.get(*a).unwrap().translation.length())
        .collect();
    left_lengths.remove(0);
    left_lengths.push(0.1);

    let left_target = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(Sphere::new(0.1))),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.8, 0.8, 0.8),
                    ..Default::default()
                }),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            XrTrackedLeftGrip,
        ))
        .id();

    // Define a pole vector pointing downward
    let pole_vector_left = Vec3::new(0.0, -1.0, 0.0);

    for j in left_joints.iter() {
        commands.entity(*j).insert(RestPose {
            global: globals.get(*j).unwrap().to_scale_rotation_translation().1,
            local: transforms.get(*j).unwrap().rotation,
        });
    }

    let mut right_lengths: Vec<f32> = right_joints
        .iter()
        .map(|a| transforms.get(*a).unwrap().translation.length())
        .collect();
    right_lengths.remove(0);
    right_lengths.push(0.1);

    let right_target = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(Sphere::new(0.1))),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.8, 0.8, 0.8),
                    ..Default::default()
                }),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            XrTrackedRightGrip,
        ))
        .id();

    // Define a pole vector pointing downward
    let pole_vector_right = Vec3::new(0.0, -1.0, 0.0);

    for j in right_joints.iter() {
        commands.entity(*j).insert(RestPose {
            global: globals.get(*j).unwrap().to_scale_rotation_translation().1,
            local: transforms.get(*j).unwrap().rotation,
        });
    }

    commands.spawn((
        HumanoidIkChain {
            left_chain: IKChain {
                joints: left_joints,
                lengths: left_lengths,
            },
            right_chain: IKChain {
                joints: right_joints,
                lengths: right_lengths,
            },
        },
        HumanoidIK {
            left_target,
            right_target,
            pole_vector_left,
            pole_vector_right,
        },
    ));
    if let Ok(root) = tracking_root.get_single() {
        commands
            .entity(root)
            .push_children(&[left_target, right_target]);
    }

    *has_run = true;
}
fn apply_pole_vector_constraint(
    desired_positions: &mut Vec<Vec3>,
    pole_vector: Vec3,
    chain: &IKChain,
    weight: f32,
) {
    let shoulder_pos = desired_positions[1];
    let elbow_pos = desired_positions[2];
    let hand_pos = desired_positions[3];

    // Direction vectors
    let shoulder_to_elbow = elbow_pos - shoulder_pos;
    let shoulder_to_hand = hand_pos - shoulder_pos;

    // Plane normal defined by the pole vector
    let plane_normal = shoulder_to_hand.cross(pole_vector).normalize();

    // Project the elbow onto the plane
    let projected_elbow =
        shoulder_pos + (shoulder_to_elbow - plane_normal * shoulder_to_elbow.dot(plane_normal));

    // Blend between the current elbow position and the projected position
    let new_elbow_pos = elbow_pos.lerp(projected_elbow, weight);

    // Update the elbow position
    desired_positions[1] =
        shoulder_pos + (new_elbow_pos - shoulder_pos).normalize() * chain.lengths[1];
}
fn humanoid_ik_system(
    mut query: Query<(&HumanoidIK, &HumanoidIkChain)>,
    mut transforms: Query<&mut Transform>,
    mut globals: Query<&mut GlobalTransform>,
    parents: Query<&Parent>,
) {
    for (ik, chain) in query.iter() {
        {
            let mut positions = Vec::new();
            let mut total_length = 0.0;

            // Get initial positions and calculate total length
            for (&joint, &length) in chain
                .right_chain
                .joints
                .iter()
                .zip(chain.right_chain.lengths.iter())
            {
                if let Ok(transform) = globals.get(joint) {
                    positions.push(transform.translation());
                    total_length += length;
                }
            }

            // Check if target is reachable
            let right_target = globals.get(ik.right_target).unwrap().translation();
            let base = positions[0];
            let to_target = right_target - base;
            let distance = to_target.length();

            let len = positions.len();

            let mut desired_positions = positions.clone();

            if distance > total_length {
                // Target is unreachable, extend towards it
                let direction = to_target.normalize();
                for (i, pos) in desired_positions.iter_mut().enumerate() {
                    if i > 0 {
                        *pos = base
                            + direction * chain.right_chain.lengths.iter().take(i).sum::<f32>();
                    }
                }
            } else {
                // FABRIK algorithm with pole vector as a soft constraint
                let iterations = 30;
                for i in 0..iterations {
                    // Forward pass
                    desired_positions[len - 1] = right_target;
                    for i in (1..len).rev() {
                        let dir = (desired_positions[i - 1] - desired_positions[i]).normalize();
                        desired_positions[i - 1] =
                            desired_positions[i] + dir * chain.right_chain.lengths[i - 1];
                    }

                    let weight = 1.0;

                   /* apply_pole_vector_constraint(
                        &mut desired_positions,
                        ik.pole_vector_right,
                        &chain.right_chain,
                        weight,
                    );*/

                    // Apply pole vector constraint after forward pass

                    // Backward pass
                    desired_positions[0] = base;
                    for i in 1..len {
                        let dir = (desired_positions[i] - desired_positions[i - 1]).normalize();
                        desired_positions[i] =
                            desired_positions[i - 1] + dir * chain.right_chain.lengths[i - 1];
                    }

                   /* apply_pole_vector_constraint(
                        &mut desired_positions,
                        ik.pole_vector_right,
                        &chain.right_chain,
                        weight,
                    );*/

                    // Apply pole vector constraint after backward pass
                }
            }

            // Update rotations
            for i in 0..len - 1 {
                let joint = chain.right_chain.joints[i];
                if let Ok(mut transform) = transforms.get_mut(joint) {
                    let current_global_transform = globals.get(joint).unwrap();
                    let current_position = current_global_transform.translation();
                    let current_rotation =
                        current_global_transform.to_scale_rotation_translation().1;

                    let desired_dir = (desired_positions[i + 1] - desired_positions[i]).normalize();
                    let current_dir = (positions[i + 1] - positions[i]).normalize();

                    let rotation_needed = Quat::from_rotation_arc(current_dir, desired_dir);

                    // Convert rotation to local space
                    if let Ok(parent) = parents.get(joint) {
                        let parent_global_rotation = globals
                            .get(parent.get())
                            .unwrap()
                            .to_scale_rotation_translation()
                            .1;
                        let local_rotation =
                            parent_global_rotation.inverse() * rotation_needed * current_rotation;
                        transform.rotation = local_rotation;
                        let mut temp = globals.get(joint).unwrap().clone().compute_transform();
                        temp.rotation = rotation_needed * current_rotation;
                        *globals.get_mut(joint).unwrap() = GlobalTransform::from(temp);
                    } else {
                        transform.rotation = rotation_needed * current_rotation;
                    }
                }
            }
        }

        {
            let mut positions = Vec::new();
            let mut total_length = 0.0;

            // Get initial positions and calculate total length
            for (&joint, &length) in chain
                .left_chain
                .joints
                .iter()
                .zip(chain.left_chain.lengths.iter())
            {
                if let Ok(transform) = globals.get(joint) {
                    positions.push(transform.translation());
                    total_length += length;
                }
            }

            // Check if target is reachable
            let left_target = globals.get(ik.left_target).unwrap().translation();
            let base = positions[0];
            let to_target = left_target - base;
            let distance = to_target.length();

            let len = positions.len();

            let mut desired_positions = positions.clone();

            if distance > total_length {
                // Target is unreachable, extend towards it
                let direction = to_target.normalize();
                for (i, pos) in desired_positions.iter_mut().enumerate() {
                    if i > 0 {
                        *pos =
                            base + direction * chain.left_chain.lengths.iter().take(i).sum::<f32>();
                    }
                }
            } else {
                // FABRIK algorithm with pole vector as a soft constraint
                let iterations = 30;
                for i in 0..iterations {
                    // Forward pass
                    desired_positions[len - 1] = left_target;
                    for i in (1..len).rev() {
                        let dir = (desired_positions[i - 1] - desired_positions[i]).normalize();
                        desired_positions[i - 1] =
                            desired_positions[i] + dir * chain.left_chain.lengths[i - 1];
                    }

                    let weight = 1.0;

                    /*apply_pole_vector_constraint(
                        &mut desired_positions,
                        ik.pole_vector_right,
                        &chain.left_chain,
                        weight,
                    );*/

                    // Apply pole vector constraint after forward pass

                    // Backward pass
                    desired_positions[0] = base;
                    for i in 1..len {
                        let dir = (desired_positions[i] - desired_positions[i - 1]).normalize();
                        desired_positions[i] =
                            desired_positions[i - 1] + dir * chain.left_chain.lengths[i - 1];
                    }

                    /*apply_pole_vector_constraint(
                        &mut desired_positions,
                        ik.pole_vector_left,
                        &chain.left_chain,
                        weight,
                    );*/

                    // Apply pole vector constraint after backward pass
                }
            }

            // Update rotations
            for i in 0..len - 1 {
                let joint = chain.left_chain.joints[i];
                if let Ok(mut transform) = transforms.get_mut(joint) {
                    let current_global_transform = globals.get(joint).unwrap();
                    let current_position = current_global_transform.translation();
                    let current_rotation =
                        current_global_transform.to_scale_rotation_translation().1;

                    let desired_dir = (desired_positions[i + 1] - desired_positions[i]).normalize();
                    let current_dir = (positions[i + 1] - positions[i]).normalize();

                    let rotation_needed = Quat::from_rotation_arc(current_dir, desired_dir);

                    // Convert rotation to local space
                    if let Ok(parent) = parents.get(joint) {
                        let parent_global_rotation = globals
                            .get(parent.get())
                            .unwrap()
                            .to_scale_rotation_translation()
                            .1;
                        let local_rotation =
                            parent_global_rotation.inverse() * rotation_needed * current_rotation;
                        transform.rotation = local_rotation;
                        let mut temp = globals.get(joint).unwrap().clone().compute_transform();
                        temp.rotation = rotation_needed * current_rotation;
                        *globals.get_mut(joint).unwrap() = GlobalTransform::from(temp);
                    } else {
                        transform.rotation = rotation_needed * current_rotation;
                    }
                }
            }
        }
    }
}

fn reset_rotations(
    mut query: Query<(&HumanoidIK, &HumanoidIkChain)>,
    mut transforms: Query<&mut Transform>,
    mut globals: Query<&mut GlobalTransform>,
    rests: Query<&RestPose>,
) {
    for (_, ik_chain) in query.iter() {
        for joint in ik_chain.left_chain.joints.iter() {
            let rest = rests.get(*joint).unwrap();
            *transforms.get_mut(*joint).unwrap().rotation = *rest.local;
            let mut temp = globals.get(*joint).unwrap().compute_transform();
            temp.rotation = rest.global;
            *globals.get_mut(*joint).unwrap() = GlobalTransform::from(temp);
        }
        for joint in ik_chain.right_chain.joints.iter() {
            let rest = rests.get(*joint).unwrap();
            *transforms.get_mut(*joint).unwrap().rotation = *rest.local;
            let mut temp = globals.get(*joint).unwrap().compute_transform();
            temp.rotation = rest.global;
            *globals.get_mut(*joint).unwrap() = GlobalTransform::from(temp);
        }
    }
}

#[derive(Resource)]
pub struct RunHumanoidIk(pub bool);

impl PartialEq for RunHumanoidIk {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub struct HumanoidIKPlugin;

impl Plugin for HumanoidIKPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RunHumanoidIk(true));
        //app.add_systems(Update, modify_ik_state);
        app.add_systems(Update, setup_ik_system).add_systems(
            Update,
            (
                propagate_transforms,
                reset_rotations,
                propagate_transforms,
                humanoid_ik_system,
            )
                .chain()
                .after(animation::weights::play_avatar_animations)
                .run_if(resource_equals::<RunHumanoidIk>(RunHumanoidIk(true))),
        );
    }
}

pub fn modify_ik_state(
    status: Option<Res<XrState>>,
    mut run_humanoid_ik: ResMut<RunHumanoidIk>,
    mut commands: Commands,
    avatars: Query<(Entity, &AvatarAnimationClips), With<Handle<AnimationGraph>>>,
) {
    let last = run_humanoid_ik.0;
    if let Some(status) = status {
        match status.as_ref() {
            XrState::Ready | XrState::Running => {
                run_humanoid_ik.0 = true;
            }
            _ => {
                run_humanoid_ik.0 = false;
            }
        }
    }
    if run_humanoid_ik.0 != last {
        println!("changed: {}", run_humanoid_ik.0);
        for (e, _) in avatars.iter() {
            commands.entity(e).remove::<Handle<AnimationGraph>>();
        }
    }
}

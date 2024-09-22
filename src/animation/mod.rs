use bevy::prelude::*;

pub mod defaults;
pub(crate) mod load;
mod mixamo;
pub(crate) mod weights;

pub use load::AvatarAnimationNodes;
use weights::{AnimationWeights, TargetAnimationWeights};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum AnimationName {
    Falling,
    #[default]
    Idle,
    Walk,
    WalkLeft,
    WalkRight,
    Other(String),
}

pub(crate) fn init_animations(
    animation_nodes: Query<&Handle<AnimationGraph>, With<AvatarAnimationNodes>>,
    mut animation_players: Query<(Entity, &Parent), Added<AnimationPlayer>>,
    mut commands: Commands,
) {
    for (entity, parent) in animation_players.iter_mut() {
        let Ok(graph) = animation_nodes.get(parent.get()) else {
            continue;
        };

        commands.entity(entity).insert((
            AnimationWeights::default(),
            TargetAnimationWeights::default(),
            graph.clone(),
        ));
    }
}

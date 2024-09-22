use bevy::prelude::*;
use bevy_vrm::{first_person::SetupFirstPerson, loader::Vrm};

#[derive(Component)]
pub struct FirstPerson;

pub(crate) fn setup_first_person(
    avatars: Query<(Entity, &Handle<Vrm>), With<FirstPerson>>,
    mut events: EventReader<AssetEvent<Vrm>>,
    mut writer: EventWriter<SetupFirstPerson>,
) {
    for event in events.read() {
        if let AssetEvent::LoadedWithDependencies { id } = event {
            let (ent, _) = avatars
                .iter()
                .find(|(_, handle)| handle.id() == *id)
                .expect("Avatar not found");

            writer.send(SetupFirstPerson(ent));
        }
    }
}

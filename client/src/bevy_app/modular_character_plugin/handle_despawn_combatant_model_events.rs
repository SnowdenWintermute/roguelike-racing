use super::spawn_combatant::CombatantMainArmatureEntityLink;
use super::spawn_combatant::MainSkeletonEntity;
use super::CombatantsById;
use crate::comm_channels::DespawnCombatantModelEvent;
use bevy::prelude::*;

pub fn handle_despawn_combatant_model_events(
    mut commands: Commands,
    mut despawn_character_event_reader: EventReader<DespawnCombatantModelEvent>,
    mut combatants_by_id: ResMut<CombatantsById>,
    armatures: Query<&CombatantMainArmatureEntityLink>,
    skeletons: Query<&MainSkeletonEntity>,
) {
    for event in despawn_character_event_reader.read() {
        let combatant_id = event.0;
        info!("reading despawn character event: {:?}", combatant_id);
        if let Some(character) = combatants_by_id.0.remove(&combatant_id) {
            if let Ok(armature) = armatures.get(character) {
                commands.entity(armature.0).despawn_recursive();
            }
            if let Ok(skeleton) = skeletons.get(character) {
                commands.entity(skeleton.0).despawn_recursive();
            }
            commands.entity(character).despawn_recursive();
        };
    }
}

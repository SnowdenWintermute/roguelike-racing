use super::{ActiveModelActions, ModelActionQueue};
use crate::bevy_app::{
    modular_character_plugin::{
        spawn_combatant::{CombatantEquipment, CombatantSpeciesComponent, MainSkeletonEntity},
        Animations, StartNextModelActionEvent,
    },
    utils::link_animations::AnimationEntityLink,
};
use bevy::prelude::*;

pub fn handle_start_next_model_action_events(
    mut start_next_model_action_event_reader: EventReader<StartNextModelActionEvent>,
    mut combatants: Query<(
        &mut ActiveModelActions,
        &mut ModelActionQueue,
        &MainSkeletonEntity,
        &CombatantSpeciesComponent,
        &CombatantEquipment,
    )>,
    animations: Res<Animations>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animation_player_links: Query<&AnimationEntityLink>,
) {
    for event in start_next_model_action_event_reader.read() {
        let StartNextModelActionEvent {
            entity,
            transition_duration_ms,
        } = event;

        let (mut active_model_actions, mut model_action_queue, skeleton_entity, species, equipment) =
            combatants
                .get_mut(*entity)
                .expect("entity to have a model action queue");

        model_action_queue.start_next_model_action(
            &mut active_model_actions,
            &animation_player_links,
            &mut animation_players,
            &animations,
            skeleton_entity.0,
            &species.0,
            &equipment.0,
            *transition_duration_ms,
        );
    }
}

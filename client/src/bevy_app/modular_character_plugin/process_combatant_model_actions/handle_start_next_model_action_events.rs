use super::ActiveModelActions;
use super::ModelActionQueue;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantPropertiesComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantSpeciesComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonEntity;
use crate::bevy_app::modular_character_plugin::Animations;
use crate::bevy_app::modular_character_plugin::StartNextModelActionEvent;
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use bevy::prelude::*;

pub fn handle_start_next_model_action_events(
    mut start_next_model_action_event_reader: EventReader<StartNextModelActionEvent>,
    mut combatants: Query<(
        &mut ActiveModelActions,
        &mut ModelActionQueue,
        &MainSkeletonEntity,
        &CombatantPropertiesComponent,
    )>,
    animations: Res<Animations>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animation_player_links: Query<&AnimationEntityLink>,
    species_query: Query<&CombatantSpeciesComponent>,
) {
    for event in start_next_model_action_event_reader.read() {
        let StartNextModelActionEvent {
            entity,
            transition_duration_ms,
        } = event;

        let (
            mut active_model_actions,
            mut model_action_queue,
            skeleton_entity,
            combatant_properties_component,
        ) = combatants
            .get_mut(*entity)
            .expect("entity to have a model action queue");

        let species = species_query
            .get(skeleton_entity.0)
            .expect("the skeleton to have a species");

        model_action_queue.start_next_model_action(
            &mut active_model_actions,
            &animation_player_links,
            &mut animation_players,
            &animations,
            skeleton_entity.0,
            &species.0,
            &combatant_properties_component.0,
            *transition_duration_ms,
        );
    }
}

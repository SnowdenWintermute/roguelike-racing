use super::get_percent_animation_completed::get_percent_animation_completed;
use super::model_actions::get_animation_name_from_model_action;
use super::model_actions::CombatantModelActions;
use super::process_active_model_actions::ModelActionCombatantQueryStructItem;
use super::process_active_model_actions::ModelActionSystemParams;
use crate::bevy_app::modular_character_plugin::process_combatant_model_actions::handle_new_attack_reaction_events::AttackResult;
use crate::bevy_app::modular_character_plugin::StartNewAttackReactionEvent;
use crate::bevy_app::modular_character_plugin::StartNextModelActionEvent;
use bevy::math::u64;
use bevy::prelude::*;

pub const UNKNOWN_ANIMATION_DURATION: u64 = 500;
pub const MH_MELEE_ANIMATION_DURATION_TRANSITION_THRESHOLD: f32 = 0.65;

pub fn attacking_with_melee_main_hand_processor(
    entity: Entity,
    elapsed: u64,
    transition_started: bool,
    model_action_params: &mut ModelActionSystemParams,
    start_next_model_action_event_writer: &mut EventWriter<StartNextModelActionEvent>,
    start_new_attack_reaction_event_writer: &mut EventWriter<StartNewAttackReactionEvent>,
) {
    let ModelActionCombatantQueryStructItem {
        skeleton_entity,
        equipment,
        species_component,
        ..
    } = model_action_params
        .combatants_query
        .get_mut(entity)
        .expect("to have this entity in the query");
    // check percent completed of animation
    let percent_completed = if let Some(animation_name) = get_animation_name_from_model_action(
        &species_component.0,
        &CombatantModelActions::AttackMeleeMainHand,
        &equipment.0,
    ) {
        get_percent_animation_completed(
            &skeleton_entity.0,
            &model_action_params.animation_player_links,
            &model_action_params.animation_players,
            &model_action_params.animations,
            &model_action_params.assets_animation_clips,
            &animation_name,
        )
    } else {
        elapsed as f32 / UNKNOWN_ANIMATION_DURATION as f32
    };

    if percent_completed > MH_MELEE_ANIMATION_DURATION_TRANSITION_THRESHOLD && !transition_started {
        info!("starting transition from mh melee attack");
        start_next_model_action_event_writer.send(StartNextModelActionEvent {
            entity,
            transition_duration_ms: 500,
        });
        let mut combatant = model_action_params
            .combatants_query
            .get_mut(entity)
            .expect("to have the combatant");
        combatant
            .active_model_actions
            .0
            .get_mut(&CombatantModelActions::AttackMeleeMainHand)
            .expect("this model action to be active")
            .transition_started = true;

        let current_action = combatant
            .action_results_manager
            .current_action_result_processing
            .as_ref()
            .expect("to have a current action result processing");

        if let Some(hp_changes) = &current_action.hp_changes_by_entity_id {
            for (entity_id, hp_change) in hp_changes {
                start_new_attack_reaction_event_writer.send(StartNewAttackReactionEvent {
                    entity_id: *entity_id,
                    attack_result: AttackResult::HpChange(*hp_change),
                });
            }
        }
        if let Some(misses) = &current_action.misses_by_entity_id {
            for entity_id in misses {
                start_new_attack_reaction_event_writer.send(StartNewAttackReactionEvent {
                    entity_id: *entity_id,
                    attack_result: AttackResult::Evade,
                });
            }
        }
    }

    if percent_completed >= 1.0 {
        let mut combatant = model_action_params
            .combatants_query
            .get_mut(entity)
            .expect("to have the combatant");
        combatant
            .active_model_actions
            .0
            .remove(&CombatantModelActions::AttackMeleeMainHand);
    }
}

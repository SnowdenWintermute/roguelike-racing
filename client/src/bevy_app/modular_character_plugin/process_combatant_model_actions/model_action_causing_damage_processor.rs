use super::get_percent_animation_completed::get_percent_animation_completed;
use super::model_actions::get_animation_name_from_model_action;
use super::model_actions::CombatantModelActions;
use super::process_active_model_actions::ModelActionCombatantQueryStructItem;
use super::process_active_model_actions::ModelActionSystemParams;
use crate::bevy_app::bevy_app_consts::UNKNOWN_ANIMATION_DURATION;
use crate::bevy_app::modular_character_plugin::process_combatant_model_actions::handle_new_attack_reaction_events::AttackResult;
use crate::bevy_app::modular_character_plugin::StartNewAttackReactionEvent;
use crate::bevy_app::modular_character_plugin::StartNextModelActionEvent;
use crate::comm_channels::messages_from_bevy::CombatantIdWithValue;
use crate::comm_channels::messages_from_bevy::MessageFromBevy;
use crate::comm_channels::BevyTransmitter;
use bevy::math::u64;
use bevy::prelude::*;

pub const MH_MELEE_ANIMATION_DURATION_TRANSITION_THRESHOLD: f32 = 0.55;

pub fn model_action_causing_damage_processor(
    entity: Entity,
    elapsed: u64,
    transition_started: bool,
    model_action_params: &mut ModelActionSystemParams,
    start_next_model_action_event_writer: &mut EventWriter<StartNextModelActionEvent>,
    start_new_attack_reaction_event_writer: &mut EventWriter<StartNewAttackReactionEvent>,
    model_action: &CombatantModelActions,
    bevy_transmitter: &mut ResMut<BevyTransmitter>,
) {
    let ModelActionCombatantQueryStructItem {
        skeleton_entity,
        combatant_properties_component,
        ..
    } = model_action_params
        .combatants_query
        .get_mut(entity)
        .expect("to have this entity in the query");
    let species_component = model_action_params
        .species_query
        .get(skeleton_entity.0)
        .expect("the skeleton to have a species");
    // check percent completed of animation
    let percent_completed = if let Some(animation_name) = get_animation_name_from_model_action(
        &species_component.0,
        model_action,
        &combatant_properties_component.0,
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
            .get_mut(&model_action)
            .expect("this model action to be active")
            .transition_started = true;

        let current_action = combatant
            .action_results_processing
            .0
            .pop()
            .expect("to have a current action result processing");

        if let Some(mp_changes) = &current_action.mp_combat_action_prices_paid_by_entity_id {
            for (entity_id, mp_change) in mp_changes {
                info!("sending mp change to yew {mp_change}");
                let _result =
                    bevy_transmitter
                        .0
                        .send(MessageFromBevy::MpChangeById(CombatantIdWithValue {
                            combatant_id: *entity_id,
                            value: *mp_change as i16 * -1,
                        }));
            }
        }

        if let Some(hp_changes) = &current_action.hp_changes_by_entity_id {
            for (entity_id, hp_change) in hp_changes {
                start_new_attack_reaction_event_writer.send(StartNewAttackReactionEvent {
                    entity_id: *entity_id,
                    attack_result: AttackResult::HpChange(*hp_change),
                    causer_id: combatant.combatant_id_component.0,
                });
            }
        }
        if let Some(misses) = &current_action.misses_by_entity_id {
            for entity_id in misses {
                start_new_attack_reaction_event_writer.send(StartNewAttackReactionEvent {
                    entity_id: *entity_id,
                    attack_result: AttackResult::Evade,
                    causer_id: combatant.combatant_id_component.0,
                });
            }
        }
    }

    if percent_completed >= 1.0 {
        let mut combatant = model_action_params
            .combatants_query
            .get_mut(entity)
            .expect("to have the combatant");
        combatant.active_model_actions.0.remove(&model_action);
    }
}

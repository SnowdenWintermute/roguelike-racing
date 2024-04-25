use super::model_actions::CombatantModelActions;
use super::process_active_model_actions::ModelActionSystemParams;
use super::process_floating_text::FLOATING_TEXT_TIME_TO_LIVE_DEFAULT;
use crate::bevy_app::modular_character_plugin::StartNewAttackReactionEvent;
use crate::bevy_app::modular_character_plugin::StartNewFloatingTextEvent;
use crate::comm_channels::messages_from_bevy::CombatantIdWithValue;
use crate::comm_channels::messages_from_bevy::MessageFromBevy;
use crate::comm_channels::BevyTransmitter;
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub enum AttackResult {
    HpChange(i16),
    Evade,
}

pub fn handle_new_attack_reaction_events(
    mut start_new_attack_reactions_event_reader: EventReader<StartNewAttackReactionEvent>,
    bevy_transmitter: ResMut<BevyTransmitter>,
    mut model_action_params: ModelActionSystemParams,
    mut start_new_floating_text_event_writer: EventWriter<StartNewFloatingTextEvent>,
) {
    for event in start_new_attack_reactions_event_reader.read() {
        let StartNewAttackReactionEvent {
            entity_id,
            attack_result,
            causer_id,
        } = event;

        let target_entity = model_action_params
            .combatants_by_id
            .0
            .get(entity_id)
            .expect("to have registered the entity");
        let mut target_combatant = model_action_params
            .combatants_query
            .get_mut(*target_entity)
            .expect("combatant entity to have an animation manager");

        let (text, color) = match attack_result {
            AttackResult::HpChange(number) => {
                target_combatant
                    .combatant_properties_component
                    .0
                    .change_hp(*number);
                let new_model_action =
                    if target_combatant.combatant_properties_component.0.hit_points == 0 {
                        CombatantModelActions::Death
                    } else {
                        CombatantModelActions::HitRecovery
                    };
                // start hit recovery model action
                target_combatant
                    .model_action_queue
                    .0
                    .push_back(new_model_action);
                // return text to float
                (
                    number.abs().to_string(),
                    if *number > 0 {
                        Vec3::new(0.0, 1.0, 0.0)
                    } else {
                        Vec3::new(1.0, 1.0, 1.0)
                    },
                )
            }
            AttackResult::Evade => {
                // start evade model action
                target_combatant
                    .model_action_queue
                    .0
                    .push_back(CombatantModelActions::Evade);
                (String::from("Evaded"), Vec3::new(1.0, 1.0, 1.0))
            }
        };
        start_new_floating_text_event_writer.send(StartNewFloatingTextEvent {
            combatant_entity: *target_entity,
            text,
            color,
            distance_to_travel: 1.5,
            time_to_live: FLOATING_TEXT_TIME_TO_LIVE_DEFAULT as u64,
        });
    }
}

use super::BevyReceiver;
use super::BevyTransmitter;
use super::CharacterPartSelectionEvent;
use super::CharacterSpawnEvent;
use super::DespawnCombatantModelEvent;
use super::MessageFromYew;
use super::StartAttackSequenceEvent;
use super::YewTransmitter;
use crate::bevy_app::modular_character_plugin::TurnResultsQueue;
use bevy::prelude::*;

pub struct CommChannelPlugin {
    bevy_transmitter: BevyTransmitter,
    yew_transmitter: YewTransmitter,
}

impl CommChannelPlugin {
    pub fn new(bevy_transmitter: BevyTransmitter, yew_transmitter: YewTransmitter) -> Self {
        CommChannelPlugin {
            bevy_transmitter,
            yew_transmitter,
        }
    }
}

impl Plugin for CommChannelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BevyReceiver(self.yew_transmitter.subscribe()))
            .insert_resource(self.bevy_transmitter.clone())
            .init_resource::<Events<CharacterPartSelectionEvent>>()
            .init_resource::<Events<CharacterSpawnEvent>>()
            .init_resource::<Events<DespawnCombatantModelEvent>>()
            .init_resource::<Events<StartAttackSequenceEvent>>()
            .add_systems(PreUpdate, handle_yew_messages);
    }
}

fn handle_yew_messages(
    mut bevy_receiver: ResMut<BevyReceiver>,
    mut part_selection_event_writer: EventWriter<CharacterPartSelectionEvent>,
    mut spawn_combatant_event_writer: EventWriter<CharacterSpawnEvent>,
    mut select_animation_event_writer: EventWriter<DespawnCombatantModelEvent>,
    mut attack_sequence_event_writer: EventWriter<StartAttackSequenceEvent>,
    mut turn_results_queue: ResMut<TurnResultsQueue>,
) {
    if let Ok(message_from_yew) = bevy_receiver.try_recv() {
        match message_from_yew {
            MessageFromYew::SelectCharacterPart(part_selection) => {
                part_selection_event_writer.send(CharacterPartSelectionEvent(part_selection));
            }
            MessageFromYew::SpawnCharacterWithHomeLocation(
                character_id,
                home_location,
                species,
                equipment,
            ) => {
                spawn_combatant_event_writer.send(CharacterSpawnEvent(
                    character_id,
                    home_location,
                    species,
                    equipment,
                ));
            }
            MessageFromYew::DespawnCombatantModel(combatant_id) => {
                select_animation_event_writer.send(DespawnCombatantModelEvent(combatant_id));
            }
            MessageFromYew::ExecuteAttackSequence(attack_command) => {
                attack_sequence_event_writer.send(StartAttackSequenceEvent(attack_command));
            }
            MessageFromYew::NewTurnResults(mut turn_results) => {
                turn_results_queue.0.append(&mut turn_results);
            }
        }
    }
}

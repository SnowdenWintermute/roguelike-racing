use super::messages_from_yew::MessageFromYew;
use super::BevyReceiver;
use super::BevyTransmitter;
use super::CharacterPartSelectionEvent;
use super::CharacterSpawnEvent;
use super::CombatantItemEvent;
use super::CombatantItemEvents;
use super::DespawnCombatantModelEvent;
use super::ProcessNextTurnResultEvent;
use super::YewTransmitter;
use crate::bevy_app::modular_character_plugin::RawActionResultsQueue;
use crate::bevy_app::modular_character_plugin::TurnResultsQueue;
use crate::bevy_app::BevyAppState;
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
            .init_resource::<Events<ProcessNextTurnResultEvent>>()
            .init_resource::<Events<CombatantItemEvent>>()
            .add_systems(PreUpdate, handle_yew_messages);
    }
}

fn handle_yew_messages(
    mut bevy_receiver: ResMut<BevyReceiver>,
    mut part_selection_event_writer: EventWriter<CharacterPartSelectionEvent>,
    mut spawn_combatant_event_writer: EventWriter<CharacterSpawnEvent>,
    mut select_animation_event_writer: EventWriter<DespawnCombatantModelEvent>,
    mut process_next_turn_result_event_writer: EventWriter<ProcessNextTurnResultEvent>,
    mut combatant_item_event_writer: EventWriter<CombatantItemEvent>,
    mut turn_results_queue: ResMut<TurnResultsQueue>,
    mut raw_action_results_queue: ResMut<RawActionResultsQueue>,
    mut next_state: ResMut<NextState<BevyAppState>>,
    // mut camera_query: Query<&mut Camera>,
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
                combatant_properties,
            ) => {
                spawn_combatant_event_writer.send(CharacterSpawnEvent(
                    character_id,
                    home_location,
                    species,
                    combatant_properties,
                ));
            }
            MessageFromYew::DespawnCombatantModel(combatant_id) => {
                select_animation_event_writer.send(DespawnCombatantModelEvent(combatant_id));
            }
            MessageFromYew::NewTurnResults(mut turn_results) => {
                turn_results_queue.0.append(&mut turn_results);
                process_next_turn_result_event_writer.send(ProcessNextTurnResultEvent(None));
            }
            MessageFromYew::SetBevyRendering(should_be_rendering) => match should_be_rendering {
                true => next_state.set(BevyAppState::Running),
                false => next_state.set(BevyAppState::PausedAndHidden),
            },
            MessageFromYew::NewRawActionResults(action_taker_id, action_results) => {
                raw_action_results_queue
                    .0
                    .push_back((action_taker_id, action_results))
            }
            MessageFromYew::CombatantPickedUpItem(combatant_id, item) => {
                combatant_item_event_writer.send(CombatantItemEvent {
                    combatant_id,
                    event_type: CombatantItemEvents::PickedUp(item),
                });
            }
            MessageFromYew::CombatantDroppedItem(combatant_id, item_id) => {
                combatant_item_event_writer.send(CombatantItemEvent {
                    combatant_id,
                    event_type: CombatantItemEvents::Dropped(item_id),
                });
            }
            MessageFromYew::CombatantEquippedItem(combatant_id, item_id, equip_to_alt_slot) => {
                combatant_item_event_writer.send(CombatantItemEvent {
                    combatant_id,
                    event_type: CombatantItemEvents::Equipped(item_id, equip_to_alt_slot),
                });
            }
        }
    }
}

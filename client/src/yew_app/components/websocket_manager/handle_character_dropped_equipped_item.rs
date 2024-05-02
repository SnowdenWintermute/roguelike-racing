use super::send_client_input::send_client_input;
use crate::comm_channels::messages_from_yew::MessageFromYew;
use crate::yew_app::components::bevy_messages_manager::send_message_to_bevy::send_message_to_bevy;
use crate::yew_app::store::bevy_communication_store::BevyCommunicationStore;
use crate::yew_app::store::game_store::GameStore;
use crate::yew_app::store::websocket_store::WebsocketStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::packets::client_to_server::PlayerInputs;
use common::packets::CharacterAndSlot;
use yewdux::Dispatch;

pub fn handle_character_dropped_equipped_item(
    game_dispatch: Dispatch<GameStore>,
    websocket_dispatch: Dispatch<WebsocketStore>,
    bevy_communication_dispatch: Dispatch<BevyCommunicationStore>,
    packet: CharacterAndSlot,
) -> Result<(), AppError> {
    let item_id_result = game_dispatch.reduce_mut(|store| -> Result<u32, AppError> {
        let character = store.get_mut_character(packet.character_id)?;
        let item = character
            .combatant_properties
            .equipment
            .remove(&packet.slot)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::DROP_EQUIPPED_ITEM_SERVER_PACKET_MISMATCH.to_string(),
            })?;
        let item_id = item.entity_properties.id;
        let party = store.get_current_party_mut()?;
        party.current_room.items.push(item);
        Ok(item_id)
    });

    match item_id_result {
        Ok(item_id) => {
            websocket_dispatch.reduce_mut(|store| {
                let websocket_option = &store.websocket;
                send_client_input(
                    &websocket_option,
                    PlayerInputs::AcknowledgeReceiptOfItemOnGroundUpdate(item_id),
                );
            });
            bevy_communication_dispatch.reduce_mut(|store| {
                send_message_to_bevy(
                    &store.transmitter_option,
                    MessageFromYew::CombatantDroppedEquippedItem(packet.character_id, packet.slot),
                )
            })
        }
        Err(err) => Err(err),
    }
}

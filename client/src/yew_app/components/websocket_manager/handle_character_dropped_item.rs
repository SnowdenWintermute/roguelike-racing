use super::send_client_input::send_client_input;
use crate::comm_channels::messages_from_yew::MessageFromYew;
use crate::yew_app::components::bevy_messages_manager::send_message_to_bevy::send_message_to_bevy;
use crate::yew_app::store::bevy_communication_store::BevyCommunicationStore;
use crate::yew_app::store::game_store::GameStore;
use crate::yew_app::store::websocket_store::WebsocketStore;
use common::errors::AppError;
use common::packets::client_to_server::PlayerInputs;
use common::packets::CharacterAndItem;
use yewdux::Dispatch;

pub fn handle_character_dropped_item(
    game_dispatch: Dispatch<GameStore>,
    websocket_dispatch: Dispatch<WebsocketStore>,
    bevy_communication_dispatch: Dispatch<BevyCommunicationStore>,
    packet: CharacterAndItem,
) -> Result<(), AppError> {
    websocket_dispatch.reduce_mut(|store| {
        let websocket_option = &store.websocket;
        send_client_input(
            &websocket_option,
            PlayerInputs::AcknowledgeReceiptOfItemOnGroundUpdate(packet.item_id),
        );
    });

    bevy_communication_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        send_message_to_bevy(
            &store.transmitter_option,
            MessageFromYew::CombatantDroppedItem(packet.character_id, packet.item_id),
        )
    })?;

    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let character = store.get_mut_character(packet.character_id)?;
        let item = character
            .combatant_properties
            .inventory
            .remove_item(packet.item_id)?;
        let party = store.get_current_party_mut()?;
        party.current_room.items.push(item);
        Ok(())
    })
}

use super::send_client_input::send_client_input;
use crate::store::game_store::GameStore;
use crate::store::websocket_store::WebsocketStore;
use common::errors::AppError;
use common::packets::client_to_server::PlayerInputs;
use common::packets::CharacterAndItem;
use yewdux::Dispatch;

pub fn handle_character_dropped_item(
    game_dispatch: Dispatch<GameStore>,
    websocket_dispatch: Dispatch<WebsocketStore>,
    packet: CharacterAndItem,
) -> Result<(), AppError> {
    websocket_dispatch.reduce_mut(|store| {
        let websocket_option = &store.websocket;
        send_client_input(
            &websocket_option,
            PlayerInputs::AcknowledgeReceiptOfItemOnGroundUpdate(packet.item_id),
        );
    });

    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let character = store.get_mut_character(packet.character_id)?;
        let item = character.combatant_properties.inventory.remove_item(packet.item_id)?;
        let party = store.get_current_party_mut()?;
        party.current_room.items.push(item);
        Ok(())
    })
}

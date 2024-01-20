use super::send_client_input::send_client_input;
use crate::store::game_store::GameStore;
use crate::store::websocket_store::WebsocketStore;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use common::packets::client_to_server::PlayerInputs;
use common::packets::server_to_client::BattleEndReportPacket;
use yewdux::Dispatch;

pub fn handle_battle_end_report(
    game_dispatch: Dispatch<GameStore>,
    websocket_dispatch: Dispatch<WebsocketStore>,
    packet: BattleEndReportPacket,
) -> Result<(), AppError> {
    websocket_dispatch.reduce_mut(|store| {
        let websocket_option = &store.websocket;
        if let Some(items) = &packet.loot {
            for item in items {
                send_client_input(
                    &websocket_option,
                    PlayerInputs::AcknowledgeReceiptOfItemOnGroundUpdate(item.entity_properties.id),
                );
            }
        }
    });
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        store.current_battle_end_report = Some(packet.clone());
        //
        let party = store.get_current_party_mut()?;
        if let Some(items) = &mut party.current_room.items {
            if let Some(mut loot) = packet.loot {
                items.append(&mut loot)
            }
        } else {
            party.current_room.items = packet.loot
        }

        Ok(())
    })
}

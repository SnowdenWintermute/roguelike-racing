use super::send_client_input::send_client_input;
use crate::yew_app::store::game_store::GameStore;
use crate::yew_app::store::websocket_store::WebsocketStore;
use common::errors::AppError;
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
        if let Some(mut loot) = packet.loot {
            party.current_room.items.append(&mut loot)
        }

        for (_, character) in party.characters.iter_mut() {
            // @TODO - remove this when revive is implemented
            if character.combatant_properties.hit_points == 0 {
                character.combatant_properties.hit_points = 1;
            }
        }

        Ok(())
    })
}

use crate::websocket_server::game_server::GameServer;
use crate::websocket_server::game_server::getters::ActorIdAssociatedGameData;
use crate::websocket_server::game_server::getters::get_mut_game_data_from_actor_id;
use crate::websocket_server::game_server::player_input_handlers::in_game::handle_new_combat_action_results::generate_loot_if_appropriate::generate_loot;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use common::packets::server_to_client::BattleConclusion;
use common::packets::server_to_client::BattleEndReportPacket;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn handle_party_battle_victory(
        &mut self,
        actor_id: u32,
        battle_id: u32,
        num_opponents: u8,
    ) -> Result<(), AppError> {
        let ActorIdAssociatedGameData { game, party_id, .. } =
            get_mut_game_data_from_actor_id(self, actor_id)?;
        let party = get_mut_party(game, party_id)?;
        let party_websocket_channel_name = party.websocket_channel_name.clone();
        let dlvl = party.current_floor;
        let mut loot = generate_loot(game, num_opponents, dlvl);

        let party = get_mut_party(game, party_id)?;
        // make sure all clients receive the item's existance or else one client can take
        // the item before another client sees it leading to desync
        for item in &loot {
            party
                .items_on_ground_not_yet_received_by_all_clients
                .insert(item.entity_properties.id, vec![]);
        }

        // REMOVE THE BATTLE
        println!("all opponents defeated, concluding battle as victory");
        party.battle_id = None;
        party.current_room.monsters = None;
        party.current_room.items.append(&mut loot);
        game.battles.remove(&battle_id);

        self.emit_packet(
            &party_websocket_channel_name,
            &WebsocketChannelNamespace::Party,
            &GameServerUpdatePackets::BattleEndReport(BattleEndReportPacket {
                conclusion: BattleConclusion::Victory,
                loot: Some(loot),
            }),
            None,
        )
    }
}

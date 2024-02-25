use crate::websocket_server::game_server::getters::get_mut_game_data_from_actor_id;
use crate::websocket_server::game_server::getters::ActorIdAssociatedGameData;
use crate::websocket_server::game_server::GameServer;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use common::packets::server_to_client::BattleConclusion;
use common::packets::server_to_client::BattleEndReportPacket;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;
use common::utils::server_log;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

impl GameServer {
    pub fn handle_party_wipe(
        &mut self,
        actor_id: u32,
        party_websocket_channel_name: &String,
        battle_id_option: &Option<u32>,
    ) -> Result<(), AppError> {
        let ActorIdAssociatedGameData { game, party_id, .. } =
            get_mut_game_data_from_actor_id(self, actor_id)?;
        let all_players_in_game = game.players.clone();
        let party = get_mut_party(game, party_id)?;
        let party_name = party.name.clone();
        let dlvl = party.current_floor;
        let player_usernames_in_party = party.player_usernames.clone();
        party.battle_id = None;
        if let Some(battle_id) = battle_id_option {
            game.battles.remove(&battle_id);
        }

        let mut actor_ids_of_players_in_other_parties = Vec::new();
        for (username, player) in &all_players_in_game {
            if !player_usernames_in_party.contains(username) {
                actor_ids_of_players_in_other_parties.push(player.actor_id)
            }
        }

        for actor_id_option in actor_ids_of_players_in_other_parties {
            if let Some(actor_id) = actor_id_option {
                self.send_packet(
                    &GameServerUpdatePackets::GameMessage(
                        common::packets::GameMessages::PartyWipe(
                            party_name.clone(),
                            dlvl,
                            SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .expect("time went backwards")
                                .as_secs(),
                        ),
                    ),
                    actor_id,
                )?;
            }
        }

        self.emit_packet(
            &party_websocket_channel_name,
            &WebsocketChannelNamespace::Party,
            &GameServerUpdatePackets::BattleEndReport(BattleEndReportPacket {
                conclusion: BattleConclusion::Defeat,
                loot: None,
                exp_changes: None,
            }),
            None,
        )?;

        let ActorIdAssociatedGameData { game, .. } =
            get_mut_game_data_from_actor_id(self, actor_id)?;
        for username in player_usernames_in_party {
            game.remove_player_from_adventuring_party(username)?;
        }

        if game.adventuring_parties.len() == 0 {
            for (_, player) in all_players_in_game {
                if let Some(actor_id) = player.actor_id {
                    let result = self.remove_player_from_game(actor_id);
                    if let Some(err) = result.err() {
                        server_log(&format!("error removing player from game {}", err.message))
                    }
                }
            }
        }
        Ok(())
    }
}

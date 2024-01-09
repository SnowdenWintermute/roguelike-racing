use crate::websocket_server::game_server::getters::get_mut_game;
use crate::websocket_server::game_server::getters::get_mut_user;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::game::getters::get_mut_player;
use common::packets::server_to_client::AdventuringPartyCreation;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn create_adventuring_party_handler(
        &mut self,
        actor_id: u32,
        party_name: String,
    ) -> Result<(), AppError> {
        let connected_user = get_mut_user(&mut self.sessions, actor_id)?;

        let current_game_name =
            connected_user
                .current_game_name
                .clone()
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::MISSING_GAME_REFERENCE.to_string(),
                })?;

        let game = get_mut_game(&mut self.games, &current_game_name)?;
        let player = get_mut_player(game, &connected_user.username)?;

        if player.party_id.is_some() {
            return Err(AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::ALREADY_IN_PARTY.to_string(),
            });
        }

        let party_id = game.id_generator.get_next_entity_id();
        game.add_adventuring_party(party_name.clone(), party_id);

        self.emit_packet(
            &current_game_name,
            &WebsocketChannelNamespace::Game,
            &GameServerUpdatePackets::AdventuringPartyCreated(AdventuringPartyCreation {
                party_id,
                party_name,
            }),
            None,
        )?;

        self.join_party_handler(actor_id, party_id)
    }
}

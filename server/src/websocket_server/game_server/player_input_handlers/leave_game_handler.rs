use crate::websocket_server::game_server::getters::get_game;
use crate::websocket_server::game_server::getters::get_mut_game;
use crate::websocket_server::game_server::getters::get_mut_user;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::app_consts::LOBBY_CHANNEL;
use common::errors::AppError;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::server_to_client::PlayerRemovedFromGame;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn leave_game_handler(&mut self, actor_id: u32) -> Result<(), AppError> {
        self.leave_party_handler(actor_id)?;
        let player_and_game = self.remove_player_from_game(actor_id)?;
        self.remove_user_from_websocket_channel(
            player_and_game.game_name.as_str(),
            &WebsocketChannelNamespace::Game,
            actor_id,
        )?;
        self.join_user_to_websocket_channel(
            LOBBY_CHANNEL,
            WebsocketChannelNamespace::Lobby,
            actor_id,
        )?;
        println!("leaving game {:#?}", player_and_game);
        let game = get_game(&self.games, player_and_game.game_name.clone());
        if let Ok(_game_in_existance) = game {
            self.emit_packet(
                player_and_game.game_name.as_str(),
                &WebsocketChannelNamespace::Game,
                &GameServerUpdatePackets::UserLeftGame(player_and_game.username.clone()),
                Some(actor_id),
            )?;
        }
        self.send_packet(&GameServerUpdatePackets::GameFullUpdate(None), actor_id)
    }

    pub fn remove_player_from_game(
        self: &mut GameServer,
        actor_id: u32,
    ) -> Result<PlayerRemovedFromGame, AppError> {
        let connected_user = get_mut_user(&mut self.sessions, actor_id)?;
        println!("user leaving game: {:#?}", connected_user);
        let game_name_leaving =
            connected_user
                .current_game_name
                .clone()
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::MISSING_GAME_REFERENCE.to_string(),
                })?;

        let game = get_mut_game(&mut self.games, &game_name_leaving)?;
        game.remove_player_from_adventuring_party(connected_user.username.clone())?;

        game.players.remove(&connected_user.username.clone());
        game.players_readied
            .remove(&connected_user.username.clone());

        if game.get_number_of_players() < 1 {
            self.games.remove(&game_name_leaving);
        }

        connected_user.current_game_name = None;
        println!("removed player from game and set their current game name to None");
        Ok(PlayerRemovedFromGame {
            username: connected_user.username.clone(),
            game_name: game_name_leaving,
        })
    }
}

mod get_users_in_websocket_channel;
use super::getters::get_game;
use super::getters::get_user;
use super::GameServer;
use common::errors::AppError;
use common::game::RoguelikeRacerGame;
use common::packets::server_to_client::ClientGameListState;
use common::packets::server_to_client::GameListEntry;
use common::packets::server_to_client::RoguelikeRacerAppState;
use common::packets::server_to_client::WebsocketChannelFullState;
use common::packets::server_to_client::WebsocketChannelsState;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn create_game_list_update(&self) -> ClientGameListState {
        let mut game_list = ClientGameListState { games: Vec::new() };
        for (game_name, game) in self.games.iter() {
            game_list.games.push(GameListEntry {
                game_name: game_name.to_string(),
                number_of_users: game.get_number_of_players(),
                time_started: game.time_started,
            })
        }
        game_list
    }

    pub fn create_game_full_update(
        &self,
        actor_id: u32,
    ) -> Result<Option<RoguelikeRacerGame>, AppError> {
        let connected_user = get_user(&self.sessions, actor_id)?;
        let current_game_name = connected_user.current_game_name.clone();
        let current_game_option = match current_game_name {
            Some(game_name) => {
                let game = get_game(&self.games, game_name)?;
                Some(game.clone())
            }
            None => None,
        };

        let mut current_game = match current_game_option {
            Some(game) => Some(game),
            None => None,
        };

        // sanitize actor ids from players
        match current_game {
            Some(ref mut game) => {
                for (_, player) in game.players.iter_mut() {
                    player.actor_id = None;
                }
            }
            None => (),
        }

        Ok(current_game)
    }

    pub fn create_client_update_packet(
        &mut self,
        actor_id: u32,
    ) -> Result<RoguelikeRacerAppState, AppError> {
        let connected_user = get_user(&self.sessions, actor_id)?;
        let current_game = self.create_game_full_update(actor_id)?;

        // get all connected channels and update
        let main_channel_update = {
            let (namespace, name) = &connected_user.websocket_channels.main;
            let usernames_in_channel = self.get_users_in_websocket_channel(namespace, &name)?;
            WebsocketChannelFullState {
                name: name.clone(),
                namespace: namespace.clone(),
                usernames_in_channel,
            }
        };

        let party_channel_update = {
            if let Some(party_channel_name) = &connected_user.websocket_channels.party {
                let usernames_in_channel = self.get_users_in_websocket_channel(
                    &WebsocketChannelNamespace::Party,
                    &party_channel_name,
                )?;
                Some(WebsocketChannelFullState {
                    name: party_channel_name.to_string(),
                    namespace: WebsocketChannelNamespace::Party,
                    usernames_in_channel,
                })
            } else {
                None
            }
        };

        let game_list = self.create_game_list_update();

        let full_update = RoguelikeRacerAppState {
            websocket_channels: WebsocketChannelsState {
                main: main_channel_update,
                party: party_channel_update,
                chat: None,
            },
            game_list,
            current_game,
        };

        Ok(full_update)
    }
}

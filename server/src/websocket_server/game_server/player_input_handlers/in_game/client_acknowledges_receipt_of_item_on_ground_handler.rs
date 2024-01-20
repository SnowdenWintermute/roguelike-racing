use crate::websocket_server::game_server::getters::get_mut_game_data_from_actor_id;
use crate::websocket_server::game_server::getters::ActorIdAssociatedGameData;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::game::getters::get_mut_party;

impl GameServer {
    pub fn client_acknowledges_receipt_of_item_on_ground_handler(
        &mut self,
        actor_id: u32,
        item_id: u32,
    ) -> Result<(), AppError> {
        println!("got receipt ack");
        let ActorIdAssociatedGameData { game, party_id, .. } =
            get_mut_game_data_from_actor_id(self, actor_id)?;
        let party = get_mut_party(game, party_id)?;
        let actor_ids = party
            .items_on_ground_not_yet_received_by_all_clients
            .get_mut(&item_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::ITEM_ON_GROUND_ACKNOWLEDGEMENT_SENT_BEFORE_ITEM_EXISTED
                    .to_string(),
            })?;
        actor_ids.push(actor_id);
        let actor_ids = actor_ids.clone();
        let player_usernames_in_party = party.player_usernames.clone();
        let players = &game.players;
        let mut all_actors_in_party_have_received_item_update = true;
        for username in player_usernames_in_party {
            let player = players.get(&username).ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ServerError,
                message: error_messages::PLAYER_NOT_FOUND.to_string(),
            })?;
            if !actor_ids.contains(&player.actor_id.expect("to have an actor id")) {
                all_actors_in_party_have_received_item_update = false;
                break;
            }
        }

        if all_actors_in_party_have_received_item_update {
            let party = get_mut_party(game, party_id)?;
            party
                .items_on_ground_not_yet_received_by_all_clients
                .remove(&item_id);
        };
        Ok(())
    }
}

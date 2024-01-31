use super::character_changes_combat_action_targets_handler::character_changes_combat_action_targets_handler;
use crate::websocket_server::game_server::getters::get_mut_game;
use crate::websocket_server::game_server::getters::get_mut_party_game_name_and_character_ids_from_actor_id;
use crate::websocket_server::game_server::getters::get_user;
use crate::websocket_server::game_server::getters::ActorIdAssociatedPartyData;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages::{self};
use common::errors::AppError;
use common::errors::AppErrorTypes;
use common::game::getters::get_mut_party;
use common::game::getters::get_mut_player;
use common::packets::client_to_server::ChangeTargetsPacket;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn character_changes_consumable_targets_handler(
        &mut self,
        actor_id: u32,
        packet: ChangeTargetsPacket,
    ) -> Result<(), AppError> {
        let ChangeTargetsPacket {
            character_id,
            new_targets,
        } = packet;

        let ActorIdAssociatedPartyData {
            player_character_ids_option,
            ..
        } = get_mut_party_game_name_and_character_ids_from_actor_id(self, actor_id)?;
        let connected_user = get_user(&self.sessions, actor_id)?;
        let username = connected_user.username.clone();
        let current_game_name =
            connected_user
                .current_game_name
                .clone()
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::MISSING_GAME_REFERENCE.to_string(),
                })?;
        let game = get_mut_game(&mut self.games, &current_game_name)?;
        let player = get_mut_player(game, &username)?;
        let party_id = player.party_id.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
        })?;

        let party = get_mut_party(game, party_id)?;
        let party_websocket_channel_name = party.websocket_channel_name.clone();
        let battle_id_option = party.battle_id.clone();
        let character_positions = party.character_positions.clone();
        let (_, combatant) = party.get_mut_combatant_by_id(&character_id)?;
        let consumable_id = combatant
            .selected_consumable
            .clone()
            .ok_or_else(|| AppError {
                error_type: AppErrorTypes::Generic,
                message: error_messages::NO_CONSUMABLE_SELECTED.to_string(),
            })?;
        let character =
            party.get_character_if_owned(player_character_ids_option.clone(), character_id)?;
        let conusmable_properties = character.combatant_properties.inventory.get_consumable(&consumable_id)?;
        let combat_action_properties = conusmable_properties
            .consumable_type
            .get_combat_action_properties();

        let (new_targets, new_target_preferences) =
            character_changes_combat_action_targets_handler(
                game,
                new_targets,
                combat_action_properties,
                character_id,
                battle_id_option,
                character_positions,
                player_character_ids_option.clone(),
                party_id,
            )?;

        let party = get_mut_party(game, party_id)?;
        let character =
            party.get_mut_character_if_owned(player_character_ids_option, character_id)?;

        character
            .combatant_properties
            .combat_action_target_preferences = new_target_preferences;
        character.combatant_properties.combat_action_targets = Some(new_targets.clone());

        self.emit_packet(
            &party_websocket_channel_name,
            &WebsocketChannelNamespace::Party,
            &GameServerUpdatePackets::CharacterChangedTargets(ChangeTargetsPacket {
                character_id,
                new_targets,
            }),
            None,
        )
    }
}

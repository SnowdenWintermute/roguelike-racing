use crate::websocket_server::game_server::getters::get_mut_game;
use crate::websocket_server::game_server::getters::get_user;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::errors::AppErrorTypes;
use common::game::getters::get_mut_party;
use common::game::getters::get_mut_player;
use common::packets::client_to_server::ClientSelectAbilityPacket;
use common::packets::server_to_client::CharacterSelectedAbilityPacket;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn character_selects_ability_handler(
        &mut self,
        actor_id: u32,
        packet: ClientSelectAbilityPacket,
    ) -> Result<(), AppError> {
        let connected_user = get_user(&self.sessions, actor_id)?;
        let username = connected_user.username.clone();
        let current_game_name =
            connected_user
                .current_game_name
                .as_ref()
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::MISSING_GAME_REFERENCE.to_string(),
                })?;
        let game = get_mut_game(&mut self.games, &current_game_name)?;
        let player = get_mut_player(game, &username)?;
        let player_character_ids_option = player.character_ids.clone();
        let party_id = player.party_id.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
        })?;

        let party = get_mut_party(game, party_id)?;
        let party_websocket_channel_name = party.websocket_channel_name.clone();

        let new_targets_option = if packet.ability_name_option.is_none() {
            let character = party
                .get_mut_character_if_owned(player_character_ids_option, packet.character_id)?;
            character.combatant_properties.selected_ability_name = None;
            character.combatant_properties.ability_targets = None;
            None
        } else {
            let battle_id_option = party.battle_id;
            println!(
                "battle_id option at character_selects_ability_handler: {:?}",
                battle_id_option
            );
            let character_positions = party.character_positions.clone();
            let character = party.get_mut_character_if_owned(
                player_character_ids_option.clone(),
                packet.character_id,
            )?;
            let target_preferences = character
                .combatant_properties
                .ability_target_preferences
                .clone();

            let ability_name = packet.ability_name_option.clone().expect("is_none checked");
            // don't allow selection of unowned ability
            let _ = character
                .combatant_properties
                .get_mut_ability_if_owned(&ability_name)?;

            let (ally_ids, opponent_ids_option) = if let Some(battle_id) = battle_id_option {
                let battle = game.battles.get(&battle_id).ok_or_else(|| AppError {
                    error_type: AppErrorTypes::Generic,
                    message: error_messages::BATTLE_NOT_FOUND.to_string(),
                })?;

                battle.get_ally_ids_and_opponent_ids_option(packet.character_id)?
            } else {
                (character_positions, None)
            };

            let party = get_mut_party(game, party_id)?;
            let character = party.get_mut_character_if_owned(
                player_character_ids_option.clone(),
                packet.character_id,
            )?;

            let new_targets = ability_name.targets_by_saved_preference_or_default(
                character.entity_properties.id,
                &target_preferences,
                ally_ids.clone(),
                opponent_ids_option.clone(),
            )?;

            let new_target_preferences = target_preferences.get_updated_preferences(
                &ability_name,
                &new_targets,
                ally_ids,
                opponent_ids_option,
            );
            let party = get_mut_party(game, party_id)?;
            let character = party
                .get_mut_character_if_owned(player_character_ids_option, packet.character_id)?;
            character.combatant_properties.selected_ability_name = Some(ability_name);
            character.combatant_properties.ability_target_preferences = new_target_preferences;
            Some(new_targets)
        };

        self.emit_packet(
            &party_websocket_channel_name,
            &WebsocketChannelNamespace::Party,
            &GameServerUpdatePackets::CharacterSelectedAbility(CharacterSelectedAbilityPacket {
                character_id: packet.character_id,
                ability_name_option: packet.ability_name_option,
                targets_option: new_targets_option,
            }),
            Some(actor_id),
        )
    }
}

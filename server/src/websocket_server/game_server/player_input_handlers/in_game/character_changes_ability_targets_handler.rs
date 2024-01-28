use crate::websocket_server::game_server::getters::get_mut_game;
use crate::websocket_server::game_server::getters::get_mut_party_game_name_and_character_ids_from_actor_id;
use crate::websocket_server::game_server::getters::get_user;
use crate::websocket_server::game_server::getters::ActorIdAssociatedPartyData;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages::{self};
use common::combat::combat_actions::filter_possible_target_ids_by_prohibited_combatant_states::filter_possible_target_ids_by_prohibited_combatant_states;
use common::errors::AppError;
use common::errors::AppErrorTypes;
use common::game::getters::get_mut_party;
use common::game::getters::get_mut_player;
use common::packets::client_to_server::ChangeTargetsPacket;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn character_changes_ability_targets_handler(
        &mut self,
        actor_id: u32,
        packet: ChangeTargetsPacket,
    ) -> Result<(), AppError> {
        let ChangeTargetsPacket {
            character_id,
            new_targets,
        } = packet;

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
        let ability_name = combatant
            .selected_ability_name
            .clone()
            .ok_or_else(|| AppError {
                error_type: AppErrorTypes::Generic,
                message: error_messages::NO_ABILITY_SELECTED.to_string(),
            })?;
        let _ = combatant.get_ability_if_owned(&ability_name)?;
        let ability_attributes = ability_name.get_attributes();
        let combat_action_properties = ability_attributes.combat_action_properties;

        let (ally_ids, opponent_ids_option) = if let Some(battle_id) = battle_id_option {
            let battle = game.battles.get(&battle_id).ok_or_else(|| AppError {
                error_type: AppErrorTypes::Generic,
                message: error_messages::BATTLE_NOT_FOUND.to_string(),
            })?;

            battle.get_ally_ids_and_opponent_ids_option(character_id)?
        } else {
            (character_positions.clone(), None)
        };

        let prohibited_target_combatant_states = combat_action_properties
            .prohibited_target_combatant_states
            .clone();
        let (ally_ids, opponent_ids_option) =
            filter_possible_target_ids_by_prohibited_combatant_states(
                game,
                &prohibited_target_combatant_states,
                ally_ids,
                opponent_ids_option,
            )?;

        let new_targets = if combat_action_properties.targets_are_valid(
            character_id,
            &new_targets,
            &ally_ids,
            &opponent_ids_option,
        ) {
            new_targets
        } else {
            combat_action_properties.get_default_targets(
                character_id,
                &ally_ids,
                &opponent_ids_option,
            )?
        };

        let ActorIdAssociatedPartyData {
            party,
            player_character_ids_option,
            ..
        } = get_mut_party_game_name_and_character_ids_from_actor_id(self, actor_id)?;
        let character =
            party.get_character_if_owned(player_character_ids_option.clone(), character_id)?;

        let target_preferences = &character.combatant_properties.ability_target_preferences;
        let new_target_preferences = target_preferences.get_updated_preferences(
            &combat_action_properties,
            &new_targets,
            ally_ids,
            opponent_ids_option,
        );
        let character =
            party.get_mut_character_if_owned(player_character_ids_option, character_id)?;

        character.combatant_properties.ability_target_preferences = new_target_preferences;
        character.combatant_properties.ability_targets = Some(new_targets.clone());

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

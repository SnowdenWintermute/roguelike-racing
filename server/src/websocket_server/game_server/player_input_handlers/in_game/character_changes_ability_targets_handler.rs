use crate::websocket_server::game_server::{
    getters::{
        get_mut_game, get_mut_party_game_name_and_character_ids_from_actor_id, get_mut_user,
        get_user, ActorIdAssociatedPartyData,
    },
    GameServer,
};
use common::{
    app_consts::error_messages::{self},
    errors::{AppError, AppErrorTypes},
    game::getters::{get_mut_party, get_mut_player},
    packets::{client_to_server::ChangeTargetsPacket, server_to_client::GameServerUpdatePackets},
};

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
        let (_, combatant) = party.get_mut_combatant_by_id(character_id)?;
        let ability = combatant.get_ability_if_owned(combatant.selected_ability_name)?;
        let ability_name = ability.ability_name.clone();

        let battle_id_option = party.battle_id;
        let (ally_ids, opponent_ids_option) = if let Some(battle_id) = battle_id_option {
            let battle = game.battles.get(&battle_id).ok_or_else(|| AppError {
                error_type: AppErrorTypes::Generic,
                message: error_messages::BATTLE_NOT_FOUND.to_string(),
            })?;

            battle.get_ally_ids_and_opponent_ids_option(character_id)?
        } else {
            (party.character_positions, None)
        };

        let new_targets = if ability.ability_name.targets_are_valid(
            character_id,
            &new_targets,
            ally_ids,
            opponent_ids_option,
        ) {
            new_targets
        } else {
            ability
                .ability_name
                .get_default_targets(character_id, ally_ids, opponent_ids_option)?
        };

        let ActorIdAssociatedPartyData {
            party,
            player_character_ids_option,
            ..
        } = get_mut_party_game_name_and_character_ids_from_actor_id(self, actor_id)?;
        let character =
            party.get_character_if_owned(player_character_ids_option.clone(), character_id)?;

        let target_preferences = &character.combatant_properties.ability_target_preferences;
        let new_target_preferences =
            target_preferences.get_updated_preferences(&ability_name, &new_targets, party);
        let character =
            party.get_mut_character_if_owned(player_character_ids_option, character_id)?;

        character.combatant_properties.ability_target_preferences = new_target_preferences;
        character.combatant_properties.ability_targets = Some(new_targets.clone());

        self.emit_packet(
            &current_game_name,
            &GameServerUpdatePackets::CharacterChangedTargets(ChangeTargetsPacket {
                character_id,
                new_targets,
            }),
            None,
        )
    }
}

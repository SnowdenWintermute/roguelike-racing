use crate::websocket_server::game_server::{
    getters::{get_mut_game_data_from_actor_id, get_user, ActorIdAssociatedGameData},
    GameServer,
};
use common::{
    app_consts::error_messages,
    combatants::abilities::get_combatant_ability_attributes::AbilityUsableContext,
    errors::AppError, game::getters::get_mut_party,
};

impl GameServer {
    pub fn character_uses_selected_ability_handler(
        &mut self,
        actor_id: u32,
        character_id: u32,
    ) -> Result<(), AppError> {
        let ActorIdAssociatedGameData {
            game,
            party_id,
            current_game_name,
            username,
            player_character_ids_option,
        } = get_mut_game_data_from_actor_id(self, actor_id);
        let party = get_mut_party(game, party_id)?;
        let character =
            party.get_mut_character_if_owned(player_character_ids_option, packet.character_id)?;
        let ability_name = character
            .combatant_properties
            .selected_ability_name
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::NO_ABILITY_SELECTED.to_string(),
            })?;
        let ability_attributes = ability_name.get_attributes();
        let selected_ability = character
            .combatant_properties
            .get_ability_if_owned(&ability_name)?;
        let targets = character
            .combatant_properties
            .ability_targets
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::NO_POSSIBLE_TARGETS_PROVIDED.to_string(),
            })?;
        // check if targets are valid
        let targets_are_valid = ability_name.targets_are_valid(
            packet.character_id,
            &targets,
            &party.character_positions,
            None,
        );

        if !targets_are_valid {
            return Err(AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::INVALID_TARGETS_SELECTED.to_string(),
            });
        }

        if party.battle_id.is_none() {
            // check if ability can be used out of combat
            if ability_attributes.usability_context == AbilityUsableContext::InCombat {
                return Err(AppError {
                    error_type: common::errors::AppErrorTypes::InvalidInput,
                    message: error_messages::INVALID_ABILITY_CONTEXT.to_string(),
                });
            };

            // return the targets and hp/mp/status effect changes.
            // client can construct animation of the effects
        }

        //
        // if in combat
        // check if character is first in turn order
        // check if ability is usable in combat
        // check if targets are valid
        // if ability ends turn
        //   if next turn is a player, return targets and their changes, including the effect that
        //   the ability user's turn has ended. client will prompt next player in turn order to
        //   move.
        //
        //   if next turn is ai controlled, return client targets and changes, as well as targets
        //   and changes for next ai ability used in turn order, repeating until a player is next.
        //
        //   client animates each ability targets/effects object, then prompts next player to move
        Ok(())
    }
}

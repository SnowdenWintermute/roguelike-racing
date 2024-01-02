use crate::websocket_server::game_server::getters::get_mut_game_data_from_actor_id;
use crate::websocket_server::game_server::getters::get_user;
use crate::websocket_server::game_server::getters::ActorIdAssociatedGameData;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::combatants::abilities::get_combatant_ability_attributes::AbilityUsableContext;
use common::errors::AppError;
use common::game::getters::get_mut_party;

use super::apply_action_results::apply_action_results;

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
        } = get_mut_game_data_from_actor_id(self, actor_id)?;
        let party = get_mut_party(game, party_id)?;
        let character =
            party.get_mut_character_if_owned(player_character_ids_option, character_id)?;
        let ability_name = character
            .combatant_properties
            .selected_ability_name
            .clone()
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::NO_ABILITY_SELECTED.to_string(),
            })?;
        let ability_attributes = ability_name.get_attributes();
        let _ = character
            .combatant_properties
            .get_ability_if_owned(&ability_name)?;
        let targets = character
            .combatant_properties
            .ability_targets
            .clone()
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::NO_POSSIBLE_TARGETS_PROVIDED.to_string(),
            })?;

        // if in combat
        if let Some(battle_id) = party.battle_id {
            let battle = game.battles.get(&battle_id).ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ServerError,
                message: error_messages::BATTLE_NOT_FOUND.to_string(),
            })?;
            // check if character is first in turn order
            if !battle.combatant_is_first_in_turn_order(character_id) {
                return Err(AppError {
                    error_type: common::errors::AppErrorTypes::InvalidInput,
                    message: error_messages::NOT_THIS_COMBATANTS_TURN.to_string(),
                });
            }
            // check if ability is usable in combat
            if ability_attributes.usability_context == AbilityUsableContext::OutOfCombat {
                return Err(AppError {
                    error_type: common::errors::AppErrorTypes::InvalidInput,
                    message: error_messages::INVALID_ABILITY_CONTEXT.to_string(),
                });
            };
            let (ally_ids, opponent_ids_option) =
                battle.get_ally_ids_and_opponent_ids_option(character_id)?;

            // check if targets are valid
            let targets_are_valid = ability_name.targets_are_valid(
                character_id,
                &targets,
                &ally_ids,
                &opponent_ids_option,
            );

            if !targets_are_valid {
                return Err(AppError {
                    error_type: common::errors::AppErrorTypes::InvalidInput,
                    message: error_messages::INVALID_TARGETS_SELECTED.to_string(),
                });
            }

            // process client ability and add it to the packet of ability execution results
            let mut action_results = game.get_ability_results(
                character_id,
                &ability_name,
                &targets,
                Some(&battle.clone()),
            )?;

            // apply changes from action results
            apply_action_results(game, &mut action_results)?;

            if ability_attributes.requires_combat_turn {
                // battle.combatant_turn_trackers

                //   sort the turn trackers
                //   if next turn is a player, return targets and their changes. client will use the
                //   action results to animate, apply changes and sort the turn orders  then prompt
                //   next player in turn order to move.
                //
                //   if next turn is ai controlled, return client targets and changes, as well as targets
                //   and changes for next ai ability used in turn order, repeating until a player is next.
                //
                //   client animates each ability targets/effects object, then prompts next player to move
            }
        } else {
            // check if ability can be used out of combat
            if ability_attributes.usability_context == AbilityUsableContext::InCombat {
                return Err(AppError {
                    error_type: common::errors::AppErrorTypes::InvalidInput,
                    message: error_messages::INVALID_ABILITY_CONTEXT.to_string(),
                });
            };
            // check if targets are valid
            let targets_are_valid = ability_name.targets_are_valid(
                character_id,
                &targets,
                &party.character_positions,
                &None,
            );

            if !targets_are_valid {
                return Err(AppError {
                    error_type: common::errors::AppErrorTypes::InvalidInput,
                    message: error_messages::INVALID_TARGETS_SELECTED.to_string(),
                });
            }

            // return the targets and hp/mp/status effect changes.
            // client can construct animation of the effects
        }

        Ok(())
    }
}

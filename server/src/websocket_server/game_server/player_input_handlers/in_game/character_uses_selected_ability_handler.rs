use super::apply_action_results::apply_action_results;
use crate::websocket_server::game_server::getters::get_mut_game_data_from_actor_id;
use crate::websocket_server::game_server::getters::ActorIdAssociatedGameData;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::combat::CombatTurnResult;
use common::combatants::abilities::get_combatant_ability_attributes::AbilityUsableContext;
use common::combatants::CombatantControlledBy;
use common::errors::AppError;
use common::game::getters::get_mut_party;

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

            let mut action_results = game.get_ability_results(
                character_id,
                &ability_name,
                &targets,
                Some(&battle.clone()),
            )?;

            apply_action_results(game, &mut action_results)?;

            if ability_attributes.requires_combat_turn {
                let mut turns: Vec<CombatTurnResult> = vec![];
                let player_turn = CombatTurnResult {
                    combatant_id: character_id,
                    action_results,
                };
                turns.push(player_turn);

                game.end_active_combatant_turn(battle_id)?;

                //   if next turn is a player, return turn with action results. client will use the
                //   action results to animate and apply changes
                let battle = game.battles.get(&battle_id).ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::BATTLE_NOT_FOUND.to_string(),
                })?;

                let active_combatant_tracker =
                    battle
                        .combatant_turn_trackers
                        .first()
                        .ok_or_else(|| AppError {
                            error_type: common::errors::AppErrorTypes::Generic,
                            message: error_messages::TURN_TRACKERS_EMPTY.to_string(),
                        })?;
                let (_, active_combatant_properties) =
                    game.get_combatant_by_id(&active_combatant_tracker.entity_id)?;
                let mut active_combatant_is_ai_controlled =
                    active_combatant_properties.controlled_by == CombatantControlledBy::AI;
                while active_combatant_is_ai_controlled {
                    // calculate AI turns, process them and add them to the turn results
                    // get active combatant
                    let (entity_properties, combatant_properties) =
                        game.get_combatant_by_id(&active_combatant_tracker.entity_id)?;
                    // select an CombatantAbilityNames and AbilityTarget
                    // get result of ability and add to list of results for this turn
                    // process result
                    // if ability required turn
                    // add all the results to a turn and push to list of
                    // turns
                    // end the active_combatant's turn and check if new active combatant
                    // is a player or AI
                    //
                }
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

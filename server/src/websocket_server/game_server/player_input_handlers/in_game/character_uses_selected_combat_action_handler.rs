use crate::websocket_server::game_server::getters::get_mut_game_data_from_actor_id;
use crate::websocket_server::game_server::getters::ActorIdAssociatedGameData;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::combat::combat_actions::AbilityUsableContext;
use common::combat::combat_actions::CombatAction;
use common::errors::AppError;
use common::errors::AppErrorTypes;
use common::game::getters::get_party;

impl GameServer {
    pub fn character_uses_selected_combat_action_handler(
        &mut self,
        actor_id: u32,
        character_id: u32,
    ) -> Result<(), AppError> {
        let ActorIdAssociatedGameData {
            game,
            party_id,
            player_character_ids_option,
            ..
        } = get_mut_game_data_from_actor_id(self, actor_id)?;
        let party = get_party(game, party_id)?;
        let character_positions = party.character_positions.clone();
        let battle_id_option = party.battle_id.clone();
        let character = party.get_character_if_owned(&player_character_ids_option, character_id)?;
        let selected_combat_action = character
            .combatant_properties
            .selected_combat_action
            .clone()
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::NO_ACTION_SELECTED.to_string(),
            })?
            .clone();

        // ENSURE OWNERSHIP OF CONSUMABLE OR ABILITY
        let combat_action_properties =
            selected_combat_action.get_properties_if_owned(game, character_id)?;

        // ENSURE TARGETING
        let targets = character
            .combatant_properties
            .combat_action_targets
            .clone()
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::NO_POSSIBLE_TARGETS_PROVIDED.to_string(),
            })?;
        // IF IN BATTLE, ONLY USE IF FIRST IN TURN ORDER
        let battle_option = if let Some(battle_id) = battle_id_option {
            game.battles.get(&battle_id)
        } else {
            None
        };

        if let Some(battle) = battle_option {
            if !battle.combatant_is_first_in_turn_order(character_id) {
                return Err(AppError {
                    error_type: AppErrorTypes::InvalidInput,
                    message: error_messages::NOT_THIS_COMBATANTS_TURN.to_string(),
                });
            }
        }
        // VALIDATE USABILITY CONTEXT
        let usability_context = combat_action_properties.usability_context;
        let invalid_usability_context = match usability_context {
            AbilityUsableContext::All => false,
            AbilityUsableContext::InCombat => battle_id_option.is_none(),
            AbilityUsableContext::OutOfCombat => battle_id_option.is_some(),
        };
        if invalid_usability_context {
            return Err(AppError {
                error_type: AppErrorTypes::InvalidInput,
                message: error_messages::INVALID_USABILITY_CONTEXT.to_string(),
            });
        }
        // GET ABILITY OR CONSUMABLE USE RESULTS
        let action_results = match selected_combat_action {
            CombatAction::AbilityUsed(ability_name) => game.get_ability_results(
                character_id,
                &ability_name,
                &targets,
                battle_option,
                character_positions,
            )?,
            CombatAction::ConsumableUsed(item_id) => {
                game.get_consumable_use_results(character_id, item_id, &targets, battle_option)?
            }
        };

        self.handle_new_combat_action_results(actor_id, action_results, character_id)
    }
}

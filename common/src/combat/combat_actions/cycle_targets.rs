use super::filter_possible_target_ids_by_prohibited_combatant_states::filter_possible_target_ids_by_prohibited_combatant_states;
use crate::app_consts::error_messages;
use crate::combat::combat_actions::filter_possible_target_ids_by_action_target_categories::filter_possible_target_ids_by_action_target_categories;
use crate::errors::AppError;
use crate::errors::AppErrorTypes;
use crate::game::getters::get_ally_ids_and_opponent_ids_option;
use crate::game::getters::get_mut_party;
use crate::game::getters::get_mut_player;
use crate::game::getters::get_party;
use crate::game::RoguelikeRacerGame;
use crate::primatives::NextOrPrevious;
use std::collections::HashSet;

impl RoguelikeRacerGame {
    pub fn cycle_character_targets(
        &mut self,
        party_id: u32,
        player_character_ids_option: Option<HashSet<u32>>,
        player_username: &String,
        character_id: u32,
        direction: &NextOrPrevious,
    ) -> Result<(), AppError> {
        let party = get_party(self, party_id)?;
        let cloned_character_positions = party.character_positions.clone();
        let battle_id_option = party.battle_id;
        let battle_option = self.get_battle_option(&battle_id_option)?;
        let battle_option = battle_option.clone();
        let character = party.get_character_if_owned(&player_character_ids_option, character_id)?;

        let selected_combat_action = character
            .combatant_properties
            .selected_combat_action
            .clone()
            .ok_or_else(|| AppError {
                error_type: AppErrorTypes::InvalidInput,
                message: error_messages::NO_ACTION_SELECTED.to_string(),
            })?;
        let current_targets = character
            .combatant_properties
            .combat_action_targets
            .as_ref()
            .ok_or_else(|| AppError {
                error_type: AppErrorTypes::ClientError,
                message: error_messages::TRIED_TO_CYCLE_TARGETS_WHEN_NO_TARGETS.to_string(),
            })?;

        let current_targets = current_targets.clone();

        let combat_action_properties =
            selected_combat_action.get_properties_if_owned(self, character_id)?;

        let prohibited_target_combatant_states = combat_action_properties
            .prohibited_target_combatant_states
            .clone();

        let (ally_ids, opponent_ids_option) = get_ally_ids_and_opponent_ids_option(
            &cloned_character_positions,
            battle_option.as_ref(),
            character_id,
        )?;

        let (ally_ids, opponent_ids_option) =
            filter_possible_target_ids_by_prohibited_combatant_states(
                self,
                &prohibited_target_combatant_states,
                ally_ids,
                opponent_ids_option,
            )?;

        let (ally_ids_option, opponent_ids_option) =
            filter_possible_target_ids_by_action_target_categories(
                &combat_action_properties.valid_target_categories,
                character_id,
                ally_ids.clone(),
                opponent_ids_option,
            );

        let new_targets = combat_action_properties.get_next_or_previous_targets(
            &current_targets,
            direction,
            &character_id,
            &ally_ids_option,
            &opponent_ids_option,
        )?;

        let player = get_mut_player(self, player_username)?;

        player.target_preferences = player.target_preferences.get_updated_preferences(
            &combat_action_properties,
            &new_targets,
            ally_ids,
            opponent_ids_option,
        );

        let party = get_mut_party(self, party_id)?;
        let character =
            party.get_mut_character_if_owned(player_character_ids_option, character_id)?;

        character.combatant_properties.combat_action_targets = Some(new_targets);

        Ok(())
    }
}

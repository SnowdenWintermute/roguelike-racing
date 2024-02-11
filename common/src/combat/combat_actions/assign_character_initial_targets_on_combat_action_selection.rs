use super::filter_possible_target_ids_by_prohibited_combatant_states::filter_possible_target_ids_by_prohibited_combatant_states;
use super::CombatActionProperties;
use super::CombatActionTarget;
use crate::app_consts::error_messages;
use crate::errors::AppError;
use crate::errors::AppErrorTypes;
use crate::game::getters::get_mut_party;
use crate::game::RoguelikeRacerGame;
use std::collections::HashSet;

impl RoguelikeRacerGame {
    pub fn assign_character_initial_targets_on_combat_action_selection(
        &mut self,
        character_id: u32,
        player_character_ids_option: &Option<HashSet<u32>>,
        party_id: u32,
        battle_id_option: Option<u32>,
        character_positions: &Vec<u32>,
        combat_action_properties_option: &Option<CombatActionProperties>,
    ) -> Result<Option<CombatActionTarget>, AppError> {
        if let Some(combat_action_properties) = combat_action_properties_option {
            let (ally_ids, opponent_ids_option) = if let Some(battle_id) = battle_id_option {
                let battle = self.battles.get(&battle_id).ok_or_else(|| AppError {
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
                    self,
                    &prohibited_target_combatant_states,
                    ally_ids,
                    opponent_ids_option,
                )?;

            let party = get_mut_party(self, party_id)?;
            let character = party
                .get_mut_character_if_owned(player_character_ids_option.clone(), character_id)?;

            let target_preferences = character
                .combatant_properties
                .combat_action_target_preferences
                .clone();

            let new_targets = combat_action_properties.targets_by_saved_preference_or_default(
                character.entity_properties.id,
                &target_preferences,
                &ally_ids,
                &opponent_ids_option,
            )?;

            let new_target_preferences = target_preferences.get_updated_preferences(
                &combat_action_properties,
                &new_targets,
                ally_ids,
                opponent_ids_option,
            );

            println!("new preferences: {:#?}", new_target_preferences);
            character
                .combatant_properties
                .combat_action_target_preferences = new_target_preferences;
            println!("new targets: {:#?}", new_targets);
            character.combatant_properties.combat_action_targets = Some(new_targets.clone());

            Ok(Some(new_targets))
        } else {
            let party = get_mut_party(self, party_id)?;
            let character = party
                .get_mut_character_if_owned(player_character_ids_option.clone(), character_id)?;
            character.combatant_properties.combat_action_targets = None;
            Ok(None)
        }
    }
}

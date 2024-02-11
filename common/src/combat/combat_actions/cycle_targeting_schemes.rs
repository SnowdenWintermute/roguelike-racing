use crate::app_consts::error_messages;
use crate::errors::AppError;
use crate::errors::AppErrorTypes;
use crate::game::getters::get_mut_party;
use crate::game::getters::get_party;
use crate::game::RoguelikeRacerGame;
use std::collections::HashSet;

impl RoguelikeRacerGame {
    pub fn cycle_targeting_schemes(
        &mut self,
        party_id: u32,
        player_character_ids_option: Option<HashSet<u32>>,
        character_id: u32,
    ) -> Result<(), AppError> {
        let party = get_mut_party(self, party_id)?;
        let character_positions = party.character_positions.clone();
        let battle_id_option = party.battle_id;

        let party = get_party(self, party_id)?;
        let character =
            party.get_character_if_owned(player_character_ids_option.clone(), character_id)?;

        let selected_combat_action = character
            .combatant_properties
            .selected_combat_action
            .clone()
            .ok_or_else(|| AppError {
                error_type: AppErrorTypes::InvalidInput,
                message: error_messages::NO_ACTION_SELECTED.to_string(),
            })?;

        let combat_action_properties =
            selected_combat_action.get_properties_if_owned(self, character_id)?;

        if combat_action_properties.targeting_schemes.len() < 2 {
            return Err(AppError {
                error_type: AppErrorTypes::InvalidInput,
                message: error_messages::ONLY_ONE_TARGETING_SCHEME_AVAILABLE.to_string(),
            });
        }

        let last_used_targeting_scheme = character
            .combatant_properties
            .combat_action_target_preferences
            .targeting_scheme_preference
            .clone();

        println!("last used scheme: {:?}", last_used_targeting_scheme);

        let new_targeting_scheme = if !combat_action_properties
            .targeting_schemes
            .contains(&last_used_targeting_scheme)
        {
            combat_action_properties
                .targeting_schemes
                .first()
                .ok_or_else(|| AppError {
                    error_type: AppErrorTypes::ClientError,
                    message: error_messages::MISSING_ABILITY_REFERENCE.to_string(),
                })?
        } else {
            let last_used_targeting_scheme_index = combat_action_properties
                .targeting_schemes
                .iter()
                .position(|scheme| scheme == &last_used_targeting_scheme)
                .expect("checked it contains above");
            let new_scheme_index = if last_used_targeting_scheme_index
                == combat_action_properties.targeting_schemes.len() - 1
            {
                0
            } else {
                last_used_targeting_scheme_index + 1
            };
            combat_action_properties
                .targeting_schemes
                .get(new_scheme_index)
                .expect("a valid index")
        };

        println!("new scheme: {:?}", new_targeting_scheme);

        let party = get_mut_party(self, party_id)?;
        let character =
            party.get_mut_character_if_owned(player_character_ids_option.clone(), character_id)?;

        character
            .combatant_properties
            .combat_action_target_preferences
            .targeting_scheme_preference = new_targeting_scheme.clone();

        let assign_new_targets_result = self
            .assign_character_initial_targets_on_combat_action_selection(
                character_id,
                &player_character_ids_option,
                party_id,
                battle_id_option,
                &character_positions,
                &Some(combat_action_properties),
            );

        match assign_new_targets_result {
            Ok(_) => Ok(()),
            Err(error) => {
                let party = get_mut_party(self, party_id)?;
                let character = party.get_mut_character_if_owned(
                    player_character_ids_option.clone(),
                    character_id,
                )?;
                character
                    .combatant_properties
                    .combat_action_target_preferences
                    .targeting_scheme_preference = last_used_targeting_scheme;

                Err(error)
            }
        }
    }
}

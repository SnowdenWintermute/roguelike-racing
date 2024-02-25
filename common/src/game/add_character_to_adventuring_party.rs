use super::getters::get_mut_party;
use super::RoguelikeRacerGame;
use crate::app_consts::error_messages;
use crate::app_consts::MAX_PARTY_SIZE;
use crate::character::Character;
use crate::combatants::combatant_classes::CombatantClass;
use crate::errors::AppError;

impl RoguelikeRacerGame {
    pub fn add_character_to_adventuring_party(
        &mut self,
        party_id: u32,
        combatant_class: CombatantClass,
        name: &str,
        name_of_controlling_user: String,
    ) -> Result<u32, AppError> {
        let party = get_mut_party(self, party_id)?;

        if party.characters.len() >= MAX_PARTY_SIZE.into() {
            return Err(AppError {
                error_type: crate::errors::AppErrorTypes::InvalidInput,
                message: error_messages::PARTY_FULL.to_string(),
            });
        }

        let new_character = Character::new(self, name, combatant_class, name_of_controlling_user);
        let character_id = new_character.entity_properties.id;

        let party = get_mut_party(self, party_id)?;
        party
            .characters
            .insert(new_character.entity_properties.id, new_character);
        party.character_positions.push(character_id);
        Ok(character_id)
    }
}

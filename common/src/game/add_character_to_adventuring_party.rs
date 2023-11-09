use super::{getters::get_mut_party, RoguelikeRacerGame};
use crate::{
    app_consts::{error_messages, MAX_PARTY_SIZE},
    character::Character,
    combatants::CombatantClass,
    errors::AppError,
};

impl RoguelikeRacerGame {
    pub fn add_character_to_adventuring_party(
        &mut self,
        party_id: u32,
        combatant_class: CombatantClass,
        name: &str,
        name_of_controlling_user: String,
    ) -> Result<(), AppError> {
        let party = get_mut_party(self, party_id)?;

        if party.characters.len() >= MAX_PARTY_SIZE.into() {
            return Err(AppError {
                error_type: crate::errors::AppErrorTypes::InvalidInput,
                message: error_messages::PARTY_FULL.to_string(),
            });
        }

        let new_character = Character::new(self, name, combatant_class, name_of_controlling_user);

        let party = get_mut_party(self, party_id)?;
        party
            .characters
            .insert(new_character.entity_properties.id, new_character);
        Ok(())
    }
}

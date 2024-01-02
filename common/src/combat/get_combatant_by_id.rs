use crate::app_consts::error_messages;
use crate::combatants::CombatantProperties;
use crate::errors::AppError;
use crate::errors::AppErrorTypes;
use crate::game::RoguelikeRacerGame;
use crate::primatives::EntityProperties;

impl RoguelikeRacerGame {
    pub fn get_combatant_by_id(
        &self,
        combatant_id: &u32,
    ) -> Result<(&EntityProperties, &CombatantProperties), AppError> {
        for (_, party) in self.adventuring_parties.iter() {
            for (entity_id, character) in party.characters.iter() {
                if entity_id == combatant_id {
                    return Ok((
                        &character.entity_properties,
                        &character.combatant_properties,
                    ));
                }
            }
            if let Some(monsters) = &party.current_room.monsters {
                for (entity_id, monster) in monsters {
                    if entity_id == combatant_id {
                        return Ok((&monster.entity_properties, &monster.combatant_properties));
                    }
                }
            }
        }
        Err(AppError {
            error_type: AppErrorTypes::Generic,
            message: error_messages::COMBATANT_NOT_FOUND.to_string(),
        })
    }

    pub fn get_mut_combatant_by_id(
        &mut self,
        combatant_id: &u32,
    ) -> Result<(&mut EntityProperties, &mut CombatantProperties), AppError> {
        for (_, party) in self.adventuring_parties.iter_mut() {
            for (entity_id, character) in party.characters.iter_mut() {
                if entity_id == combatant_id {
                    return Ok((
                        &mut character.entity_properties,
                        &mut character.combatant_properties,
                    ));
                }
            }
            if let Some(monsters) = &mut party.current_room.monsters {
                for (entity_id, monster) in monsters {
                    if entity_id == combatant_id {
                        return Ok((
                            &mut monster.entity_properties,
                            &mut monster.combatant_properties,
                        ));
                    }
                }
            }
        }
        Err(AppError {
            error_type: AppErrorTypes::Generic,
            message: error_messages::COMBATANT_NOT_FOUND.to_string(),
        })
    }
}

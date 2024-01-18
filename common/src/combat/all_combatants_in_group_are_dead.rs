use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;

impl RoguelikeRacerGame {
    pub fn all_combatants_in_group_are_dead(
        &self,
        combatant_ids: Vec<u32>,
    ) -> Result<bool, AppError> {
        for id in combatant_ids {
            let (_, combatant_properties) = self.get_combatant_by_id(&id)?;
            if combatant_properties.hit_points > 0 {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

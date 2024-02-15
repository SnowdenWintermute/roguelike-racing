use crate::app_consts::MIN_HIT_CHANCE;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
use rand::Rng;

impl RoguelikeRacerGame {
    pub fn roll_evaded(&self, user_accuracy: u16, target_id: u32) -> Result<bool, AppError> {
        let (_, target_combatant_properties) = self.get_combatant_by_id(&target_id)?;
        let target_combat_attributes = target_combatant_properties.get_total_attributes();
        let target_evasion = target_combat_attributes
            .get(&CombatAttributes::Evasion)
            .unwrap_or_else(|| &0);
        let acc_eva_compared = user_accuracy as i16 - *target_evasion as i16;
        let percent_chance_to_hit = std::cmp::max(MIN_HIT_CHANCE, acc_eva_compared);
        let mut rng = rand::thread_rng();
        let evade_roll = rng.gen_range(0..=100);
        Ok(evade_roll > percent_chance_to_hit)
    }
}

use crate::combatants::combat_attributes::CombatAttributes;
use crate::combatants::combatant_traits::CombatantTraits;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;

impl RoguelikeRacerGame {
    pub fn get_healing_hp_change_on_target_combatant(
        &self,
        target_id: &u32,
        base_hp_change: f32,
    ) -> Result<i16, AppError> {
        let mut target_hp_change = base_hp_change;
        //  - add Resilience multiplier
        let (_, target_combatant_properties) = self.get_combatant_by_id(&target_id)?;
        let target_combat_attributes = target_combatant_properties.get_total_attributes();
        let target_resilience = target_combat_attributes
            .get(&CombatAttributes::Resilience)
            .unwrap_or_else(|| &0);
        let resilience_multiplier = *target_resilience as f32 / 100.0 + 1.0;
        let is_undead = target_combatant_properties
            .traits
            .contains(&CombatantTraits::Undead);
        //  - if not undead trait, convert to healing and add resilience_multiplier
        if !is_undead {
            target_hp_change += resilience_multiplier;
            target_hp_change *= -1.0;
        }
        Ok(target_hp_change as i16)
    }
}

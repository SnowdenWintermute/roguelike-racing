use super::battle::Battle;
use super::CombatActionResult;
use crate::combatants::abilities::AbilityTarget;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
mod ability_resolution_calculators;
pub mod attack;
pub mod validate_ability_use;

impl RoguelikeRacerGame {
    pub fn get_ability_results(
        &mut self,
        ability_user_id: u32,
        ability_name: &CombatantAbilityNames,
        ability_target: &AbilityTarget,
        battle_option: Option<&Battle>,
    ) -> Result<Vec<CombatActionResult>, AppError> {
        match ability_name {
            CombatantAbilityNames::Attack => {
                self.attack_handler(ability_user_id, ability_target, battle_option)
            }
            CombatantAbilityNames::ArmorBreak => todo!(),
            CombatantAbilityNames::HeatLance => todo!(),
            CombatantAbilityNames::Fire => todo!(),
            CombatantAbilityNames::RainStorm => todo!(),
            CombatantAbilityNames::Heal => todo!(),
        }

        // Ok(effects)
    }
}

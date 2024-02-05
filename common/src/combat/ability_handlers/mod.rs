use super::battle::Battle;
use super::ActionResult;
use crate::combat::combat_actions::CombatActionTarget;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
mod ability_resolution_calculators;
pub mod attack;
pub mod fire;

impl RoguelikeRacerGame {
    pub fn get_ability_results(
        &self,
        ability_user_id: u32,
        ability_name: &CombatantAbilityNames,
        ability_target: &CombatActionTarget,
        battle_option: Option<&Battle>,
    ) -> Result<Vec<ActionResult>, AppError> {
        match ability_name {
            CombatantAbilityNames::Attack => {
                self.attack_handler(ability_user_id, ability_target, battle_option)
            }
            _ => Ok(Vec::new()), // CombatantAbilityNames::ArmorBreak => todo!(),
                                 // CombatantAbilityNames::HeatLance => todo!(),
                                 // CombatantAbilityNames::Fire => todo!(),
                                 // CombatantAbilityNames::RainStorm => todo!(),
                                 // CombatantAbilityNames::Heal => todo!(),
        }

        // Ok(effects)
    }
}

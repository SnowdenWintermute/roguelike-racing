use super::battle::Battle;
use super::ActionResult;
use crate::combat::combat_actions::CombatActionTarget;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
mod ability_resolution_calculators;
mod add_weapon_damage_to_hp_change_range;
pub mod apply_elemental_affinity_to_hp_change_range;
pub mod attack;
pub mod fire;
pub mod get_ability_base_hp_change_range;
pub mod get_crit_range;
pub mod roll_crit;
pub mod split_combat_action_hp_change_by_number_of_targets;

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

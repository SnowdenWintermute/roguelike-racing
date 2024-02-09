use super::battle::Battle;
use super::combat_actions::CombatAction;
use super::ActionResult;
use crate::combat::combat_actions::CombatActionTarget;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
mod ability_resolution_calculators;
mod add_weapon_damage_to_hp_change_range;
mod apply_crit_multiplier_to_hp_change;
pub mod apply_elemental_affinity_to_hp_change;
pub mod attack;
pub mod calculate_combat_action_hp_changes;
mod calculate_combat_action_mp_changes;
mod calculate_healing_hp_change_and_add_to_action_result;
mod calculate_magical_damage_hp_change_and_add_to_action_result;
mod calculate_physical_damage_hp_change_and_add_to_action_result;
pub mod get_ability_base_hp_change_range;
mod get_healing_hp_change_on_target_combatant;
pub mod roll_crit;
mod roll_evaded;
pub mod split_combat_action_hp_change_by_number_of_targets;

impl RoguelikeRacerGame {
    pub fn get_ability_results(
        &self,
        ability_user_id: u32,
        ability_name: &CombatantAbilityNames,
        ability_targets: &CombatActionTarget,
        battle_option: Option<&Battle>,
    ) -> Result<Vec<ActionResult>, AppError> {
        match ability_name {
            CombatantAbilityNames::Attack => {
                self.attack_handler(ability_user_id, ability_targets, battle_option)
            }
            CombatantAbilityNames::Fire => {
                let combat_action = CombatAction::AbilityUsed(ability_name.clone());
                let action_result = ActionResult::new(
                    ability_user_id,
                    combat_action.clone(),
                    ability_targets.clone(),
                );

                let action_result = self.calculate_combat_action_mp_changes(
                    &action_result,
                    ability_user_id,
                    ability_targets,
                    battle_option,
                    &combat_action,
                )?;

                let action_result = self.calculate_combat_action_hp_changes(
                    &action_result,
                    ability_user_id,
                    ability_targets,
                    battle_option,
                    &combat_action,
                )?;
                Ok(vec![action_result])
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

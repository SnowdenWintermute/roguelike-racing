use super::battle::Battle;
use super::combat_actions::CombatAction;
use super::ActionResult;
use crate::combat::combat_actions::CombatActionTarget;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
mod add_damage_type_from_weapon_to_hp_change_properties;
mod add_element_from_weapon_to_hp_change_properties;
mod add_weapon_damage_to_hp_change_range;
pub mod apply_affinity_to_hp_change;
mod apply_crit_multiplier_to_hp_change;
pub mod attack;
mod calculate_action_hp_and_mp_changes;
pub mod calculate_combat_action_hp_change_range;
pub mod calculate_combat_action_hp_changes;
mod calculate_combat_action_mp_changes;
mod calculate_healing_hp_change_and_add_to_action_result;
mod calculate_magical_damage_hp_change_and_add_to_action_result;
mod calculate_physical_damage_hp_change_and_add_to_action_result;
mod get_healing_hp_change_on_target_combatant;
pub mod roll_crit;
mod roll_evaded;
pub mod split_combat_action_hp_change_by_number_of_targets;
mod test_calculate_combat_action_hp_changes;

impl RoguelikeRacerGame {
    pub fn get_ability_results(
        &self,
        ability_user_id: u32,
        ability_name: &CombatantAbilityNames,
        ability_targets: &CombatActionTarget,
        battle_option: Option<&Battle>,
        ally_ids: Vec<u32>,
    ) -> Result<Vec<ActionResult>, AppError> {
        let (_, combatant_properties) = self.get_combatant_by_id(&ability_user_id)?;
        let ability = combatant_properties.get_ability_if_owned(ability_name)?;
        let ability_attributes = ability_name.get_attributes();

        match ability_name {
            CombatantAbilityNames::Attack => {
                self.attack_handler(ability_user_id, ability_targets, battle_option, ally_ids)
            }
            CombatantAbilityNames::Fire
            | CombatantAbilityNames::Healing
            | CombatantAbilityNames::Ice => {
                let combat_action = CombatAction::AbilityUsed(ability_name.clone());
                let action_result = self.calculate_action_hp_and_mp_changes(
                    combat_action,
                    ability_user_id,
                    ability_targets,
                    battle_option,
                    ally_ids,
                    Some((
                        ability.level,
                        ability_attributes.base_hp_change_values_level_multiplier,
                    )),
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

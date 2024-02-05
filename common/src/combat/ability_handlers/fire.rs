use rand::Rng;

use crate::app_consts::error_messages;
use crate::app_consts::MAX_SPELL_CRIT_CHANCE;
use crate::combat::battle::Battle;
use crate::combat::combat_actions::CombatActionTarget;
use crate::combat::combat_actions::TargetingScheme;
use crate::combat::ActionResult;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
use crate::primatives::Range;

impl RoguelikeRacerGame {
    pub fn fire_handler(
        &self,
        ability_user_id: u32,
        ability_target: &CombatActionTarget,
        battle_option: Option<&Battle>,
    ) -> Result<Vec<ActionResult>, AppError> {
        let mut action_results = vec![];
        let battle = battle_option.ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::Generic,
            message: error_messages::INVALID_ABILITY_CONTEXT.to_string(),
        })?;

        let (ally_ids, opponent_ids_option) =
            battle.get_ally_ids_and_opponent_ids_option(ability_user_id)?;

        let target_entity_ids = ability_target.get_targets_if_scheme_valid(
            ally_ids,
            opponent_ids_option,
            ability_user_id,
            vec![TargetingScheme::All],
        )?;

        let (_, user_combatant_properties) = self.get_combatant_by_id(&ability_user_id)?;
        let user_combat_attributes = user_combatant_properties.get_total_attributes();
        let user_focus_attribute = user_combat_attributes
            .get(&CombatAttributes::Focus)
            .unwrap_or_else(|| &0);
        let ability =
            user_combatant_properties.get_ability_if_owned(&CombatantAbilityNames::Fire)?;

        // get base damage from spell level and int
        let base_damage_range = ability
            .base_values
            .clone()
            .unwrap_or_else(|| Range::new(1, 1));
        let min_base_damage = (ability.level as u16 * base_damage_range.min as u16)
            + user_combat_attributes
                .get(&CombatAttributes::Intelligence)
                .unwrap_or_else(|| &0);
        let max_base_damage = (ability.level as u16 * base_damage_range.max as u16)
            + user_combat_attributes
                .get(&CombatAttributes::Intelligence)
                .unwrap_or_else(|| &0);

        // add bonus if multiple targeted
        let num_targets = target_entity_ids.len();
        let multi_target_bonus = num_targets as f32 * 0.1;
        let min_damage = min_base_damage as f32 * multi_target_bonus;
        let max_damage = max_base_damage as f32 * multi_target_bonus;
        // split damage between all targets
        let mut min_damage = min_damage / num_targets as f32;
        let mut max_damage = max_damage / num_targets as f32;
        // roll if crit and multiply
        let spell_crit_chance = user_focus_attribute;
        let spell_crit_chance = std::cmp::min(MAX_SPELL_CRIT_CHANCE, *spell_crit_chance);
        let mut rng = rand::thread_rng();
        let crit_roll = rng.gen_range(0..=100);
        let is_crit = crit_roll < spell_crit_chance;
        if is_crit {
            let crit_multiplier_from_focus = *user_focus_attribute as f32 / 100.0;
            let crit_multiplier = 0.50 + crit_multiplier_from_focus;
            min_damage += min_damage * crit_multiplier;
            max_damage += max_damage * crit_multiplier;
        }

        // multiply damage by weakness/affinity traits
        // calculate resiliance % reduction/increase(if healing)

        // let (_, target_combatant_properties) =
        //     self.get_combatant_in_battle_by_id(&battle, target_entity_id)?;
        // let target_combatant_properties = target_combatant_properties.clone();
        // let (_, user_combatant_properties) =
        //     self.get_combatant_in_battle_by_id(&battle, &ability_user_id)?;
        // let user_total_attributes = user_combatant_properties.get_total_attributes();
        // let target_total_attributes = target_combatant_properties.get_total_attributes();

        Ok(action_results)
    }
}

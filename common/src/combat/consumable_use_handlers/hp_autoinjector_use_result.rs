use crate::app_consts::error_messages;
use crate::combat::combat_actions::CombatAction;
use crate::combat::combat_actions::CombatActionTarget;
use crate::combat::ActionResult;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::combatants::combatant_traits::CombatantTraits;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
use rand::Rng;
use std::collections::HashMap;

pub fn hp_autoinjector_use_result(
    game: &RoguelikeRacerGame,
    user_id: u32,
    consumable_item_id: u32,
    target: &CombatActionTarget,
) -> Result<Vec<ActionResult>, AppError> {
    let target_id = match target {
        CombatActionTarget::Single(id) => id,
        CombatActionTarget::Group(_) | CombatActionTarget::All => {
            return Err(AppError {
                error_type: crate::errors::AppErrorTypes::InvalidInput,
                message: error_messages::INVALID_TARGETS_SELECTED.to_string(),
            })
        }
    };
    let (_, target_combatant_properties) = game.get_combatant_by_id(&target_id)?;

    if target_combatant_properties.hit_points == 0 {
        return Err(AppError {
            error_type: crate::errors::AppErrorTypes::InvalidInput,
            message: error_messages::CANT_BE_USED_ON_DEAD_TARGET.to_string(),
        });
    }

    let bonus_multiplier = {
        let mut to_return: f32 = 1.0;
        for combatant_trait in &target_combatant_properties.traits {
            match combatant_trait {
                CombatantTraits::HpBioavailabilityPercent(value) => {
                    to_return = *value as f32 / 100.0
                }
                _ => (),
            }
        }
        to_return
    };
    let max_hp = *target_combatant_properties
        .get_total_attributes()
        .get(&CombatAttributes::Hp)
        .unwrap_or_else(|| &0) as f32;
    // don't allow healing full hp target
    if max_hp as u16 == target_combatant_properties.hit_points {
        return Err(AppError {
            error_type: crate::errors::AppErrorTypes::InvalidInput,
            message: error_messages::ALREADY_FULL_HP.to_string(),
        });
    }
    // calculate healing amount
    let min_healing = bonus_multiplier * max_hp / 8.0;
    let max_healing = bonus_multiplier * 3.0 * max_hp / 8.0;
    let mut rng = rand::thread_rng();
    let rolled_healing = rng.gen_range(min_healing..=max_healing);
    let final_healing = rolled_healing.clamp(0.0, i16::MAX as f32) as i16;

    Ok(vec![ActionResult {
        user_id,
        action: CombatAction::ConsumableUsed(consumable_item_id),
        targets: target.clone(),
        hp_changes_by_entity_id: Some(HashMap::from([(*target_id, final_healing)])),
        mp_changes_by_entity_id: None,
        misses_by_entity_id: None,
        is_crit: false,
        status_effect_changes_by_entity_id: None,
        ends_turn: false,
    }])
}

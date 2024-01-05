use common::combat::ActionResult;
use common::combat::IdAndValue;
use common::combatants::combat_attributes::CombatAttributes;
use common::errors::AppError;
use common::game::RoguelikeRacerGame;
use common::utils::add_i16_to_u16_and_clamp_to_max;

pub fn apply_action_results(
    game: &mut RoguelikeRacerGame,
    action_results: &Vec<ActionResult>,
) -> Result<(), AppError> {
    for action_result in action_results {
        apply_action_result(game, &action_result)?;
    }

    Ok(())
}

pub fn apply_action_result(
    game: &mut RoguelikeRacerGame,
    action_result: &ActionResult,
) -> Result<(), AppError> {
    if let Some(hp_changes) = &action_result.hp_changes_by_entity_id {
        for id_and_hp_change in hp_changes {
            let IdAndValue(entity_id, hp_change) = id_and_hp_change;
            let (_, combatant_properties) = game.get_mut_combatant_by_id(entity_id)?;
            let combatant_total_attributes = combatant_properties.get_total_attributes();
            let max_hp = combatant_total_attributes
                .get(&CombatAttributes::Hp)
                .unwrap_or_else(|| &0);
            let new_hp = add_i16_to_u16_and_clamp_to_max(
                combatant_properties.hit_points,
                *hp_change,
                *max_hp,
            );
            combatant_properties.hit_points = new_hp;
        }
    }

    if let Some(mp_changes) = &action_result.mp_changes_by_entity_id {
        for id_and_mp_change in mp_changes {
            let IdAndValue(entity_id, mp_change) = id_and_mp_change;
            let (_, combatant_properties) = game.get_mut_combatant_by_id(entity_id)?;
            let combatant_total_attributes = combatant_properties.get_total_attributes();
            let max_mp = combatant_total_attributes
                .get(&CombatAttributes::Mp)
                .unwrap_or_else(|| &0);
            let new_mp = add_i16_to_u16_and_clamp_to_max(
                combatant_properties.hit_points,
                *mp_change,
                *max_mp,
            );
            combatant_properties.mana = new_mp;
        }
    }

    Ok(())
}

use super::battle::Battle;
use super::combat_actions::CombatActionTarget;
use super::ActionResult;
use crate::combat::combat_actions::CombatAction;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
use crate::items::consumables::ConsumableProperties;
use crate::items::consumables::ConsumableTypes;

impl RoguelikeRacerGame {
    pub fn get_consumable_use_results(
        &self,
        user_id: u32,
        consumable_item_id: u32,
        target: &CombatActionTarget,
        battle_option: Option<&Battle>,
    ) -> Result<Vec<ActionResult>, AppError> {
        let mut action_results: Vec<ActionResult> = vec![];
        let (_, user_combatant_properties) = self.get_combatant_by_id(&user_id)?;
        let consumable_properties = user_combatant_properties
            .inventory
            .get_consumable(&consumable_item_id)?;
        match consumable_properties.consumable_type {
            ConsumableTypes::Autoinjector => action_results.push(ActionResult {
                user_id,
                action: CombatAction::ConsumableUsed(consumable_item_id),
                targets: target.clone(),
                hp_changes_by_entity_id: None,
                mp_changes_by_entity_id: None,
                misses_by_entity_id: None,
                resists_by_entity_id: None,
                is_crit: false,
                status_effect_changes_by_entity_id: None,
                ends_turn: false,
            }),
            ConsumableTypes::Grenade => (),
            ConsumableTypes::SmokeBomb => (),
        }
        todo!()
    }
}

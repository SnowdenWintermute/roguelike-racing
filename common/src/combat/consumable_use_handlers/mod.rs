mod hp_autoinjector_use_result;
use self::hp_autoinjector_use_result::hp_autoinjector_use_result;
use super::battle::Battle;
use super::combat_actions::CombatActionTarget;
use super::ActionResult;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
use crate::items::consumables::ConsumableTypes;

impl RoguelikeRacerGame {
    pub fn get_consumable_use_results(
        &self,
        user_id: u32,
        item_id: u32,
        target: &CombatActionTarget,
        _battle_option: Option<&Battle>,
    ) -> Result<Vec<ActionResult>, AppError> {
        let (_, user_combatant_properties) = self.get_combatant_by_id(&user_id)?;
        let consumable_properties = user_combatant_properties
            .inventory
            .get_consumable(&item_id)?;
        let results = match consumable_properties.consumable_type {
            ConsumableTypes::HpAutoinjector => {
                hp_autoinjector_use_result(self, user_id, item_id, target)?
            }
            ConsumableTypes::Grenade => todo!(),
            ConsumableTypes::SmokeBomb => todo!(),
        };
        Ok(results)
    }
}

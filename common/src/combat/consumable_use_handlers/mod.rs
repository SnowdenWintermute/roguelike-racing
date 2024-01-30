use super::battle::Battle;
use super::combat_actions::CombatActionTarget;
use super::ActionResult;
use crate::errors::AppError;
use crate::game::getters::get_character;
use crate::game::getters::get_party;
use crate::game::RoguelikeRacerGame;

impl RoguelikeRacerGame {
    pub fn get_consumable_use_results(
        &self,
        party_id: u32,
        user_id: u32,
        consumable_id: u32,
        target: &CombatActionTarget,
        battle_option: Option<&Battle>,
    ) -> Result<Vec<ActionResult>, AppError> {
        let character = get_character(game, party_id, character_id)?;
        todo!()
    }
}

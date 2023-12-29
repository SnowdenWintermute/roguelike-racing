use crate::{
    combat::{battle::Battle, CombatActionResult},
    combatants::abilities::{AbilityTarget, CombatantAbilityNames},
    errors::AppError,
    game::RoguelikeRacerGame,
};

impl RoguelikeRacerGame {
    pub fn attack_handler(
        &mut self,
        ability_user_id: u32,
        ability_name: &CombatantAbilityNames,
        ability_target: &AbilityTarget,
        battle_option: Option<&Battle>,
    ) -> Result<CombatActionResult, AppError> {
        // get the battle

        // get their targeted entity
        // calculate damage
        // return result
        todo!();
    }
}

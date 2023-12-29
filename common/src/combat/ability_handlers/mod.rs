use super::{battle::Battle, CombatActionResult};
use crate::{
    combatants::abilities::{AbilityTarget, CombatantAbilityNames},
    errors::AppError,
    game::RoguelikeRacerGame,
};
pub mod attack;

impl RoguelikeRacerGame {
    pub fn process_ability(
        &mut self,
        ability_user_id: u32,
        ability_name: &CombatantAbilityNames,
        ability_target: &AbilityTarget,
        battle_option: Option<&Battle>,
    ) -> Result<CombatActionResult, AppError> {
        match ability_name {
            CombatantAbilityNames::Attack => {
                self.attack_handler(ability_user_id, ability_name, ability_target, battle_option)
            }
            CombatantAbilityNames::ArmorBreak => todo!(),
            CombatantAbilityNames::HeatLance => todo!(),
            CombatantAbilityNames::Fire => todo!(),
            CombatantAbilityNames::RainStorm => todo!(),
            CombatantAbilityNames::Heal => todo!(),
        }

        // Ok(effects)
    }
}

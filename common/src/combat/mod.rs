use crate::adventuring_party::AdventuringParty;
use crate::combatants::abilities::{AbilityTarget, CombatantAbility, CombatantAbilityNames};
use crate::errors::AppError;
use crate::items::consumables::ConsumableTypes;
use crate::items::Item;
use crate::status_effects::StatusEffects;

#[derive(Debug)]
pub enum CombatAction {
    AbilityUsed(CombatantAbility, AbilityTarget),
    ItemUsed(Item, AbilityTarget),
}

#[derive(Debug)]
pub enum CombatActionEffect {
    AbilityUsed(CombatantAbilityNames, Vec<u32>),
    ConsumableUsed(ConsumableTypes, Vec<u32>),
    CurrentHpChange(i16, u32),
    CurrentMpChange(i16, u32),
    StatusEffectGained(StatusEffects, u32),
    StatusEffectLost(StatusEffects, u32),
    EndTurn,
}

impl AdventuringParty {
    pub fn get_combat_action_effects(
        &mut self,
        action: CombatAction,
        combatant_id: u32,
    ) -> Result<Vec<CombatActionEffect>, AppError> {
        match action {
            CombatAction::AbilityUsed(ability, targets) => {
                return self.get_ability_used_combat_action_effects(
                    combatant_id,
                    &ability,
                    &targets,
                );
            }
            CombatAction::ItemUsed(_, _) => todo!(),
        }
    }

    pub fn get_ability_used_combat_action_effects(
        &mut self,
        combatant_id: u32,
        ability: &CombatantAbility,
        targets: &AbilityTarget,
    ) -> Result<Vec<CombatActionEffect>, AppError> {
        let mut effects = vec![];
        let combatant = self.get_combatant_by_id(combatant_id)?;

        Ok(effects)
    }
}

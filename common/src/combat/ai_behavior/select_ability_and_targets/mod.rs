use crate::app_consts::error_messages;
use crate::combat::battle::BattleGroup;
use crate::combat::combat_actions::CombatActionTarget;
use crate::combat::combat_actions::FriendOrFoe;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
use rand::thread_rng;
use rand::Rng;

// @TODO - make a b tree

impl RoguelikeRacerGame {
    pub fn ai_select_ability_and_targets(
        &self,
        ability_user_id: u32,
        _ally_battle_group: BattleGroup,
        enemy_battle_group: BattleGroup,
    ) -> Result<(CombatantAbilityNames, CombatActionTarget), AppError> {
        let mut random_enemy_id = get_random_enemy_id(&enemy_battle_group)?;
        let mut combatant = self.get_combatant_by_id(random_enemy_id)?;
        while combatant.1.hit_points == 0 {
            random_enemy_id = get_random_enemy_id(&enemy_battle_group)?;
            combatant = self.get_combatant_by_id(random_enemy_id)?;
        }

        let (_, user_combatant_properties) = self.get_combatant_by_id(&ability_user_id)?;

        if user_combatant_properties
            .abilities
            .contains_key(&CombatantAbilityNames::Fire)
        {
            let level_adjusted_mp_cost = user_combatant_properties
                .get_ability_cost_if_owned(&CombatantAbilityNames::Fire)?;
            if user_combatant_properties.mana > level_adjusted_mp_cost as u16 {
                return Ok((
                    CombatantAbilityNames::Fire,
                    CombatActionTarget::Group(FriendOrFoe::Hostile),
                ));
            }
        }
        if user_combatant_properties
            .abilities
            .contains_key(&CombatantAbilityNames::Ice)
        {
            let level_adjusted_mp_cost =
                user_combatant_properties.get_ability_cost_if_owned(&CombatantAbilityNames::Ice)?;
            if user_combatant_properties.mana > level_adjusted_mp_cost as u16 {
                return Ok((
                    CombatantAbilityNames::Ice,
                    CombatActionTarget::Single(*random_enemy_id),
                ));
            }
        }
        if user_combatant_properties
            .abilities
            .contains_key(&CombatantAbilityNames::Healing)
        {
            let level_adjusted_mp_cost = user_combatant_properties
                .get_ability_cost_if_owned(&CombatantAbilityNames::Healing)?;
            if user_combatant_properties.mana > level_adjusted_mp_cost as u16 {
                return Ok((
                    CombatantAbilityNames::Healing,
                    CombatActionTarget::Group(FriendOrFoe::Friendly),
                ));
            }
        }

        Ok((
            CombatantAbilityNames::Attack,
            CombatActionTarget::Single(*random_enemy_id),
        ))
    }
}

fn get_random_enemy_id<'a>(enemy_battle_group: &'a BattleGroup) -> Result<&'a u32, AppError> {
    let max = enemy_battle_group.combatant_ids.len() - 1;
    let min = 0;
    let mut rng = thread_rng();
    let random_index = rng.gen_range(min..=max);
    enemy_battle_group
        .combatant_ids
        .get(random_index)
        .ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::Generic,
            message: error_messages::NO_POSSIBLE_TARGETS_PROVIDED.to_string(),
        })
}

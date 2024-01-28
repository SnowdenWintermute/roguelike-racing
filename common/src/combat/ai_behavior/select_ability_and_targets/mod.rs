use crate::app_consts::error_messages;
use crate::combat::battle::BattleGroup;
use crate::combat::combat_actions::CombatActionTarget;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
use rand::thread_rng;
use rand::Rng;

// @TODO - make a b tree

impl RoguelikeRacerGame {
    pub fn ai_select_ability_and_targets(
        &self,
        _ability_user_id: u32,
        _ally_battle_group: BattleGroup,
        enemy_battle_group: BattleGroup,
    ) -> Result<(CombatantAbilityNames, CombatActionTarget), AppError> {
        let random_enemy_id = {
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
                })?
        };

        Ok((
            CombatantAbilityNames::Attack,
            CombatActionTarget::Single(*random_enemy_id),
        ))
    }
}

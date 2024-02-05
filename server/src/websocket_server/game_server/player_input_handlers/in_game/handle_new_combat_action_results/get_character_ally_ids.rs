use common::combat::battle::Battle;
use common::errors::AppError;

pub fn get_character_ally_ids(
    battle_option: &Option<Battle>,
    party_member_ids: &Vec<u32>,
    action_taker_character_id: &u32,
) -> Result<Vec<u32>, AppError> {
    if let Some(battle) = &battle_option {
        let (ally_ids, _) =
            battle.get_ally_ids_and_opponent_ids_option(*action_taker_character_id)?;
        Ok(ally_ids)
    } else {
        Ok(party_member_ids.clone())
    }
}

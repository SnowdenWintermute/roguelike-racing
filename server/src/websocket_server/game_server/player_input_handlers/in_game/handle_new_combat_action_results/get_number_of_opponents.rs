use common::combat::battle::Battle;
use common::errors::AppError;

pub fn get_number_of_opponents(
    battle_option: &Option<Battle>,
    character_id: u32,
) -> Result<u8, AppError> {
    if let Some(battle) = &battle_option {
        let (_, opponent_ids_option) = battle.get_ally_ids_and_opponent_ids_option(character_id)?;
        Ok(opponent_ids_option.unwrap_or_else(|| vec![]).len() as u8)
    } else {
        Ok(0)
    }
}

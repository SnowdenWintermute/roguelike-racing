use crate::yew_app::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::combat::battle::Battle;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use std::collections::HashMap;
use yewdux::Dispatch;

pub fn handle_battle_full_update(
    game_dispatch: Dispatch<GameStore>,
    battle_option: Option<Battle>,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|game_store| {
        let game = game_store.game.as_mut().ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::GAME_NOT_FOUND.to_string(),
        })?;
        if let Some(battle) = battle_option {
            game_store.current_battle_id = Some(battle.id);
            if let Some(party_id) = game_store.current_party_id {
                let party = get_mut_party(game, party_id)?;
                party.battle_id = Some(battle.id);
            }

            game.battles.insert(battle.id, battle);
        } else {
            game_store.current_battle_id = None;
            game.battles = HashMap::new();
        }
        Ok(())
    })
}

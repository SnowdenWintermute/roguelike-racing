use crate::yew_app::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::game::RoguelikeRacerGame;
use yewdux::Dispatch;

pub fn hp_change_message_handler(
    game_dispatch: Dispatch<GameStore>,
    target_id: u32,
    hp_change: i16,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let party = store.get_current_party_mut()?;
        let battle_id_option = party.battle_id;
        let game = store.get_current_game_mut()?;
        let (_, combatant_properties) = game.get_mut_combatant_by_id(&target_id)?;
        let new_hp = combatant_properties.change_hp(hp_change);

        if new_hp == 0 {
            remove_combatant_turn_tracker(game, battle_id_option, target_id)?;
        }

        Ok(())
    })
}

fn remove_combatant_turn_tracker(
    game: &mut RoguelikeRacerGame,
    battle_id_option: Option<u32>,
    entity_id: u32,
) -> Result<(), AppError> {
    if let Some(battle_id) = battle_id_option {
        let battle = game.battles.get_mut(&battle_id).ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::BATTLE_NOT_FOUND.to_string(),
        })?;
        let mut index_to_remove_option = None;
        for (i, turn_tracker) in battle.combatant_turn_trackers.iter().enumerate() {
            if turn_tracker.entity_id == entity_id {
                index_to_remove_option = Some(i)
            }
        }
        if let Some(index_to_remove) = index_to_remove_option {
            let _ = battle.combatant_turn_trackers.remove(index_to_remove);
        }
    }
    Ok(())
}

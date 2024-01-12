use crate::components::mesh_manager::ClientCombatantEvent;
use crate::store::game_store::get_current_battle_option;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::combatants::abilities::AbilityTarget;
use common::combatants::abilities::FriendOrFoe;
use common::errors::AppError;
use common::game::getters::get_party;
use yewdux::Dispatch;

pub fn handle_animation_finished(
    combatant_event: ClientCombatantEvent,
    game_dispatch: Dispatch<GameStore>,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        // - queue and start damage taken animations on affected entities
        // - subtract hp from affected entities
        // - if any affected entity is dead, queue death animation on that entity
        // - if action required turn, end active combatant turn for the current battle if any
        match combatant_event {
            ClientCombatantEvent::TookAction(action_result) => {
                let battle_option = get_current_battle_option(store);
                let ability_user_id = action_result.user_id;
                let party_id = store.current_party_id.ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ClientError,
                    message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
                })?;
                let game = store.get_current_game()?;
                let target_ids = game.get_ids_from_ability_target(
                    party_id,
                    battle_option,
                    &action_result.targets,
                    ability_user_id,
                );
            }
            _ => (),
        }
        // for any event animation finishing
        //  - if still alive, process next event in that entity's queue
        //  - if all entity event queues are empty and no animations are ongoing,
        //    and in combat
        //    query the ActionResultsManager turn_results_queue queue for the next action_result to process/animate
        Ok(())
    })
}

use crate::yew_app::components::game::combat_log::combat_log_message::CombatLogMessage;
use crate::yew_app::components::game::combat_log::combat_log_message::CombatLogMessageStyle;
use crate::yew_app::components::mesh_manager::AutoinjectorTypes;
use crate::yew_app::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use yew::AttrValue;
use yewdux::Dispatch;

pub fn autoinjector_use_animation_finished_handler(
    game_dispatch: Dispatch<GameStore>,
    autoinjector_type: AutoinjectorTypes,
    value_change: i16,
    user_id: u32,
    target_id: u32,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| {
        let game = store.get_current_game_mut()?;
        let (user_entity_properties, _) = game.get_mut_combatant_by_id(&user_id)?;
        let user_name = user_entity_properties.name.clone();
        let (target_entity_properties, target_combatant_properties) =
            game.get_mut_combatant_by_id(&target_id)?;
        let target_name = target_entity_properties.name.clone();
        match autoinjector_type {
            AutoinjectorTypes::Hp => {
                let _ = target_combatant_properties.change_hp(value_change);
                let target_text = if user_id == target_id {
                    "themselves".to_string()
                } else {
                    format!("{}", target_name)
                };
                store.combat_log.push(CombatLogMessage::new(
                    AttrValue::from(format!(
                        "{} injected {} for {value_change} HP",
                        user_name, target_text
                    )),
                    CombatLogMessageStyle::Basic,
                    0,
                ));
            }
            AutoinjectorTypes::Mp => {
                let _ = target_combatant_properties.change_mp(value_change);
                let target_text = if user_id == target_id {
                    "themselves".to_string()
                } else {
                    format!("{}", target_name)
                };
                store.combat_log.push(CombatLogMessage::new(
                    AttrValue::from(format!(
                        "{} injected {} for {value_change} MP",
                        user_name, target_text
                    )),
                    CombatLogMessageStyle::Basic,
                    0,
                ));
            }
        }

        let event_manager = store
            .action_results_manager
            .combantant_event_managers
            .get_mut(&user_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND.to_string(),
            })?;
        event_manager.action_result_queue.pop_front();
        Ok(())
    })
}

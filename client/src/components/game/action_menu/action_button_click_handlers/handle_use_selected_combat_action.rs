use crate::components::alerts::set_alert;
use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::alert_store::AlertStore;
use crate::store::game_store::GameStore;
use common::errors::AppError;
use common::packets::client_to_server::PlayerInputs;
use web_sys::WebSocket;
use yewdux::Dispatch;

pub fn handle_use_selected_combat_action(
    game_dispatch: Dispatch<GameStore>,
    alert_dispatch: Dispatch<AlertStore>,
    websocket_option: &Option<WebSocket>,
) {
    let result = (|| -> Result<(), AppError> {
        game_dispatch.reduce_mut(|store| {
            store.selected_item = None;
            store.detailed_entity = None;
            if let Some(page_number) = store.parent_menu_pages.pop() {
                store.action_menu_current_page_number = page_number;
            }

            send_client_input(
                websocket_option,
                PlayerInputs::UseSelectedCombatAction(store.focused_character_id),
            );
            Ok(())
        })
    })();

    match result {
        Ok(_) => (),
        Err(error) => set_alert(alert_dispatch, error.message),
    }
}

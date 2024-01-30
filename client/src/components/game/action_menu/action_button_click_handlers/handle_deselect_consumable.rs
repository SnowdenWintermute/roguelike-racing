use crate::components::alerts::set_alert;
use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::alert_store::AlertStore;
use crate::store::game_store::GameStore;
use common::errors::AppError;
use common::packets::client_to_server::ClientSelectConsumablePacket;
use common::packets::client_to_server::PlayerInputs;
use web_sys::WebSocket;
use yewdux::prelude::Dispatch;

pub fn handle_deselect_consumable(
    game_dispatch: Dispatch<GameStore>,
    alert_dispatch: Dispatch<AlertStore>,
    websocket_option: &Option<WebSocket>,
) {
    let result = game_dispatch.reduce_mut(|game_store| -> Result<(), AppError> {
        let focused_character: _ = game_store.get_mut_character(game_store.focused_character_id)?;
        focused_character.combatant_properties.selected_consumable = None;
        focused_character.combatant_properties.combat_action_targets = None;
        send_client_input(
            websocket_option,
            PlayerInputs::SelectConsumable(ClientSelectConsumablePacket {
                character_id: focused_character.entity_properties.id,
                consumable_id_option: None,
            }),
        );
        Ok(())
    });

    if let Some(err) = result.err() {
        set_alert(alert_dispatch, err.message)
    }
}

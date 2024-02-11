use super::handle_select_combat_action::handle_select_combat_action;
use super::handle_select_consumable::handle_select_consumable;
use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::alert_store::AlertStore;
use crate::store::game_store::get_focused_character;
use crate::store::game_store::GameStore;
use crate::store::lobby_store::LobbyStore;
use crate::store::ui_store::UIStore;
use crate::store::websocket_store::WebsocketStore;
use common::combat::combat_actions::CombatAction;
use common::items::Item;
use common::packets::client_to_server::EquipItemRequest;
use common::packets::client_to_server::PlayerInputs;
use common::packets::CharacterAndSlot;
use std::rc::Rc;
use yewdux::Dispatch;

pub fn use_item_handler(
    game_dispatch: Dispatch<GameStore>,
    game_state: Rc<GameStore>,
    ui_state: Rc<UIStore>,
    websocket_state: Rc<WebsocketStore>,
    alert_dispatch: Dispatch<AlertStore>,
    lobby_state: Rc<LobbyStore>,
    item_id: &u32,
) {
    let item_option = &game_state.selected_item;
    let character_id = game_state.focused_character_id;
    let alt_slot = ui_state.mod_key_held;
    if let Some(item) = item_option {
        match item.item_properties {
            common::items::ItemProperties::Consumable(_) => handle_select_combat_action(
                game_dispatch,
                alert_dispatch,
                lobby_state,
                &websocket_state.websocket,
                Some(CombatAction::ConsumableUsed(*item_id)),
            ),
            common::items::ItemProperties::Equipment(_) => {
                use_equipment_handler(game_dispatch, websocket_state, character_id, item, alt_slot)
            }
        }
    }
}

fn use_equipment_handler(
    game_dispatch: Dispatch<GameStore>,
    websocket_state: Rc<WebsocketStore>,
    character_id: u32,
    item: &Item,
    alt_slot: bool,
) {
    game_dispatch.reduce_mut(|game_store| {
        let focused_character = get_focused_character(game_store).expect("to be in game");
        let slot_item_is_equipped = focused_character
            .combatant_properties
            .slot_item_is_equipped(&item.entity_properties.id);
        if let Some(slot) = slot_item_is_equipped {
            send_client_input(
                &websocket_state.websocket,
                PlayerInputs::UnequipEquipmentSlot(CharacterAndSlot { character_id, slot }),
            )
        } else {
            send_client_input(
                &websocket_state.websocket,
                PlayerInputs::EquipInventoryItem(EquipItemRequest {
                    character_id,
                    item_id: item.entity_properties.id,
                    alt_slot,
                }),
            )
        }
    });
}

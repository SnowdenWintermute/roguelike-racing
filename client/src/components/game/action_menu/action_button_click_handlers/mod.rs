pub mod handle_cycle_combat_action_targeting_schemes;
pub mod handle_cycle_combat_action_targets;
pub mod handle_select_combat_action;
mod handle_use_selected_combat_action;
mod use_item_handler;
use self::handle_cycle_combat_action_targeting_schemes::handle_cycle_combat_action_targeting_schemes;
use self::handle_cycle_combat_action_targets::handle_cycle_combat_action_targets;
use self::handle_select_combat_action::handle_select_combat_action;
use self::handle_use_selected_combat_action::handle_use_selected_combat_action;
use super::enums::GameActions;
use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::alert_store::AlertStore;
use crate::store::game_store::get_focused_character;
use crate::store::game_store::get_item_owned_by_focused_character;
use crate::store::game_store::select_item;
use crate::store::game_store::GameStore;
use crate::store::lobby_store::LobbyStore;
use crate::store::ui_store::UIStore;
use crate::store::websocket_store::WebsocketStore;
use common::packets::client_to_server::PlayerInputs;
use common::packets::CharacterAndItem;
use common::packets::CharacterAndSlot;
use gloo::console::log;
use std::rc::Rc;
use use_item_handler::use_item_handler;
use yewdux::prelude::Dispatch;

pub fn create_action_button_click_handler<'a>(
    game_action: GameActions,
    game_dispatch: Dispatch<GameStore>,
    game_state: Rc<GameStore>,
    ui_state: Rc<UIStore>,
    lobby_state: Rc<LobbyStore>,
    websocket_state: Rc<WebsocketStore>,
    alert_dispatch: Dispatch<AlertStore>,
) -> Box<dyn Fn()> {
    match game_action {
        GameActions::ToggleReadyToExplore => Box::new(move || {
            send_client_input(
                &websocket_state.websocket,
                PlayerInputs::ToggleReadyToExplore,
            )
        }),
        GameActions::SetInventoryOpen(status) => Box::new(move || {
            game_dispatch.reduce_mut(|game_state| game_state.viewing_inventory = status.clone());
            game_dispatch.reduce_mut(|game_state| game_state.viewing_equipped_items = false);
        }),
        GameActions::ToggleViewingEquipedItems => Box::new(move || {
            game_dispatch.reduce_mut(|game_state| {
                game_state.viewing_equipped_items = !game_state.viewing_equipped_items
            });
        }),
        GameActions::DeselectItem => Box::new(move || {
            game_dispatch.reduce_mut(|game_state| game_state.selected_item = None);
            game_dispatch.reduce_mut(|store| {
                let parent_page_number = store.parent_menu_pages.pop();
                if let Some(page_number) = parent_page_number {
                    store.action_menu_current_page_number = page_number;
                } else {
                    store.action_menu_current_page_number = 0
                };
                log!(format!(
                    "setting page number to :{} after deselecting item",
                    store.action_menu_current_page_number
                ));
                store.detailed_entity = None;
            });
        }),
        GameActions::SelectItem(id, _) => Box::new(move || {
            let item = get_item_owned_by_focused_character(&id, game_state.clone())
                .expect("a character should only be able to select their own items");
            let cloned_dispatch = game_dispatch.clone();
            select_item(cloned_dispatch, Some(item));
        }),
        GameActions::UseItem(id) => Box::new(move || {
            let cloned_game_state = game_state.clone();
            let cloned_game_dispatch = game_dispatch.clone();
            let cloned_ui_state = ui_state.clone();
            let cloned_websocket_state = websocket_state.clone();
            let cloned_alert_dispatch = alert_dispatch.clone();
            let cloned_lobby_state = lobby_state.clone();
            use_item_handler(
                cloned_game_dispatch,
                cloned_game_state,
                cloned_ui_state,
                cloned_websocket_state,
                cloned_alert_dispatch,
                cloned_lobby_state,
                &id,
            )
        }),
        GameActions::DeselectCombatAction => Box::new(move || {
            let cloned_game_dispatch = game_dispatch.clone();
            let cloned_alert_dispatch = alert_dispatch.clone();
            let cloned_lobby_state = lobby_state.clone();
            handle_select_combat_action(
                cloned_game_dispatch,
                cloned_alert_dispatch,
                cloned_lobby_state,
                &websocket_state.websocket,
                None,
            );
        }),
        GameActions::SelectCombatAction(combat_action) => Box::new(move || {
            let cloned_game_dispatch = game_dispatch.clone();
            let cloned_alert_dispatch = alert_dispatch.clone();
            let cloned_lobby_state = lobby_state.clone();
            handle_select_combat_action(
                cloned_game_dispatch,
                cloned_alert_dispatch,
                cloned_lobby_state,
                &websocket_state.websocket,
                Some(combat_action.clone()),
            );
        }),
        GameActions::CycleTargets(next_or_previous) => Box::new(move || {
            let cloned_dispatch = game_dispatch.clone();
            let cloned_lobby_state = lobby_state.clone();
            let _ = handle_cycle_combat_action_targets(
                cloned_dispatch,
                &websocket_state.websocket,
                cloned_lobby_state,
                &next_or_previous,
            );
        }),
        GameActions::CycleTargetingScheme => Box::new(move || {
            let cloned_dispatch = game_dispatch.clone();
            let cloned_lobby_state = lobby_state.clone();
            let _ = handle_cycle_combat_action_targeting_schemes(
                cloned_dispatch,
                &websocket_state.websocket,
                cloned_lobby_state,
            );
        }),
        GameActions::UseSelectedCombatAction => Box::new(move || {
            let cloned_dispatch = game_dispatch.clone();
            let cloned_alert_dispatch = alert_dispatch.clone();
            handle_use_selected_combat_action(
                cloned_dispatch,
                cloned_alert_dispatch,
                &websocket_state.websocket,
            );
        }),
        GameActions::DropItem(item_id) => Box::new(move || {
            game_dispatch.reduce_mut(|store| {
                let focused_character = get_focused_character(store).expect("to be in game");
                let slot_item_is_equipped = focused_character
                    .combatant_properties
                    .slot_item_is_equipped(&item_id);
                if let Some(slot) = slot_item_is_equipped {
                    send_client_input(
                        &websocket_state.websocket,
                        PlayerInputs::DropEquippedItem({
                            CharacterAndSlot {
                                character_id: store.focused_character_id,
                                slot,
                            }
                        }),
                    );
                } else {
                    send_client_input(
                        &websocket_state.websocket,
                        PlayerInputs::DropItem({
                            CharacterAndItem {
                                character_id: store.focused_character_id,
                                item_id,
                            }
                        }),
                    );
                }
                store.selected_item = None;
                store.detailed_entity = None;
                let parent_page_number = store.parent_menu_pages.pop();
                if let Some(page_number) = parent_page_number {
                    store.action_menu_current_page_number = page_number;
                } else {
                    store.action_menu_current_page_number = 0
                };
            });
        }),
        GameActions::ToggleReadyToDescend => Box::new(move || {
            send_client_input(
                &websocket_state.websocket,
                PlayerInputs::ToggleReadyToGoDownStairs,
            );
        }),
        _ => Box::new(|| ()),
    }
}

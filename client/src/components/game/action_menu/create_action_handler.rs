use super::{
    available_actions::GameActions, get_character_owned_item_by_id::get_character_owned_item_by_id,
};
use crate::{
    components::websocket_manager::send_client_input::send_client_input,
    store::{
        game_store::{get_focused_character, select_item, GameStore},
        ui_store::UIStore,
        websocket_store::WebsocketStore,
    },
};
use common::packets::client_to_server::{EquipItemRequest, PlayerInputs, UnequipSlotRequest};
use gloo::console::log;
use std::rc::Rc;
use yewdux::prelude::Dispatch;

pub fn create_action_handler<'a>(
    game_action: GameActions,
    game_dispatch: Dispatch<GameStore>,
    game_state: Rc<GameStore>,
    ui_state: Rc<UIStore>,
    websocket_state: Rc<WebsocketStore>,
) -> Box<dyn Fn()> {
    match game_action {
            GameActions::ToggleReadyToExplore => Box::new(|| (log!("ready to explore selected"))),
            GameActions::UseAutoinjector => Box::new(move || {
                // send_client_input(&websocket_state.websocket, PlayerInputs::RequestGameList)
            }),
            GameActions::SetInventoryOpen(status) =>Box::new(move || {
                game_dispatch.reduce_mut(|game_state| game_state.viewing_inventory = status.clone());
            }),
            GameActions::ToggleInventoryOpen => Box::new(move || {
                game_dispatch.reduce_mut(|game_state| game_state.viewing_inventory = !game_state.viewing_inventory);
                game_dispatch.reduce_mut(|game_state| game_state.viewing_equipped_items = false);
            }),
            GameActions::ToggleViewingEquipedItems => Box::new(move || {
                game_dispatch.reduce_mut(|game_state| game_state.viewing_equipped_items = !game_state.viewing_equipped_items);

            }),
            GameActions::DeselectItem => Box::new(move || {
                game_dispatch.reduce_mut(|game_state| game_state.selected_item = None);
                game_dispatch.reduce_mut(|store| {
                    let parent_page_number = store.parent_menu_pages.pop();
                    if let Some(page_number) = parent_page_number {
                        store.action_menu_current_page_number = page_number;
                    }
                    store.detailed_entity = None;
                });
            }),
            GameActions::SelectItem(id) => Box::new(move || {
                let item = get_character_owned_item_by_id(&id, game_state.clone())
                    .expect("a character should only be able to select their own items");
                let cloned_dispatch = game_dispatch.clone();
                select_item(cloned_dispatch, Some(item));
            }),
            GameActions::UseItem(_id) => Box::new(move || {
                let item_option = &game_state.selected_item;
                let character_id = game_state.focused_character_id;
                let alt_slot = ui_state.mod_key_held;
                if let Some(item) = item_option {
                    game_dispatch.reduce_mut(|game_store| {
                        let focused_character = get_focused_character(game_store)
                            .expect("to be in game");
                        let slot_item_is_equipped = focused_character.slot_item_is_equipped(&item.entity_properties.id);
                        if let Some(slot) = slot_item_is_equipped {
                        send_client_input(
                            &websocket_state.websocket,
                            PlayerInputs::UnequipEquipmentSlot(UnequipSlotRequest { character_id, slot}))
                        } else {
                            send_client_input(
                                &websocket_state.websocket,
                                PlayerInputs::EquipInventoryItem(
                                    EquipItemRequest { 
                                        character_id, 
                                        item_id: item.entity_properties.id,
                                        alt_slot 
                                    }
                                )
                            )
                        }
                    });
                }

            }),
            _ => Box::new(||())
            // GameActions::OpenTreasureChest => || (),
            // GameActions::TakeItem => || (),
            // GameActions::DropItem => || (),
            // GameActions::ShardItem => || (),
            // GameActions::Attack => || (),
            // GameActions::UseAbility(_) => || (),
            // GameActions::LevelUpAbility(_) => || (),
            // GameActions::SetAssignAttributePointsMenuOpen(_) => || (),
            // GameActions::AssignAttributePoint(_) => || (),
        }
}

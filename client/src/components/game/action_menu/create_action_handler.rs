use super::{
    available_actions::GameActions, get_character_owned_item_by_id::get_character_owned_item_by_id,
};
use crate::{
    components::websocket_manager::send_client_input::send_client_input,
    store::{game_store::GameStore, websocket_store::WebsocketStore},
};
use common::packets::client_to_server::PlayerInputs;
use gloo::console::log;
use std::rc::Rc;
use yewdux::prelude::Dispatch;

pub fn create_action_handler<'a>(
    game_action: GameActions,
    game_dispatch: Dispatch<GameStore>,
    game_state: Rc<GameStore>,
    websocket_state: Rc<WebsocketStore>,
) -> Box<dyn Fn()> {
    match game_action {
            GameActions::ToggleReadyToExplore => Box::new(|| (log!("ready to explore selected"))),
            GameActions::UseAutoinjector => Box::new(move || {
                send_client_input(&websocket_state.websocket, PlayerInputs::RequestGameList)
            }),
            GameActions::SetInventoryOpen(status) =>Box::new(move || {
                game_dispatch.reduce_mut(|game_state| game_state.viewing_inventory = status.clone());
            }),
            GameActions::ToggleInventoryOpen => Box::new(move || {
                game_dispatch.reduce_mut(|game_state| game_state.viewing_inventory = !game_state.viewing_inventory);
            }),
            GameActions::DeselectItem => Box::new(move || {
                game_dispatch.reduce_mut(|game_state| game_state.selected_item = None);
                game_dispatch.reduce_mut(|store| {
                    let parent_page_number = store.parent_menu_pages.pop();
                    if let Some(page_number) = parent_page_number {
                        store.action_menu_current_page_number = page_number;
                    }
                });
            }),
            GameActions::SelectItem(id) => Box::new(move || {
                let item = get_character_owned_item_by_id(&id, game_state.clone())
                    .expect("a character should only be able to select their own items");
                game_dispatch.reduce_mut(|store| store.selected_item = Some(item));
                game_dispatch.reduce_mut(|store| store.parent_menu_pages.push(store.action_menu_current_page_number));
                game_dispatch.reduce_mut(|store| store.action_menu_current_page_number = 0);
            }),
            _ => Box::new(||())
            // GameActions::OpenTreasureChest => || (),
            // GameActions::TakeItem => || (),
            // GameActions::UseItem => || (),
            // GameActions::DropItem => || (),
            // GameActions::ShardItem => || (),
            // GameActions::Attack => || (),
            // GameActions::UseAbility(_) => || (),
            // GameActions::LevelUpAbility(_) => || (),
            // GameActions::SetAssignAttributePointsMenuOpen(_) => || (),
            // GameActions::AssignAttributePoint(_) => || (),
        }
}

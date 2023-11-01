use super::available_actions::GameActions;
use crate::{
    components::websocket_manager::send_client_input::send_client_input,
    store::{game_store::GameStore, websocket_store::WebsocketStore},
};
use common::packets::client_to_server::PlayerInputs;
use gloo::console::log;
use std::rc::Rc;
use yewdux::prelude::Dispatch;

pub fn generate_action_menu_handlers(
    menu_items: Vec<GameActions>,
    game_dispatch: Dispatch<GameStore>,
    websocket_state: Rc<WebsocketStore>,
) -> Vec<Box<dyn Fn()>> {
    let mut handlers = Vec::new();
    for game_action in menu_items {
        let cloned_game_dispatch = game_dispatch.clone();
        let cloned_websocket_state = websocket_state.clone();
        let handler: Box<dyn Fn()> = match game_action {
            GameActions::ToggleReadyToExplore => Box::new(|| (log!("ready to explore selected"))),
            GameActions::UseAutoinjector => Box::new(move || {
                send_client_input(&cloned_websocket_state.websocket, PlayerInputs::RequestGameList)
            }),
            GameActions::SetInventoryOpen(status) =>Box::new(move || {
                cloned_game_dispatch.reduce_mut(|game_state| game_state.viewing_inventory = status.clone());
            }),
            GameActions::ToggleInventoryOpen => Box::new(move || {
                cloned_game_dispatch.reduce_mut(|game_state| game_state.viewing_inventory = !game_state.viewing_inventory);
            }),
            _ => Box::new(||())
            // GameActions::SelectItem(_) => || (),
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
        };
        handlers.push(handler);
    }

    handlers
}

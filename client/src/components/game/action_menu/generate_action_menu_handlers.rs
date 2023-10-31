use super::available_actions::GameActions;
use crate::store::{game_store::GameStore, websocket_store::WebsocketStore};
use gloo::console::log;
use std::rc::Rc;
use yewdux::prelude::Dispatch;

pub fn generate_action_menu_handlers(
    menu_items: Vec<GameActions>,
    game_dispatch: Dispatch<GameStore>,
    websocket_state: Rc<WebsocketStore>,
) -> Vec<fn()> {
    let mut handlers = Vec::new();
    for game_action in menu_items {
        let handler = match game_action {
            GameActions::ToggleReadyToExplore => || (log!("ready to explore selected")),
            GameActions::SetInventoryOpen(_) => || (),
            GameActions::ToggleInventoryOpen => || (),
            GameActions::UseAutoinjector => || (),
            GameActions::SelectItem(_) => || (),
            GameActions::OpenTreasureChest => || (),
            GameActions::TakeItem => || (),
            GameActions::UseItem => || (),
            GameActions::DropItem => || (),
            GameActions::ShardItem => || (),
            GameActions::Attack => || (),
            GameActions::UseAbility(_) => || (),
            GameActions::LevelUpAbility(_) => || (),
            GameActions::SetAssignAttributePointsMenuOpen(_) => || (),
            GameActions::AssignAttributePoint(_) => || (),
        };
        handlers.push(handler);
    }

    handlers
}

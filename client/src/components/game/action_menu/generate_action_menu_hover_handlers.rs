use std::rc::Rc;

use super::{
    available_actions::GameActions, get_character_owned_item_by_id::get_character_owned_item_by_id,
};
use crate::store::game_store::{DetailableEntities, GameStore};
use yewdux::prelude::Dispatch;

pub fn generate_action_menu_hover_handlers(
    menu_items: Vec<GameActions>,
    game_dispatch: Dispatch<GameStore>,
    game_state: Rc<GameStore>,
) -> Vec<Box<dyn Fn()>> {
    let mut handlers = Vec::new();
    for game_action in menu_items {
        let cloned_game_dispatch = game_dispatch.clone();
        let cloned_game_state = game_state.clone();

        let handler: Box<dyn Fn()> = match game_action {
            GameActions::UseAutoinjector => Box::new(move || {
                // send_client_input(&cloned_websocket_state.websocket, PlayerInputs::RequestGameList)
            }),
            GameActions::SelectItem(id) => Box::new(move || {
                let item = get_character_owned_item_by_id(&id, &cloned_game_state)
                    .expect("a character should only be able to select their own items");
                cloned_game_dispatch.reduce_mut(|store| store.hovered_entity = Some(DetailableEntities::Item(item)));
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
        };
        handlers.push(handler);
    }

    handlers
}

use super::enums::GameActions;
use crate::store::game_store::get_item_on_ground;
use crate::store::game_store::get_item_owned_by_focused_character;
use crate::store::game_store::DetailableEntities;
use crate::store::game_store::GameStore;
use std::rc::Rc;
use yewdux::prelude::Dispatch;

pub fn create_action_mouse_enter_handler(
    action: GameActions,
    game_dispatch: Dispatch<GameStore>,
    game_state: Rc<GameStore>,
) -> Box<dyn Fn()> {
    match action {
        GameActions::SelectItem(id) => Box::new(move || {
            let item_result = get_item_owned_by_focused_character(&id, game_state.clone());
            if let Ok(item) = item_result {
                game_dispatch.reduce_mut(|store| {
                    store.hovered_entity = Some(DetailableEntities::Item(item))
                });
            } else {
                let item_on_ground_result = get_item_on_ground(&id, game_state.clone());
                if let Ok(item) = item_on_ground_result {
                    game_dispatch.reduce_mut(|store| {
                        store.hovered_entity = Some(DetailableEntities::Item(item))
                    });
                }
            }
        }),
        _ => Box::new(|| ()),
    }
}

pub fn create_action_mouse_leave_handler(
    action: GameActions,
    game_dispatch: Dispatch<GameStore>,
) -> Box<dyn Fn()> {
    match action {
        GameActions::SelectItem(_id) => Box::new(move || {
            game_dispatch.reduce_mut(|store| store.hovered_entity = None);
        }),
        _ => Box::new(|| ()),
    }
}

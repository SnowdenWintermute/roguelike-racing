use super::{
    available_actions::GameActions, get_character_owned_item_by_id::get_character_owned_item_by_id,
};
use crate::store::game_store::{DetailableEntities, GameStore};
use std::rc::Rc;
use yewdux::prelude::Dispatch;

pub fn create_action_mouse_enter_handler(
    action: GameActions,
    game_dispatch: Dispatch<GameStore>,
    game_state: Rc<GameStore>,
) -> Box<dyn Fn()> {
    match action {
            GameActions::SelectItem(id) => Box::new(move || {
                let item = get_character_owned_item_by_id(&id, game_state.clone())
                    .expect("a character should only be able to select their own items");
                game_dispatch.reduce_mut(|store| store.hovered_entity = Some(DetailableEntities::Item(item)));
            }),
            _ => Box::new(||())
            // GameActions::OpenTreasureChest => || (),
            // GameActions::TakeItem => || (),
            // GameActions::UseItem => || (),
            // GameActions::DropItem => || (),
            // GameActions::ShardItem => || (),
            // GameActions::Attack => || (),
            // GameActions::LevelUpAbility(_) => || (),
            // GameActions::SetAssignAttributePointsMenuOpen(_) => || (),
            // GameActions::AssignAttributePoint(_) => || (),
        }
}

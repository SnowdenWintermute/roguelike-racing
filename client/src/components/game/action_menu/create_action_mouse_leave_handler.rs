use super::available_actions::GameActions;
use crate::store::game_store::GameStore;
use std::rc::Rc;
use yewdux::prelude::Dispatch;

pub fn create_action_mouse_leave_handler(
    action: GameActions,
    game_dispatch: Dispatch<GameStore>,
    _game_state: Rc<GameStore>,
) -> Box<dyn Fn()> {
    match action {
            GameActions::SelectItem(_id) => Box::new(move || {
                game_dispatch.reduce_mut(|store| store.hovered_entity = None);
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

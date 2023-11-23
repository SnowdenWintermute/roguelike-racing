use super::available_actions::GameActions;
use crate::store::game_store::GameStore;
use common::game::getters::get_character;
use std::{ops::Deref, rc::Rc};

pub fn determine_action_menu_buttons_disabled(
    action: &GameActions,
    game_state: &Rc<GameStore>,
) -> bool {
    match action {
        GameActions::UseItem(_) => {
            let item = &game_state
                .deref()
                .selected_item
                .as_ref()
                .expect("button should only be shown when item is selected");
            let game = &game_state
                .deref()
                .game
                .as_ref()
                .expect("game to be in progress");
            let current_party_id = game_state.clone().current_party_id.expect("party to exist");
            let focused_character_id = game_state.clone().focused_character_id;
            let focused_character = get_character(*game, current_party_id, focused_character_id)
                .expect("should always be a focused characer in a game");
            if !focused_character.combatant_properties.can_use_item(&item)
                && focused_character
                    .slot_item_is_equipped(&item.entity_properties.id)
                    .is_none()
            {
                return true;
            }
            false
        }
        _ => false,
    }
}

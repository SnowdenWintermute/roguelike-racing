use crate::store::game_store::GameStore;
use common::{app_consts::error_messages, errors::AppError, game::getters::get_party, items::Item};
use std::rc::Rc;

pub fn get_character_owned_item_by_id(
    id: &u32,
    game_state: &Rc<GameStore>,
) -> Result<Item, AppError> {
    let party_id = game_state
        .current_party_id
        .expect("only call this fn if char is in a party");
    let game = game_state
        .game
        .as_ref()
        .expect("only should be called in a game");
    let party = get_party(&game, party_id).expect("only should be called when in a party");
    let character = party
        .characters
        .get(&game_state.focused_character_id)
        .expect("only should be called in a game");

    for (_, item) in &character.combatant_properties.equipment {
        if item.entity_properties.id == *id {
            return Ok(item.clone());
        }
    }

    for item in &character.inventory.items {
        if item.entity_properties.id == *id {
            return Ok(item.clone());
        }
    }

    return Err(AppError {
        error_type: common::errors::AppErrorTypes::ClientError,
        message: error_messages::INVALID_ITEM_ID.to_string(),
    });
}

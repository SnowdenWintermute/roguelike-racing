use crate::yew_app::store::game_store::GameStore;
use std::rc::Rc;

pub fn combatant_is_ally(game_state: Rc<GameStore>, combatant_id: u32) -> bool {
    let mut value = false;
    if let Some(game) = &game_state.game {
        if let Some(party) = game
            .adventuring_parties
            .get(&game_state.current_party_id.expect("to be in a party"))
        {
            for (character_id, _) in party.characters.iter() {
                if character_id == &combatant_id {
                    value = true;
                }
            }
        }
    }
    value
}

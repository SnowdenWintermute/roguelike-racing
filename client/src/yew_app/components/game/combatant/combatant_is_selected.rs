use crate::yew_app::store::game_store::DetailableEntities;
use crate::yew_app::store::game_store::GameStore;
use std::rc::Rc;

pub fn combatant_is_selected(game_state: Rc<GameStore>, combatant_id: u32) -> bool {
    match &game_state.detailed_entity {
        Some(combatant_details) => match combatant_details {
            DetailableEntities::Combatant(combatant_details) => {
                combatant_details.entity_properties.id == combatant_id
            }
            DetailableEntities::Item(_) => false,
        },
        None => false,
    }
}

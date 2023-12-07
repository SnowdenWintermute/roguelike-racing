use crate::store::game_store::GameStore;
use common::{
    combatants::abilities::get_combatant_ability_attributes::{TargetCategories, TargetingScheme},
    game::getters::{get_mut_party, get_party},
};
use web_sys::WebSocket;

pub fn handle_cycle_targeting_schemes(
    game_store: &mut GameStore,
    websocket_option: &Option<WebSocket>,
) {
    let game = game_store
        .game
        .as_mut()
        .expect("only use abilities in game");
    let party_id = game_store
        .current_party_id
        .expect("only use abilities in party");
    let party = get_mut_party(game, party_id).expect("only use in party");
    let focused_character = party
        .characters
        .get_mut(&game_store.focused_character_id)
        .expect("to have a character");
    let focused_character_id = focused_character.entity_properties.id.clone();
    let ability_name = focused_character
        .combatant_properties
        .selected_ability_name
        .clone()
        .expect("to have an ability selected");
    let ability = focused_character
        .combatant_properties
        .abilities
        .get_mut(&ability_name)
        .expect("the character to have selected an ability they own");
    // if only one targeting scheme, return early
    let ability_attributes = ability.ability_name.get_attributes();
    if ability_attributes.targeting_schemes.len() < 2 {
        return;
    }
    match ability.selected_targeting_scheme {
        TargetingScheme::Single => ability.selected_targeting_scheme = TargetingScheme::Area,
        TargetingScheme::Area => ability.selected_targeting_scheme = TargetingScheme::Single,
    }

    let party = get_party(&game, party_id).expect("only use in party");
    let focused_character = party
        .characters
        .get(&game_store.focused_character_id)
        .expect("to have a character");
    let ability = focused_character
        .combatant_properties
        .abilities
        .get(&ability_name)
        .expect("the character to have selected an ability they own");
    let default_target_ids = ability.get_default_target_ids(&party, focused_character_id);

    // let new_target_ids = match ability.selected_targeting_scheme {
    //     TargetingScheme::Single => {
    //         if ability.targets_are_valid(&ability.most_recently_targeted, &party) {
    //             ability.most_recently_targeted.clone();
    //         } else {
    //             default_target_ids
    //         }
    //     }
    //     TargetingScheme::Area => ,
    // };
}

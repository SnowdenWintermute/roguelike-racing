use crate::store::game_store::get_active_combatant;
use crate::store::game_store::GameStore;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(CharacterAutofocusManager)]
pub fn character_autofocus_manager() -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let prev_active_combatant_id_option = use_state(|| None);
    let focused_character_id = game_state.focused_character_id;
    let active_combatant_result = get_active_combatant(&game_state);
    let active_combatant_id_option = match active_combatant_result {
        Ok(combatant_option) => match combatant_option {
            Some((entity_properties, _)) => Some(entity_properties.id),
            None => None,
        },
        Err(_) => None,
    };

    use_effect_with(
        (focused_character_id, active_combatant_id_option),
        move |_| {
            game_dispatch.reduce_mut(|store| match *prev_active_combatant_id_option {
                Some(prev_active_combatant_id) => match active_combatant_id_option {
                    Some(new_active_combatant_id) => {
                        let party_result = game_state.get_current_party();
                        if let Ok(party) = party_result {
                            if party.character_positions.contains(&new_active_combatant_id) {
                                if focused_character_id == prev_active_combatant_id {
                                    store.focused_character_id = new_active_combatant_id
                                }
                            }
                        }
                    }
                    None => (),
                },
                None => (),
            });
            prev_active_combatant_id_option.set(active_combatant_id_option)
        },
    );

    html!()
}

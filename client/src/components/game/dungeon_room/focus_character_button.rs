use crate::store::game_store::{get_focused_character, GameStore};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: u32,
}

#[function_component(FocusCharacterButton)]
pub fn focus_character_button(props: &Props) -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let focused_character_option = get_focused_character(&game_state);
    let is_focused_character = match focused_character_option {
        Ok(focused_character) => {
            if focused_character.entity_properties.id == props.id {
                true
            } else {
                false
            }
        }
        Err(_) => false,
    };

    let is_ally = {
        let mut value = false;
        if let Some(game) = &game_state.game {
            if let Some(party) = game
                .adventuring_parties
                .get(&game_state.current_party_id.expect("to be in a party"))
            {
                for (character_id, _) in party.characters.iter() {
                    if character_id == &props.id {
                        value = true;
                    }
                }
            }
        }
        value
    };

    html!(
        if is_focused_character {
            <div class="absolute bottom-0 right-0 border border-slate-400 p-2 z-10 bg-green-700" >
                {"focused"}
            </div>
        } else if is_ally {
            <button class="absolute bottom-0 right-0 border border-slate-400 p-2 z-10" >
            {"focus"}
            </button>
        }
    )
}

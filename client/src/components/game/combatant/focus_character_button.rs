use crate::store::game_store::get_focused_character;
use crate::store::game_store::GameStore;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: u32,
    pub is_ally: bool,
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

    let id = props.id;
    let handle_click =
        Callback::from(move |_| game_dispatch.reduce_mut(|store| store.focused_character_id = id));
    let conditional_styles = if is_focused_character {
        "bg-green-700"
    } else {
        ""
    };

    html!(
        <div class={format!("border-l border-slate-400 w-10 max-w-10 min-w-10 {}", conditional_styles)} >
            if is_focused_character {
                <button class="flex items-center justify-center w-full h-full">
                {"X"}
                </button>
            } else if props.is_ally {
                <button class="flex items-center justify-center w-full h-full m-0" onclick={handle_click} >
                {"O"}
                </button>
            }
        </div>
    )
}

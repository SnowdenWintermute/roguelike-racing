mod item_on_ground;
use crate::yew_app::components::common_components::atoms::divider::Divider;
use crate::yew_app::components::game::items_on_ground::item_on_ground::ItemOnGround;
use crate::yew_app::store::game_store::get_current_party_option;
use crate::yew_app::store::game_store::GameStore;
use crate::yew_app::store::lobby_store::LobbyStore;
use yew::prelude::*;
use yewdux::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub max_height: f32,
}

#[function_component(ItemsOnGround)]
pub fn items_on_ground(props: &Props) -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let (lobby_state, _) = use_store::<LobbyStore>();
    let party = get_current_party_option(&game_state);
    if !party.is_some() {
        return html!({ "no party found" });
    }
    let party = party.expect("none checked");
    let items_to_display = party.current_room.items.clone();
    let player_owns_character =
        party.player_owns_character(&lobby_state.username, game_state.focused_character_id);

    if items_to_display.len() < 1 {
        return html!();
    }

    html!(
    <div class="w-full border border-slate-400 bg-slate-700 p-2 pointer-events-auto flex flex-col h-fit"
         style={format!("max-height: {}rem", props.max_height)}
    >
        {"Items on the ground"}
        <Divider />
        <ul class="list-none flex-grow overflow-y-auto">
            {items_to_display.iter().map(|item|
                html!(
                        <ItemOnGround
                            id={item.entity_properties.id}
                            name={item.entity_properties.name.clone()}
                            disabled={!player_owns_character}
                        />
                    )
                ).collect::<Html>()}
        </ul>
    </div>
    )
}

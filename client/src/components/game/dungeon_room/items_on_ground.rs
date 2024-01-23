use crate::components::common_components::atoms::button_basic::ButtonBasic;
use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::game_store::get_current_party_option;
use crate::store::game_store::GameStore;
use crate::store::lobby_store::LobbyStore;
use crate::store::websocket_store::WebsocketStore;
use common::packets::client_to_server::PlayerInputs;
use common::packets::CharacterAndItem;
use yew::function_component;
use yew::prelude::*;
use yew::Html;
use yewdux::use_store;

#[function_component(ItemsOnGround)]
pub fn items_on_ground() -> Html {
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

    html!(
    <ul id="items on ground"
        class="list-none overflow-y-auto"
    >
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
    )
}

#[derive(Properties, PartialEq)]
pub struct ItemOnGroundProps {
    id: u32,
    name: String,
    disabled: bool,
}

#[function_component(ItemOnGround)]
pub fn item_on_ground(props: &ItemOnGroundProps) -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let (websocket_state, _) = use_store::<WebsocketStore>();

    let cloned_websocket_state = websocket_state.clone();
    let item_id = props.id;
    let focused_character_id = game_state.focused_character_id;
    let take_item = Callback::from(move |_| {
        send_client_input(
            &cloned_websocket_state.websocket,
            PlayerInputs::TakeItemOnGround(CharacterAndItem {
                character_id: focused_character_id,
                item_id,
            }),
        )
    });

    html!(
    <li class="h-10 w-full max-w-full flex border border-slate-400 mb-2 last:mb-0 whitespace-nowrap text-ellipsis overflow-hidden" >
        <ButtonBasic
            extra_styles="border-0 border-r hover:bg-slate-950 h-full"
            onclick={take_item}
            disabled={props.disabled}
        >
            {"Take"}
        </ButtonBasic>
        <div class="flex items-center h-full w-full ">
            <div class="pl-2 overflow-hidden whitespace-nowrap text-ellipsis ">
                {&props.name}{" "}{&props.id}
            </div>
        </div>
    </li>
    )
}
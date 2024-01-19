use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::game_store::get_current_party_option;
use crate::store::game_store::GameStore;
use crate::store::websocket_store::WebsocketStore;
use common::packets::client_to_server::PlayerInputs;
use common::packets::CharacterPickedUpItemPacket;
use gloo::console::log;
use yew::function_component;
use yew::prelude::*;
use yew::Html;
use yewdux::use_store;

#[function_component(ItemsOnGround)]
pub fn items_on_ground() -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let party = get_current_party_option(&game_state);
    if !party.is_some() {
        return html!({ "no party found" });
    }
    let party = party.expect("none checked");
    let items_to_display = party.current_room.items.clone().unwrap_or_default();

    html!(
    <ul id="items on ground"
        class="list-none"
    >
        {items_to_display.iter().map(|item|
            html!(
                    <ItemOnGround
                    id={item.entity_properties.id}
                    name={item.entity_properties.name.clone()}
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
            PlayerInputs::TakeItemOnGround(CharacterPickedUpItemPacket {
                character_id: focused_character_id,
                item_id,
            }),
        )
    });

    let cloned_websocket_state = websocket_state.clone();
    use_effect_with((), move |_| {
        log!("sending ack");
        send_client_input(
            &cloned_websocket_state.websocket,
            PlayerInputs::AcknowledgeReceiptOfItemOnGroundUpdate(item_id),
        )
    });

    html!(
    <li>
        <button onclick={take_item}>
            {"Take"}
        </button>
        <div>
            {&props.name}{" "}{&props.id}
        </div>
    </li>
    )
}

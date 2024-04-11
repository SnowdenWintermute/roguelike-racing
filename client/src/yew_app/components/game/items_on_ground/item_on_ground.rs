use common::packets::client_to_server::PlayerInputs;
use common::packets::CharacterAndItem;
use yew::prelude::*;
use yewdux::use_store;

use crate::yew_app::components::game::action_menu::action_button_hover_handlers::create_action_mouse_enter_handler;
use crate::yew_app::components::game::action_menu::action_button_hover_handlers::create_action_mouse_leave_handler;
use crate::yew_app::components::game::action_menu::enums::GameActions;
use crate::yew_app::components::websocket_manager::send_client_input::send_client_input;
use crate::yew_app::store::game_store::GameStore;
use crate::yew_app::store::websocket_store::WebsocketStore;

#[derive(Properties, PartialEq)]
pub struct ItemOnGroundProps {
    pub id: u32,
    pub name: String,
    pub disabled: bool,
}

#[function_component(ItemOnGround)]
pub fn item_on_ground(props: &ItemOnGroundProps) -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let (websocket_state, _) = use_store::<WebsocketStore>();

    let cloned_websocket_state = websocket_state.clone();
    let item_id = props.id;
    let focused_character_id = game_state.focused_character_id;
    let cloned_game_dispatch = game_dispatch.clone();
    let take_item = Callback::from(move |_| {
        cloned_game_dispatch.reduce_mut(|store| {
            store.hovered_entity = None;
        });
        send_client_input(
            &cloned_websocket_state.websocket,
            PlayerInputs::TakeItemOnGround(CharacterAndItem {
                character_id: focused_character_id,
                item_id,
            }),
        )
    });
    let cloned_game_state = game_state.clone();
    let cloned_game_dispatch = game_dispatch.clone();
    let mouse_enter_handler = Callback::from(move |_| {
        create_action_mouse_enter_handler(
            GameActions::SelectItem(item_id, 1),
            cloned_game_dispatch.clone(),
            cloned_game_state.clone(),
        )()
    });
    let cloned_game_dispatch = game_dispatch.clone();
    let mouse_leave_handler = Callback::from(move |_| {
        create_action_mouse_leave_handler(
            GameActions::SelectItem(item_id, 1),
            cloned_game_dispatch.clone(),
        )()
    });
    let cloned_game_state = game_state.clone();
    let cloned_game_dispatch = game_dispatch.clone();
    let focus_handler = Callback::from(move |_| {
        create_action_mouse_enter_handler(
            GameActions::SelectItem(item_id, 1),
            cloned_game_dispatch.clone(),
            cloned_game_state.clone(),
        )()
    });
    let cloned_game_dispatch = game_dispatch.clone();
    let blur_handler = Callback::from(move |_| {
        create_action_mouse_leave_handler(
            GameActions::SelectItem(item_id, 1),
            cloned_game_dispatch.clone(),
        )()
    });

    html!(
    <li class="h-10 w-full max-w-full flex border-r border-l border-b border-slate-400 first:border-t
    hover:border-white hover:border-t box-border
    whitespace-nowrap text-ellipsis overflow-hidden cursor-default"
        onmouseenter={mouse_enter_handler}
        onmouseleave={mouse_leave_handler}
        >
        <button
            class="cursor-pointer pr-4 pl-4 box-border
            flex justify-center items-center disabled:opacity-50 disabled:cursor-auto
            border-slate-400 border-r h-full hover:bg-slate-950"
            onclick={take_item}
            onfocus={focus_handler}
            onblur={blur_handler}
            disabled={props.disabled}
        >
            {"Take"}
        </button>
        <div class="flex items-center h-full w-full "
        >
            <div class="pl-2 overflow-hidden whitespace-nowrap text-ellipsis ">
                {&props.name}{" "}
            </div>
        </div>
    </li>
    )
}

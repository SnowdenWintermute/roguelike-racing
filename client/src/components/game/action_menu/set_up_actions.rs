use super::create_action_handler::create_action_handler;
use super::create_action_mouse_enter_handler::create_action_mouse_enter_handler;
use super::create_action_mouse_leave_handler::create_action_mouse_leave_handler;
use super::determine_action_menu_buttons_disabled::determine_action_menu_buttons_disabled;
use super::generate_action_menu_items;
use super::generate_button_text::generate_button_text;
use crate::store::game_store::GameStore;
use crate::store::lobby_store::LobbyStore;
use crate::store::ui_store::UIStore;
use crate::store::websocket_store::WebsocketStore;
use common::adventuring_party::AdventuringParty;
use std::rc::Rc;
use web_sys::FocusEvent;
use web_sys::MouseEvent;
use yew::Callback;
use yewdux::prelude::Dispatch;

#[derive(PartialEq, Clone, Debug)]
pub struct ActionMenuButtonProperties {
    pub text: String,
    pub click_handler: Callback<MouseEvent>,
    pub focus_handler: Callback<FocusEvent>,
    pub blur_handler: Callback<FocusEvent>,
    pub mouse_enter_handler: Callback<MouseEvent>,
    pub mouse_leave_handler: Callback<MouseEvent>,
    pub should_be_disabled: bool,
}

pub fn set_up_actions(
    websocket_state: Rc<WebsocketStore>,
    game_state: Rc<GameStore>,
    game_dispatch: Dispatch<GameStore>,
    ui_state: Rc<UIStore>,
    lobby_state: Rc<LobbyStore>,
    party: &AdventuringParty,
) -> Vec<ActionMenuButtonProperties> {
    let new_actions =
        generate_action_menu_items::generate_action_menu_items(&game_state, &lobby_state, party);
    let mut button_properties = Vec::new();

    for action in new_actions {
        let cloned_websocket_state = websocket_state.clone();
        let cloned_action = action.clone();
        let cloned_game_state = game_state.clone();
        let cloned_game_dispatch = game_dispatch.clone();
        let cloned_ui_state = ui_state.clone();
        let click_handler = Callback::from(move |_| {
            create_action_handler(
                cloned_action.clone(),
                cloned_game_dispatch.clone(),
                cloned_game_state.clone(),
                cloned_ui_state.clone(),
                cloned_websocket_state.clone(),
            )()
        });
        let cloned_action = action.clone();
        let cloned_game_state = game_state.clone();
        let cloned_game_dispatch = game_dispatch.clone();
        let mouse_enter_handler = Callback::from(move |_| {
            create_action_mouse_enter_handler(
                cloned_action.clone(),
                cloned_game_dispatch.clone(),
                cloned_game_state.clone(),
            )()
        });
        let cloned_action = action.clone();
        let cloned_game_state = game_state.clone();
        let cloned_game_dispatch = game_dispatch.clone();
        let mouse_leave_handler = Callback::from(move |_| {
            create_action_mouse_leave_handler(
                cloned_action.clone(),
                cloned_game_dispatch.clone(),
                cloned_game_state.clone(),
            )()
        });
        let cloned_action = action.clone();
        let cloned_game_state = game_state.clone();
        let cloned_game_dispatch = game_dispatch.clone();
        let focus_handler = Callback::from(move |_| {
            create_action_mouse_enter_handler(
                cloned_action.clone(),
                cloned_game_dispatch.clone(),
                cloned_game_state.clone(),
            )()
        });
        let cloned_action = action.clone();
        let cloned_game_state = game_state.clone();
        let cloned_game_dispatch = game_dispatch.clone();
        let blur_handler = Callback::from(move |_| {
            create_action_mouse_leave_handler(
                cloned_action.clone(),
                cloned_game_dispatch.clone(),
                cloned_game_state.clone(),
            )()
        });
        let cloned_game_state = game_state.clone();
        let text = generate_button_text(action.clone(), cloned_game_state);

        let should_be_disabled =
            determine_action_menu_buttons_disabled(&action, &game_state, &lobby_state);

        button_properties.push(ActionMenuButtonProperties {
            text,
            click_handler,
            mouse_enter_handler,
            mouse_leave_handler,
            focus_handler,
            blur_handler,
            should_be_disabled,
        })
    }

    button_properties
}

use super::action_button_click_handlers::create_action_button_click_handler;
use super::action_button_hover_handlers::create_action_mouse_enter_handler;
use super::action_button_hover_handlers::create_action_mouse_leave_handler;
use super::action_menu_button::determine_action_button_text::determine_action_button_text;
use super::determine_action_menu_buttons_disabled::determine_action_menu_buttons_disabled;
use super::determine_menu_actions::determine_menu_actions;
use crate::store::alert_store::AlertStore;
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

pub fn build_action_button_properties(
    websocket_state: Rc<WebsocketStore>,
    game_state: Rc<GameStore>,
    game_dispatch: Dispatch<GameStore>,
    alert_dispatch: Dispatch<AlertStore>,
    ui_state: Rc<UIStore>,
    lobby_state: Rc<LobbyStore>,
    party: &AdventuringParty,
) -> Vec<ActionMenuButtonProperties> {
    let new_actions = determine_menu_actions(&game_state, &lobby_state, party);
    let mut button_properties = Vec::new();

    for action in new_actions {
        let cloned_websocket_state = websocket_state.clone();
        let cloned_action = action.clone();
        let cloned_game_state = game_state.clone();
        let cloned_game_dispatch = game_dispatch.clone();
        let cloned_ui_state = ui_state.clone();
        let cloned_lobby_state = lobby_state.clone();
        let cloned_alert_dispatch = alert_dispatch.clone();
        let click_handler = Callback::from(move |_| {
            create_action_button_click_handler(
                cloned_action.clone(),
                cloned_game_dispatch.clone(),
                cloned_game_state.clone(),
                cloned_ui_state.clone(),
                cloned_lobby_state.clone(),
                cloned_websocket_state.clone(),
                cloned_alert_dispatch.clone(),
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
        let cloned_game_dispatch = game_dispatch.clone();
        let mouse_leave_handler = Callback::from(move |_| {
            create_action_mouse_leave_handler(cloned_action.clone(), cloned_game_dispatch.clone())()
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
        let cloned_game_dispatch = game_dispatch.clone();
        let blur_handler = Callback::from(move |_| {
            create_action_mouse_leave_handler(cloned_action.clone(), cloned_game_dispatch.clone())()
        });
        let cloned_game_state = game_state.clone();
        let text = determine_action_button_text(action.clone(), cloned_game_state);

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

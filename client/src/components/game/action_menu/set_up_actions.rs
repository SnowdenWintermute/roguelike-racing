use super::{
    available_actions::GameActions, generate_action_menu_handlers::generate_action_menu_handlers,
    generate_action_menu_hover_handlers::generate_action_menu_hover_handlers,
    generate_action_menu_items::generate_action_menu_items,
};
use crate::store::{
    game_store::{DetailableEntities, GameStore},
    websocket_store::WebsocketStore,
};
use common::adventuring_party::AdventuringParty;
use std::rc::Rc;
use web_sys::{FocusEvent, MouseEvent};
use yew::{Callback, UseStateHandle};
use yewdux::prelude::Dispatch;

pub struct ActionMenuButtonProperties {
    text: String,
    click_handler: Callback<MouseEvent>,
    focus_handler: Callback<FocusEvent>,
    blur_handler: Callback<FocusEvent>,
    mouse_enter_handler: Callback<MouseEvent>,
    mouse_leave_handler: Callback<MouseEvent>,
    detailed_entity: DetailableEntities,
}

pub fn set_up_actions(
    websocket_state: Rc<WebsocketStore>,
    game_state: Rc<GameStore>,
    game_dispatch: Dispatch<GameStore>,
    actions_state: UseStateHandle<Vec<GameActions>>,
    handlers_state: UseStateHandle<Vec<Box<dyn Fn()>>>,
    hover_handlers_state: UseStateHandle<Vec<Box<dyn Fn()>>>,
    party: AdventuringParty,
) {
    let new_actions = generate_action_menu_items(game_state.clone(), &party);
    actions_state.set(new_actions.clone());

    let new_handlers = generate_action_menu_handlers(
        new_actions,
        game_dispatch.clone(),
        game_state.clone(),
        websocket_state,
    );
    handlers_state.set(new_handlers);

    let new_actions = generate_action_menu_items(game_state.clone(), &party);

    let new_hover_handlers =
        generate_action_menu_hover_handlers(new_actions, game_dispatch.clone(), game_state);
    hover_handlers_state.set(new_hover_handlers);
}

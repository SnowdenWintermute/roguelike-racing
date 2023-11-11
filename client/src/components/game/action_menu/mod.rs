mod action_menu_button;
mod available_actions;
mod generate_action_menu_handlers;
pub mod generate_action_menu_hover_handlers;
mod generate_action_menu_items;
mod generate_button_text;
mod get_character_owned_item_by_id;
mod set_keyup_listeners;
mod set_up_actions;
use crate::{
    components::game::action_menu::{
        action_menu_button::ActionMenuButton, available_actions::GameActions,
    },
    store::{game_store::GameStore, websocket_store::WebsocketStore},
};
use common::adventuring_party::AdventuringParty;
use gloo::events::EventListener;
use std::ops::Deref;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, Eq, PartialEq)]
pub struct Props {
    pub adventuring_party: AdventuringParty,
}

#[function_component(ActionMenu)]
pub fn action_menu(props: &Props) -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let (websocket_state, _) = use_store::<WebsocketStore>();
    let actions_state = use_state(|| Vec::<GameActions>::new());
    let page = use_state(|| 1 as u8);
    let page_size = 6;
    let handlers_state = use_state(|| Vec::new());
    let hover_handlers_state = use_state(|| Vec::new());
    let party = props.adventuring_party.clone();

    let cloned_handlers_state = handlers_state.clone();
    let cloned_hover_handlers_state = hover_handlers_state.clone();
    let cloned_game_state = game_state.clone();
    let cloned_game_dispatch = game_dispatch.clone();
    let cloned_actions_state = actions_state.clone();
    let cloned_websocket_state = websocket_state.clone();
    use_effect_with(
        (
            game_state.focused_character_id,
            game_state.viewing_inventory,
            game_state.selected_item.is_some(),
            game_state.viewing_items_on_ground,
            game_state.viewing_skill_level_up_menu,
            game_state.viewing_attribute_point_assignment_menu,
            party.current_room.monsters.is_some(),
        ),
        move |_| {
            set_up_actions::set_up_actions(
                cloned_websocket_state,
                cloned_game_state,
                cloned_game_dispatch,
                cloned_actions_state,
                cloned_handlers_state,
                cloned_hover_handlers_state,
                party,
            )
        },
    );

    let keyup_listener_state = use_state(|| None::<EventListener>);
    let cloned_handlers = handlers_state.clone();
    let num_actions = actions_state.len();
    use_effect_with(num_actions, move |_| {
        set_keyup_listeners::set_keyup_listeners(cloned_handlers, keyup_listener_state, num_actions)
    });

    html!(
        <section class="w-1/3 max-w-[733px] border border-slate-400 bg-slate-700 mr-4 overflow-y-auto">
        {actions_state.deref().iter().enumerate().map(|(i, action)| {
            // let i = *i;
            let cloned_game_state = game_state.clone();
            let button_text = generate_button_text::generate_button_text(&action, cloned_game_state);

            let cloned_click_handlers = handlers_state.clone();
            let click_handler = Callback::from(move |_| {
                cloned_click_handlers[i]()
            });
            let cloned_hover_handlers = hover_handlers_state.clone();
            let mouse_enter_handler = Callback::from(move |_| {
                cloned_hover_handlers[i]()
            });
            let cloned_game_dispatch = game_dispatch.clone();
            let mouse_leave_handler = Callback::from(move |_| {
                cloned_game_dispatch.reduce_mut(|store| store.hovered_entity = None)
            });
            let cloned_hover_handlers = hover_handlers_state.clone();
            let focus_handler = Callback::from(move |_| {
                cloned_hover_handlers[i]()
            });
            let cloned_game_dispatch = game_dispatch.clone();
            let blur_handler =  Callback::from(move |_| {
                cloned_game_dispatch.reduce_mut(|store| store.hovered_entity = None)
            });


              html!(
                  <ActionMenuButton
                    number={i+1}
                    click_handler={click_handler}
                    focus_handler={focus_handler}
                    mouse_enter_handler={mouse_enter_handler}
                    mouse_leave_handler={mouse_leave_handler}
                    blur_handler={blur_handler}
                    button_text={button_text}
                  />
                  )
              }).collect::<Html>() }
        </section>
    )
}

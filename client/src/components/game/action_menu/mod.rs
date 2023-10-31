pub mod available_actions;
pub mod generate_action_menu_handlers;
pub mod generate_action_menu_items;
use gloo_utils::window;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
mod generate_button_text;
use crate::{
    components::{
        common_components::atoms::button_blank::ButtonBlank,
        game::action_menu::available_actions::GameActions,
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
    let handlers_state = use_state(|| Vec::new());
    let viewing_inventory = game_state.viewing_inventory.clone();

    let party = props.adventuring_party.clone();
    let cloned_actions_state = actions_state.clone();
    let cloned_handlers_state = handlers_state.clone();
    use_effect_with(game_state.focused_character_id, move |_| {
        let new_actions =
            generate_action_menu_items::generate_action_menu_items(game_state, &party);
        cloned_actions_state.set(new_actions.clone());

        let new_handlers = generate_action_menu_handlers::generate_action_menu_handlers(
            new_actions,
            game_dispatch,
            websocket_state,
        );
        cloned_handlers_state.set(new_handlers);
    });

    let keyup_listener_state = use_state(|| None::<EventListener>);
    let cloned_handlers = handlers_state.clone();
    use_effect_with(actions_state.len(), move |_| {
        let listener = EventListener::new(&window(), "keyup", move |event| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            for i in 0..=6 {
                let key = (i + 1).to_string();
                if event.key() == key {
                    cloned_handlers[i]()
                }
            }
        });
        keyup_listener_state.set(Some(listener));
    });

    html!(
        <section class="w-1/3 max-w-[733px] border border-slate-400 bg-slate-700 mr-4 overflow-y-auto">
        {actions_state.deref().iter().enumerate().map(|(i, action)| {
        let button_text = generate_button_text::generate_button_text(&action);
        let cloned_handlers = handlers_state.clone();
        let handler = Callback::from(move |_| {
            cloned_handlers[i]()
        });

          html!(
              <ButtonBlank class="h-10 w-full border-b border-slate-400 flex items-center hover:bg-slate-950"
              onclick={handler}
              >
                  <span class="h-full w-10 border-r border-slate-400 flex justify-center items-center mr-2">{i+1}</span> {button_text}
              </ButtonBlank>
              )
          }).collect::<Html>() }
        <div>{"Inventory is open: "}{if viewing_inventory {
            "true"
        } else {
            "false"
        }
        }</div>
        </section>
    )
}

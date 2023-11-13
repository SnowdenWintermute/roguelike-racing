use crate::store::game_store::GameStore;
use gloo::events::EventListener;
use gloo_utils::window;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub number_of_pages: usize,
}

#[function_component(ActionPageButtons)]
pub fn action_page_buttons(props: &Props) -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();

    let cloned_game_dispatch = game_dispatch.clone();
    let cloned_current_page_number = game_state.action_menu_current_page_number.clone();
    let cloned_number_of_pages = props.number_of_pages.clone();
    let next_page = move || {
        if cloned_current_page_number as usize == cloned_number_of_pages - 1 {
            cloned_game_dispatch.reduce_mut(|store| store.action_menu_current_page_number = 0)
        } else {
            cloned_game_dispatch.reduce_mut(|store| {
                store.action_menu_current_page_number = cloned_current_page_number + 1
            })
        }
    };

    let cloned_game_dispatch = game_dispatch.clone();
    let cloned_current_page_number = game_state.action_menu_current_page_number.clone();
    let cloned_number_of_pages = props.number_of_pages.clone();
    let prev_page = move || {
        if cloned_current_page_number as usize == 0 {
            let new_page_number = cloned_number_of_pages - 1;
            cloned_game_dispatch
                .reduce_mut(|store| store.action_menu_current_page_number = new_page_number as u8)
        } else {
            cloned_game_dispatch.reduce_mut(|store| {
                store.action_menu_current_page_number = cloned_current_page_number - 1
            })
        }
    };

    let cloned_prev_page = prev_page.clone();
    let cloned_next_page = next_page.clone();
    let next_page_callback = Callback::from(move |_| cloned_next_page());
    let prev_page_callback = Callback::from(move |_| cloned_prev_page());

    let keyup_listener_state = use_state(|| None::<EventListener>);
    let cloned_prev_page = prev_page.clone();
    let cloned_next_page = next_page.clone();
    use_effect_with(game_state.action_menu_current_page_number, move |_| {
        let listener = EventListener::new(&window(), "keyup", move |event| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            if event.code() == "KeyW" {
                cloned_prev_page();
            }
            if event.code() == "KeyE" {
                cloned_next_page();
            }
        });
        keyup_listener_state.set(Some(listener));
    });

    html!(
        <div class="h-10 border-t border-slate-400 flex">
            <button
                onclick={prev_page_callback}
                class="h-full w-1/2 flex items-center justify-center">
                    {"Prev"}
            </button >
            <div class="h-full w-[1px] bg-slate-400"/>
            <button
                onclick={next_page_callback}
                class="h-full w-1/2">
                    {"Next"}
            </button>
        </div>
    )
}

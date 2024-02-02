pub mod page_turning;
use crate::components::game::action_menu::action_menu_page_buttons::page_turning::next_page;
use crate::components::game::action_menu::action_menu_page_buttons::page_turning::prev_page;
use crate::store::game_store::GameStore;
use gloo::events::EventListener;
use gloo_utils::window;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
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
    let next_page_callback = Callback::from(move |_| {
        next_page(
            cloned_game_dispatch.clone(),
            cloned_current_page_number,
            cloned_number_of_pages,
        )
    });
    let cloned_game_dispatch = game_dispatch.clone();
    let prev_page_callback = Callback::from(move |_| {
        prev_page(
            cloned_game_dispatch.clone(),
            cloned_current_page_number,
            cloned_number_of_pages,
        )
    });

    let keyup_listener_state = use_state(|| None::<EventListener>);
    use_effect_with(game_state.action_menu_current_page_number, move |_| {
        let listener = EventListener::new(&window(), "keypress", move |event| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            if event.code() == "KeyW" {
                prev_page(
                    game_dispatch.clone(),
                    cloned_current_page_number,
                    cloned_number_of_pages,
                )
            }
            if event.code() == "KeyR" {
                next_page(
                    game_dispatch.clone(),
                    cloned_current_page_number,
                    cloned_number_of_pages,
                )
            }
        });
        keyup_listener_state.set(Some(listener));
    });

    html!(
        <div class="h-10 min-h-10 border-t border-slate-400 flex">
            <button
                onclick={prev_page_callback}
                class="h-full w-[40%] flex items-center justify-center">
                    {"Prev"}
            </button >
            <div class="h-full w-[20%] border-r border-l border-slate-400 flex items-center justify-center">
                {(cloned_current_page_number + 1).to_string()}{"/"}{cloned_number_of_pages.to_string()}
            </div>
            <button
                onclick={next_page_callback}
                class="h-full w-[40%]">
                    {"Next"}
            </button>
        </div>
    )
}

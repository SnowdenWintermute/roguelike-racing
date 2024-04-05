pub mod page_turning;
use crate::yew_app::components::game::action_menu::action_menu_page_buttons::page_turning::next_page;
use crate::yew_app::components::game::action_menu::action_menu_page_buttons::page_turning::prev_page;
use crate::yew_app::components::game::tailwind_class_loader::BUTTON_HEIGHT_SMALL;
use crate::yew_app::store::game_store::GameStore;
use gloo::events::EventListener;
use gloo_utils::window;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub number_of_pages: usize,
    pub hidden: bool,
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

    let keypress_listener_state = use_state(|| None::<EventListener>);
    let keyup_listener_state = use_state(|| None::<EventListener>);
    let cloned_game_dispatch = game_dispatch.clone();
    let number_of_pages = props.number_of_pages;
    use_effect_with(
        (number_of_pages, game_state.action_menu_current_page_number),
        move |_| {
            let keypress_listener = EventListener::new(&window(), "keypress", move |event| {
                let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
                if number_of_pages < 1 {
                    return;
                }
                if event.code() == "KeyW" {
                    prev_page(
                        cloned_game_dispatch.clone(),
                        cloned_current_page_number,
                        cloned_number_of_pages,
                    )
                }
                if event.code() == "KeyE" {
                    next_page(
                        cloned_game_dispatch.clone(),
                        cloned_current_page_number,
                        cloned_number_of_pages,
                    )
                }
            });
            keypress_listener_state.set(Some(keypress_listener));
            let keyup_listener = EventListener::new(&window(), "keyup", move |event| {
                let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
                if number_of_pages < 1 {
                    return;
                }
                if event.code() == "ArrowLeft" {
                    prev_page(
                        game_dispatch.clone(),
                        cloned_current_page_number,
                        cloned_number_of_pages,
                    )
                }
                if event.code() == "ArrowRight" {
                    next_page(
                        game_dispatch.clone(),
                        cloned_current_page_number,
                        cloned_number_of_pages,
                    )
                }
            });
            keyup_listener_state.set(Some(keyup_listener));
        },
    );

    let hidden_style = if props.hidden {
        "opacity-0 pointer-events-none"
    } else {
        ""
    };

    html!(
        <ul class={ format!( "flex list-none border border-slate-400 bg-slate-700 w-full justify-between items-center {hidden_style}" )}>
            <button
                onclick={prev_page_callback}
                class="pr-2 pl-2"
                style={format!("height: {}rem; ", BUTTON_HEIGHT_SMALL)}
                >
                    {"Previous page (W)"}
            </button >
            <div class="h-full">
                {(cloned_current_page_number + 1).to_string()}{"/"}{cloned_number_of_pages.to_string()}
            </div>
            <button
                onclick={next_page_callback}
                class="pr-2 pl-2"
                style={format!("height: {}rem; ", BUTTON_HEIGHT_SMALL)}
                >
                    {"Next page (E)"}
            </button>
        </ul>
    )
}

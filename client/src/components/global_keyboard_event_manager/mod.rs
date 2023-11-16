use crate::store::ui_store::UIStore;
use gloo::events::EventListener;
use gloo_utils::window;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(GlobalKeyboardEventManager)]
pub fn global_keyboard_event_manager() -> Html {
    let (_, ui_dispatch) = use_store::<UIStore>();

    let keydown_listener_state = use_state(|| None::<EventListener>);
    let keyup_listener_state = use_state(|| None::<EventListener>);
    let cloned_ui_dispatch = ui_dispatch.clone();
    use_effect_with((), move |_| {
        let listener = EventListener::new(&window(), "keydown", move |event| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            if event.code() == "ShiftLeft" || event.code() == "ShiftRight" {
                cloned_ui_dispatch.reduce_mut(|store| store.mod_key_held = true)
            }
        });
        keydown_listener_state.set(Some(listener));
    });
    let cloned_ui_dispatch = ui_dispatch.clone();
    use_effect_with((), move |_| {
        let listener = EventListener::new(&window(), "keyup", move |event| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            if event.code() == "ShiftLeft" || event.code() == "ShiftRight" {
                cloned_ui_dispatch.reduce_mut(|store| store.mod_key_held = false)
            }
        });
        keyup_listener_state.set(Some(listener));
    });

    html!()
}

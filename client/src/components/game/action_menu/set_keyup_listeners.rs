use super::set_up_actions::ActionMenuButtonProperties;
use gloo::events::EventListener;
use gloo_utils::window;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::MouseEvent;
use yew::UseStateHandle;

pub fn set_keyup_listeners(
    button_properties_state: UseStateHandle<Vec<ActionMenuButtonProperties>>,
    keyup_listener_state: UseStateHandle<Option<EventListener>>,
) {
    let listener = EventListener::new(&window(), "keypress", move |event| {
        let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
        for (i, properties) in button_properties_state.iter().enumerate() {
            let key = (i + 1).to_string();
            if event.key() == key {
                properties
                    .click_handler
                    .emit(MouseEvent::new("mouseup").unwrap_throw());
            }
        }
    });
    keyup_listener_state.set(Some(listener));
}

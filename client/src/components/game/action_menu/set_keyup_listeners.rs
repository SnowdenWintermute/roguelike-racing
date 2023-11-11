use gloo::events::EventListener;
use gloo_utils::window;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use yew::UseStateHandle;

pub fn set_keyup_listeners(
    handlers_state: UseStateHandle<Vec<Box<dyn Fn()>>>,
    keyup_listener_state: UseStateHandle<Option<EventListener>>,
    num_actions: usize,
) {
    let listener = EventListener::new(&window(), "keyup", move |event| {
        let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
        for i in 0..num_actions {
            let key = (i + 1).to_string();
            if event.key() == key {
                handlers_state[i]()
            }
        }
    });
    keyup_listener_state.set(Some(listener));
}

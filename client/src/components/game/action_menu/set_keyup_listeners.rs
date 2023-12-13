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
            if properties.should_be_disabled {
                continue;
            }
            let key = (i + 1).to_string();
            let event_key_as_number = shifted_number_key_to_number_key(event.key());
            if event_key_as_number == key {
                properties
                    .click_handler
                    .emit(MouseEvent::new("mouseup").unwrap_throw());
            }
        }
    });
    keyup_listener_state.set(Some(listener));
}

fn shifted_number_key_to_number_key(key: String) -> String {
    let key_str = key.as_str();
    let key_as_number = match key_str {
        "!" => "1",
        "@" => "2",
        "#" => "3",
        "$" => "4",
        "%" => "5",
        "^" => "6",
        _ => key.as_str(),
    };
    key_as_number.to_string()
}

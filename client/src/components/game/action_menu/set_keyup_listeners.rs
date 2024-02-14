use super::build_action_button_properties::ActionMenuButtonProperties;
use gloo::console::log;
use gloo::events::EventListener;
use gloo_utils::window;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::MouseEvent;
use yew::UseStateHandle;

#[derive(Debug, Clone, PartialEq)]
pub enum GameKeys {
    Cancel,
    Confirm,
    Next,
    Previous,
}

pub fn set_keyup_listeners(
    button_properties_state: UseStateHandle<Vec<ActionMenuButtonProperties>>,
    keyup_listener_state: UseStateHandle<Option<EventListener>>,
) {
    let listener = EventListener::new(&window(), "keyup", move |event| {
        let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
        let mut next_number_to_assign = 1;
        let key_pressed = event.code();
        log!(format!("key pressed: {key_pressed}"));

        for (_, properties) in button_properties_state.iter().enumerate() {
            if properties.should_be_disabled {
                continue;
            }

            let assigned_keys =
                if let Some(dedicated_key_for_action) = &properties.dedicated_key_option {
                    match dedicated_key_for_action {
                        GameKeys::Cancel => vec!["Escape".to_string()],
                        GameKeys::Confirm => vec!["KeyR".to_string(), "Enter".to_string()],
                        GameKeys::Next => vec!["KeyE".to_string(), "ArrowRight".to_string()],
                        GameKeys::Previous => vec!["KeyW".to_string(), "ArrowLeft".to_string()],
                    }
                } else {
                    let number_to_assign_as_string = next_number_to_assign.to_string();
                    next_number_to_assign += 1;
                    vec![format!("Digit{number_to_assign_as_string}")]
                };
            if assigned_keys.contains(&key_pressed) {
                properties
                    .click_handler
                    .emit(MouseEvent::new("mouseup").unwrap_throw());
            }
        }
    });
    keyup_listener_state.set(Some(listener));
}

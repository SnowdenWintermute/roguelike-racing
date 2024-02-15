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
    keypress_listener_state: UseStateHandle<Option<EventListener>>,
) {
    let cloned_button_properties_state = button_properties_state.clone();
    let keypress_listener = EventListener::new(&window(), "keypress", move |event| {
        let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
        let mut next_number_to_assign = 1;
        let key_pressed = event.code();
        log!(format!("keypress code: {key_pressed}"));
        for (_, properties) in cloned_button_properties_state.iter().enumerate() {
            let assigned_keys =
                if let Some(dedicated_key_for_action) = &properties.dedicated_key_option {
                    if properties.should_be_disabled {
                        continue;
                    }
                    match dedicated_key_for_action {
                        GameKeys::Cancel => vec![], // escape key must be set as a keyup, not
                        // keypress
                        GameKeys::Confirm => vec!["KeyR".to_string(), "Enter".to_string()],
                        GameKeys::Next => vec!["KeyE".to_string()],
                        GameKeys::Previous => vec!["KeyW".to_string()],
                    }
                } else {
                    if properties.should_be_disabled {
                        next_number_to_assign += 1;
                        continue;
                    }
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
    keypress_listener_state.set(Some(keypress_listener));
    let keyup_listener = EventListener::new(&window(), "keyup", move |event| {
        let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
        let key_pressed = event.code();

        for (_, properties) in button_properties_state.iter().enumerate() {
            if properties.should_be_disabled {
                continue;
            }

            let assigned_keys =
                if let Some(dedicated_key_for_action) = &properties.dedicated_key_option {
                    match dedicated_key_for_action {
                        GameKeys::Cancel => vec!["Escape".to_string()],
                        GameKeys::Confirm => vec![],
                        GameKeys::Next => vec!["ArrowRight".to_string()],
                        GameKeys::Previous => vec!["ArrowLeft".to_string()],
                    }
                } else {
                    vec![]
                };
            if assigned_keys.contains(&key_pressed) {
                properties
                    .click_handler
                    .emit(MouseEvent::new("mouseup").unwrap_throw());
            }
        }
    });

    keyup_listener_state.set(Some(keyup_listener));
}

use super::build_action_button_properties::ActionMenuButtonProperties;
use gloo::events::EventListener;
use gloo_utils::window;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::MouseEvent;
use yew::UseStateHandle;

#[derive(Debug, Clone, PartialEq)]
pub enum ActionButtonCategories {
    Top,
    Numbered,
    NextPrevious,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameKeys {
    Cancel,
    Confirm,
    Next,
    Previous,
    S,
    I,
    D,
    O,
    F,
    P,
    T,
}

pub fn set_keyup_listeners(
    top_button_properties: Vec<ActionMenuButtonProperties>,
    numbered_button_properties_on_current_page: Vec<ActionMenuButtonProperties>,
    next_prev_button_properties: Vec<ActionMenuButtonProperties>,
    keyup_listener_state: UseStateHandle<Option<EventListener>>,
    keypress_listener_state: UseStateHandle<Option<EventListener>>,
) {
    let cloned_top_button_properties = top_button_properties.clone();
    let cloned_numbered_button_properties_on_current_page =
        numbered_button_properties_on_current_page.clone();
    let cloned_next_prev_button_properties = next_prev_button_properties.clone();
    let keypress_listener = EventListener::new(&window(), "keypress", move |event| {
        let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
        let key_pressed = event.code();
        set_listeners(&cloned_top_button_properties, &key_pressed);
        set_listeners(
            &cloned_numbered_button_properties_on_current_page,
            &key_pressed,
        );
        set_listeners(&cloned_next_prev_button_properties, &key_pressed);
    });
    keypress_listener_state.set(Some(keypress_listener));

    let cloned_top_button_properties = top_button_properties.clone();
    let cloned_numbered_button_properties_on_current_page =
        numbered_button_properties_on_current_page.clone();
    let cloned_next_prev_button_properties = next_prev_button_properties.clone();
    let keyup_listener = EventListener::new(&window(), "keyup", move |event| {
        let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
        let key_pressed = event.code();

        set_keypress_listeners(&cloned_top_button_properties, &key_pressed);
        set_keypress_listeners(
            &cloned_numbered_button_properties_on_current_page,
            &key_pressed,
        );
        set_keypress_listeners(&cloned_next_prev_button_properties, &key_pressed);
    });

    keyup_listener_state.set(Some(keyup_listener));
}

fn set_keypress_listeners(
    button_properties: &Vec<ActionMenuButtonProperties>,
    key_pressed: &String,
) {
    for (_, properties) in button_properties.iter().enumerate() {
        let assigned_keys =
            if let Some(dedicated_keys_for_action) = &properties.dedicated_keys_option {
                if properties.should_be_disabled {
                    continue;
                }
                let mut key_names = vec![];

                for key in dedicated_keys_for_action {
                    match key {
                        GameKeys::Cancel => key_names.push("Escape".to_string()), // escape key must be set as a keyup, not
                        GameKeys::Next => key_names.push("ArrowRight".to_string()),
                        GameKeys::Previous => key_names.push("ArrowLeft".to_string()),
                        _ => (),
                    }
                }
                key_names
            } else {
                vec![]
            };
        if assigned_keys.contains(&key_pressed) {
            properties
                .click_handler
                .emit(MouseEvent::new("mouseup").unwrap_throw());
        }
    }
}

fn set_listeners(button_properties: &Vec<ActionMenuButtonProperties>, key_pressed: &String) {
    let mut next_number_to_assign = 1;
    for (_, properties) in button_properties.iter().enumerate() {
        let assigned_keys =
            if let Some(dedicated_keys_for_action) = &properties.dedicated_keys_option {
                if properties.should_be_disabled {
                    continue;
                }
                let mut key_names = vec![];
                for key in dedicated_keys_for_action {
                    match key {
                        GameKeys::Cancel => (), // escape key must be set as a keyup, not
                        GameKeys::Confirm => {
                            key_names.push("KeyR".to_string());
                            key_names.push("Enter".to_string());
                        }
                        GameKeys::Next => key_names.push("KeyE".to_string()),
                        GameKeys::Previous => key_names.push("KeyW".to_string()),
                        GameKeys::S => key_names.push("KeyS".to_string()),
                        GameKeys::I => key_names.push("KeyI".to_string()),
                        GameKeys::D => key_names.push("KeyD".to_string()),
                        GameKeys::O => key_names.push("KeyO".to_string()),
                        GameKeys::F => key_names.push("KeyF".to_string()),
                        GameKeys::P => key_names.push("KeyP".to_string()),
                        GameKeys::T => key_names.push("KeyT".to_string()),
                    }
                }
                key_names
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
}

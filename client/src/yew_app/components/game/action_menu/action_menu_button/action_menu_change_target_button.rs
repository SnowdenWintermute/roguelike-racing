use crate::yew_app::components::game::action_menu::build_action_button_properties::ActionMenuButtonProperties;
use crate::yew_app::components::game::action_menu::set_keyup_listeners::GameKeys;
use crate::yew_app::components::game::tailwind_class_loader::BUTTON_HEIGHT_SMALL;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub properties: ActionMenuButtonProperties,
    pub dedicated_key: GameKeys,
}

#[function_component(ActionMenuChangeTargetButton)]
pub fn action_menu_numbered_button(props: &Props) -> Html {
    let text = match props.dedicated_key {
        GameKeys::Next => "Next target (E)",
        GameKeys::Previous => "Previous target (W)",
        _ => "",
    };

    html!(

    <button
        class="pr-2 pl-2 pointer-events-auto"
        style={format!("height: {}rem; ", BUTTON_HEIGHT_SMALL)}
        onclick={props.properties.click_handler.clone()}
        onmouseenter={props.properties.mouse_enter_handler.clone()}
        onmouseleave={props.properties.mouse_leave_handler.clone()}
        onfocus={props.properties.focus_handler.clone()}
        onblur={props.properties.blur_handler.clone()}
        disabled={props.properties.should_be_disabled}
        >
            {text}
        </button>
    )
}

pub mod action_button_click_handlers;
pub mod action_button_hover_handlers;
pub mod action_menu_button;
mod action_menu_change_detection_manager;
mod action_menu_page_manager;
mod action_page_buttons;
mod build_action_button_properties;
mod change_target_buttons;
mod determine_action_menu_buttons_disabled;
mod determine_menu_actions;
pub mod enums;
mod get_game_actions_by_menu_type;
mod set_keyup_listeners;
use crate::yew_app::components::game::action_menu::action_menu_button::create_action_menu_buttons;
use crate::yew_app::components::game::action_menu::action_menu_change_detection_manager::ActionMenuChangeDetectionManager;
use crate::yew_app::components::game::action_menu::action_page_buttons::page_turning::next_page;
use crate::yew_app::components::game::action_menu::action_page_buttons::page_turning::prev_page;
use crate::yew_app::components::game::action_menu::action_page_buttons::ActionPageButtons;
use crate::yew_app::components::game::action_menu::build_action_button_properties::ActionMenuButtonProperties;
use crate::yew_app::components::game::action_menu::change_target_buttons::ChangeTargetButtons;
use crate::yew_app::components::game::context_dependant_information_display::action_details_context_info::ActionDetailsContextInfo;
use crate::yew_app::components::game::tailwind_class_loader::BUTTON_HEIGHT;
use crate::yew_app::components::game::tailwind_class_loader::SPACING_REM;
use crate::yew_app::components::game::tailwind_class_loader::SPACING_REM_SMALL;
use crate::yew_app::store::game_store::get_focused_character;
use crate::yew_app::store::game_store::GameStore;
use common::utils::calculate_number_of_pages;
use gloo::events::EventListener;
use web_sys::HtmlElement;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, Eq, PartialEq)]
pub struct Props {}

#[derive(Default, Clone, PartialEq)]
pub struct ActionButtonPropertiesByCategory {
    top_action_buttons: Vec<ActionMenuButtonProperties>,
    numbered_action_buttons: Vec<ActionMenuButtonProperties>,
    next_prev_action_buttons: Vec<ActionMenuButtonProperties>,
}

const PAGE_SIZE: u8 = 6;
#[function_component(ActionMenu)]
pub fn action_menu(_: &Props) -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let action_menu_button_properties = use_state(|| ActionButtonPropertiesByCategory::default());
    let numbered_button_props_on_current_page =
        use_state(|| Vec::<ActionMenuButtonProperties>::new());
    let last_page_number_filtered = use_state(|| 0);

    let cloned_current_page_number = game_state.action_menu_current_page_number.clone();
    let cloned_action_button_properties = action_menu_button_properties.clone();
    let cloned_button_props_on_current_page = numbered_button_props_on_current_page.clone();
    let cloned_game_dispatch = game_dispatch.clone();
    let cloned_last_page_number_filtered = last_page_number_filtered.clone();
    use_effect_with(
        (
            game_state.action_menu_current_page_number,
            action_menu_button_properties.clone(),
        ),
        move |_| {
            let min_index = cloned_current_page_number * PAGE_SIZE;
            let max_index = cloned_current_page_number * PAGE_SIZE + PAGE_SIZE - 1;
            let filtered_actions = cloned_action_button_properties
                .numbered_action_buttons
                .iter()
                .enumerate()
                .filter(|(i, _)| *i as u8 >= min_index && *i as u8 <= max_index)
                .map(|(_, item)| item.clone())
                .collect::<Vec<ActionMenuButtonProperties>>();
            let num_actions = filtered_actions.len();
            cloned_button_props_on_current_page.set(filtered_actions);

            if cloned_current_page_number != 0
                && num_actions == 0
                && cloned_action_button_properties
                    .numbered_action_buttons
                    .len()
                    != 0
                && cloned_current_page_number == *cloned_last_page_number_filtered
            {
                cloned_game_dispatch.reduce_mut(|store| {
                    store.action_menu_current_page_number -= 1;
                });
            }
            cloned_last_page_number_filtered.set(cloned_current_page_number);
        },
    );

    let keyup_listener_state = use_state(|| None::<EventListener>);
    let keypress_listener_state = use_state(|| None::<EventListener>);
    let cloned_button_props_on_current_page = numbered_button_props_on_current_page.clone();
    let cloned_button_props_on_current_page_for_effect_change =
        numbered_button_props_on_current_page.clone();
    let cloned_action_button_properties_for_effect_change = action_menu_button_properties.clone();
    let cloned_action_button_properties = action_menu_button_properties.clone();
    use_effect_with(
        (
            cloned_action_button_properties_for_effect_change,
            cloned_button_props_on_current_page_for_effect_change,
        ),
        move |_| {
            set_keyup_listeners::set_keyup_listeners(
                cloned_action_button_properties.top_action_buttons.clone(),
                cloned_button_props_on_current_page.to_vec().clone(),
                cloned_action_button_properties
                    .next_prev_action_buttons
                    .clone(),
                keyup_listener_state,
                keypress_listener_state,
            )
        },
    );

    let cloned_button_props_on_current_page = numbered_button_props_on_current_page.clone();
    let cloned_action_button_properties = action_menu_button_properties.clone();
    let num_actions = cloned_action_button_properties
        .numbered_action_buttons
        .len();
    let number_of_pages = calculate_number_of_pages(PAGE_SIZE as usize, num_actions);

    let action_menu_node_ref = use_node_ref();
    let cloned_action_menu_node_ref = action_menu_node_ref.clone();
    let handle_wheel = Callback::from(move |e: WheelEvent| {
        let element_option = cloned_action_menu_node_ref.cast::<HtmlElement>();
        if let Some(element) = element_option {
            let scroll_height = element.scroll_height();
            let client_height = element.client_height();
            if scroll_height != client_height {
                return;
            }
        }
        if e.delta_y() > 0.0 {
            prev_page(
                game_dispatch.clone(),
                cloned_current_page_number,
                number_of_pages,
            )
        } else if e.delta_y() < 0.0 {
            next_page(
                game_dispatch.clone(),
                cloned_current_page_number,
                number_of_pages,
            )
        }
    });

    let (top_action_buttons, numbered_action_buttons, next_prev_action_buttons) =
        create_action_menu_buttons(
            &action_menu_button_properties.top_action_buttons,
            &numbered_button_props_on_current_page.to_vec(),
            &action_menu_button_properties.next_prev_action_buttons,
        );

    let hovered_action_display = if let Some(hovered_action) = &game_state.hovered_action {
        html!(
            <div class="absolute top-0 left-full pl-2">
                <div class="border border-slate-400 bg-slate-700 min-w-[25rem] max-w-[25rem] p-2">
                    <ActionDetailsContextInfo combat_action={hovered_action.clone()} hide_title={false} />
                </div>
            </div>
        )
    } else {
        html!()
    };

    let selected_action_display = {
        let mut selected_action_option = None;
        let focused_character_result = get_focused_character(&game_state);
        if let Ok(focused_character) = focused_character_result {
            selected_action_option = focused_character
                .combatant_properties
                .selected_combat_action
                .as_ref();
        }
        if let Some(selected_action) = selected_action_option {
            html!(
                    <div class="border border-slate-400 bg-slate-700 min-w-[25rem] max-w-[25rem] p-2"
                        style={format!("height: {}rem; ", BUTTON_HEIGHT * PAGE_SIZE as f32)}
                    >
                        <ActionDetailsContextInfo combat_action={selected_action.clone()} hide_title={false} />
                    </div>
            )
        } else {
            html!()
        }
    };

    html!(
        <section class=" max-h-fit
                        flex flex-col justify-between pointer-events-auto"
                 style={format!("margin-right: {}rem; ", SPACING_REM)}
        >
        <ActionMenuChangeDetectionManager action_menu_button_properties={action_menu_button_properties} />
            <ul class="flex list-none min-w-[25rem] max-w-[25rem]"
                style={ format!( "margin-bottom: {}rem;" , SPACING_REM_SMALL )}
            >
                {top_action_buttons}
            </ul>
                <ul class="list-none relative mb-2"
                    style={format!("height: {}rem; ", BUTTON_HEIGHT * PAGE_SIZE as f32)}
                    ref={action_menu_node_ref}
                    onwheel={handle_wheel}
                >
                    {numbered_action_buttons}
                    {hovered_action_display}
                    {selected_action_display}
                </ul>
            {

        if next_prev_action_buttons.len() > 0 {
            html!(
                <ChangeTargetButtons
                    next_prev_buttons={next_prev_action_buttons.clone()}
                />
            )
        } else {
                html!(
                <ActionPageButtons
                        number_of_pages={number_of_pages}
                        hidden={( cloned_action_button_properties.numbered_action_buttons.len() as u8) <= PAGE_SIZE}
                    />
                )
        }
        }
        </section>
    )
}

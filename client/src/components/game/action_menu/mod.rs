pub mod action_button_click_handlers;
pub mod action_button_hover_handlers;
pub mod action_menu_button;
mod action_menu_change_detection_manager;
mod action_menu_page_buttons;
mod action_menu_page_manager;
mod build_action_button_properties;
mod determine_action_menu_buttons_disabled;
mod determine_menu_actions;
pub mod enums;
mod get_game_actions_by_menu_type;
mod set_keyup_listeners;
use crate::components::game::action_menu::action_menu_button::ActionMenuButton;
use crate::components::game::action_menu::action_menu_change_detection_manager::ActionMenuChangeDetectionManager;
use crate::components::game::action_menu::action_menu_page_buttons::page_turning::next_page;
use crate::components::game::action_menu::action_menu_page_buttons::page_turning::prev_page;
use crate::components::game::action_menu::action_menu_page_buttons::ActionPageButtons;
use crate::components::game::action_menu::build_action_button_properties::ActionMenuButtonProperties;
use crate::store::game_store::GameStore;
use common::utils::calculate_number_of_pages;
use gloo::console::log;
use gloo::events::EventListener;
use std::ops::Deref;
use web_sys::HtmlElement;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, Eq, PartialEq)]
pub struct Props {}

const PAGE_SIZE: u8 = 6;
#[function_component(ActionMenu)]
pub fn action_menu(_: &Props) -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let action_menu_button_properties = use_state(|| Vec::<ActionMenuButtonProperties>::new());
    let button_props_on_current_page = use_state(|| Vec::<ActionMenuButtonProperties>::new());
    let last_page_number_filtered = use_state(|| 0);
    let cloned_current_page_number = game_state.action_menu_current_page_number.clone();

    let cloned_current_page_number = game_state.action_menu_current_page_number.clone();
    let cloned_action_button_properties = action_menu_button_properties.clone();
    let cloned_button_props_on_current_page = button_props_on_current_page.clone();
    let cloned_game_dispatch = game_dispatch.clone();
    let cloned_last_page_number_filtered = last_page_number_filtered.clone();
    use_effect_with(
        (
            game_state.action_menu_current_page_number,
            action_menu_button_properties.clone(),
        ),
        move |_| {
            log!(format!(
                "button_props: {:#?}",
                cloned_action_button_properties.len()
            ));
            let min_index = cloned_current_page_number * PAGE_SIZE;
            let max_index = cloned_current_page_number * PAGE_SIZE + PAGE_SIZE - 1;
            let filtered_actions = cloned_action_button_properties
                .deref()
                .iter()
                .enumerate()
                .filter(|(i, _)| *i as u8 >= min_index && *i as u8 <= max_index)
                .map(|(_, item)| item.clone())
                .collect::<Vec<ActionMenuButtonProperties>>();
            let num_actions = filtered_actions.len();
            log!(format!("filtered_actions: {:#?}", filtered_actions.len()));
            cloned_button_props_on_current_page.set(filtered_actions);

            if cloned_current_page_number != 0
                && num_actions == 0
                && cloned_action_button_properties.len() != 0
                && cloned_current_page_number == *cloned_last_page_number_filtered
            {
                log!(format!(
                    "num props on curr page num_actions filtered: {} setting current page number -1",
                    num_actions
                ));
                cloned_game_dispatch.reduce_mut(|store| {
                    store.action_menu_current_page_number -= 1;
                });
            }
            cloned_last_page_number_filtered.set(cloned_current_page_number);
        },
    );

    let keyup_listener_state = use_state(|| None::<EventListener>);
    let cloned_button_props_on_current_page = button_props_on_current_page.clone();
    let cloned_button_props_on_current_page_for_effect_change =
        button_props_on_current_page.clone();
    use_effect_with(
        cloned_button_props_on_current_page_for_effect_change,
        move |_| {
            set_keyup_listeners::set_keyup_listeners(
                cloned_button_props_on_current_page,
                keyup_listener_state,
            )
        },
    );

    let cloned_button_props_on_current_page = button_props_on_current_page.clone();
    let cloned_action_button_properties = action_menu_button_properties.clone();
    let num_actions = cloned_action_button_properties.len();
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

    html!(
        <section class="w-[22rem] border border-slate-400 bg-slate-700 mr-4 overflow-y-auto
        flex flex-col justify-between"
        >
        <ActionMenuChangeDetectionManager action_menu_button_properties={action_menu_button_properties} />
            <div class="overflow-y-auto flex-grow"
                ref={action_menu_node_ref}
                onwheel={handle_wheel}
            >
                {cloned_button_props_on_current_page.deref().iter().enumerate().map(|(i, action)| {
                      html!(
                          <ActionMenuButton
                            properties={action.clone()}
                            number={i+1}
                          />
                          )
                      }).collect::<Html>() }
            </div>
            {html!(
                if cloned_action_button_properties.deref().len() as u8 > PAGE_SIZE {
                    <ActionPageButtons
                        number_of_pages={number_of_pages}
                        />
                }
            )}
        </section>
    )
}

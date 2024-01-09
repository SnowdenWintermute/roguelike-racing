pub mod action_handlers;
mod action_menu_button;
mod action_page_buttons;
mod available_actions;
mod create_action_handler;
mod create_action_mouse_enter_handler;
mod create_action_mouse_leave_handler;
mod determine_action_menu_buttons_disabled;
mod generate_action_menu_items;
mod generate_button_text;
mod get_character_owned_item_by_id;
mod set_keyup_listeners;
mod set_up_actions;
use crate::components::game::action_menu::action_menu_button::ActionMenuButton;
use crate::components::game::action_menu::action_page_buttons::ActionPageButtons;
use crate::components::game::action_menu::set_up_actions::ActionMenuButtonProperties;
use crate::store::game_store::GameStore;
use crate::store::lobby_store::LobbyStore;
use crate::store::ui_store::UIStore;
use crate::store::websocket_store::WebsocketStore;
use common::utils::calculate_number_of_pages;
use gloo::events::EventListener;
use std::ops::Deref;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, Eq, PartialEq)]
pub struct Props {}

const PAGE_SIZE: u8 = 6;
#[function_component(ActionMenu)]
pub fn action_menu(_: &Props) -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let (ui_state, _) = use_store::<UIStore>();
    let (lobby_state, _) = use_store::<LobbyStore>();
    let (websocket_state, _) = use_store::<WebsocketStore>();
    let action_button_properties = use_state(|| Vec::<ActionMenuButtonProperties>::new());
    let button_props_on_current_page = use_state(|| Vec::<ActionMenuButtonProperties>::new());

    let cloned_current_page_number = game_state.action_menu_current_page_number.clone();
    let cloned_action_button_properties = action_button_properties.clone();
    let cloned_button_props_on_current_page = button_props_on_current_page.clone();
    use_effect_with(
        (
            game_state.action_menu_current_page_number,
            action_button_properties.clone(),
        ),
        move |_| {
            let min_index = cloned_current_page_number * PAGE_SIZE;
            let max_index = cloned_current_page_number * PAGE_SIZE + PAGE_SIZE - 1;
            let filtered_actions = cloned_action_button_properties
                .deref()
                .iter()
                .enumerate()
                .filter(|(i, _)| *i as u8 >= min_index && *i as u8 <= max_index)
                .map(|(_, item)| item.clone())
                .collect::<Vec<ActionMenuButtonProperties>>();
            cloned_button_props_on_current_page.set(filtered_actions);
        },
    );

    let cloned_action_button_properties = action_button_properties.clone();
    let cloned_game_state = game_state.clone();
    let selected_item_id = match &game_state.selected_item {
        Some(item) => Some(item.entity_properties.id),
        None => None,
    };

    let party = game_state.get_current_party().expect("to be in a party");
    let battle_id = party.battle_id;
    let focused_character_option = party.characters.get(&game_state.focused_character_id);
    let focused_character_equipment_ids = match focused_character_option {
        Some(focused_character) => Some(
            focused_character
                .combatant_properties
                .equipment
                .iter()
                .map(|(_slot, item)| item.entity_properties.id)
                .collect::<Vec<u32>>(),
        ),
        None => None,
    };

    let ability_targets = match focused_character_option {
        Some(focused_character) => focused_character
            .combatant_properties
            .ability_targets
            .clone(),

        None => None,
    };

    let cloned_ui_state = ui_state.clone();
    let cloned_game_dispatch = game_dispatch.clone();
    use_effect_with(
        (
            cloned_game_state.focused_character_id,
            cloned_game_state.viewing_inventory,
            cloned_game_state.viewing_equipped_items,
            ability_targets,
            selected_item_id,
            cloned_game_state.viewing_items_on_ground,
            cloned_game_state.viewing_skill_level_up_menu,
            cloned_game_state.viewing_attribute_point_assignment_menu,
            battle_id,
            cloned_ui_state.mod_key_held,
            focused_character_equipment_ids,
        ),
        move |_| {
            let re_cloned_game_state = cloned_game_state.clone();
            let party = re_cloned_game_state
                .get_current_party()
                .expect("to be in a party");
            let actions = set_up_actions::set_up_actions(
                websocket_state.clone(),
                cloned_game_state,
                cloned_game_dispatch,
                cloned_ui_state,
                lobby_state,
                party,
            );
            cloned_action_button_properties.set(actions);
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
    let cloned_action_button_properties = action_button_properties.clone();
    let num_actions = cloned_action_button_properties.len();
    let number_of_pages = calculate_number_of_pages(PAGE_SIZE as usize, num_actions);

    html!(
        <section class="min-w-[350px] w-[350px] border border-slate-400 bg-slate-700 mr-4 overflow-y-auto
        flex flex-col justify-between">
            <div>
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

use super::ActionButtonPropertiesByCategory;
use crate::yew_app::components::game::action_menu::build_action_button_properties::build_action_button_properties;
use crate::yew_app::store::alert_store::AlertStore;
use crate::yew_app::store::game_store::get_active_combatant;
use crate::yew_app::store::game_store::GameStore;
use crate::yew_app::store::lobby_store::LobbyStore;
use crate::yew_app::store::ui_store::UIStore;
use crate::yew_app::store::websocket_store::WebsocketStore;
use yew::prelude::*;
use yewdux::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub action_menu_button_properties: UseStateHandle<ActionButtonPropertiesByCategory>,
}

#[function_component(ActionMenuChangeDetectionManager)]
pub fn action_menu_change_detection_manager(props: &Props) -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let (ui_state, _) = use_store::<UIStore>();
    let (lobby_state, _) = use_store::<LobbyStore>();
    let (websocket_state, _) = use_store::<WebsocketStore>();
    let (_, alert_dspatch) = use_store::<AlertStore>();
    let cloned_game_state = game_state.clone();
    let previously_focused_character_id_state = use_state(|| game_state.focused_character_id);

    let active_combatant_result = get_active_combatant(&game_state);
    let active_combatant_id_option = match active_combatant_result {
        Ok(combatant_option) => match combatant_option {
            Some((entity_properties, _)) => Some(entity_properties.id),
            None => None,
        },
        Err(_) => None,
    };
    let party = game_state.get_current_party().expect("to be in a party");
    let room_type = party.current_room.room_type;
    let num_items_on_ground = party.current_room.items.len();
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
    let selected_item_id = match &game_state.selected_item {
        Some(item) => Some(item.entity_properties.id),
        None => None,
    };

    let num_items_in_focused_character_inventory = match focused_character_option {
        Some(focused_character) => {
            Some(focused_character.combatant_properties.inventory.items.len())
        }
        None => None,
    };
    let ability_targets = match focused_character_option {
        Some(focused_character) => focused_character
            .combatant_properties
            .combat_action_targets
            .clone(),

        None => None,
    };
    let focused_character_selected_combat_action_option = match focused_character_option {
        Some(focused_character) => focused_character
            .combatant_properties
            .selected_combat_action
            .clone(),
        None => None,
    };

    let focused_character_current_animation_processing_option = match focused_character_option {
        Some(focused_character) => game_state
            .action_results_manager
            .combantant_event_managers
            .get(&focused_character.entity_properties.id)
            .expect("to have an event queue for every combatant entity")
            .animation_queue
            .front(),
        None => None,
    };
    let focused_character_unspent_attribute_points = match focused_character_option {
        Some(focused_character) => {
            focused_character
                .combatant_properties
                .unspent_attribute_points
        }
        None => 0,
    };

    let cloned_focused_character_current_animation_processing_option =
        match focused_character_current_animation_processing_option {
            Some(action_result) => Some(action_result.clone()),
            None => None,
        };

    let cloned_ui_state = ui_state.clone();
    let cloned_game_dispatch = game_dispatch.clone();
    let cloned_alert_dispatch = alert_dspatch.clone();
    let cloned_action_menu_button_properties = props.action_menu_button_properties.clone();
    let focused_character_id = game_state.focused_character_id;
    use_effect_with(
        (
            active_combatant_id_option,
            focused_character_id,
            cloned_game_state.viewing_inventory,
            cloned_game_state.viewing_equipped_items,
            ability_targets,
            (
                room_type,
                num_items_in_focused_character_inventory,
                num_items_on_ground,
                selected_item_id,
                focused_character_selected_combat_action_option,
                cloned_focused_character_current_animation_processing_option,
                focused_character_unspent_attribute_points,
            ),
            cloned_game_state.viewing_items_on_ground,
            cloned_game_state.viewing_skill_level_up_menu,
            cloned_game_state.viewing_attribute_point_assignment_menu,
            battle_id,
            cloned_ui_state.mod_key_held,
            focused_character_equipment_ids,
        ),
        move |_| {
            if *previously_focused_character_id_state != focused_character_id {
                cloned_game_dispatch.reduce_mut(|store| store.action_menu_current_page_number = 0);
            }
            previously_focused_character_id_state.set(focused_character_id);
            let re_cloned_game_state = cloned_game_state.clone();
            let party = re_cloned_game_state
                .get_current_party()
                .expect("to be in a party");
            let action_button_properties_by_category = build_action_button_properties(
                websocket_state.clone(),
                cloned_game_state,
                cloned_game_dispatch,
                cloned_alert_dispatch,
                cloned_ui_state,
                lobby_state,
                party,
            );
            cloned_action_menu_button_properties.set(action_button_properties_by_category);
        },
    );
    html!()
}

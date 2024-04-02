mod action_details_context_info;
mod combatant_details_context_info;
mod damage_type_badge;
mod item_details;
use crate::yew_app::components::game::combat_log::CombatLog;
use crate::yew_app::components::game::context_dependant_information_display::combatant_details_context_info::CombatantDetailsContextInfo;
use crate::yew_app::components::game::context_dependant_information_display::item_details::ItemDetails;
use crate::yew_app::store::game_store::DetailableEntities;
use crate::yew_app::store::game_store::GameStore;
use crate::yew_app::store::game_store::get_focused_character;
use yew::prelude::*;
use yewdux::prelude::use_store;

use self::action_details_context_info::ActionDetailsContextInfo;

#[function_component(ContextDependantInformationDisplay)]
pub fn context_dependant_information_display() -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let detailed_entity = &game_state.detailed_entity;
    let hovered_entity = &game_state.hovered_entity;

    let mut hovered_tab = match hovered_entity {
        Some(detailable) => match detailable {
            DetailableEntities::Combatant(combatant_properties) => Some(
                html!(<CombatantDetailsContextInfo combatant_id={combatant_properties.entity_properties.id}/>),
            ),
            DetailableEntities::Item(item) => Some(html!(<ItemDetails item={item.clone()}  />)),
        },
        None => None,
    };

    if hovered_tab.is_none() && game_state.hovered_action.is_some() {
        let hovered_action = game_state.hovered_action.clone().expect("checked");
        hovered_tab =
            Some(html!(<ActionDetailsContextInfo combat_action={hovered_action.clone()} />))
    }

    let detailed_tab = match detailed_entity {
        Some(detailable) => match detailable {
            DetailableEntities::Combatant(combatant_properties) => Some(
                html!(<CombatantDetailsContextInfo combatant_id={combatant_properties.entity_properties.id}/>),
            ),
            DetailableEntities::Item(item) => Some(html!(<ItemDetails item={item.clone()} />)),
        },
        None => {
            let mut to_return = None;
            let focused_character_result = get_focused_character(&game_state);
            if let Ok(focused_character) = focused_character_result {
                let selected_action_option = &focused_character
                    .combatant_properties
                    .selected_combat_action;
                if let Some(selected_action) = selected_action_option {
                    to_return = Some(
                        html!(<ActionDetailsContextInfo combat_action={selected_action.clone()} />),
                    )
                }
            }
            to_return
        }
    };

    let displayed_tab = {
        if let Some(tab) = hovered_tab {
            tab
        } else if let Some(tab) = detailed_tab {
            tab
        } else {
            html!(<CombatLog/>)
        }
    };

    html!(
        <section class="p-2 flex-grow border border-slate-400 bg-slate-700 overflow-y-auto">
            {displayed_tab}
        </section>
    )
}

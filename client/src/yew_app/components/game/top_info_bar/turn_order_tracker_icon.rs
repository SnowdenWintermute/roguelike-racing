use crate::yew_app::components::game::combatant::combatant_is_ally::combatant_is_ally;
use crate::yew_app::store::game_store::GameStore;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub entity_id: u32,
}

#[function_component(TurnOrderTrackerIcon)]
pub fn turn_order_tracker_icon(props: &Props) -> Html {
    let (game_state, _) = use_store::<GameStore>();

    let combatant_is_ally = combatant_is_ally(game_state.clone(), props.entity_id);

    let combatant_name_option = if let Some(_) = game_state.current_battle_id {
        if let Some(game) = &game_state.game {
            let combatant_result = game.get_combatant_by_id(&props.entity_id);
            if let Ok(combatant) = combatant_result {
                Some(combatant.0.name.clone())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    let button_content = match combatant_name_option {
        Some(name) => html!({ name.chars().next().unwrap_or_else(|| '?') }),
        None => {
            html!({ "Error - no entity found" })
        }
    };

    let mut conditional_classes = String::from("");
    if combatant_is_ally {
        conditional_classes.push_str("bg-emerald-900")
    } else {
        conditional_classes.push_str("bg-amber-900")
    }

    html!(
    <button class={format!("border border-slate-400 h-10 w-10 {conditional_classes} mr-2 last:mr-0")}>
        <div class="h-full w-full rounded-full bg-slate-600 border border-slate-400 flex items-center justify-center">
            {button_content}
        </div>
    </button>
    )
}

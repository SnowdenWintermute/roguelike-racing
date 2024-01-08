use crate::store::alert_store::AlertStore;
use crate::store::game_store::get_current_battle_option;
use crate::store::game_store::GameStore;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub entity_id: u32,
}

#[function_component(TurnOrderTrackerCard)]
pub fn turn_order_tracker_card(props: &Props) -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let (_, alert_dispatch) = use_store::<AlertStore>();
    let battle_option = get_current_battle_option(&game_state);
    let combatant_option = if let Some(battle_id) = game_state.current_battle_id {
        if let Some(game) = game_state.game {
            let combatant_result = game.get_combatant_by_id(&props.entity_id);
            if let Ok(combatant) = combatant_result {
                Some(combatant)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    let button_content = match combatant_option {
        Some((entity_properties, combatant_properties)) => html!({ &entity_properties.name }),
        None => {
            html!({ "Error - no entity found" })
        }
    };

    html!(
    <button class="border-r border-slate-400 p-2 last:border-r-0 w-20 whitespace-nowrap overflow-hidden text-ellipsis" >
        {button_content}
    </button>
    )
}

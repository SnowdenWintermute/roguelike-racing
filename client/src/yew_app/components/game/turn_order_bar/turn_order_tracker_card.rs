use crate::yew_app::store::game_store::GameStore;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub entity_id: u32,
}

#[function_component(TurnOrderTrackerCard)]
pub fn turn_order_tracker_card(props: &Props) -> Html {
    let (game_state, _) = use_store::<GameStore>();

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
        Some(name) => html!({ &name }),
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

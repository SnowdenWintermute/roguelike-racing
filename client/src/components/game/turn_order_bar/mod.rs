mod turn_order_tracker_card;
use crate::{
    components::game::turn_order_bar::turn_order_tracker_card::TurnOrderTrackerCard,
    store::game_store::GameStore,
};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(TurnOrderBar)]
pub fn turn_order_bar() -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let party_result = game_state.get_current_party();
    let turn_trackers_option = {
        if let Ok(party) = party_result {
            &party.combatant_turn_trackers
        } else {
            &None
        }
    };

    let bar_content = match turn_trackers_option {
        Some(trackers) => {
            html!({
                trackers
                    .iter()
                    .map(|tracker| {
                        html!(
                            <TurnOrderTrackerCard id={tracker.entity_id} />
                        )
                    })
                    .collect::<Html>()
            })
        }
        None => html!({ "Error - no turn order found" }),
    };

    html!(
    <ul class="list-none flex" >
        {bar_content}
    </ul>
    )
}

mod turn_order_tracker_card;
use crate::yew_app::components::game::turn_order_bar::turn_order_tracker_card::TurnOrderTrackerCard;
use crate::yew_app::store::game_store::get_current_battle_option;
use crate::yew_app::store::game_store::GameStore;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(TurnOrderBar)]
pub fn turn_order_bar() -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let battle_option = get_current_battle_option(&game_state);
    let turn_trackers_option = if let Some(battle) = battle_option {
        Some(&battle.combatant_turn_trackers)
    } else {
        None
    };

    let bar_content = match turn_trackers_option {
        Some(trackers) => {
            html!({
                trackers
                    .iter()
                    .map(|tracker| {
                        html!(
                            <TurnOrderTrackerCard entity_id={tracker.entity_id} />
                        )
                    })
                    .collect::<Html>()
            })
        }
        None => html!({ "Error - no turn order found" }),
    };

    html!(
        <div class="flex">
            <div class="mr-4 flex justify-center items-center pr-2 pl-2 border-slate-400" >
            {"Turn order: "}
            </div>
            <ul class="list-none flex border-l border-slate-400" >
                {bar_content}
            </ul>
        </div>

    )
}

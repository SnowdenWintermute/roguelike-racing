use crate::yew_app::components::game::combatant::combatant_is_ally::combatant_is_ally;
// use crate::yew_app::store::game_store::get_current_battle_option_mut;
use crate::yew_app::store::game_store::GameStore;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub entity_id: u32,
}

const SHOWN_CLASSES: &str = "w-10 mr-2 last:mr-0";

#[function_component(TurnOrderTrackerIcon)]
pub fn turn_order_tracker_icon(props: &Props) -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let pre_removal_classes_state = use_state(|| SHOWN_CLASSES.to_string());
    let transition_style = use_state(|| "transition: width 1s;");
    // let timer_state = use_state(|| None);

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

    // let cloned_pre_removal_classes_state = pre_removal_classes_state.clone();
    // let cloned_dispatch = game_dispatch.clone();
    // let entity_id = props.entity_id;
    // let cloned_transition_style = transition_style.clone();
    let handle_click = Callback::from(move |_e: MouseEvent| {
        // cloned_pre_removal_classes_state.set("w-0 opacity-50 m-0 overflow-hidden".to_string());
        // let cloned_dispatch = cloned_dispatch.clone();
        // let cloned_pre_removal_classes_state = cloned_pre_removal_classes_state.clone();
        // let cloned_transition_style = cloned_transition_style.clone();
        // timer_state.set(Some(gloo::timers::callback::Timeout::new(
        //     1000,
        //     move || {
        //         cloned_dispatch.reduce_mut(|store| {
        //             let battle_option = get_current_battle_option_mut(store);
        //             if let Some(battle) = battle_option {
        //                 let mut new_trackers = battle.combatant_turn_trackers.clone();
        //                 new_trackers.retain(|item| item.entity_id != entity_id);
        //                 battle.combatant_turn_trackers = new_trackers;
        //             }
        //         });
        //         // there seems to be a bug in yew/(yewdux?) where this element's state is applied
        //         // to the next element in the vec after this element is removed, so a workaround is
        //         // to set the state back to original
        //         cloned_transition_style.set("");
        //         cloned_pre_removal_classes_state.set(format!("{SHOWN_CLASSES}"));
        //     },
        // )));
    });

    let mut conditional_classes = String::from("");
    if combatant_is_ally {
        conditional_classes.push_str("bg-emerald-900")
    } else {
        conditional_classes.push_str("bg-amber-900")
    }

    html!(
    <button
        class={format!("border border-slate-400 h-10 {conditional_classes} mr-2 last:mr-0 {}", *pre_removal_classes_state)}
        style={*transition_style}
        onclick={handle_click.clone()}
        >
        <div class="h-full w-full rounded-full bg-slate-600 border border-slate-400 flex items-center justify-center">
            // {props.entity_id}
            {button_content}
        </div>
    </button>
    )
}

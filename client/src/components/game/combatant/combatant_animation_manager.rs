use crate::components::game::combatant::process_next_action_result_in_combatant_event_queue::process_next_action_result_in_combatant_event_queue;
use crate::components::game::combatant::process_next_animation_in_combatant_animation_queue::process_next_animation_in_combatant_animation_queue;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use gloo::timers::callback::Timeout;
use yew::prelude::*;
use yewdux::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub combatant_id: u32,
}

#[function_component(CombatantAnimationManager)]
pub fn combatant_animation_manager(props: &Props) -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let timer_state: UseStateHandle<Option<Timeout>> = use_state(|| None);
    let Props { combatant_id } = props;
    let combatant_id = combatant_id.clone();
    let event_manager_option = game_state
        .action_results_manager
        .combantant_event_managers
        .get(&combatant_id);
    if event_manager_option.is_none() {
        return html!({ error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND });
    };
    let event_manager = event_manager_option.expect("none checked");
    let front = event_manager.action_result_queue.front();
    let current_action_processing = match front {
        Some(action_result) => Some(action_result.clone()),
        None => None,
    };

    let first_render_completed = use_state(|| false);
    let cloned_first_render_completed = first_render_completed.clone();
    use_effect_with((), move |_| cloned_first_render_completed.set(true));

    let cloned_game_dispatch = game_dispatch.clone();
    let combatant_id = combatant_id.clone();
    let cloned_first_render_completed = first_render_completed.clone();
    use_effect_with(
        (current_action_processing, combatant_id),
        move |(current_action_processing, combatant_id)| {
            if !*cloned_first_render_completed {
                return ();
            }
            let _result = process_next_action_result_in_combatant_event_queue(
                cloned_game_dispatch,
                current_action_processing,
                *combatant_id,
            );
        },
    );

    let cloned_animation_queue = event_manager.animation_queue.clone();
    let cloned_game_dispatch = game_dispatch.clone();
    let cloned_first_render_completed = first_render_completed.clone();
    use_effect_with(cloned_animation_queue, move |cloned_animation_queue| {
        if !*cloned_first_render_completed {
            return ();
        }
        process_next_animation_in_combatant_animation_queue(
            cloned_game_dispatch,
            cloned_animation_queue,
            timer_state,
            combatant_id,
        )
    });

    let animation_to_show = if let Some(animation) = event_manager.animation_queue.front() {
        format!("{}", animation)
    } else {
        "".to_string()
    };

    let debug = true;
    if debug {
        return html! {
            <span class="whitespace-nowrap text-ellipsis overflow-hidden">
                {animation_to_show}
            </span>
        };
    }
    html!()
}

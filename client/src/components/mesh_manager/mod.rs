use super::websocket_manager::handle_combat_turn_results::handle_animation_finished::handle_event_finished_animating;
use crate::store::alert_store::AlertStore;
use crate::store::game_store::GameStore;
use common::combat::ActionResult;
use common::combatants::abilities::CombatantAbilityNames;
use common::utils::vec_shift;
use std::collections::HashMap;
use yewdux::Dispatch;

// IN BATTLE
// queue action results to the ActionResultsManager turn_results_queue
// take the first action result and pass it as a TookAction(ActionResult) ClientCombatantEvent to the user entity's
// event queue
// if not already processing an event, process the first ClientCombatantEvent in the entity's queue
// - set the entity's current_event_animating to the ClientCombatantEvent
// - when animation finishes:
//   - if was TookAction
//     - queue and start damage taken animations on affected entities
//     - subtract hp from affected entities
//     - if any affected entity is dead, queue death animation on that entity
//     - if action required turn, end active combatant turn for the current battle if any
//   - for any event animation finishing
//     - if still alive, process next event in that entity's queue
//     - if all entity event queues are empty and no animations are ongoing,
//       query the ActionResultsManager turn_results_queue queue for the next action_result to process/animate

// OUT OF COMBAT

// queue action results directly on to the CombatantEventManagers
// if not already processing an event, process the first ClientCombatantEvent in the entity's queue
// - set the entity's current_event_animating to the ClientCombatantEvent
// - when animation finishes:
//   - if was TookAction
//     - queue and start damage taken animations on affected entities
//     - subtract hp from affected entities
//     - if any affected entity is dead, queue death animation on that entity
//     - if action required turn, no action is required since out of combat
//   - for any event animation finishing
//     - if still alive, process next event in that entity's queue
//     - if all entity event queues are empty and no animations are ongoing,
//       no action is required since out of combat

#[derive(PartialEq, Clone)]
pub enum ClientCombatantEvent {
    HpChange(i16),
    Died,
    TookAction(ActionResult),
}

#[derive(PartialEq, Clone)]
pub struct CombatantEventManager {
    pub associated_combatant_id: u32,
    pub event_queue: Vec<ClientCombatantEvent>,
    pub current_event_processing: Option<ClientCombatantEvent>,
    pub mesh_manager: CombatantMeshManager,
}

impl CombatantEventManager {
    pub fn process_next_event(
        &mut self,
        game_dispatch: Dispatch<GameStore>,
        alert_dispatch: Dispatch<AlertStore>,
    ) {
        if self.current_event_processing.is_none() {
            // process the first event in their queue
            // upon finishing, events will query the queue for the next event to process
            if let Some(event) = vec_shift(&mut self.event_queue) {
                // start animation
                self.current_event_processing = Some(event.clone());
                let cloned_event = event.clone();
                let associated_combatant_id = self.associated_combatant_id;
                gloo::timers::callback::Timeout::new(1500, move || {
                    handle_event_finished_animating(
                        associated_combatant_id,
                        cloned_event,
                        game_dispatch,
                        alert_dispatch,
                    )
                })
                .forget();
            }
        }
    }
}

#[derive(Default, PartialEq, Clone)]
pub struct ActionResultsManager {
    pub turn_results_queue: Vec<ActionResult>,
    pub combantant_event_managers: HashMap<u32, CombatantEventManager>,
}

#[derive(Default, PartialEq, Eq, Clone)]
pub struct CombatantMeshManager {
    pub combatant_entity_id: u32,
    pub facing_towards: Option<u32>,
    pub ability_animation: Option<CombatantAbilityNames>,
    pub taking_damage_animation: bool,
}

//impl ClientMeshManager {
//    pub fn play_next_turn_action_animation() {
//        //
//    }
//}

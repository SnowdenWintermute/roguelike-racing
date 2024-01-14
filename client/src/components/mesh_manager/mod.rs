use super::websocket_manager::handle_combat_turn_results::handle_animation_finished::handle_event_finished_animating;
use crate::store::alert_store::AlertStore;
use crate::store::game_store::GameStore;
use common::combat::ActionResult;
use common::combat::CombatAction;
use common::combatants::abilities::CombatantAbilityNames;
use common::utils::vec_shift;
use std::collections::HashMap;
use std::fmt::Display;
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

impl Display for ClientCombatantEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let to_write = match self {
            ClientCombatantEvent::HpChange(hp_change) => format!("hp change: {hp_change}"),
            ClientCombatantEvent::Died => "Death".to_owned(),
            ClientCombatantEvent::TookAction(action_result) => format!(
                "taking action: {}",
                match &action_result.action {
                    CombatAction::AbilityUsed(ability_name) => format!("{ability_name}"),
                    CombatAction::ItemUsed(_) => "using consumable".to_string(),
                }
            ),
        };
        write!(f, "{to_write}")
    }
}

#[derive(PartialEq, Clone)]
pub struct CombatantEventManager {
    pub associated_combatant_id: u32,
    pub event_queue: Vec<ClientCombatantEvent>,
    pub current_event_processing: Option<ClientCombatantEvent>,
}

impl CombatantEventManager {
    pub fn new(associated_combatant_id: u32) -> Self {
        CombatantEventManager {
            associated_combatant_id,
            event_queue: vec![],
            current_event_processing: None,
        }
    }

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

impl ActionResultsManager {
    pub fn new() -> Self {
        ActionResultsManager {
            turn_results_queue: vec![],
            combantant_event_managers: HashMap::new(),
        }
    }
}

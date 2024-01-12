use common::combat::ActionResult;
use common::combatants::abilities::CombatantAbilityNames;
use std::collections::HashMap;

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
    DamageTaken(u16),
    DamageHealed(u16),
    Died,
    TookAction(ActionResult),
}

#[derive(PartialEq, Clone)]
pub struct CombatantEventManager {
    pub event_queue: Vec<ClientCombatantEvent>,
    pub current_event_processing: Option<ClientCombatantEvent>,
    pub mesh_manager: CombatantMeshManager,
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

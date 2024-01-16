use common::combat::ActionResult;
use common::combat::CombatTurnResult;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Display;
use yew::AttrValue;

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

#[derive(PartialEq, Clone, Debug)]
pub enum CombatantAnimation {
    TurnToFaceCombatant(u32),
    ApproachCombatant(u32),
    SwingMainHandToHit(u32, Option<i16>),
    SwingOffHandToHit,
    MainHandFollowThroughSwing,
    OffHandFollowThroughSwing,
    ReturnToReadyPosition,
    HitRecovery(i16),
    Death(Option<i16>),
}

impl Display for CombatantAnimation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let to_write = match self {
            CombatantAnimation::TurnToFaceCombatant(id) => format!("turned to face {}", id),
            CombatantAnimation::ApproachCombatant(id) => format!("approached combatant {}", id),
            CombatantAnimation::SwingMainHandToHit(id, hp_change) => {
                format!("swung main hand to hit {:?} for {:?}", id, hp_change)
            }
            CombatantAnimation::SwingOffHandToHit => format!("swung offhand to hit"),
            CombatantAnimation::MainHandFollowThroughSwing => format!("main hand follow through"),
            CombatantAnimation::OffHandFollowThroughSwing => format!("off hand follow through"),
            CombatantAnimation::ReturnToReadyPosition => format!("returned to ready position"),
            CombatantAnimation::HitRecovery(hp_change) => format!("hit recovery {hp_change}"),
            CombatantAnimation::Death(hp_change) => format!("died {:?}", hp_change),
        };
        write!(f, "{:?}", to_write)
    }
}

#[derive(PartialEq, Clone)]
pub struct FloatingNumber {
    pub value: i16,
    pub color: AttrValue,
}

#[derive(PartialEq, Clone)]
pub struct CombatantEventManager {
    pub associated_combatant_id: u32,
    pub action_result_queue: VecDeque<ActionResult>,
    pub animation_queue: VecDeque<CombatantAnimation>,
    pub floating_numbers_queue: VecDeque<FloatingNumber>,
    pub visual_location: CombatantVisualLocation,
}

#[derive(PartialEq, Clone)]
pub enum CombatantVisualLocation {
    HomePosition,
    StandingInFrontOf(u32),
}

impl CombatantEventManager {
    pub fn new(associated_combatant_id: u32) -> Self {
        CombatantEventManager {
            associated_combatant_id,
            action_result_queue: vec![].into(),
            animation_queue: vec![].into(),
            floating_numbers_queue: vec![].into(),
            visual_location: CombatantVisualLocation::HomePosition,
        }
    }
}

#[derive(Default, PartialEq, Clone)]
pub struct ActionResultsManager {
    pub turn_results_queue: VecDeque<CombatTurnResult>,
    pub combantant_event_managers: HashMap<u32, CombatantEventManager>,
}

impl ActionResultsManager {
    pub fn new() -> Self {
        ActionResultsManager {
            turn_results_queue: vec![].into(),
            combantant_event_managers: HashMap::new(),
        }
    }
}

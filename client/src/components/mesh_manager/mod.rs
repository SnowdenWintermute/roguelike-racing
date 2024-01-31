#![allow(unused)]
use common::combat::ActionResult;
use common::combat::CombatTurnResult;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Display;
use yew::AttrValue;

#[derive(PartialEq, Clone, Debug)]
pub enum AutoinjectorTypes {
    Hp,
}

impl Display for AutoinjectorTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            AutoinjectorTypes::Hp => write!(f, "HP"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum CombatantAnimation {
    TurnToFaceCombatant(u32),
    ApproachCombatant(u32),
    SwingMainHandToHit(u32, Option<i16>, bool),
    SwingOffHandToHit,
    MainHandFollowThroughSwing,
    OffHandFollowThroughSwing,
    ReturnToReadyPosition,
    HitRecovery(i16),
    Death(Option<i16>),
    Evasion,
    UseAutoinjector(AutoinjectorTypes, i16),
}

impl Display for CombatantAnimation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let to_write = match self {
            CombatantAnimation::TurnToFaceCombatant(id) => format!("turing to face {}", id),
            CombatantAnimation::ApproachCombatant(id) => format!("approaching combatant {}", id),
            CombatantAnimation::SwingMainHandToHit(id, hp_change, evaded) => {
                format!(
                    "swinging main hand to hit {:?} for {:?} evaded: {evaded}",
                    id, hp_change
                )
            }
            CombatantAnimation::SwingOffHandToHit => format!("swung offhand to hit"),
            CombatantAnimation::MainHandFollowThroughSwing => format!("main hand follow through"),
            CombatantAnimation::OffHandFollowThroughSwing => format!("off hand follow through"),
            CombatantAnimation::ReturnToReadyPosition => format!("returing to ready position"),
            CombatantAnimation::HitRecovery(hp_change) => format!("hit recovery {hp_change}"),
            CombatantAnimation::Death(hp_change) => format!("death {:?}", hp_change),
            CombatantAnimation::Evasion => format!("evaded"),
            CombatantAnimation::UseAutoinjector(autoinjector_type, hp_change) => {
                format!("using autoinjector ({autoinjector_type}, {hp_change})")
            }
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

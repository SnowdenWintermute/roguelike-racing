#![allow(unused)]
use common::combat::combat_actions::CombatAction;
use common::combat::ActionResult;
use common::combat::CombatTurnResult;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Display;
use yew::AttrValue;

#[derive(PartialEq, Clone, Debug)]
pub enum AutoinjectorTypes {
    Hp,
    Mp,
}

impl Display for AutoinjectorTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            AutoinjectorTypes::Hp => write!(f, "HP"),
            AutoinjectorTypes::Mp => write!(f, "MP"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct HpChange {
    pub value: i16,
    pub is_crit: bool,
}

#[derive(PartialEq, Clone, Debug)]
pub enum HpChangeResult {
    Damaged(HpChange),
    Healed(HpChange),
    Evaded,
}

#[derive(PartialEq, Clone, Debug)]
pub struct TargetAndHpChangeResults {
    pub target_id: u32,
    pub hp_change_result: HpChangeResult,
    pub combat_action: CombatAction,
}

#[derive(PartialEq, Clone, Debug)]
pub enum CombatantAnimation {
    TurnToFaceCombatant(u32),
    ApproachCombatant(u32),
    SwingMainHandToHit(Vec<TargetAndHpChangeResults>),
    SwingOffHandToHit,
    MainHandFollowThroughSwing,
    OffHandFollowThroughSwing,
    ReturnToReadyPosition(bool), // Ends turn
    HitRecovery(i16),
    Death(Option<i16>),
    Evasion,
    UseAutoinjector(AutoinjectorTypes, u32, i16),
    MoveForwardToCastSpell(u8),
    CastSpellOnTargets(Vec<TargetAndHpChangeResults>),
}

impl Display for CombatantAnimation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let to_write = match self {
            CombatantAnimation::TurnToFaceCombatant(id) => format!("turing to face {}", id),
            CombatantAnimation::ApproachCombatant(id) => format!("approaching combatant {}", id),
            CombatantAnimation::SwingMainHandToHit(targets_and_hp_changes) => {
                format!("swinging main hand to hit",)
            }
            CombatantAnimation::SwingOffHandToHit => format!("swung offhand to hit"),
            CombatantAnimation::MainHandFollowThroughSwing => format!("main hand follow through"),
            CombatantAnimation::OffHandFollowThroughSwing => format!("off hand follow through"),
            CombatantAnimation::ReturnToReadyPosition(_ends_turn) => {
                format!("returing to ready position")
            }
            CombatantAnimation::HitRecovery(hp_change) => format!("hit recovery {hp_change}"),
            CombatantAnimation::Death(hp_change) => format!("death {:?}", hp_change),
            CombatantAnimation::Evasion => format!("evaded"),
            CombatantAnimation::UseAutoinjector(autoinjector_type, _user_id, hp_change) => {
                format!("using autoinjector ({autoinjector_type}, {hp_change})")
            }
            CombatantAnimation::CastSpellOnTargets(targets_and_hp_changes) => {
                format!("casting spell on targets")
            }
            CombatantAnimation::MoveForwardToCastSpell(_) => {
                "moving forward to cast spell".to_string()
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
    pub last_processed_action_ended_turn: bool,
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
            last_processed_action_ended_turn: false,
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

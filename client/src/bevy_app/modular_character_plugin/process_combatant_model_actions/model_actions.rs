use super::Timestamp;
use crate::frontend_common::CombatantSpecies;
use common::combatants::CombatantProperties;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum CombatantModelActions {
    ApproachDestination,
    ReturnHome,
    Recenter,
    TurnToFaceTarget,
    AttackMeleeMainHand,
    AttackMeleeOffHand,
    CastSpell,
    HitRecovery,
    Evade,
    Death,
    Idle,
}

#[derive(Debug, Clone)]
pub struct CombatantModelActionProgressTracker {
    pub time_started: Timestamp,
    pub transition_started: bool,
}

pub fn get_animation_name_from_model_action(
    species: &CombatantSpecies,
    model_action: &CombatantModelActions,
    combatant_properties: &CombatantProperties,
) -> Option<String> {
    let to_return = match species {
        CombatantSpecies::Humanoid => match model_action {
            CombatantModelActions::ApproachDestination => Some("Run"),
            CombatantModelActions::ReturnHome => Some("Run_Back"),
            CombatantModelActions::Recenter => Some("Idle"),
            CombatantModelActions::TurnToFaceTarget => Some("Run"),
            CombatantModelActions::AttackMeleeMainHand => Some("Sword_Slash"),
            CombatantModelActions::AttackMeleeOffHand => Some("Sword_Slash"),
            CombatantModelActions::HitRecovery => Some("HitRecieve"),
            CombatantModelActions::Death => Some("Death"),
            CombatantModelActions::Idle => Some("Idle_Sword"),
            CombatantModelActions::Evade => None,
            CombatantModelActions::CastSpell => None,
        },
        CombatantSpecies::Wasp => match model_action {
            CombatantModelActions::ApproachDestination
            | CombatantModelActions::ReturnHome
            | CombatantModelActions::TurnToFaceTarget
            | CombatantModelActions::Idle
            | CombatantModelActions::Evade
            | CombatantModelActions::Recenter => Some("Wasp_Flying"),
            CombatantModelActions::HitRecovery => None,
            CombatantModelActions::AttackMeleeOffHand
            | CombatantModelActions::AttackMeleeMainHand => Some("Wasp_Attack"),
            CombatantModelActions::Death => Some("Wasp_Death"),
            CombatantModelActions::CastSpell => None,
        },
        CombatantSpecies::Frog => match model_action {
            CombatantModelActions::ApproachDestination => Some("Frog_Jump"),
            CombatantModelActions::ReturnHome => Some("Frog_Jump"),
            CombatantModelActions::Recenter => Some("Frog_Idle"),
            CombatantModelActions::TurnToFaceTarget => Some("Frog_Jump"),
            CombatantModelActions::AttackMeleeMainHand => Some("Frog_Attack"),
            CombatantModelActions::AttackMeleeOffHand => Some("Frog_Attack"),
            CombatantModelActions::HitRecovery => Some("Frog_Jump"),
            CombatantModelActions::Death => Some("Frog_Death"),
            CombatantModelActions::Idle => Some("Frog_Idle"),
            CombatantModelActions::Evade => None,
            CombatantModelActions::CastSpell => None,
        },
    };
    match to_return {
        Some(str) => Some(str.to_string()),
        None => None,
    }
}

use super::Timestamp;
use common::combatants::combatant_species::CombatantSpecies;
use common::combatants::CombatantProperties;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum CombatantModelActions {
    ApproachDestination,
    ReturnHome,
    Recenter,
    TurnToFaceTarget,
    AttackMeleeMainHand,
    AttackMeleeOffHand,
    UseConsumable,
    CastSpell,
    HitRecovery,
    Evade,
    Death,
    Idle,
    EndTurn,
}

#[derive(Debug, Clone)]
pub struct CombatantModelActionProgressTracker {
    pub time_started: Timestamp,
    pub transition_started: bool,
}

pub fn get_animation_name_from_model_action(
    species: &CombatantSpecies,
    model_action: &CombatantModelActions,
    _combatant_properties: &CombatantProperties,
) -> Option<String> {
    let to_return = match species {
        CombatantSpecies::Humanoid => match model_action {
            CombatantModelActions::ApproachDestination => Some("Run"),
            CombatantModelActions::ReturnHome => Some("Run_Back"),
            CombatantModelActions::TurnToFaceTarget => Some("Run"),
            CombatantModelActions::AttackMeleeMainHand => Some("Sword_Slash"),
            CombatantModelActions::AttackMeleeOffHand => Some("Sword_Slash"),
            CombatantModelActions::HitRecovery => Some("HitRecieve"),
            CombatantModelActions::Death => Some("Death"),
            CombatantModelActions::Idle
            | CombatantModelActions::EndTurn
            | CombatantModelActions::Recenter => Some("Idle_Sword"),
            CombatantModelActions::Evade => None,
            CombatantModelActions::CastSpell => None,
            CombatantModelActions::UseConsumable => None,
        },
        CombatantSpecies::Wasp => match model_action {
            CombatantModelActions::ApproachDestination
            | CombatantModelActions::ReturnHome
            | CombatantModelActions::TurnToFaceTarget
            | CombatantModelActions::Idle
            | CombatantModelActions::EndTurn
            | CombatantModelActions::Evade
            | CombatantModelActions::Recenter => Some("Wasp_Flying"),
            CombatantModelActions::HitRecovery => None,
            CombatantModelActions::AttackMeleeOffHand
            | CombatantModelActions::AttackMeleeMainHand => Some("Wasp_Attack"),
            CombatantModelActions::Death => Some("Wasp_Death"),
            CombatantModelActions::CastSpell => None,
            CombatantModelActions::UseConsumable => None,
        },
        CombatantSpecies::Frog => match model_action {
            CombatantModelActions::ApproachDestination => Some("Frog_Jump"),
            CombatantModelActions::ReturnHome => Some("Frog_Jump"),
            CombatantModelActions::EndTurn | CombatantModelActions::Recenter => Some("Frog_Idle"),
            CombatantModelActions::TurnToFaceTarget => Some("Frog_Jump"),
            CombatantModelActions::AttackMeleeMainHand => Some("Frog_Attack"),
            CombatantModelActions::AttackMeleeOffHand => Some("Frog_Attack"),
            CombatantModelActions::HitRecovery => Some("Frog_Jump"),
            CombatantModelActions::Death => Some("Frog_Death"),
            CombatantModelActions::Idle => Some("Frog_Idle"),
            CombatantModelActions::Evade => None,
            CombatantModelActions::CastSpell => None,
            CombatantModelActions::UseConsumable => None,
        },
        CombatantSpecies::Dragon => match model_action {
            CombatantModelActions::AttackMeleeMainHand => Some("Dragon_Attack"),
            CombatantModelActions::AttackMeleeOffHand => Some("Dragon_Attack2"),
            CombatantModelActions::UseConsumable => None,
            CombatantModelActions::CastSpell => Some("Dragon_Attack"),
            CombatantModelActions::HitRecovery => Some("Dragon_Hit"),
            CombatantModelActions::Death => Some("Dragon_Death"),
            _ => Some("Dragon_Flying"),
        },
        CombatantSpecies::Skeleton => match model_action {
            CombatantModelActions::Idle
            | CombatantModelActions::Evade
            | CombatantModelActions::EndTurn
            | CombatantModelActions::TurnToFaceTarget
            | CombatantModelActions::Recenter => Some("Skeleton_Idle"),
            CombatantModelActions::ApproachDestination | CombatantModelActions::ReturnHome => {
                Some("Skeleton_Running")
            }
            CombatantModelActions::AttackMeleeMainHand
            | CombatantModelActions::AttackMeleeOffHand => Some("Skeleton_Attack"),
            CombatantModelActions::HitRecovery => None,
            CombatantModelActions::Death => Some("Skeleton_Death"),
            _ => None,
        },
        CombatantSpecies::Velociraptor => match model_action {
            CombatantModelActions::ApproachDestination | CombatantModelActions::ReturnHome => {
                Some("Velociraptor_Run")
            }
            CombatantModelActions::Recenter
            | CombatantModelActions::TurnToFaceTarget
            | CombatantModelActions::Evade
            | CombatantModelActions::Idle
            | CombatantModelActions::EndTurn => Some("Velociraptor_Idle"),
            CombatantModelActions::AttackMeleeOffHand
            | CombatantModelActions::AttackMeleeMainHand => Some("Velociraptor_Attack"),
            CombatantModelActions::HitRecovery => Some("Velociraptor_Jump"),
            CombatantModelActions::Death => Some("Velociraptor_Death"),
            _ => None,
        },
    };
    match to_return {
        Some(str) => Some(str.to_string()),
        None => None,
    }
}

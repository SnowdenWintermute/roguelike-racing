use crate::bevy_app::modular_character_plugin::animation_manager_component::Timestamp;
use crate::frontend_common::CombatantSpecies;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum CombatantModelActions {
    ApproachMeleeTarget,
    ReturnHome,
    Recenter,
    TurnToFaceTarget,
    AttackMeleeMainHand,
    AttackMeleeOffHand,
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
) -> Option<String> {
    let to_return = match species {
        CombatantSpecies::Humanoid => match model_action {
            CombatantModelActions::ApproachMeleeTarget => Some("Run"),
            CombatantModelActions::ReturnHome => Some("Run_Back"),
            CombatantModelActions::Recenter => Some("Run"),
            CombatantModelActions::TurnToFaceTarget => Some("Run"),
            CombatantModelActions::AttackMeleeMainHand => Some("Sword_Slash"),
            CombatantModelActions::AttackMeleeOffHand => Some("Sword_Slash"),
            CombatantModelActions::HitRecovery => Some("HitRecieve"),
            CombatantModelActions::Death => Some("Death"),
            CombatantModelActions::Idle => Some("Idle_Sword"),
            CombatantModelActions::Evade => None,
        },
        CombatantSpecies::Wasp => match model_action {
            CombatantModelActions::ApproachMeleeTarget => todo!(),
            CombatantModelActions::ReturnHome => todo!(),
            CombatantModelActions::Recenter => todo!(),
            CombatantModelActions::TurnToFaceTarget => todo!(),
            CombatantModelActions::AttackMeleeMainHand => todo!(),
            CombatantModelActions::AttackMeleeOffHand => todo!(),
            CombatantModelActions::HitRecovery => todo!(),
            CombatantModelActions::Death => todo!(),
            CombatantModelActions::Idle => todo!(),
            CombatantModelActions::Evade => todo!(),
            // AnimationType::Run => "Wasp_Flying",
            // AnimationType::HitRecovery => "Wasp_Death",
            // AnimationType::Death => "Wasp_Death",
            // AnimationType::Idle => "Wasp_Flying",
            // AnimationType::Attack => "Wasp_Attack",
            // AnimationType::ReturningToHome => "Wasp_Flying",
        },
        CombatantSpecies::Frog => match model_action {
            CombatantModelActions::ApproachMeleeTarget => Some("Frog_Jump"),
            CombatantModelActions::ReturnHome => Some("Frog_Jump"),
            CombatantModelActions::Recenter => Some("Frog_Idle"),
            CombatantModelActions::TurnToFaceTarget => Some("Frog_Jump"),
            CombatantModelActions::AttackMeleeMainHand => Some("Frog_Attack"),
            CombatantModelActions::AttackMeleeOffHand => Some("Frog_Attack"),
            CombatantModelActions::HitRecovery => Some("Frog_Jump"),
            CombatantModelActions::Death => Some("Frog_Death"),
            CombatantModelActions::Idle => Some("Frog_Idle"),
            CombatantModelActions::Evade => None,
        },
    };
    match to_return {
        Some(str) => Some(str.to_string()),
        None => None,
    }
}

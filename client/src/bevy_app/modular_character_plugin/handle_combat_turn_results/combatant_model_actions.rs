use crate::bevy_app::modular_character_plugin::animation_manager_component::Timestamp;
use crate::frontend_common::animation_names::AnimationType;
use crate::frontend_common::CombatantSpecies;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum CombatantModelActions {
    ApproachMeleeTarget,
    ReturnHome,
    Recenter,
    TurnToFaceTarget,
    AttackMeleeMainHand,
    AttackMeleeOffHand,
}

pub struct CombatantModelActionProgressTracker {
    pub time_started: Timestamp,
    pub transition_started: bool,
}

pub fn get_animation_name_from_model_action(
    species: &CombatantSpecies,
    model_action: &CombatantModelActions,
) -> Option<str> {
    match species {
        CombatantSpecies::Humanoid => match model_action {
            CombatantModelActions::ApproachMeleeTarget => Some("Run"),
            CombatantModelActions::ReturnHome => todo!(),
            CombatantModelActions::Recenter => todo!(),
            CombatantModelActions::TurnToFaceTarget => todo!(),
            CombatantModelActions::AttackMeleeMainHand => todo!(),
            CombatantModelActions::AttackMeleeOffHand => todo!(),
            // AnimationType::Run => "Run",
            // AnimationType::HitRecovery => "HitRecieve",
            // AnimationType::Death => "Death",
            // AnimationType::Idle => "Idle_Sword",
            // AnimationType::Attack => "Sword_Slash",
            // AnimationType::ReturningToHome => "Run_Back",
        },
        CombatantSpecies::Wasp => match model_action {
            CombatantModelActions::ApproachMeleeTarget => todo!(),
            CombatantModelActions::ReturnHome => todo!(),
            CombatantModelActions::Recenter => todo!(),
            CombatantModelActions::TurnToFaceTarget => todo!(),
            CombatantModelActions::AttackMeleeMainHand => todo!(),
            CombatantModelActions::AttackMeleeOffHand => todo!(),
            // AnimationType::Run => "Wasp_Flying",
            // AnimationType::HitRecovery => "Wasp_Death",
            // AnimationType::Death => "Wasp_Death",
            // AnimationType::Idle => "Wasp_Flying",
            // AnimationType::Attack => "Wasp_Attack",
            // AnimationType::ReturningToHome => "Wasp_Flying",
        },
        CombatantSpecies::Frog => match model_action {
            CombatantModelActions::ApproachMeleeTarget => todo!(),
            CombatantModelActions::ReturnHome => todo!(),
            CombatantModelActions::Recenter => todo!(),
            CombatantModelActions::TurnToFaceTarget => todo!(),
            CombatantModelActions::AttackMeleeMainHand => todo!(),
            CombatantModelActions::AttackMeleeOffHand => todo!(),
            // AnimationType::Run => "Frog_Jump",
            // AnimationType::HitRecovery => "Frog_Jump",
            // AnimationType::Death => "Frog_Death",
            // AnimationType::Idle => "Frog_Idle",
            // AnimationType::Attack => "Frog_Attack",
            // AnimationType::ReturningToHome => "Frog_Jump",
        },
    }

    // to_return.to_string()
}

impl CombatantModelActions {
    pub fn get_associated_animation_name(&self, species: CombatantSpecies) -> String {
        match self {
            CombatantModelActions::ApproachMeleeTarget => match species {
                CombatantSpecies::Humanoid => todo!(),
                CombatantSpecies::Wasp => todo!(),
                CombatantSpecies::Frog => todo!(),
            },
            CombatantModelActions::ReturnHome => todo!(),
            CombatantModelActions::Recenter => todo!(),
            CombatantModelActions::TurnToFaceTarget => todo!(),
            CombatantModelActions::AttackMeleeMainHand => todo!(),
            CombatantModelActions::AttackMeleeOffHand => todo!(),
        }
    }
}

use super::CombatantSpecies;

pub enum AnimationType {
    Run,
    ReturningToHome,
    HitRecovery,
    Death,
    Idle,
    Attack,
}

pub trait CombatantAnimations {
    fn animation_name(&self, animation_type: AnimationType) -> String;
}

impl CombatantAnimations for CombatantSpecies {
    fn animation_name(&self, animation_type: AnimationType) -> String {
        let to_return = match self {
            CombatantSpecies::Humanoid => match animation_type {
                AnimationType::Run => "Run",
                AnimationType::HitRecovery => "HitRecieve",
                AnimationType::Death => "Death",
                AnimationType::Idle => "Idle_Sword",
                AnimationType::Attack => "Sword_Slash",
                AnimationType::ReturningToHome => "Run_Back",
            },
            CombatantSpecies::Wasp => match animation_type {
                AnimationType::Run => "Wasp_Flying",
                AnimationType::HitRecovery => "Wasp_Death",
                AnimationType::Death => "Wasp_Death",
                AnimationType::Idle => "Wasp_Flying",
                AnimationType::Attack => "Wasp_Attack",
                AnimationType::ReturningToHome => "Wasp_Flying",
            },
            CombatantSpecies::Frog => match animation_type {
                AnimationType::Run => "Frog_Jump",
                AnimationType::HitRecovery => "Frog_Jump",
                AnimationType::Death => "Frog_Death",
                AnimationType::Idle => "Frog_Idle",
                AnimationType::Attack => "Frog_Attack",
                AnimationType::ReturningToHome => "Frog_Jump",
            },
        };

        to_return.to_string()
    }
}

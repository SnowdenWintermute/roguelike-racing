use crate::bevy_app::modular_character_plugin::animation_manager_component::Timestamp;

pub enum CombatantModelActions {
    ApproachMeleeTarget,
    ReturnHome,
    Recenter,
    TurnToFaceTarget,
    AttackMeleeMainHand,
    AttackMeleeOffHand,
}

pub struct CombatantModelActionProgressTracker {
    time_started: Option<Timestamp>,
    transition_started: bool,
}

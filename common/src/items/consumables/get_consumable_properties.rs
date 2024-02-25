use super::ConsumableTypes;
use crate::combat::combat_actions::AbilityUsableContext;
use crate::combat::combat_actions::CombatActionProperties;
use crate::combat::combat_actions::ProhibitedTargetCombatantStates;
use crate::combat::combat_actions::TargetCategories;
use crate::combat::combat_actions::TargetingScheme;

impl ConsumableTypes {
    pub fn get_combat_action_properties(&self) -> CombatActionProperties {
        match self {
            ConsumableTypes::HpAutoinjector => CombatActionProperties {
                targeting_schemes: vec![TargetingScheme::Single],
                valid_target_categories: TargetCategories::Friendly,
                usability_context: AbilityUsableContext::All,
                prohibited_target_combatant_states: Some(vec![
                    ProhibitedTargetCombatantStates::Dead,
                ]),
                requires_combat_turn: false,
                hp_change_properties: None,
                description: "Heal a friendly target".to_string(),
            },
            ConsumableTypes::MpAutoinjector => CombatActionProperties {
                targeting_schemes: vec![TargetingScheme::Single],
                valid_target_categories: TargetCategories::Friendly,
                usability_context: AbilityUsableContext::All,
                prohibited_target_combatant_states: Some(vec![
                    ProhibitedTargetCombatantStates::Dead,
                ]),
                requires_combat_turn: false,
                hp_change_properties: None,
                description: "Restore MP to a friendly target".to_string(),
            },
            ConsumableTypes::Grenade => CombatActionProperties {
                targeting_schemes: vec![TargetingScheme::Area],
                valid_target_categories: TargetCategories::Opponent,
                usability_context: AbilityUsableContext::InCombat,
                prohibited_target_combatant_states: None,
                requires_combat_turn: true,
                hp_change_properties: None,
                ..Default::default()
            },
            ConsumableTypes::SmokeBomb => CombatActionProperties {
                targeting_schemes: vec![TargetingScheme::Area],
                valid_target_categories: TargetCategories::Friendly,
                usability_context: AbilityUsableContext::InCombat,
                prohibited_target_combatant_states: None,
                requires_combat_turn: false,
                hp_change_properties: None,
                ..Default::default()
            },
            // _ => CombatActionProperties::default(),
        }
    }
}

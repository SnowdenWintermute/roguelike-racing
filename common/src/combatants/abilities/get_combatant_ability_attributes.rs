use super::CombatantAbilityNames;
use crate::combat::combat_actions::CombatActionProperties;
use crate::combat::combat_actions::ProhibitedTargetCombatantStates;
use crate::combat::combat_actions::TargetingScheme;

pub struct CombatantAbilityAttributes {
    pub combat_action_properties: CombatActionProperties,
    pub is_melee: bool,
    pub mana_cost: u8,
    pub mana_cost_level_multiplier: u8,
    pub shard_cost: u8,
}

impl Default for CombatantAbilityAttributes {
    fn default() -> Self {
        CombatantAbilityAttributes {
            combat_action_properties: CombatActionProperties::default(),
            is_melee: false,
            mana_cost: 1,
            mana_cost_level_multiplier: 1,
            shard_cost: 0,
        }
    }
}

impl CombatantAbilityNames {
    pub fn get_attributes(&self) -> CombatantAbilityAttributes {
        match self {
            CombatantAbilityNames::Attack => CombatantAbilityAttributes {
                mana_cost: 0,
                is_melee: true,
                combat_action_properties: CombatActionProperties {
                    prohibited_target_combatant_states: Some(vec![
                        ProhibitedTargetCombatantStates::Dead,
                    ]),
                    ..Default::default()
                },
                ..Default::default()
            },
            CombatantAbilityNames::ArmorBreak => CombatantAbilityAttributes {
                is_melee: true,
                ..Default::default()
            },
            CombatantAbilityNames::HeatLance => CombatantAbilityAttributes {
                ..Default::default()
            },
            CombatantAbilityNames::Fire => CombatantAbilityAttributes {
                combat_action_properties: CombatActionProperties {
                    targeting_schemes: vec![TargetingScheme::Area, TargetingScheme::Single],
                    ..Default::default()
                },
                ..Default::default()
            },
            CombatantAbilityNames::Heal => CombatantAbilityAttributes {
                ..Default::default()
            },
            CombatantAbilityNames::RainStorm => CombatantAbilityAttributes {
                ..Default::default()
            },
        }
    }
}

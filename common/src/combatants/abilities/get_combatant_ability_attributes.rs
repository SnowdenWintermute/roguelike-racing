use super::CombatantAbilityNames;
use crate::combat::combat_actions::CombatActionHpChangeProperties;
use crate::combat::combat_actions::CombatActionProperties;
use crate::combat::combat_actions::ProhibitedTargetCombatantStates;
use crate::combat::combat_actions::TargetCategories;
use crate::combat::combat_actions::TargetingScheme;
use crate::combat::hp_change_source_types::Evadable;
use crate::combat::hp_change_source_types::HpChangeSource;
use crate::combat::hp_change_source_types::HpChangeSourceCategories;
use crate::combat::magical_elements::MagicalElements;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::primatives::Range;

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
                mana_cost: 2,
                mana_cost_level_multiplier: 1,
                combat_action_properties: CombatActionProperties {
                    targeting_schemes: vec![TargetingScheme::Area, TargetingScheme::Single],
                    hp_change_properties: Some(CombatActionHpChangeProperties {
                        base_values: Range::new(8, 15),
                        base_final_percent_multiplier: 100,
                        additive_attribute_and_scaling_factor: Some((
                            CombatAttributes::Intelligence,
                            1,
                        )),
                        source_properties: HpChangeSource::new(
                            HpChangeSourceCategories::MagicalDamage(Evadable::new(false)),
                            None,
                            Some(MagicalElements::Ice),
                        ),
                        add_weapon_damage_from: None,
                        crit_chance_attribute: Some(CombatAttributes::Focus),
                        crit_multiplier_attribute: Some(CombatAttributes::Focus),
                    }),
                    valid_target_categories: TargetCategories::Any,
                    prohibited_target_combatant_states: Some(vec![
                        ProhibitedTargetCombatantStates::Dead,
                    ]),
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

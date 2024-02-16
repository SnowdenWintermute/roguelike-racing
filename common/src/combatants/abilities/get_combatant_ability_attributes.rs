use super::CombatantAbilityNames;
use crate::app_consts::OFF_HAND_ACCURACY_MODIFIER;
use crate::app_consts::OFF_HAND_DAMAGE_MODIFIER;
use crate::combat::combat_actions::AbilityUsableContext;
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
use crate::primatives::WeaponSlot;

pub struct CombatantAbilityAttributes {
    pub combat_action_properties: CombatActionProperties,
    pub is_melee: bool,
    pub mana_cost: u8,
    pub mana_cost_level_multiplier: u8,
    pub base_hp_change_values_level_multiplier: f32,
    pub shard_cost: u8,
}

impl Default for CombatantAbilityAttributes {
    fn default() -> Self {
        CombatantAbilityAttributes {
            combat_action_properties: CombatActionProperties::default(),
            is_melee: false,
            mana_cost: 1,
            mana_cost_level_multiplier: 1,
            base_hp_change_values_level_multiplier: 1.0,
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
            CombatantAbilityNames::AttackMeleeMainhand => CombatantAbilityAttributes {
                combat_action_properties: CombatActionProperties {
                    prohibited_target_combatant_states: Some(vec![
                        ProhibitedTargetCombatantStates::Dead,
                    ]),
                    hp_change_properties: Some(CombatActionHpChangeProperties {
                        base_values: Range::new(1, 1),
                        add_weapon_damage_from: Some(vec![WeaponSlot::MainHand]),
                        additive_attribute_and_percent_scaling_factor: Some((
                            CombatAttributes::Strength,
                            100,
                        )),
                        crit_chance_attribute: Some(CombatAttributes::Dexterity),
                        crit_multiplier_attribute: Some(CombatAttributes::Strength),
                        source_properties: HpChangeSource {
                            category: HpChangeSourceCategories::PhysicalDamage,
                            sub_category: None,
                            element: None,
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                is_melee: true,
                base_hp_change_values_level_multiplier: 1.0,
                ..Default::default()
            },
            CombatantAbilityNames::AttackMeleeOffhand => CombatantAbilityAttributes {
                combat_action_properties: CombatActionProperties {
                    prohibited_target_combatant_states: Some(vec![
                        ProhibitedTargetCombatantStates::Dead,
                    ]),
                    hp_change_properties: Some(CombatActionHpChangeProperties {
                        base_values: Range::new(1, 1),
                        final_damage_percent_multiplier: OFF_HAND_DAMAGE_MODIFIER,
                        accuracy_percent_modifier: OFF_HAND_ACCURACY_MODIFIER,
                        add_weapon_damage_from: Some(vec![WeaponSlot::MainHand]),
                        additive_attribute_and_percent_scaling_factor: Some((
                            CombatAttributes::Strength,
                            100,
                        )),
                        crit_chance_attribute: Some(CombatAttributes::Dexterity),
                        crit_multiplier_attribute: Some(CombatAttributes::Strength),
                        source_properties: HpChangeSource {
                            category: HpChangeSourceCategories::PhysicalDamage,
                            sub_category: None,
                            element: None,
                        },
                    }),
                    ..Default::default()
                },
                is_melee: true,
                base_hp_change_values_level_multiplier: 1.0,
                ..Default::default()
            },
            CombatantAbilityNames::AttackRangedMainhand => CombatantAbilityAttributes {
                combat_action_properties: CombatActionProperties {
                    prohibited_target_combatant_states: Some(vec![
                        ProhibitedTargetCombatantStates::Dead,
                    ]),
                    hp_change_properties: Some(CombatActionHpChangeProperties {
                        base_values: Range::new(1, 1),
                        add_weapon_damage_from: Some(vec![WeaponSlot::MainHand]),
                        additive_attribute_and_percent_scaling_factor: Some((
                            CombatAttributes::Dexterity,
                            100,
                        )),
                        crit_chance_attribute: Some(CombatAttributes::Dexterity),
                        crit_multiplier_attribute: Some(CombatAttributes::Dexterity),
                        source_properties: HpChangeSource {
                            category: HpChangeSourceCategories::PhysicalDamage,
                            sub_category: None,
                            element: None,
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                is_melee: true,
                base_hp_change_values_level_multiplier: 1.0,
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
                        additive_attribute_and_percent_scaling_factor: Some((
                            CombatAttributes::Intelligence,
                            100,
                        )),
                        source_properties: HpChangeSource::new(
                            HpChangeSourceCategories::MagicalDamage(Evadable::new(false)),
                            None,
                            Some(MagicalElements::Fire),
                        ),
                        add_weapon_damage_from: None,
                        crit_chance_attribute: Some(CombatAttributes::Focus),
                        crit_multiplier_attribute: Some(CombatAttributes::Focus),
                        ..Default::default()
                    }),
                    valid_target_categories: TargetCategories::Opponent,
                    prohibited_target_combatant_states: Some(vec![
                        ProhibitedTargetCombatantStates::Dead,
                    ]),
                    ..Default::default()
                },
                ..Default::default()
            },
            CombatantAbilityNames::Healing => CombatantAbilityAttributes {
                mana_cost: 2,
                mana_cost_level_multiplier: 1,
                combat_action_properties: CombatActionProperties {
                    targeting_schemes: vec![TargetingScheme::Single, TargetingScheme::Area],
                    hp_change_properties: Some(CombatActionHpChangeProperties {
                        base_values: Range::new(6, 12),
                        additive_attribute_and_percent_scaling_factor: Some((
                            CombatAttributes::Intelligence,
                            100,
                        )),
                        source_properties: HpChangeSource::new(
                            HpChangeSourceCategories::Healing,
                            None,
                            Some(MagicalElements::Light),
                        ),
                        add_weapon_damage_from: None,
                        crit_chance_attribute: Some(CombatAttributes::Focus),
                        crit_multiplier_attribute: Some(CombatAttributes::Focus),
                        ..Default::default()
                    }),
                    valid_target_categories: TargetCategories::Any,
                    prohibited_target_combatant_states: Some(vec![
                        ProhibitedTargetCombatantStates::Dead,
                    ]),
                    usability_context: AbilityUsableContext::All,
                    ..Default::default()
                },
                ..Default::default()
            },
            CombatantAbilityNames::RainStorm => CombatantAbilityAttributes {
                ..Default::default()
            },
        }
    }
}

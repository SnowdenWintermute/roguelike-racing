pub mod filter_possible_target_ids_by_prohibited_combatant_states;
pub mod get_default_targets;
pub mod get_next_or_previous_targets;
mod get_targets;
pub mod targets_are_valid;
pub mod targets_by_saved_preference_or_default;
pub mod validate_use;
use super::hp_change_source_types::HpChangeSource;
use crate::app_consts::error_messages;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::errors::AppError;
use crate::errors::AppErrorTypes;
use crate::primatives::Range;
use crate::primatives::WeaponSlot;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FriendOrFoe {
    Friendly,
    Hostile,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CombatActionTarget {
    Single(u32),
    Group(FriendOrFoe),
    All,
}

impl CombatActionTarget {
    pub fn get_single_target_id(&self) -> Result<u32, AppError> {
        match self {
            CombatActionTarget::Single(id) => Ok(*id),
            _ => Err(AppError {
                error_type: AppErrorTypes::Generic,
                message: error_messages::INVALID_TARGETING_SCHEME.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TargetingScheme {
    Single,
    Area,
    All,
}

impl Display for TargetingScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_write = match self {
            TargetingScheme::Single => "Single",
            TargetingScheme::Area => "Area",
            TargetingScheme::All => "All",
        };
        write!(f, "{}", to_write)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TargetCategories {
    Opponent,
    User,
    Friendly,
    Any,
}

impl Display for TargetCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_write = match self {
            TargetCategories::Opponent => "Enemy",
            TargetCategories::User => "Self",
            TargetCategories::Friendly => "Friendly",
            TargetCategories::Any => "Any",
        };
        write!(f, "{}", to_write)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProhibitedTargetCombatantStates {
    Dead,
    Alive,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AbilityUsableContext {
    All,
    InCombat,
    OutOfCombat,
}

impl Display for AbilityUsableContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_write = match self {
            AbilityUsableContext::All => "any time",
            AbilityUsableContext::InCombat => "in combat",
            AbilityUsableContext::OutOfCombat => "out of combat",
        };
        write!(f, "{}", to_write)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CombatAction {
    AbilityUsed(CombatantAbilityNames),
    ConsumableUsed(u32),
}

pub struct CombatActionProperties {
    pub targeting_schemes: Vec<TargetingScheme>,
    pub valid_target_categories: TargetCategories,
    pub usability_context: AbilityUsableContext,
    pub prohibited_target_combatant_states: Option<Vec<ProhibitedTargetCombatantStates>>,
    pub requires_combat_turn: bool,
    pub hp_change_properties: Option<CombatActionHpChangeProperties>,
}

impl Default for CombatActionProperties {
    fn default() -> Self {
        CombatActionProperties {
            targeting_schemes: vec![TargetingScheme::Single],
            valid_target_categories: TargetCategories::Opponent,
            usability_context: AbilityUsableContext::InCombat,
            prohibited_target_combatant_states: None,
            requires_combat_turn: true,
            hp_change_properties: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CombatActionHpChangeProperties {
    pub base_values: Range<u16>,
    pub base_final_percent_multiplier: u8,
    pub add_weapon_damage_from: Option<Vec<WeaponSlot>>,
    pub additive_attribute_and_scaling_factor: Option<(CombatAttributes, u8)>,
    pub crit_chance_attribute: Option<CombatAttributes>,
    pub crit_multiplier_attribute: Option<CombatAttributes>,
    pub source_properties: HpChangeSource,
}

impl Default for CombatActionHpChangeProperties {
    fn default() -> Self {
        CombatActionHpChangeProperties {
            base_values: Range::new(0, 0),
            base_final_percent_multiplier: 1,
            add_weapon_damage_from: None,
            additive_attribute_and_scaling_factor: None,
            crit_chance_attribute: None,
            crit_multiplier_attribute: None,
            source_properties: HpChangeSource::default(),
        }
    }
}

impl Display for CombatAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let to_write = match self {
            CombatAction::AbilityUsed(ability_name) => format!("ability {ability_name}"),
            CombatAction::ConsumableUsed(consumable_id) => {
                format!("consumable used {consumable_id}")
            }
        };
        write!(f, "{to_write}")
    }
}

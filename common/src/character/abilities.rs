use crate::character::combatant_properties::CombatantClass;

#[derive(Debug)]
pub enum TargetingScheme {
    Single,
    Area,
    CentralizedArea,
}

#[derive(Debug)]
pub enum ValidTargets {
    Opponent,
    AllyOrSelf,
    Any,
}

#[derive(Debug)]
pub struct CombatantAbility {
    pub ability_type: CombatantAbilities,
    pub class: Option<CombatantClass>,
    pub level: u8,
    pub mana_cost: u8,
    pub mana_cost_level_multiplier: u8,
    pub shard_cost: u8,
    pub requires_combat_turn: bool,
    pub combat_use_only: bool,
    pub targeting_schemes: Vec<TargetingScheme>,
    pub valid_targets: ValidTargets,
}

impl Default for CombatantAbility {
    fn default() -> CombatantAbility {
        CombatantAbility {
            ability_type: CombatantAbilities::Attack,
            class: None,
            level: 0,
            mana_cost: 0,
            mana_cost_level_multiplier: 1,
            shard_cost: 0,
            requires_combat_turn: true,
            combat_use_only: true,
            targeting_schemes: vec![TargetingScheme::Single],
            valid_targets: ValidTargets::Opponent,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum CombatantAbilities {
    Attack,
    HeatLance,
    ArmorBreak,
    ShootArrow,
}

impl CombatantAbilities {
    pub fn new(&self) -> CombatantAbility {
        match self {
            CombatantAbilities::Attack => CombatantAbility {
                ability_type: CombatantAbilities::Attack,
                class: None,
                level: 1,
                ..Default::default()
            },
            CombatantAbilities::HeatLance => CombatantAbility {
                ability_type: CombatantAbilities::HeatLance,
                class: Some(CombatantClass::Mage),
                mana_cost: 1,
                ..Default::default()
            },
            CombatantAbilities::ArmorBreak => CombatantAbility {
                ability_type: CombatantAbilities::ArmorBreak,
                class: Some(CombatantClass::Warrior),
                mana_cost: 1,
                ..Default::default()
            },
            CombatantAbilities::ShootArrow => CombatantAbility {
                ability_type: CombatantAbilities::ShootArrow,
                class: Some(CombatantClass::Rogue),
                shard_cost: 1,
                ..Default::default()
            },
        }
    }
}

use crate::character::CharacterClasses;

#[derive(Debug)]
pub struct CharacterAbility {
    pub ability_type: CharacterAbilities,
    pub class: Option<CharacterClasses>,
    pub level: u8,
    pub mana_cost: u8,
    pub mana_cost_level_multiplier: u8,
    pub shard_cost: u8,
    pub requires_combat_turn: bool,
    pub combat_use_only: bool,
}

impl Default for CharacterAbility {
    fn default() -> CharacterAbility {
        CharacterAbility {
            ability_type: CharacterAbilities::Attack,
            class: None,
            level: 0,
            mana_cost: 0,
            mana_cost_level_multiplier: 1,
            shard_cost: 0,
            requires_combat_turn: true,
            combat_use_only: true,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum CharacterAbilities {
    Attack,
    HeatLance,
    ArmorBreak,
    ShootArrow,
}

impl CharacterAbilities {
    pub fn new(&self) -> CharacterAbility {
        match self {
            CharacterAbilities::Attack => CharacterAbility {
                ability_type: CharacterAbilities::Attack,
                class: None,
                level: 1,
                ..Default::default()
            },
            CharacterAbilities::HeatLance => CharacterAbility {
                ability_type: CharacterAbilities::HeatLance,
                class: Some(CharacterClasses::Mage),
                mana_cost: 1,
                ..Default::default()
            },
            CharacterAbilities::ArmorBreak => CharacterAbility {
                ability_type: CharacterAbilities::ArmorBreak,
                class: Some(CharacterClasses::Warrior),
                mana_cost: 1,
                ..Default::default()
            },
            CharacterAbilities::ShootArrow => CharacterAbility {
                ability_type: CharacterAbilities::ShootArrow,
                class: Some(CharacterClasses::Rogue),
                shard_cost: 1,
                ..Default::default()
            },
        }
    }
}
